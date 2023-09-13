//! Source implementation for Postgres database, including the TLS support (client only).

mod connection;
mod errors;
mod typesystem;

pub use self::errors::PostgresSourceError;
pub use connection::rewrite_tls_args;
pub use typesystem::{PostgresTypePairs, PostgresTypeSystem};

use crate::constants::DB_BUFFER_SIZE;
use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::{PartitionParser, Produce, Source, SourcePartition},
    sql::{count_query, CXQuery},
};
use anyhow::anyhow;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use csv::{ReaderBuilder, StringRecord, StringRecordsIntoIter};
use fehler::{throw, throws};
use hex::decode;
use postgres::{
    binary_copy::{BinaryCopyOutIter, BinaryCopyOutRow},
    fallible_iterator::FallibleIterator,
    tls::{MakeTlsConnect, TlsConnect},
    Config, CopyOutReader, Row, RowIter, SimpleQueryMessage, Socket,
};
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use rust_decimal::Decimal;
use serde_json::{from_str, Value};
use sqlparser::dialect::PostgreSqlDialect;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::marker::PhantomData;
use uuid::Uuid;

/// Protocol - Binary based bulk load
pub enum BinaryProtocol {}

/// Protocol - CSV based bulk load
pub enum CSVProtocol {}

/// Protocol - use Cursor
pub enum CursorProtocol {}

/// Protocol - use Simple Query
pub enum SimpleProtocol {}

type PgManager<C> = PostgresConnectionManager<C>;
type PgConn<C> = PooledConnection<PgManager<C>>;

// take a row and unwrap the interior field from column 0
fn convert_row<'b, R: TryFrom<usize> + postgres::types::FromSql<'b> + Clone>(row: &'b Row) -> R {
    let nrows: Option<R> = row.get(0);
    nrows.expect("Could not parse int result from count_query")
}

#[throws(PostgresSourceError)]
fn get_total_rows<C>(conn: &mut PgConn<C>, query: &CXQuery<String>) -> usize
where
    C: MakeTlsConnect<Socket> + Clone + 'static + Sync + Send,
    C::TlsConnect: Send,
    C::Stream: Send,
    <C::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    let dialect = PostgreSqlDialect {};

    let row = conn.query_one(count_query(query, &dialect)?.as_str(), &[])?;
    let col_type = PostgresTypeSystem::from(row.columns()[0].type_());
    match col_type {
        PostgresTypeSystem::Int2(_) => convert_row::<i16>(&row) as usize,
        PostgresTypeSystem::Int4(_) => convert_row::<i32>(&row) as usize,
        PostgresTypeSystem::Int8(_) => convert_row::<i64>(&row) as usize,
        _ => throw!(anyhow!(
            "The result of the count query was not an int, aborting."
        )),
    }
}

pub struct PostgresSource<P, C>
where
    C: MakeTlsConnect<Socket> + Clone + 'static + Sync + Send,
    C::TlsConnect: Send,
    C::Stream: Send,
    <C::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    pool: Pool<PgManager<C>>,
    origin_query: Option<String>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<PostgresTypeSystem>,
    pg_schema: Vec<postgres::types::Type>,
    _protocol: PhantomData<P>,
}

impl<P, C> PostgresSource<P, C>
where
    C: MakeTlsConnect<Socket> + Clone + 'static + Sync + Send,
    C::TlsConnect: Send,
    C::Stream: Send,
    <C::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    #[throws(PostgresSourceError)]
    pub fn new(config: Config, tls: C, nconn: usize) -> Self {
        let manager = PostgresConnectionManager::new(config, tls);
        let pool = Pool::builder().max_size(nconn as u32).build(manager)?;

        Self {
            pool,
            origin_query: None,
            queries: vec![],
            names: vec![],
            schema: vec![],
            pg_schema: vec![],
            _protocol: PhantomData,
        }
    }
}

impl<P, C> Source for PostgresSource<P, C>
where
    PostgresSourcePartition<P, C>:
        SourcePartition<TypeSystem = PostgresTypeSystem, Error = PostgresSourceError>,
    P: Send,
    C: MakeTlsConnect<Socket> + Clone + 'static + Sync + Send,
    C::TlsConnect: Send,
    C::Stream: Send,
    <C::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = PostgresSourcePartition<P, C>;
    type TypeSystem = PostgresTypeSystem;
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorXError::UnsupportedDataOrder(data_order));
        }
    }

    fn set_queries<Q: ToString>(&mut self, queries: &[CXQuery<Q>]) {
        self.queries = queries.iter().map(|q| q.map(Q::to_string)).collect();
    }

    fn set_origin_query(&mut self, query: Option<String>) {
        self.origin_query = query;
    }

    #[throws(PostgresSourceError)]
    fn fetch_metadata(&mut self) {
        assert!(!self.queries.is_empty());

        let mut conn = self.pool.get()?;
        let first_query = &self.queries[0];

        let stmt = conn.prepare(first_query.as_str())?;

        let (names, pg_types): (Vec<String>, Vec<postgres::types::Type>) = stmt
            .columns()
            .iter()
            .map(|col| (col.name().to_string(), col.type_().clone()))
            .unzip();

        self.names = names;
        self.schema = pg_types
            .iter()
            .map(PostgresTypeSystem::from)
            .collect();
        self.pg_schema = self
            .schema
            .iter()
            .zip(pg_types.iter())
            .map(|(t1, t2)| PostgresTypePairs(t2, t1).into())
            .collect();
    }

    #[throws(PostgresSourceError)]
    fn result_rows(&mut self) -> Option<usize> {
        match &self.origin_query {
            Some(q) => {
                let cxq = CXQuery::Naked(q.clone());
                let mut conn = self.pool.get()?;
                let nrows = get_total_rows(&mut conn, &cxq)?;
                Some(nrows)
            }
            None => None,
        }
    }

    fn names(&self) -> Vec<String> {
        self.names.clone()
    }

    fn schema(&self) -> Vec<Self::TypeSystem> {
        self.schema.clone()
    }

    #[throws(PostgresSourceError)]
    fn partition(self) -> Vec<Self::Partition> {
        let mut ret = vec![];
        for query in self.queries {
            let conn = self.pool.get()?;

            ret.push(PostgresSourcePartition::<P, C>::new(
                conn,
                &query,
                &self.schema,
                &self.pg_schema,
            ));
        }
        ret
    }
}

