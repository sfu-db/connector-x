mod typesystem;

use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::sources::{PartitionParser, Produce, Source, SourcePartition};
use crate::sql::{count_query, get_limit, limit1_query};
use anyhow::anyhow;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use csv::{ReaderBuilder, StringRecord, StringRecordsIntoIter};
use fehler::throw;
use hex::decode;
use log::debug;
use postgres::{
    binary_copy::{BinaryCopyOutIter, BinaryCopyOutRow},
    fallible_iterator::FallibleIterator,
    CopyOutReader,
};
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use rust_decimal::Decimal;
use serde_json::{from_str, Value};
use sqlparser::dialect::PostgreSqlDialect;
use std::marker::PhantomData;
pub use typesystem::PostgresTypeSystem;
use uuid::Uuid;

type PgManager = PostgresConnectionManager<NoTls>;
type PgConn = PooledConnection<PgManager>;

pub enum Binary {}
pub enum CSV {}

pub struct PostgresSource<P> {
    pool: Pool<PgManager>,
    queries: Vec<String>,
    names: Vec<String>,
    schema: Vec<PostgresTypeSystem>,
    buf_size: usize,
    _protocol: PhantomData<P>,
}

impl<P> PostgresSource<P> {
    pub fn new(conn: &str, nconn: usize) -> Result<Self> {
        let manager = PostgresConnectionManager::new(conn.parse()?, NoTls);
        let pool = Pool::builder().max_size(nconn as u32).build(manager)?;

        Ok(Self {
            pool,
            queries: vec![],
            names: vec![],
            schema: vec![],
            buf_size: 32,
            _protocol: PhantomData,
        })
    }

    pub fn buf_size(&mut self, buf_size: usize) {
        self.buf_size = buf_size;
    }
}

impl<P> Source for PostgresSource<P>
where
    PostgresSourcePartition<P>: SourcePartition<TypeSystem = PostgresTypeSystem>,
    P: Send,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = PostgresSourcePartition<P>;
    type TypeSystem = PostgresTypeSystem;

    fn set_data_order(&mut self, data_order: DataOrder) -> Result<()> {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order));
        }
        Ok(())
    }

    fn set_queries<Q: AsRef<str>>(&mut self, queries: &[Q]) {
        self.queries = queries.iter().map(|q| q.as_ref().to_string()).collect();
    }

    fn fetch_metadata(&mut self) -> Result<()> {
        assert!(self.queries.len() != 0);

        let mut conn = self.pool.get()?;
        let mut success = false;
        let mut error = None;
        for query in &self.queries {
            // assuming all the partition queries yield same schema
            match conn.query_one(&limit1_query(query, &PostgreSqlDialect {})?[..], &[]) {
                Ok(row) => {
                    let (names, types) = row
                        .columns()
                        .into_iter()
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
                    break;
                }
                Err(e) => {
                    debug!("cannot get metadata for '{}', try next query: {}", query, e);
                    error = Some(e);
                }
            }
        }

        if !success {
            throw!(anyhow!(
                "Cannot get metadata for the queries, last error: {:?}",
                error
            ))
        }

        Ok(())
    }

    fn names(&self) -> Vec<String> {
        self.names.clone()
    }

    fn schema(&self) -> Vec<Self::TypeSystem> {
        self.schema.clone()
    }

    fn partition(self) -> Result<Vec<Self::Partition>> {
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
        Ok(ret)
    }
}

pub struct PostgresSourcePartition<P> {
    conn: PgConn,
    query: String,
    schema: Vec<PostgresTypeSystem>,
    nrows: usize,
    ncols: usize,
    buf_size: usize,
    _protocol: PhantomData<P>,
}

impl<P> PostgresSourcePartition<P> {
    pub fn new(conn: PgConn, query: &str, schema: &[PostgresTypeSystem], buf_size: usize) -> Self {
        Self {
            conn,
            query: query.to_string(),
            schema: schema.to_vec(),
            nrows: 0,
            ncols: schema.len(),
            buf_size,
            _protocol: PhantomData,
        }
    }
}

impl SourcePartition for PostgresSourcePartition<Binary> {
    type TypeSystem = PostgresTypeSystem;
    type Parser<'a> = PostgresBinarySourcePartitionParser<'a>;

    fn prepare(&mut self) -> Result<()> {
        let dialect = PostgreSqlDialect {};
        self.nrows = match get_limit(&self.query, &dialect)? {
            None => {
                let row = self
                    .conn
                    .query_one(&count_query(&self.query, &dialect)?[..], &[])?;
                row.get::<_, i64>(0) as usize
            }
            Some(n) => n,
        };
        Ok(())
    }

    fn parser(&mut self) -> Result<Self::Parser<'_>> {
        let query = format!("COPY ({}) TO STDOUT WITH BINARY", self.query);
        let reader = self.conn.copy_out(&*query)?; // unless reading the data, it seems like issue the query is fast
        let pg_schema: Vec<_> = self.schema.iter().map(|&dt| dt.into()).collect();
        let iter = BinaryCopyOutIter::new(reader, &pg_schema);

