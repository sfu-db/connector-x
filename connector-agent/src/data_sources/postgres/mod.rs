mod sql;
mod types;

use crate::data_order::DataOrder;
use crate::data_sources::{Parser, PartitionedSource, Produce, Source};
use crate::errors::{ConnectorAgentError, Result};
use anyhow::anyhow;
use bytes::Bytes;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use fehler::throw;
use log::debug;
use postgres::{
    binary_copy::{BinaryCopyOutIter, BinaryCopyOutRow},
    fallible_iterator::FallibleIterator,
    types::Type,
};
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use sql::{count_query, limit1_query};
use std::sync::Arc;
use std::{mem::transmute, ops::Range};
pub use types::PostgresDTypes;

type PgManager = PostgresConnectionManager<NoTls>;
type PgConn = PooledConnection<PgManager>;

pub struct PostgresSource {
    pool: Pool<PgManager>,
    queries: Vec<String>,
    names: Vec<String>,
    schema: Vec<PostgresDTypes>,
    buf_size: usize,
}

impl PostgresSource {
    pub fn new(conn: &str, nconn: usize) -> Self {
        let manager = PostgresConnectionManager::new(conn.parse().unwrap(), NoTls);
        let pool = Pool::builder()
            .max_size(nconn as u32)
            .build(manager)
            .unwrap();

        Self {
            pool,
            queries: vec![],
            names: vec![],
            schema: vec![],
            buf_size: 32,
        }
    }

    pub fn buf_size(&mut self, buf_size: usize) {
        self.buf_size = buf_size;
    }
}

impl Source for PostgresSource {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = PostgresSourcePartition;
    type TypeSystem = PostgresDTypes;

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
            match conn.query_one(&limit1_query(query)[..], &[]) {
                Ok(row) => {
                    let (names, types) = row
                        .columns()
                        .into_iter()
                        .map(|col| (col.name().to_string(), PostgresDTypes::from(col.type_())))
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

            ret.push(PostgresSourcePartition::new(
                conn,
                &query,
                &self.schema,
                self.buf_size,
            ));
        }
        Ok(ret)
    }
}

pub struct PostgresSourcePartition {
    conn: PgConn,
    query: String,
    schema: Vec<PostgresDTypes>,
    nrows: usize,
    ncols: usize,
    buf_size: usize,
}

impl PostgresSourcePartition {
    pub fn new(conn: PgConn, query: &str, schema: &[PostgresDTypes], buf_size: usize) -> Self {
        Self {
            conn,
            query: query.to_string(),
            schema: schema.to_vec(),
            nrows: 0,
            ncols: schema.len(),
            buf_size,
        }
    }
}

impl PartitionedSource for PostgresSourcePartition {
    type TypeSystem = PostgresDTypes;
    type Parser<'a> = PostgresSourceParser<'a>;

    fn prepare(&mut self) -> Result<()> {
        let row = self.conn.query_one(&count_query(&self.query)[..], &[])?;
        self.nrows = row.get::<_, i64>(0) as usize;
        Ok(())
    }

    fn parser(&mut self) -> Result<Self::Parser<'_>> {
        let query = format!("COPY ({}) TO STDOUT WITH BINARY", self.query);
        let reader = self.conn.copy_out(&*query)?; // unless reading the data, it seems like issue the query is fast
        let pg_schema: Vec<_> = self.schema.iter().map(|&dt| dt.into()).collect();
        let iter = BinaryCopyOutIter::new(reader, &pg_schema);

        Ok(PostgresSourceParser::new(iter, &self.schema, self.buf_size))
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

pub struct PostgresSourceParser<'a> {
    iter: BinaryCopyOutIter<'a>,
    buf_size: usize,
    rowbuf: Vec<BinaryCopyOutRow>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
}

impl<'a> PostgresSourceParser<'a> {
    pub fn new(iter: BinaryCopyOutIter<'a>, schema: &[PostgresDTypes], buf_size: usize) -> Self {
        Self {
            iter,
            buf_size: buf_size,
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

impl<'a> Parser<'a> for PostgresSourceParser<'a> {
    type TypeSystem = PostgresDTypes;
}

macro_rules! impl_produce {
    ($($t: ty),+) => {
        $(
            impl<'a> Produce<$t> for PostgresSourceParser<'a> {
                fn produce(&mut self) -> Result<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let val = self.rowbuf[ridx].try_get(cidx)?;
                    Ok(val)
                }
            }

            impl<'a> Produce<Option<$t>> for PostgresSourceParser<'a> {
                fn produce(&mut self) -> Result<Option<$t>> {
                    let (ridx, cidx) = self.next_loc()?;
                    let val = self.rowbuf[ridx].try_get(cidx)?;
                    Ok(val)
                }
            }
        )+
    };
}

impl_produce!(
    i32,
    i64,
    f32,
    f64,
    bool,
    DateTime<Utc>,
    NaiveDateTime,
    NaiveDate
);

// unbox the binary copy result
pub struct MyBinaryCopyOutRow {
    buf: Bytes,
    ranges: Vec<Option<Range<usize>>>,
    _types: Arc<Vec<Type>>,
}

impl<'a> Produce<Bytes> for PostgresSourceParser<'a> {
    fn produce(&mut self) -> Result<Bytes> {
        let (ridx, cidx) = self.next_loc()?;
        let row = &self.rowbuf[ridx];
        let row: &MyBinaryCopyOutRow = unsafe { transmute(row) };
        let val = row.ranges[cidx]
            .clone()
            .map(|rg| row.buf.slice(rg))
            .unwrap();

        Ok(val)
    }
}

impl<'a> Produce<Option<Bytes>> for PostgresSourceParser<'a> {
    fn produce(&mut self) -> Result<Option<Bytes>> {
        let (ridx, cidx) = self.next_loc()?;
        let row = &self.rowbuf[ridx];
        let row: &MyBinaryCopyOutRow = unsafe { transmute(row) };
        let val = row.ranges[cidx].clone().map(|rg| row.buf.slice(rg));

        Ok(val)
    }
}