pub struct PostgresSourcePartition<P, C>
where
    C: MakeTlsConnect<Socket> + Clone + 'static + Sync + Send,
    C::TlsConnect: Send,
    C::Stream: Send,
    <C::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    conn: PgConn<C>,
    query: CXQuery<String>,
    schema: Vec<PostgresTypeSystem>,
    pg_schema: Vec<postgres::types::Type>,
    nrows: usize,
    ncols: usize,
    _protocol: PhantomData<P>,
}

impl<P, C> PostgresSourcePartition<P, C>
where
    C: MakeTlsConnect<Socket> + Clone + 'static + Sync + Send,
    C::TlsConnect: Send,
    C::Stream: Send,
    <C::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    pub fn new(
        conn: PgConn<C>,
        query: &CXQuery<String>,
        schema: &[PostgresTypeSystem],
        pg_schema: &[postgres::types::Type],
    ) -> Self {
        Self {
            conn,
            query: query.clone(),
            schema: schema.to_vec(),
            pg_schema: pg_schema.to_vec(),
            nrows: 0,
            ncols: schema.len(),
            _protocol: PhantomData,
        }
    }
}

impl<C> SourcePartition for PostgresSourcePartition<BinaryProtocol, C>
where
    C: MakeTlsConnect<Socket> + Clone + 'static + Sync + Send,
    C::TlsConnect: Send,
    C::Stream: Send,
    <C::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    type TypeSystem = PostgresTypeSystem;
    type Parser<'a> = PostgresBinarySourcePartitionParser<'a>;
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn result_rows(&mut self) -> () {
        self.nrows = get_total_rows(&mut self.conn, &self.query)?;
    }

    #[throws(PostgresSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        let query = format!("COPY ({}) TO STDOUT WITH BINARY", self.query);
        let reader = self.conn.copy_out(&*query)?; // unless reading the data, it seems like issue the query is fast
        let iter = BinaryCopyOutIter::new(reader, &self.pg_schema);

        PostgresBinarySourcePartitionParser::new(iter, &self.schema)
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

impl<C> SourcePartition for PostgresSourcePartition<CSVProtocol, C>
where
    C: MakeTlsConnect<Socket> + Clone + 'static + Sync + Send,
    C::TlsConnect: Send,
    C::Stream: Send,
    <C::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    type TypeSystem = PostgresTypeSystem;
    type Parser<'a> = PostgresCSVSourceParser<'a>;
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn result_rows(&mut self) {
        self.nrows = get_total_rows(&mut self.conn, &self.query)?;
    }

    #[throws(PostgresSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        let query = format!("COPY ({}) TO STDOUT WITH CSV", self.query);
        let reader = self.conn.copy_out(&*query)?; // unless reading the data, it seems like issue the query is fast
        let iter = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(reader)
            .into_records();

        PostgresCSVSourceParser::new(iter, &self.schema)
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

impl<C> SourcePartition for PostgresSourcePartition<CursorProtocol, C>
where
    C: MakeTlsConnect<Socket> + Clone + 'static + Sync + Send,
    C::TlsConnect: Send,
    C::Stream: Send,
    <C::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    type TypeSystem = PostgresTypeSystem;
    type Parser<'a> = PostgresRawSourceParser<'a>;
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn result_rows(&mut self) {
        self.nrows = get_total_rows(&mut self.conn, &self.query)?;
    }

    #[throws(PostgresSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        let iter = self
            .conn
            .query_raw::<_, bool, _>(self.query.as_str(), vec![])?; // unless reading the data, it seems like issue the query is fast
        PostgresRawSourceParser::new(iter, &self.schema)
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}
pub struct PostgresBinarySourcePartitionParser<'a> {
    iter: BinaryCopyOutIter<'a>,
    rowbuf: Vec<BinaryCopyOutRow>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
    is_finished: bool,
}

impl<'a> PostgresBinarySourcePartitionParser<'a> {
    pub fn new(iter: BinaryCopyOutIter<'a>, schema: &[PostgresTypeSystem]) -> Self {
        Self {
            iter,
            rowbuf: Vec::with_capacity(DB_BUFFER_SIZE),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
            is_finished: false,
        }
    }

    #[throws(PostgresSourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        ret
    }
}

