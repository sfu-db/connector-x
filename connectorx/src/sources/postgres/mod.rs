mod connection;
mod errors;
mod typesystem;

pub use self::errors::PostgresSourceError;
use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::{PartitionParser, Produce, Source, SourcePartition},
    sql::{count_query, get_limit, limit1_query, CXQuery},
};
use anyhow::anyhow;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use csv::{ReaderBuilder, StringRecord, StringRecordsIntoIter};
use fehler::{throw, throws};
use hex::decode;
use log::debug;
use postgres::Config;
use postgres::{
    binary_copy::{BinaryCopyOutIter, BinaryCopyOutRow},
    fallible_iterator::FallibleIterator,
    CopyOutReader, Row, RowIter,
};
use postgres_openssl::MakeTlsConnector;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use rust_decimal::Decimal;
use serde_json::{from_str, Value};
use sqlparser::dialect::PostgreSqlDialect;
use std::collections::HashMap;
use std::io::BufRead;
use std::marker::PhantomData;
pub use typesystem::PostgresTypeSystem;
use url::Url;
use uuid::Uuid;

/// Protocol - Binary based bulk load
pub enum BinaryProtocol {}

/// Protocol - CSV based bulk load
pub enum CSVProtocol {}

/// Protocol - use Cursor
pub enum CursorProtocol {}

type PgManager = PostgresConnectionManager<MakeTlsConnector>;
type PgConn = PooledConnection<PgManager>;

pub struct PostgresSource<P> {
    pool: Pool<PgManager>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<PostgresTypeSystem>,
    buf_size: usize,
    _protocol: PhantomData<P>,
}

impl<P> PostgresSource<P> {
    #[throws(PostgresSourceError)]
    pub fn new(conn: &str, nconn: usize) -> Self {
        // parse the config, then strip unsupported SSL opts before calling conn.parse()
        let parsed_conn_str = Url::parse(conn).unwrap();

        let params: HashMap<String, String> = connection::get_query_params(parsed_conn_str.clone());
        let (client_cert, root_cert) = connection::parse_ssl_opts(params);

        let stripped_url = connection::strip_bad_opts(parsed_conn_str.clone());

        let c: Config = stripped_url.as_str().parse()?;
        let ssl_mode = c.get_ssl_mode();

        let tls_connector = connection::from_tls_config(connection::TlsConfig {
            ssl_mode,
            client_cert,
            root_cert,
        })
        .unwrap();

        let manager = PostgresConnectionManager::new(c, tls_connector);

        let pool = Pool::builder().max_size(nconn as u32).build(manager)?;

        Self {
            pool,
            queries: vec![],
            names: vec![],
            schema: vec![],
            buf_size: 32,
            _protocol: PhantomData,
        }
    }

    pub fn buf_size(&mut self, buf_size: usize) {
        self.buf_size = buf_size;
    }
}

impl<P> Source for PostgresSource<P>
where
    PostgresSourcePartition<P>:
        SourcePartition<TypeSystem = PostgresTypeSystem, Error = PostgresSourceError>,
    P: Send,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = PostgresSourcePartition<P>;
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

    #[throws(PostgresSourceError)]
    fn fetch_metadata(&mut self) {
        assert!(!self.queries.is_empty());

        let mut conn = self.pool.get()?;
        let mut success = false;
        let mut zero_tuple = true;
        let mut error = None;
        for query in &self.queries {
            // assuming all the partition queries yield same schema
            match conn.query_opt(limit1_query(query, &PostgreSqlDialect {})?.as_str(), &[]) {
                Ok(Some(row)) => {
                    let (names, types) = row
                        .columns()
                        .iter()
                        .map(|col| {
                            (
                                col.name().to_string(),
                                PostgresTypeSystem::from(col.type_()),
                            )
                        })
                        .unzip();

                    self.names = names;
                    self.schema = types;

                    success = true;
                    zero_tuple = false;
                    break;
                }
                Ok(None) => {}
                Err(e) => {
                    debug!("cannot get metadata for '{}', try next query: {}", query, e);
                    error = Some(e);
                    zero_tuple = false;
                }
            }
        }

        if !success {
            if zero_tuple {
                // try to use COPY command get the column headers
                let copy_query = format!("COPY ({}) TO STDOUT WITH CSV HEADER", self.queries[0]);
                let mut reader = conn.copy_out(&*copy_query)?;
                let mut buf = String::new();
                reader.read_line(&mut buf)?;
                self.names = buf[0..buf.len() - 1] // remove last '\n'
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();
                // set all columns as string (align with pandas)
                self.schema = vec![PostgresTypeSystem::Text(false); self.names.len()];
            } else {
                throw!(anyhow!(
                    "Cannot get metadata for the queries, last error: {:?}",
                    error
                ))
            }
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

            ret.push(PostgresSourcePartition::<P>::new(
                conn,
                &query,
                &self.schema,
                self.buf_size,
            ));
        }
        ret
    }
}