        Ok(PostgresBinarySourcePartitionParser::new(
            iter,
            &self.schema,
            self.buf_size,
        ))
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

impl SourcePartition for PostgresSourcePartition<CSV> {
    type TypeSystem = PostgresTypeSystem;
    type Parser<'a> = PostgresCSVSourceParser<'a>;

    fn prepare(&mut self) -> Result<()> {
        let row = self
            .conn
            .query_one(&count_query(&self.query, &PostgreSqlDialect {})?[..], &[])?;
        self.nrows = row.get::<_, i64>(0) as usize;
        Ok(())
    }

    fn parser(&mut self) -> Result<Self::Parser<'_>> {
        let query = format!("COPY ({}) TO STDOUT WITH CSV", self.query);
        let reader = self.conn.copy_out(&*query)?; // unless reading the data, it seems like issue the query is fast
        let iter = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(reader)
            .into_records();

        Ok(PostgresCSVSourceParser::new(
            iter,
            &self.schema,
            self.buf_size,
        ))
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

    fn next_loc(&mut self) -> Result<(usize, usize)> {
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
        Ok(ret)
    }
}

impl<'a> PartitionParser<'a> for PostgresBinarySourcePartitionParser<'a> {
    type TypeSystem = PostgresTypeSystem;
}

macro_rules! impl_produce {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for PostgresBinarySourcePartitionParser<'a> {
                fn produce(&'r mut self) -> Result<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let row = &self.rowbuf[ridx];
                    let val = row.try_get(cidx)?;
                    Ok(val)
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for PostgresBinarySourcePartitionParser<'a> {
                fn produce(&'r mut self) -> Result<Option<$t>> {
                    let (ridx, cidx) = self.next_loc()?;
                    let row = &self.rowbuf[ridx];
                    let val = row.try_get(cidx)?;
                    Ok(val)
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

    fn next_loc(&mut self) -> Result<(usize, usize)> {
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
        Ok(ret)
    }
}

impl<'a> PartitionParser<'a> for PostgresCSVSourceParser<'a> {
    type TypeSystem = PostgresTypeSystem;
}

macro_rules! impl_csv_produce {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for PostgresCSVSourceParser<'a> {
                fn produce(&'r mut self) -> Result<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    self.rowbuf[ridx][cidx].parse().map_err(|_| {
                        ConnectorAgentError::cannot_produce::<$t>(Some(self.rowbuf[ridx][cidx].into()))
                    })
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for PostgresCSVSourceParser<'a> {
                fn produce(&'r mut self) -> Result<Option<$t>> {
                    let (ridx, cidx) = self.next_loc()?;
                    match &self.rowbuf[ridx][cidx][..] {
                        "" => Ok(None),
                        v => Ok(Some(v.parse().map_err(|_| {
                            ConnectorAgentError::cannot_produce::<$t>(Some(self.rowbuf[ridx][cidx].into()))
                        })?)),
                    }
                }
            }
        )+
    };
}

impl_csv_produce!(i8, i16, i32, i64, f32, f64, Decimal, Uuid,);

impl<'r, 'a> Produce<'r, bool> for PostgresCSVSourceParser<'a> {
    fn produce(&mut self) -> Result<bool> {
        let (ridx, cidx) = self.next_loc()?;
        let ret = match &self.rowbuf[ridx][cidx][..] {
            "t" => true,
            "f" => false,
            _ => throw!(ConnectorAgentError::cannot_produce::<bool>(Some(
                self.rowbuf[ridx][cidx].into()
            ))),
        };
        Ok(ret)
    }
}

impl<'r, 'a> Produce<'r, Option<bool>> for PostgresCSVSourceParser<'a> {
    fn produce(&mut self) -> Result<Option<bool>> {
        let (ridx, cidx) = self.next_loc()?;
        let ret = match &self.rowbuf[ridx][cidx][..] {
            "" => None,
            "t" => Some(true),
            "f" => Some(false),
            _ => throw!(ConnectorAgentError::cannot_produce::<bool>(Some(
                self.rowbuf[ridx][cidx].into()
            ))),
        };
        Ok(ret)
    }
}

impl<'r, 'a> Produce<'r, DateTime<Utc>> for PostgresCSVSourceParser<'a> {
    fn produce(&mut self) -> Result<DateTime<Utc>> {
        let (ridx, cidx) = self.next_loc()?;
        self.rowbuf[ridx][cidx].parse().map_err(|_| {
            ConnectorAgentError::cannot_produce::<DateTime<Utc>>(Some(
                self.rowbuf[ridx][cidx].into(),
            ))
        })
    }
}

impl<'r, 'a> Produce<'r, Option<DateTime<Utc>>> for PostgresCSVSourceParser<'a> {
    fn produce(&mut self) -> Result<Option<DateTime<Utc>>> {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx][..] {
            "" => Ok(None),
            v => Ok(Some(v.parse().map_err(|_| {
                ConnectorAgentError::cannot_produce::<DateTime<Utc>>(Some(v.into()))
            })?)),
        }
    }
}