impl<'a> PartitionParser<'a> for PostgresBinarySourcePartitionParser<'a> {
    type TypeSystem = PostgresTypeSystem;
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn fetch_next(&mut self) -> (usize, bool) {
        assert!(self.current_col == 0);
        let remaining_rows = self.rowbuf.len() - self.current_row;
        if remaining_rows > 0 {
            return (remaining_rows, self.is_finished);
        } else if self.is_finished {
            return (0, self.is_finished);
        }

        // clear the buffer
        if !self.rowbuf.is_empty() {
            self.rowbuf.drain(..);
        }
        for _ in 0..DB_BUFFER_SIZE {
            match self.iter.next()? {
                Some(row) => {
                    self.rowbuf.push(row);
                }
                None => {
                    self.is_finished = true;
                    break;
                }
            }
        }

        // reset current cursor positions
        self.current_row = 0;
        self.current_col = 0;

        (self.rowbuf.len(), self.is_finished)
    }
}

macro_rules! impl_produce {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for PostgresBinarySourcePartitionParser<'a> {
                type Error = PostgresSourceError;

                #[throws(PostgresSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    let row = &self.rowbuf[ridx];
                    let val = row.try_get(cidx)?;
                    val
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for PostgresBinarySourcePartitionParser<'a> {
                type Error = PostgresSourceError;

                #[throws(PostgresSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let row = &self.rowbuf[ridx];
                    let val = row.try_get(cidx)?;
                    val
                }
            }
        )+
    };
}

impl_produce!(
    i8,
    i16,
    i32,
    i64,
    f32,
    f64,
    Decimal,
    Vec<i16>,
    Vec<i32>,
    Vec<i64>,
    Vec<f32>,
    Vec<f64>,
    Vec<Decimal>,
    bool,
    Vec<bool>,
    &'r str,
    Vec<u8>,
    NaiveTime,
    NaiveDateTime,
    DateTime<Utc>,
    NaiveDate,
    Uuid,
    Value,
    Vec<String>,
);

impl<'r, 'a> Produce<'r, HashMap<String, Option<String>>>
    for PostgresBinarySourcePartitionParser<'a>
{
    type Error = PostgresSourceError;
    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> HashMap<String, Option<String>> {
        unimplemented!("Please use `cursor` protocol for hstore type");
    }
}

impl<'r, 'a> Produce<'r, Option<HashMap<String, Option<String>>>>
    for PostgresBinarySourcePartitionParser<'a>
{
    type Error = PostgresSourceError;
    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> Option<HashMap<String, Option<String>>> {
        unimplemented!("Please use `cursor` protocol for hstore type");
    }
}

pub struct PostgresCSVSourceParser<'a> {
    iter: StringRecordsIntoIter<CopyOutReader<'a>>,
    rowbuf: Vec<StringRecord>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
    is_finished: bool,
}

impl<'a> PostgresCSVSourceParser<'a> {
    pub fn new(
        iter: StringRecordsIntoIter<CopyOutReader<'a>>,
        schema: &[PostgresTypeSystem],
    ) -> Self {
        Self {
            iter,
            rowbuf: Vec::with_capacity(DB_BUFFER_SIZE),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
            is_finished: false,
        }
    }

    #[throws(PostgresSourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        ret
    }
}

impl<'a> PartitionParser<'a> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;
    type TypeSystem = PostgresTypeSystem;

    #[throws(PostgresSourceError)]
    fn fetch_next(&mut self) -> (usize, bool) {
        assert!(self.current_col == 0);
        let remaining_rows = self.rowbuf.len() - self.current_row;
        if remaining_rows > 0 {
            return (remaining_rows, self.is_finished);
        } else if self.is_finished {
            return (0, self.is_finished);
        }

        if !self.rowbuf.is_empty() {
            self.rowbuf.drain(..);
        }
        for _ in 0..DB_BUFFER_SIZE {
            if let Some(row) = self.iter.next() {
                self.rowbuf.push(row?);
            } else {
                self.is_finished = true;
                break;
            }
        }
        self.current_row = 0;
        self.current_col = 0;
        (self.rowbuf.len(), self.is_finished)
    }
}

macro_rules! impl_csv_produce {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for PostgresCSVSourceParser<'a> {
                type Error = PostgresSourceError;

                #[throws(PostgresSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    self.rowbuf[ridx][cidx].parse().map_err(|_| {
                        ConnectorXError::cannot_produce::<$t>(Some(self.rowbuf[ridx][cidx].into()))
                    })?
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for PostgresCSVSourceParser<'a> {
                type Error = PostgresSourceError;

                #[throws(PostgresSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    match &self.rowbuf[ridx][cidx][..] {
                        "" => None,
                        v => Some(v.parse().map_err(|_| {
                            ConnectorXError::cannot_produce::<$t>(Some(self.rowbuf[ridx][cidx].into()))
                        })?),
                    }
                }
            }
        )+
    };
}

impl_csv_produce!(i8, i16, i32, i64, f32, f64, Decimal, Uuid,);