pub struct PostgresSourcePartition<P> {
    conn: PgConn,
    query: CXQuery<String>,
    schema: Vec<PostgresTypeSystem>,
    nrows: usize,
    ncols: usize,
    buf_size: usize,
    _protocol: PhantomData<P>,
}

impl<P> PostgresSourcePartition<P> {
    pub fn new(
        conn: PgConn,
        query: &CXQuery<String>,
        schema: &[PostgresTypeSystem],
        buf_size: usize,
    ) -> Self {
        Self {
            conn,
            query: query.clone(),
            schema: schema.to_vec(),
            nrows: 0,
            ncols: schema.len(),
            buf_size,
            _protocol: PhantomData,
        }
    }
}

impl SourcePartition for PostgresSourcePartition<BinaryProtocol> {
    type TypeSystem = PostgresTypeSystem;
    type Parser<'a> = PostgresBinarySourcePartitionParser<'a>;
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn prepare(&mut self) -> () {
        let dialect = PostgreSqlDialect {};
        self.nrows = match get_limit(&self.query, &dialect)? {
            None => {
                let row = self
                    .conn
                    .query_one(count_query(&self.query, &dialect)?.as_str(), &[])?;
                row.get::<_, i64>(0) as usize
            }
            Some(n) => n,
        };
    }

    #[throws(PostgresSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        let query = format!("COPY ({}) TO STDOUT WITH BINARY", self.query);
        let reader = self.conn.copy_out(&*query)?; // unless reading the data, it seems like issue the query is fast
        let pg_schema: Vec<_> = self.schema.iter().map(|&dt| dt.into()).collect();
        let iter = BinaryCopyOutIter::new(reader, &pg_schema);

        PostgresBinarySourcePartitionParser::new(iter, &self.schema, self.buf_size)
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

impl SourcePartition for PostgresSourcePartition<CSVProtocol> {
    type TypeSystem = PostgresTypeSystem;
    type Parser<'a> = PostgresCSVSourceParser<'a>;
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn prepare(&mut self) {
        let row = self.conn.query_one(
            count_query(&self.query, &PostgreSqlDialect {})?.as_str(),
            &[],
        )?;
        self.nrows = row.get::<_, i64>(0) as usize;
    }

    #[throws(PostgresSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        let query = format!("COPY ({}) TO STDOUT WITH CSV", self.query);
        let reader = self.conn.copy_out(&*query)?; // unless reading the data, it seems like issue the query is fast
        let iter = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(reader)
            .into_records();

        PostgresCSVSourceParser::new(iter, &self.schema, self.buf_size)
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

impl SourcePartition for PostgresSourcePartition<CursorProtocol> {
    type TypeSystem = PostgresTypeSystem;
    type Parser<'a> = PostgresRawSourceParser<'a>;
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn prepare(&mut self) {
        let row = self.conn.query_one(
            count_query(&self.query, &PostgreSqlDialect {})?.as_str(),
            &[],
        )?;
        self.nrows = row.get::<_, i64>(0) as usize;
    }

    #[throws(PostgresSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        let iter = self
            .conn
            .query_raw::<_, bool, _>(self.query.as_str(), vec![])?; // unless reading the data, it seems like issue the query is fast
        PostgresRawSourceParser::new(iter, &self.schema, self.buf_size)
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
    buf_size: usize,
    rowbuf: Vec<BinaryCopyOutRow>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
}

impl<'a> PostgresBinarySourcePartitionParser<'a> {
    pub fn new(
        iter: BinaryCopyOutIter<'a>,
        schema: &[PostgresTypeSystem],
        buf_size: usize,
    ) -> Self {
        Self {
            iter,
            buf_size,
            rowbuf: Vec::with_capacity(buf_size),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
        }
    }

