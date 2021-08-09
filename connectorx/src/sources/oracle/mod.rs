mod errors;
mod typesystem;

use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::{PartitionParser, Produce, Source, SourcePartition},
    sql::{count_query, get_limit, limit1_query, CXQuery},
};
use anyhow::anyhow;
use url::Url;
use r2d2::{Pool, PooledConnection};
use r2d2_oracle::{oracle::Row, OracleConnectionManager};
type OracleManager = OracleConnectionManager;
type OracleConn = PooledConnection<OracleManager>;

pub use self::errors::OracleSourceError;
use sqlparser::dialect::GenericDialect;
use std::marker::PhantomData;
pub use typesystem::OracleTypeSystem;
use r2d2_oracle::oracle::ResultSet;

pub enum TextProtocol {}

pub struct OracleSource<P> {
    pool: Pool<OracleManager>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<OracleTypeSystem>,
    buf_size: usize,
    _protocol: PhantomData<P>,
}

impl<P> OracleSource<P> {
    #[throws(OracleSourceError)]
    pub fn new(conn: &str, nconn: usize) -> Self {
        let conn = Url::parse(conn).unwrap();
        let user = conn.username();
        let password = conn.password().unwrap();
        let host = "//".to_owned() + conn.host_str().unwrap() + conn.path();
        let manager = OracleConnectionManager::new(user, password, host.as_str());
        let pool = r2d2::Pool::builder()
            .max_size(nconn as u32)
            .build(manager)?;

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

impl<P> Source for OracleSource<P>
where
    OracleSourcePartition<P>:
        SourcePartition<TypeSystem = OracleTypeSystem, Error = OracleSourceError>,
    P: Send,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = OracleSourcePartition<P>;
    type TypeSystem = OracleTypeSystem;
    type Error = OracleSourceError;

    #[throws(OracleSourceError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorXError::UnsupportedDataOrder(data_order));
        }
    }

    fn set_queries<Q: ToString>(&mut self, queries: &[CXQuery<Q>]) {
        self.queries = queries.iter().map(|q| q.map(Q::to_string)).collect();
    }

    #[throws(OracleSourceError)]
    fn fetch_metadata(&mut self) {
        assert!(!self.queries.is_empty());

        let mut conn = self.pool.get()?;
        let mut success = false;
        let mut zero_tuple = true;
        let mut error = None;
        for query in &self.queries {
            // assuming all the partition queries yield same schema
            let mut stmt = conn.prepare(query.as_ref(), &[]).unwrap();
            match stmt.query(&[]).unwrap() {
                Ok(rows) => {
                    let (names, types) = rows
                        .column_info()
                        .iter()
                        .map(|col| (col.name(), col.oracle_type()))
                        .unzip();
                    self.names = names;
                    self.schema = types;
                    success = true;
                    zero_tuple = false;
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
                let iter = conn.query_iter(self.queries[0].as_str())?;
                let (names, types) = iter
                    .column_info()
                    .iter()
                    .map(|col| (col.name(), string_type))
                    .unzip();
                self.names = names;
                self.schema = types;
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

    #[throws(OracleSourceError)]
    fn partition(self) -> Vec<Self::Partition> {
        let mut ret = vec![];
        for query in self.queries {
            let conn = self.pool.get()?;
            ret.push(OracleSourcePartition::new(
                conn,
                &query,
                &self.schema,
                self.buf_size,
            ));
        }
        ret
    }
}

pub struct OracleSourcePartition<P> {
    conn: OracleConn,
    query: CXQuery<String>,
    schema: Vec<OracleTypeSystem>,
    nrows: usize,
    ncols: usize,
    buf_size: usize,
    _protocol: PhantomData<P>,
}

impl<P> OracleSourcePartition<P> {
    pub fn new(
        conn: OracleConn,
        query: &CXQuery<String>,
        schema: &[OracleTypeSystem],
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

impl SourcePartition for OracleSourcePartition<TextProtocol> {
    type TypeSystem = OracleTypeSystem;
    type Parser<'a> = OracleTextSourceParser<'a>;
    type Error = OracleSourceError;

    #[throws(OracleSourceError)]
    fn prepare(&mut self) {
        self.nrows = match get_limit(&self.query, &GenericDialect {})? {
            None => {
                let row = self
                    .conn
                    .query_row_as::<usize>(&count_query(&self.query, &GenericDialect {})?, &[])?;
                row
            }
            Some(n) => n,
        };
    }

    #[throws(MySQLSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        let query = self.query.clone();
        let iter = self.conn.query(query.as_str(), &[]).unwrap();
        OracleTextSourceParser::new(iter, &self.schema, self.buf_size)
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

pub struct MySQLTextSourceParser<'a> {
    iter: ResultSet<'a, Row>,
    buf_size: usize,
    rowbuf: Vec<Row>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
}

impl<'a> MySQLTextSourceParser<'a> {
    pub fn new(iter: ResultSet<Row>, schema: &[OracleTypeSystem], buf_size: usize) -> Self {
        Self {
            iter,
            buf_size,
            rowbuf: Vec::with_capacity(buf_size),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
        }
    }

    #[throws(MySQLSourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        if self.current_row >= self.rowbuf.len() {
            if !self.rowbuf.is_empty() {
                self.rowbuf.drain(..);
            }

            for _ in 0..self.buf_size {
                if let Some(item) = self.iter.next() {
                    self.rowbuf.push(item?);
                } else {
                    break;
                }
            }

            if self.rowbuf.is_empty() {
                throw!(anyhow!("Mysql EOF"));
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

impl<'a> PartitionParser<'a> for OracleTextSourceParser<'a> {
    type TypeSystem = OracleTypeSystem;
    type Error = OracleSourceError;
}

macro_rules! impl_produce_text {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for OracleTextSourceParser<'a> {
                type Error = OracleSourceError;

                #[throws(OracleSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    let res = self.rowbuf[ridx].get(cidx).unwrap_or_else(|| anyhow!("oracle get None at position: ({}, {})", ridx, cidx))?;
                    res
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for OracleTextSourceParser<'a> {
                type Error = OracleSourceError;

                #[throws(OracleSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let res = self.rowbuf[ridx].get(cidx);
                    res
                }
            }
        )+
    };
}

impl_produce_text!(
    i64,
);