macro_rules! impl_csv_vec_produce {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, Vec<$t>> for PostgresCSVSourceParser<'a> {
                type Error = PostgresSourceError;

                #[throws(PostgresSourceError)]
                fn produce(&mut self) -> Vec<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let s = &self.rowbuf[ridx][cidx][..];
                    match s {
                        "{}" => vec![],
                        _ if s.len() < 3 => throw!(ConnectorXError::cannot_produce::<$t>(Some(s.into()))),
                        s => s[1..s.len() - 1]
                            .split(",")
                            .map(|v| {
                                v.parse()
                                    .map_err(|_| ConnectorXError::cannot_produce::<$t>(Some(s.into())))
                            })
                            .collect::<Result<Vec<$t>, ConnectorXError>>()?,
                    }
                }
            }

            impl<'r, 'a> Produce<'r, Option<Vec<$t>>> for PostgresCSVSourceParser<'a> {
                type Error = PostgresSourceError;

                #[throws(PostgresSourceError)]
                fn produce(&mut self) -> Option<Vec<$t>> {
                    let (ridx, cidx) = self.next_loc()?;
                    let s = &self.rowbuf[ridx][cidx][..];
                    match s {
                        "" => None,
                        "{}" => Some(vec![]),
                        _ if s.len() < 3 => throw!(ConnectorXError::cannot_produce::<$t>(Some(s.into()))),
                        s => Some(
                            s[1..s.len() - 1]
                                .split(",")
                                .map(|v| {
                                    v.parse()
                                        .map_err(|_| ConnectorXError::cannot_produce::<$t>(Some(s.into())))
                                })
                                .collect::<Result<Vec<$t>, ConnectorXError>>()?,
                        ),
                    }
                }
            }
        )+
    };
}

impl_csv_vec_produce!(i8, i16, i32, i64, f32, f64, Decimal, String,);

impl<'r, 'a> Produce<'r, HashMap<String, Option<String>>> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;
    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> HashMap<String, Option<String>> {
        unimplemented!("Please use `cursor` protocol for hstore type");
    }
}

impl<'r, 'a> Produce<'r, Option<HashMap<String, Option<String>>>> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;
    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> Option<HashMap<String, Option<String>>> {
        unimplemented!("Please use `cursor` protocol for hstore type");
    }
}

impl<'r, 'a> Produce<'r, bool> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> bool {
        let (ridx, cidx) = self.next_loc()?;
        let ret = match &self.rowbuf[ridx][cidx][..] {
            "t" => true,
            "f" => false,
            _ => throw!(ConnectorXError::cannot_produce::<bool>(Some(
                self.rowbuf[ridx][cidx].into()
            ))),
        };
        ret
    }
}

impl<'r, 'a> Produce<'r, Option<bool>> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> Option<bool> {
        let (ridx, cidx) = self.next_loc()?;
        let ret = match &self.rowbuf[ridx][cidx][..] {
            "" => None,
            "t" => Some(true),
            "f" => Some(false),
            _ => throw!(ConnectorXError::cannot_produce::<bool>(Some(
                self.rowbuf[ridx][cidx].into()
            ))),
        };
        ret
    }
}

impl<'r, 'a> Produce<'r, Vec<bool>> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> Vec<bool> {
        let (ridx, cidx) = self.next_loc()?;
        let s = &self.rowbuf[ridx][cidx][..];
        match s {
            "{}" => vec![],
            _ if s.len() < 3 => throw!(ConnectorXError::cannot_produce::<bool>(Some(s.into()))),
            s => s[1..s.len() - 1]
                .split(',')
                .map(|v| match v {
                    "t" => Ok(true),
                    "f" => Ok(false),
                    _ => throw!(ConnectorXError::cannot_produce::<bool>(Some(s.into()))),
                })
                .collect::<Result<Vec<bool>, ConnectorXError>>()?,
        }
    }
}

impl<'r, 'a> Produce<'r, Option<Vec<bool>>> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> Option<Vec<bool>> {
        let (ridx, cidx) = self.next_loc()?;
        let s = &self.rowbuf[ridx][cidx][..];
        match s {
            "" => None,
            "{}" => Some(vec![]),
            _ if s.len() < 3 => throw!(ConnectorXError::cannot_produce::<bool>(Some(s.into()))),
            s => Some(
                s[1..s.len() - 1]
                    .split(',')
                    .map(|v| match v {
                        "t" => Ok(true),
                        "f" => Ok(false),
                        _ => throw!(ConnectorXError::cannot_produce::<bool>(Some(s.into()))),
                    })
                    .collect::<Result<Vec<bool>, ConnectorXError>>()?,
            ),
        }
    }
}

impl<'r, 'a> Produce<'r, DateTime<Utc>> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> DateTime<Utc> {
        let (ridx, cidx) = self.next_loc()?;
        let s: &str = &self.rowbuf[ridx][cidx][..];
        // postgres csv return example: 1970-01-01 00:00:01+00
        format!("{}:00", s).parse().map_err(|_| {
            ConnectorXError::cannot_produce::<DateTime<Utc>>(Some(self.rowbuf[ridx][cidx].into()))
        })?
    }
}

impl<'r, 'a> Produce<'r, Option<DateTime<Utc>>> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> Option<DateTime<Utc>> {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx][..] {
            "" => None,
            v => {
                // postgres csv return example: 1970-01-01 00:00:01+00
                Some(format!("{}:00", v).parse().map_err(|_| {
                    ConnectorXError::cannot_produce::<DateTime<Utc>>(Some(v.into()))
                })?)
            }
        }
    }
}

impl<'r, 'a> Produce<'r, NaiveDate> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> NaiveDate {
        let (ridx, cidx) = self.next_loc()?;
        NaiveDate::parse_from_str(&self.rowbuf[ridx][cidx], "%Y-%m-%d").map_err(|_| {
            ConnectorXError::cannot_produce::<NaiveDate>(Some(self.rowbuf[ridx][cidx].into()))
        })?
    }
}