impl<'r, 'a> Produce<'r, NaiveDate> for PostgresCSVSourceParser<'a> {
    fn produce(&mut self) -> Result<NaiveDate> {
        let (ridx, cidx) = self.next_loc()?;
        NaiveDate::parse_from_str(&self.rowbuf[ridx][cidx], "%Y-%m-%d").map_err(|_| {
            ConnectorAgentError::cannot_produce::<NaiveDate>(Some(self.rowbuf[ridx][cidx].into()))
        })
    }
}

impl<'r, 'a> Produce<'r, Option<NaiveDate>> for PostgresCSVSourceParser<'a> {
    fn produce(&mut self) -> Result<Option<NaiveDate>> {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx][..] {
            "" => Ok(None),
            v => Ok(Some(NaiveDate::parse_from_str(v, "%Y-%m-%d").map_err(
                |_| ConnectorAgentError::cannot_produce::<NaiveDate>(Some(v.into())),
            )?)),
        }
    }
}

impl<'r, 'a> Produce<'r, NaiveDateTime> for PostgresCSVSourceParser<'a> {
    fn produce(&mut self) -> Result<NaiveDateTime> {
        let (ridx, cidx) = self.next_loc()?;
        NaiveDateTime::parse_from_str(&self.rowbuf[ridx][cidx], "%Y-%m-%d %H:%M:%S").map_err(|_| {
            ConnectorAgentError::cannot_produce::<NaiveDateTime>(Some(
                self.rowbuf[ridx][cidx].into(),
            ))
        })
    }
}

impl<'r, 'a> Produce<'r, Option<NaiveDateTime>> for PostgresCSVSourceParser<'a> {
    fn produce(&mut self) -> Result<Option<NaiveDateTime>> {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx][..] {
            "" => Ok(None),
            v => Ok(Some(
                NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M:%S").map_err(|_| {
                    ConnectorAgentError::cannot_produce::<NaiveDateTime>(Some(v.into()))
                })?,
            )),
        }
    }
}

impl<'r, 'a> Produce<'r, NaiveTime> for PostgresCSVSourceParser<'a> {
    fn produce(&mut self) -> Result<NaiveTime> {
        let (ridx, cidx) = self.next_loc()?;
        NaiveTime::parse_from_str(&self.rowbuf[ridx][cidx], "%H:%M:%S").map_err(|_| {
            ConnectorAgentError::cannot_produce::<NaiveTime>(Some(self.rowbuf[ridx][cidx].into()))
        })
    }
}

impl<'r, 'a> Produce<'r, Option<NaiveTime>> for PostgresCSVSourceParser<'a> {
    fn produce(&mut self) -> Result<Option<NaiveTime>> {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx][..] {
            "" => Ok(None),
            v => Ok(Some(NaiveTime::parse_from_str(v, "%H:%M:%S").map_err(
                |_| ConnectorAgentError::cannot_produce::<NaiveTime>(Some(v.into())),
            )?)),
        }
    }
}

impl<'r, 'a> Produce<'r, &'r str> for PostgresCSVSourceParser<'a> {
    fn produce(&'r mut self) -> Result<&'r str> {
        let (ridx, cidx) = self.next_loc()?;
        Ok(&self.rowbuf[ridx][cidx])
    }
}

impl<'r, 'a> Produce<'r, Option<&'r str>> for PostgresCSVSourceParser<'a> {
    fn produce(&'r mut self) -> Result<Option<&'r str>> {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx][..] {
            "" => Ok(None),
            v => Ok(Some(&v)),
        }
    }
}

impl<'r, 'a> Produce<'r, Vec<u8>> for PostgresCSVSourceParser<'a> {
    fn produce(&'r mut self) -> Result<Vec<u8>> {
        let (ridx, cidx) = self.next_loc()?;
        Ok(decode(&self.rowbuf[ridx][cidx][2..])?) // escape \x in the beginning
    }
}

impl<'r, 'a> Produce<'r, Option<Vec<u8>>> for PostgresCSVSourceParser<'a> {
    fn produce(&'r mut self) -> Result<Option<Vec<u8>>> {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx][2..] {
            // escape \x in the beginning
            "" => Ok(None),
            v => Ok(Some(decode(&v)?)),
        }
    }
}

impl<'r, 'a> Produce<'r, Value> for PostgresCSVSourceParser<'a> {
    fn produce(&'r mut self) -> Result<Value> {
        let (ridx, cidx) = self.next_loc()?;
        let v = &self.rowbuf[ridx][cidx];
        from_str(v).map_err(|_| ConnectorAgentError::cannot_produce::<Value>(Some(v.into())))
    }
}

impl<'r, 'a> Produce<'r, Option<Value>> for PostgresCSVSourceParser<'a> {
    fn produce(&'r mut self) -> Result<Option<Value>> {
        let (ridx, cidx) = self.next_loc()?;

        match &self.rowbuf[ridx][cidx][..] {
            "" => Ok(None),
            v => from_str(v)
                .map_err(|_| ConnectorAgentError::cannot_produce::<Value>(Some(v.into()))),
        }
    }
}
