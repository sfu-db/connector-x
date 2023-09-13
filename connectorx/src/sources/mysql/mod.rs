//! Source implementation for MySQL database.

mod errors;
mod typesystem;

pub use self::errors::MySQLSourceError;
use crate::constants::DB_BUFFER_SIZE;
use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::{PartitionParser, Produce, Source, SourcePartition},
    sql::{count_query, limit1_query, CXQuery},
};
use anyhow::anyhow;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use fehler::{throw, throws};
use log::{debug, warn};
use r2d2::{Pool, PooledConnection};
use r2d2_mysql::{
    mysql::{prelude::Queryable, Binary, Opts, OptsBuilder, QueryResult, Row, Text},
    MySqlConnectionManager,
};
use rust_decimal::Decimal;
use serde_json::Value;
use sqlparser::dialect::MySqlDialect;
use std::marker::PhantomData;
pub use typesystem::MySQLTypeSystem;

type MysqlConn = PooledConnection<MySqlConnectionManager>;

pub enum BinaryProtocol {}
pub enum TextProtocol {}

#[throws(MySQLSourceError)]
fn get_total_rows(conn: &mut MysqlConn, query: &CXQuery<String>) -> usize {
    conn.query_first(&count_query(query, &MySqlDialect {})?)?
        .ok_or_else(|| anyhow!("mysql failed to get the count of query: {}", query))?
}

pub struct MySQLSource<P> {
    pool: Pool<MySqlConnectionManager>,
    origin_query: Option<String>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<MySQLTypeSystem>,
    _protocol: PhantomData<P>,
}

impl<P> MySQLSource<P> {
    #[throws(MySQLSourceError)]
    pub fn new(conn: &str, nconn: usize) -> Self {
        let manager = MySqlConnectionManager::new(OptsBuilder::from_opts(Opts::from_url(conn)?));
        let pool = r2d2::Pool::builder()
            .max_size(nconn as u32)
            .build(manager)?;

        Self {
            pool,
            origin_query: None,
            queries: vec![],
            names: vec![],
            schema: vec![],
            _protocol: PhantomData,
        }
    }
}

impl<P> Source for MySQLSource<P>
where
    MySQLSourcePartition<P>:
        SourcePartition<TypeSystem = MySQLTypeSystem, Error = MySQLSourceError>,
    P: Send,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = MySQLSourcePartition<P>;
    type TypeSystem = MySQLTypeSystem;
    type Error = MySQLSourceError;

    #[throws(MySQLSourceError)]
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

    #[throws(MySQLSourceError)]
    fn fetch_metadata(&mut self) {
        assert!(!self.queries.is_empty());

        let mut conn = self.pool.get()?;
        let first_query = &self.queries[0];

        match conn.prep(first_query) {
            Ok(stmt) => {
                let (names, types) = stmt
                    .columns()
                    .iter()
                    .map(|col| {
                        (
                            col.name_str().to_string(),
                            MySQLTypeSystem::from((&col.column_type(), &col.flags())),
                        )
                    })
                    .unzip();
                self.names = names;
                self.schema = types;
            }
            Err(e) => {
                warn!(
                    "mysql text prepared statement error: {:?}, switch to limit1 method",
                    e
                );
                for (i, query) in self.queries.iter().enumerate() {
                    // assuming all the partition queries yield same schema
                    match conn
                        .query_first::<Row, _>(limit1_query(query, &MySqlDialect {})?.as_str())
                    {
                        Ok(Some(row)) => {
                            let (names, types) = row
                                .columns_ref()
                                .iter()
                                .map(|col| {
                                    (
                                        col.name_str().to_string(),
                                        MySQLTypeSystem::from((&col.column_type(), &col.flags())),
                                    )
                                })
                                .unzip();
                            self.names = names;
                            self.schema = types;
                            return;
                        }
                        Ok(None) => {}
                        Err(e) if i == self.queries.len() - 1 => {
                            // tried the last query but still get an error
                            debug!("cannot get metadata for '{}', try next query: {}", query, e);
                            throw!(e)
                        }
                        Err(_) => {}
                    }
                }

                // tried all queries but all get empty result set
                let iter = conn.query_iter(self.queries[0].as_str())?;
                let (names, types) = iter
                    .columns()
                    .as_ref()
                    .iter()
                    .map(|col| {
                        (
                            col.name_str().to_string(),
                            MySQLTypeSystem::VarChar(false), // set all columns as string (align with pandas)
                        )
                    })
                    .unzip();
                self.names = names;
                self.schema = types;
            }
        }
    }

    #[throws(MySQLSourceError)]
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

    #[throws(MySQLSourceError)]
    fn partition(self) -> Vec<Self::Partition> {
        let mut ret = vec![];
        for query in self.queries {
            let conn = self.pool.get()?;
            ret.push(MySQLSourcePartition::new(conn, &query, &self.schema));
        }
        ret
    }
}

pub struct MySQLSourcePartition<P> {
    conn: MysqlConn,
    query: CXQuery<String>,
    schema: Vec<MySQLTypeSystem>,
    nrows: usize,
    ncols: usize,
    _protocol: PhantomData<P>,
}