impl<'r, 'a> Produce<'r, Option<NaiveDate>> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> Option<NaiveDate> {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx][..] {
            "" => None,
            v => Some(
                NaiveDate::parse_from_str(v, "%Y-%m-%d")
                    .map_err(|_| ConnectorXError::cannot_produce::<NaiveDate>(Some(v.into())))?,
            ),
        }
    }
}

impl<'r, 'a> Produce<'r, NaiveDateTime> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> NaiveDateTime {
        let (ridx, cidx) = self.next_loc()?;
        NaiveDateTime::parse_from_str(&self.rowbuf[ridx][cidx], "%Y-%m-%d %H:%M:%S").map_err(
            |_| {
                ConnectorXError::cannot_produce::<NaiveDateTime>(Some(
                    self.rowbuf[ridx][cidx].into(),
                ))
            },
        )?
    }
}

impl<'r, 'a> Produce<'r, Option<NaiveDateTime>> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> Option<NaiveDateTime> {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx][..] {
            "" => None,
            v => Some(
                NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M:%S").map_err(|_| {
                    ConnectorXError::cannot_produce::<NaiveDateTime>(Some(v.into()))
                })?,
            ),
        }
    }
}

impl<'r, 'a> Produce<'r, NaiveTime> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> NaiveTime {
        let (ridx, cidx) = self.next_loc()?;
        NaiveTime::parse_from_str(&self.rowbuf[ridx][cidx], "%H:%M:%S").map_err(|_| {
            ConnectorXError::cannot_produce::<NaiveTime>(Some(self.rowbuf[ridx][cidx].into()))
        })?
    }
}

impl<'r, 'a> Produce<'r, Option<NaiveTime>> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> Option<NaiveTime> {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx][..] {
            "" => None,
            v => Some(
                NaiveTime::parse_from_str(v, "%H:%M:%S")
                    .map_err(|_| ConnectorXError::cannot_produce::<NaiveTime>(Some(v.into())))?,
            ),
        }
    }
}

impl<'r, 'a> Produce<'r, &'r str> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> &'r str {
        let (ridx, cidx) = self.next_loc()?;
        &self.rowbuf[ridx][cidx]
    }
}

impl<'r, 'a> Produce<'r, Option<&'r str>> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> Option<&'r str> {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx][..] {
            "" => None,
            v => Some(v),
        }
    }
}

impl<'r, 'a> Produce<'r, Vec<u8>> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> Vec<u8> {
        let (ridx, cidx) = self.next_loc()?;
        decode(&self.rowbuf[ridx][cidx][2..])? // escape \x in the beginning
    }
}

impl<'r, 'a> Produce<'r, Option<Vec<u8>>> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> Option<Vec<u8>> {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx] {
            // escape \x in the beginning, empty if None
            "" => None,
            v => Some(decode(&v[2..])?),
        }
    }
}

impl<'r, 'a> Produce<'r, Value> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> Value {
        let (ridx, cidx) = self.next_loc()?;
        let v = &self.rowbuf[ridx][cidx];
        from_str(v).map_err(|_| ConnectorXError::cannot_produce::<Value>(Some(v.into())))?
    }
}

impl<'r, 'a> Produce<'r, Option<Value>> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> Option<Value> {
        let (ridx, cidx) = self.next_loc()?;

        match &self.rowbuf[ridx][cidx][..] {
            "" => None,
            v => {
                from_str(v).map_err(|_| ConnectorXError::cannot_produce::<Value>(Some(v.into())))?
            }
        }
    }
}

pub struct PostgresRawSourceParser<'a> {
    iter: RowIter<'a>,
    rowbuf: Vec<Row>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
    is_finished: bool,
}

impl<'a> PostgresRawSourceParser<'a> {
    pub fn new(iter: RowIter<'a>, schema: &[PostgresTypeSystem]) -> Self {
        Self {
            iter,
            rowbuf: Vec::with_capacity(DB_BUFFER_SIZE),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
            is_finished: false,
        }
    }

    #[throws(PostgresSourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        ret
    }
}

impl<'a> PartitionParser<'a> for PostgresRawSourceParser<'a> {
    type TypeSystem = PostgresTypeSystem;
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn fetch_next(&mut self) -> (usize, bool) {
        assert!(self.current_col == 0);
        let remaining_rows = self.rowbuf.len() - self.current_row;
        if remaining_rows > 0 {
            return (remaining_rows, self.is_finished);
        } else if self.is_finished {
            return (0, self.is_finished);
        }

        if !self.rowbuf.is_empty() {
            self.rowbuf.drain(..);
        }
        for _ in 0..DB_BUFFER_SIZE {
            if let Some(row) = self.iter.next()? {
                self.rowbuf.push(row);
            } else {
                self.is_finished = true;
                break;
            }
        }
        self.current_row = 0;
        self.current_col = 0;
        (self.rowbuf.len(), self.is_finished)
    }
}

macro_rules! impl_produce {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for PostgresRawSourceParser<'a> {
                type Error = PostgresSourceError;

                #[throws(PostgresSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    let row = &self.rowbuf[ridx];
                    let val = row.try_get(cidx)?;
                    val
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for PostgresRawSourceParser<'a> {
                type Error = PostgresSourceError;

                #[throws(PostgresSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let row = &self.rowbuf[ridx];
                    let val = row.try_get(cidx)?;
                    val
                }
            }
        )+
    };
}