    #[throws(PostgresSourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        if self.current_row >= self.rowbuf.len() {
            if !self.rowbuf.is_empty() {
                self.rowbuf.drain(..);
            }

            for _ in 0..self.buf_size {
                match self.iter.next()? {
                    Some(row) => {
                        self.rowbuf.push(row);
                    }
                    None => break,
                }
            }

            if self.rowbuf.is_empty() {
                throw!(anyhow!("Postgres EOF"));
            }
            self.current_row = 0;
            self.current_col = 0;
        }

        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        ret
    }
}

impl<'a> PartitionParser<'a> for PostgresBinarySourcePartitionParser<'a> {
    type TypeSystem = PostgresTypeSystem;
    type Error = PostgresSourceError;
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
    bool,
    &'r str,
    Vec<u8>,
    NaiveTime,
    NaiveDateTime,
    DateTime<Utc>,
    NaiveDate,
    Uuid,
    Value,
);

pub struct PostgresCSVSourceParser<'a> {
    iter: StringRecordsIntoIter<CopyOutReader<'a>>,
    buf_size: usize,
    rowbuf: Vec<StringRecord>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
}

impl<'a> PostgresCSVSourceParser<'a> {
    pub fn new(
        iter: StringRecordsIntoIter<CopyOutReader<'a>>,
        schema: &[PostgresTypeSystem],
        buf_size: usize,
    ) -> Self {
        Self {
            iter,
            buf_size,
            rowbuf: Vec::with_capacity(buf_size),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
        }
    }

    #[throws(PostgresSourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        if self.current_row >= self.rowbuf.len() {
            if !self.rowbuf.is_empty() {
                self.rowbuf.drain(..);
            }

            for _ in 0..self.buf_size {
                if let Some(row) = self.iter.next() {
                    self.rowbuf.push(row?);
                } else {
                    break;
                }
            }

            if self.rowbuf.is_empty() {
                throw!(anyhow!("Postgres EOF"));
            }
            self.current_row = 0;
            self.current_col = 0;
        }

        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        ret
    }
}

impl<'a> PartitionParser<'a> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;
    type TypeSystem = PostgresTypeSystem;
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

impl<'r, 'a> Produce<'r, DateTime<Utc>> for PostgresCSVSourceParser<'a> {
    type Error = PostgresSourceError;

    #[throws(PostgresSourceError)]
    fn produce(&mut self) -> DateTime<Utc> {
        let (ridx, cidx) = self.next_loc()?;
        self.rowbuf[ridx][cidx].parse().map_err(|_| {
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
                Some(v.parse().map_err(|_| {
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
    buf_size: usize,
    rowbuf: Vec<Row>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
}

impl<'a> PostgresRawSourceParser<'a> {
    pub fn new(iter: RowIter<'a>, schema: &[PostgresTypeSystem], buf_size: usize) -> Self {
        Self {
            iter,
            buf_size,
            rowbuf: Vec::with_capacity(buf_size),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
        }
    }

    #[throws(PostgresSourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        if self.current_row >= self.rowbuf.len() {
            if !self.rowbuf.is_empty() {
                self.rowbuf.drain(..);
            }

            for _ in 0..self.buf_size {
                if let Some(row) = self.iter.next()? {
                    self.rowbuf.push(row);
                } else {
                    break;
                }
            }

            if self.rowbuf.is_empty() {
                throw!(anyhow!("Postgres EOF"));
            }
            self.current_row = 0;
            self.current_col = 0;
        }

        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        ret
    }
}

impl<'a> PartitionParser<'a> for PostgresRawSourceParser<'a> {
    type TypeSystem = PostgresTypeSystem;
    type Error = PostgresSourceError;
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
    bool,
    &'r str,
    Vec<u8>,
    NaiveTime,
    NaiveDateTime,
    DateTime<Utc>,
    NaiveDate,
    Uuid,
    Value,
);