impl<P> MySQLSourcePartition<P> {
    pub fn new(conn: MysqlConn, query: &CXQuery<String>, schema: &[MySQLTypeSystem]) -> Self {
        Self {
            conn,
            query: query.clone(),
            schema: schema.to_vec(),
            nrows: 0,
            ncols: schema.len(),
            _protocol: PhantomData,
        }
    }
}

impl SourcePartition for MySQLSourcePartition<BinaryProtocol> {
    type TypeSystem = MySQLTypeSystem;
    type Parser<'a> = MySQLBinarySourceParser<'a>;
    type Error = MySQLSourceError;

    #[throws(MySQLSourceError)]
    fn result_rows(&mut self) {
        self.nrows = get_total_rows(&mut self.conn, &self.query)?;
    }

    #[throws(MySQLSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        let stmt = self.conn.prep(self.query.as_str())?;
        let iter = self.conn.exec_iter(stmt, ())?;
        MySQLBinarySourceParser::new(iter, &self.schema)
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

impl SourcePartition for MySQLSourcePartition<TextProtocol> {
    type TypeSystem = MySQLTypeSystem;
    type Parser<'a> = MySQLTextSourceParser<'a>;
    type Error = MySQLSourceError;

    #[throws(MySQLSourceError)]
    fn result_rows(&mut self) {
        self.nrows = get_total_rows(&mut self.conn, &self.query)?;
    }

    #[throws(MySQLSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        let query = self.query.clone();
        let iter = self.conn.query_iter(query)?;
        MySQLTextSourceParser::new(iter, &self.schema)
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

pub struct MySQLBinarySourceParser<'a> {
    iter: QueryResult<'a, 'a, 'a, Binary>,
    rowbuf: Vec<Row>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
    is_finished: bool,
}

impl<'a> MySQLBinarySourceParser<'a> {
    pub fn new(iter: QueryResult<'a, 'a, 'a, Binary>, schema: &[MySQLTypeSystem]) -> Self {
        Self {
            iter,
            rowbuf: Vec::with_capacity(DB_BUFFER_SIZE),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
            is_finished: false,
        }
    }

    #[throws(MySQLSourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        ret
    }
}

impl<'a> PartitionParser<'a> for MySQLBinarySourceParser<'a> {
    type TypeSystem = MySQLTypeSystem;
    type Error = MySQLSourceError;

    #[throws(MySQLSourceError)]
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
            if let Some(item) = self.iter.next() {
                self.rowbuf.push(item?);
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

macro_rules! impl_produce_binary {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for MySQLBinarySourceParser<'a> {
                type Error = MySQLSourceError;

                #[throws(MySQLSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    let res = self.rowbuf[ridx].take(cidx).ok_or_else(|| anyhow!("mysql cannot parse at position: ({}, {})", ridx, cidx))?;
                    res
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for MySQLBinarySourceParser<'a> {
                type Error = MySQLSourceError;

                #[throws(MySQLSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let res = self.rowbuf[ridx].take(cidx).ok_or_else(|| anyhow!("mysql cannot parse at position: ({}, {})", ridx, cidx))?;
                    res
                }
            }
        )+
    };
}

impl_produce_binary!(
    i8,
    i16,
    i32,
    i64,
    u8,
    u16,
    u32,
    u64,
    f32,
    f64,
    NaiveDate,
    NaiveTime,
    NaiveDateTime,
    Decimal,
    String,
    Vec<u8>,
    Value,
);

pub struct MySQLTextSourceParser<'a> {
    iter: QueryResult<'a, 'a, 'a, Text>,
    rowbuf: Vec<Row>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
    is_finished: bool,
}

impl<'a> MySQLTextSourceParser<'a> {
    pub fn new(iter: QueryResult<'a, 'a, 'a, Text>, schema: &[MySQLTypeSystem]) -> Self {
        Self {
            iter,
            rowbuf: Vec::with_capacity(DB_BUFFER_SIZE),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
            is_finished: false,
        }
    }

    #[throws(MySQLSourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        ret
    }
}

impl<'a> PartitionParser<'a> for MySQLTextSourceParser<'a> {
    type TypeSystem = MySQLTypeSystem;
    type Error = MySQLSourceError;

    #[throws(MySQLSourceError)]
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
            if let Some(item) = self.iter.next() {
                self.rowbuf.push(item?);
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

macro_rules! impl_produce_text {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for MySQLTextSourceParser<'a> {
                type Error = MySQLSourceError;

                #[throws(MySQLSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    let res = self.rowbuf[ridx].take(cidx).ok_or_else(|| anyhow!("mysql cannot parse at position: ({}, {})", ridx, cidx))?;
                    res
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for MySQLTextSourceParser<'a> {
                type Error = MySQLSourceError;

                #[throws(MySQLSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let res = self.rowbuf[ridx].take(cidx).ok_or_else(|| anyhow!("mysql cannot parse at position: ({}, {})", ridx, cidx))?;
                    res
                }
            }
        )+
    };
}

impl_produce_text!(
    i8,
    i16,
    i32,
    i64,
    u8,
    u16,
    u32,
    u64,
    f32,
    f64,
    NaiveDate,
    NaiveTime,
    NaiveDateTime,
    Decimal,
    String,
    Vec<u8>,
    Value,
);