impl_produce!(
    i8,
    i16,
    i32,
    i64,
    f32,
    f64,
    Decimal,
    Vec<i16>,
    Vec<i32>,
    Vec<i64>,
    Vec<f32>,
    Vec<f64>,
    Vec<Decimal>,
    bool,
    Vec<bool>,
    &'r str,
    Vec<u8>,
    NaiveTime,
    NaiveDateTime,
    DateTime<Utc>,
    NaiveDate,
    Uuid,
    Value,
    HashMap<String, Option<String>>,
    Vec<String>,
);

impl<C> SourcePartition for PostgresSourcePartition<SimpleProtocol, C>
where
    C: MakeTlsConnect<Socket> + Clone + 'static + Sync + Send,
    C::TlsConnect: Send,
    C::Stream: Send,
    <C::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    type TypeSystem = PostgresTypeSystem;
    type Parser<'a> = PostgresSimpleSourceParser;
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn result_rows(&mut self) {
        self.nrows = get_total_rows(&mut self.conn, &self.query)?;
    }

    #[throws(PostgresSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        let rows = self.conn.simple_query(self.query.as_str())?; // unless reading the data, it seems like issue the query is fast
        PostgresSimpleSourceParser::new(rows, &self.schema)
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

pub struct PostgresSimpleSourceParser {
    rows: Vec<SimpleQueryMessage>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
}
impl<'a> PostgresSimpleSourceParser {
    pub fn new(rows: Vec<SimpleQueryMessage>, schema: &[PostgresTypeSystem]) -> Self {
        Self {
            rows,
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
        }
    }

    #[throws(PostgresSourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        ret
    }
}

impl<'a> PartitionParser<'a> for PostgresSimpleSourceParser {
    type TypeSystem = PostgresTypeSystem;
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn fetch_next(&mut self) -> (usize, bool) {
        self.current_row = 0;
        self.current_col = 0;
        (self.rows.len() - 1, true) // last message is command complete
    }
}

macro_rules! impl_simple_produce_unimplemented {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for PostgresSimpleSourceParser {
                type Error = PostgresSourceError;

                #[throws(PostgresSourceError)]
                fn produce(&'r mut self) -> $t {
                   unimplemented!("not implemented!");
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for PostgresSimpleSourceParser {
                type Error = PostgresSourceError;

                #[throws(PostgresSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                   unimplemented!("not implemented!");
                }
            }
        )+
    };
}

macro_rules! impl_simple_produce {
    ($($t: ty,)+) => {
        $(
            impl<'r> Produce<'r, $t> for PostgresSimpleSourceParser {
                type Error = PostgresSourceError;

                #[throws(PostgresSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    let val = match &self.rows[ridx] {
                        SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                            Some(s) => s
                                .parse()
                                .map_err(|_| ConnectorXError::cannot_produce::<$t>(Some(s.into())))?,
                            None => throw!(anyhow!(
                                "Cannot parse NULL in NOT NULL column."
                            )),
                        },
                        SimpleQueryMessage::CommandComplete(c) => {
                            panic!("get command: {}", c);
                        }
                        _ => {
                            panic!("what?");
                        }
                    };
                    val
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for PostgresSimpleSourceParser {
                type Error = PostgresSourceError;

                #[throws(PostgresSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let val = match &self.rows[ridx] {
                        SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                            Some(s) => Some(
                                s.parse()
                                    .map_err(|_| ConnectorXError::cannot_produce::<$t>(Some(s.into())))?,
                            ),
                            None => None,
                        },
                        SimpleQueryMessage::CommandComplete(c) => {
                            panic!("get command: {}", c);
                        }
                        _ => {
                            panic!("what?");
                        }
                    };
                    val
                }
            }
        )+
    };
}

impl_simple_produce!(i8, i16, i32, i64, f32, f64, Decimal, Uuid, bool,);
impl_simple_produce_unimplemented!(
    Value,
    HashMap<String, Option<String>>,);

impl<'r> Produce<'r, &'r str> for PostgresSimpleSourceParser {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> &'r str {
        let (ridx, cidx) = self.next_loc()?;
        let val = match &self.rows[ridx] {
            SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                Some(s) => s,
                None => throw!(anyhow!("Cannot parse NULL in non-NULL column.")),
            },
            SimpleQueryMessage::CommandComplete(c) => {
                panic!("get command: {}", c);
            }
            _ => {
                panic!("what?");
            }
        };
        val
    }
}

impl<'r, 'a> Produce<'r, Option<&'r str>> for PostgresSimpleSourceParser {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> Option<&'r str> {
        let (ridx, cidx) = self.next_loc()?;
        let val = match &self.rows[ridx] {
            SimpleQueryMessage::Row(row) => row.try_get(cidx)?,
            SimpleQueryMessage::CommandComplete(c) => {
                panic!("get command: {}", c);
            }
            _ => {
                panic!("what?");
            }
        };
        val
    }
}

impl<'r> Produce<'r, Vec<u8>> for PostgresSimpleSourceParser {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> Vec<u8> {
        let (ridx, cidx) = self.next_loc()?;
        let val = match &self.rows[ridx] {
            SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                Some(s) => {
                    let mut res = s.chars();
                    res.next();
                    res.next();
                    decode(
                        res.enumerate()
                            .fold(String::new(), |acc, (_i, c)| format!("{}{}", acc, c))
                            .chars()
                            .map(|c| c as u8)
                            .collect::<Vec<u8>>(),
                    )?
                }
                None => throw!(anyhow!("Cannot parse NULL in non-NULL column.")),
            },
            SimpleQueryMessage::CommandComplete(c) => {
                panic!("get command: {}", c);
            }
            _ => {
                panic!("what?");
            }
        };
        val
    }
}

impl<'r, 'a> Produce<'r, Option<Vec<u8>>> for PostgresSimpleSourceParser {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> Option<Vec<u8>> {
        let (ridx, cidx) = self.next_loc()?;
        let val = match &self.rows[ridx] {
            SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                Some(s) => {
                    let mut res = s.chars();
                    res.next();
                    res.next();
                    Some(decode(
                        res.enumerate()
                            .fold(String::new(), |acc, (_i, c)| format!("{}{}", acc, c))
                            .chars()
                            .map(|c| c as u8)
                            .collect::<Vec<u8>>(),
                    )?)
                }
                None => None,
            },
            SimpleQueryMessage::CommandComplete(c) => {
                panic!("get command: {}", c);
            }
            _ => {
                panic!("what?");
            }
        };
        val
    }
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

macro_rules! impl_simple_vec_produce {
    ($($t: ty,)+) => {
        $(
            impl<'r> Produce<'r, Vec<$t>> for PostgresSimpleSourceParser {
                type Error = PostgresSourceError;

                #[throws(PostgresSourceError)]
                fn produce(&'r mut self) -> Vec<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let val = match &self.rows[ridx] {
                        SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                            Some(s) => match s{
                                "" => throw!(anyhow!("Cannot parse NULL in non-NULL column.")),
                                "{}" => vec![],
                                _ => rem_first_and_last(s).split(",").map(|token| token.parse().map_err(|_| ConnectorXError::cannot_produce::<Vec<$t>>(Some(s.into())))).collect::<Result<Vec<$t>, ConnectorXError>>()?
                            },
                            None => throw!(anyhow!("Cannot parse NULL in non-NULL column.")),
                        },
                        SimpleQueryMessage::CommandComplete(c) => {
                            panic!("get command: {}", c);
                        }
                        _ => {
                            panic!("what?");
                        }
                    };
                    val
                }
            }

            impl<'r, 'a> Produce<'r, Option<Vec<$t>>> for PostgresSimpleSourceParser {
                type Error = PostgresSourceError;

                #[throws(PostgresSourceError)]
                fn produce(&'r mut self) -> Option<Vec<$t>> {
                    let (ridx, cidx) = self.next_loc()?;
                    let val = match &self.rows[ridx] {

                        SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                            Some(s) => match s{
                                "" => None,
                                "{}" => Some(vec![]),
                                _ => Some(rem_first_and_last(s).split(",").map(|token| token.parse().map_err(|_| ConnectorXError::cannot_produce::<Vec<$t>>(Some(s.into())))).collect::<Result<Vec<$t>, ConnectorXError>>()?)
                            },
                            None => None,
                        },

                        SimpleQueryMessage::CommandComplete(c) => {
                            panic!("get command: {}", c);
                        }
                        _ => {
                            panic!("what?");
                        }
                    };
                    val
                }
            }
        )+
    };
}
impl_simple_vec_produce!(i16, i32, i64, f32, f64, Decimal, String,);

impl<'r> Produce<'r, Vec<bool>> for PostgresSimpleSourceParser {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> Vec<bool> {
        let (ridx, cidx) = self.next_loc()?;
        let val = match &self.rows[ridx] {
            SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                Some(s) => match s {
                    "" => throw!(anyhow!("Cannot parse NULL in non-NULL column.")),
                    "{}" => vec![],
                    _ => rem_first_and_last(s)
                        .split(',')
                        .map(|token| match token {
                            "t" => Ok(true),
                            "f" => Ok(false),
                            _ => {
                                throw!(ConnectorXError::cannot_produce::<Vec<bool>>(Some(s.into())))
                            }
                        })
                        .collect::<Result<Vec<bool>, ConnectorXError>>()?,
                },
                None => throw!(anyhow!("Cannot parse NULL in non-NULL column.")),
            },
            SimpleQueryMessage::CommandComplete(c) => {
                panic!("get command: {}", c);
            }
            _ => {
                panic!("what?");
            }
        };
        val
    }
}

impl<'r> Produce<'r, Option<Vec<bool>>> for PostgresSimpleSourceParser {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> Option<Vec<bool>> {
        let (ridx, cidx) = self.next_loc()?;
        let val = match &self.rows[ridx] {
            SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                Some(s) => match s {
                    "" => None,
                    "{}" => Some(vec![]),
                    _ => Some(
                        rem_first_and_last(s)
                            .split(',')
                            .map(|token| match token {
                                "t" => Ok(true),
                                "f" => Ok(false),
                                _ => throw!(ConnectorXError::cannot_produce::<Vec<bool>>(Some(
                                    s.into()
                                ))),
                            })
                            .collect::<Result<Vec<bool>, ConnectorXError>>()?,
                    ),
                },
                None => None,
            },
            SimpleQueryMessage::CommandComplete(c) => {
                panic!("get command: {}", c);
            }
            _ => {
                panic!("what?");
            }
        };
        val
    }
}

impl<'r> Produce<'r, NaiveDate> for PostgresSimpleSourceParser {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> NaiveDate {
        let (ridx, cidx) = self.next_loc()?;
        let val = match &self.rows[ridx] {
            SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                Some(s) => NaiveDate::parse_from_str(s, "%Y-%m-%d")
                    .map_err(|_| ConnectorXError::cannot_produce::<NaiveDate>(Some(s.into())))?,
                None => throw!(anyhow!("Cannot parse NULL in non-NULL column.")),
            },
            SimpleQueryMessage::CommandComplete(c) => {
                panic!("get command: {}", c);
            }
            _ => {
                panic!("what?");
            }
        };
        val
    }
}

impl<'r> Produce<'r, Option<NaiveDate>> for PostgresSimpleSourceParser {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> Option<NaiveDate> {
        let (ridx, cidx) = self.next_loc()?;
        let val = match &self.rows[ridx] {
            SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                Some(s) => Some(NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|_| {
                    ConnectorXError::cannot_produce::<Option<NaiveDate>>(Some(s.into()))
                })?),
                None => None,
            },
            SimpleQueryMessage::CommandComplete(c) => {
                panic!("get command: {}", c);
            }
            _ => {
                panic!("what?");
            }
        };
        val
    }
}

impl<'r> Produce<'r, NaiveTime> for PostgresSimpleSourceParser {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> NaiveTime {
        let (ridx, cidx) = self.next_loc()?;
        let val = match &self.rows[ridx] {
            SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                Some(s) => NaiveTime::parse_from_str(s, "%H:%M:%S")
                    .map_err(|_| ConnectorXError::cannot_produce::<NaiveTime>(Some(s.into())))?,
                None => throw!(anyhow!("Cannot parse NULL in non-NULL column.")),
            },
            SimpleQueryMessage::CommandComplete(c) => {
                panic!("get command: {}", c);
            }
            _ => {
                panic!("what?");
            }
        };
        val
    }
}

impl<'r> Produce<'r, Option<NaiveTime>> for PostgresSimpleSourceParser {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> Option<NaiveTime> {
        let (ridx, cidx) = self.next_loc()?;
        let val = match &self.rows[ridx] {
            SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                Some(s) => Some(NaiveTime::parse_from_str(s, "%H:%M:%S").map_err(|_| {
                    ConnectorXError::cannot_produce::<Option<NaiveTime>>(Some(s.into()))
                })?),
                None => None,
            },
            SimpleQueryMessage::CommandComplete(c) => {
                panic!("get command: {}", c);
            }
            _ => {
                panic!("what?");
            }
        };
        val
    }
}

impl<'r> Produce<'r, NaiveDateTime> for PostgresSimpleSourceParser {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> NaiveDateTime {
        let (ridx, cidx) = self.next_loc()?;
        let val = match &self.rows[ridx] {
            SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                Some(s) => NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").map_err(|_| {
                    ConnectorXError::cannot_produce::<NaiveDateTime>(Some(s.into()))
                })?,
                None => throw!(anyhow!("Cannot parse NULL in non-NULL column.")),
            },
            SimpleQueryMessage::CommandComplete(c) => {
                panic!("get command: {}", c);
            }
            _ => {
                panic!("what?");
            }
        };
        val
    }
}

impl<'r> Produce<'r, Option<NaiveDateTime>> for PostgresSimpleSourceParser {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> Option<NaiveDateTime> {
        let (ridx, cidx) = self.next_loc()?;
        let val = match &self.rows[ridx] {
            SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                Some(s) => Some(
                    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").map_err(|_| {
                        ConnectorXError::cannot_produce::<Option<NaiveDateTime>>(Some(s.into()))
                    })?,
                ),
                None => None,
            },
            SimpleQueryMessage::CommandComplete(c) => {
                panic!("get command: {}", c);
            }
            _ => {
                panic!("what?");
            }
        };
        val
    }
}

impl<'r> Produce<'r, DateTime<Utc>> for PostgresSimpleSourceParser {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> DateTime<Utc> {
        let (ridx, cidx) = self.next_loc()?;
        let val = match &self.rows[ridx] {
            SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                Some(s) => {
                    let time_string = format!("{}:00", s).to_owned();
                    let slice: &str = &time_string[..];
                    let time: DateTime<FixedOffset> =
                        DateTime::parse_from_str(slice, "%Y-%m-%d %H:%M:%S%:z").unwrap();

                    time.with_timezone(&Utc)
                }
                None => throw!(anyhow!("Cannot parse NULL in non-NULL column.")),
            },
            SimpleQueryMessage::CommandComplete(c) => {
                panic!("get command: {}", c);
            }
            _ => {
                panic!("what?");
            }
        };
        val
    }
}

impl<'r> Produce<'r, Option<DateTime<Utc>>> for PostgresSimpleSourceParser {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&'r mut self) -> Option<DateTime<Utc>> {
        let (ridx, cidx) = self.next_loc()?;
        let val = match &self.rows[ridx] {
            SimpleQueryMessage::Row(row) => match row.try_get(cidx)? {
                Some(s) => {
                    let time_string = format!("{}:00", s).to_owned();
                    let slice: &str = &time_string[..];
                    let time: DateTime<FixedOffset> =
                        DateTime::parse_from_str(slice, "%Y-%m-%d %H:%M:%S%:z").unwrap();

                    Some(time.with_timezone(&Utc))
                }
                None => None,
            },
            SimpleQueryMessage::CommandComplete(c) => {
                panic!("get command: {}", c);
            }
            _ => {
                panic!("what?");
            }
        };
        val
    }
}
