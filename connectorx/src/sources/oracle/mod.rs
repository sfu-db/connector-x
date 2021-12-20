mod errors;
mod typesystem;

use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::{PartitionParser, Produce, Source, SourcePartition},
    sql::{count_query, get_limit, CXQuery},
};
use anyhow::anyhow;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use fehler::{throw, throws};
use log::debug;
use r2d2::{Pool, PooledConnection};
use r2d2_oracle::{oracle::Row, OracleConnectionManager};
use url::Url;
type OracleManager = OracleConnectionManager;
type OracleConn = PooledConnection<OracleManager>;

pub use self::errors::OracleSourceError;
use crate::sql::limit1_query_oracle;
use r2d2_oracle::oracle::ResultSet;
use sqlparser::dialect::Dialect;
use std::env;
pub use typesystem::OracleTypeSystem;
use urlencoding::decode;

#[derive(Debug)]
pub struct OracleDialect {}

// implementation copy from AnsiDialect
impl Dialect for OracleDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch)
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch)
            || ('A'..='Z').contains(&ch)
            || ('0'..='9').contains(&ch)
            || ch == '_'
    }
}

pub struct OracleSource {
    pool: Pool<OracleManager>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<OracleTypeSystem>,
    buf_size: usize,
}

impl OracleSource {
    #[throws(OracleSourceError)]
    pub fn new(conn: &str, nconn: usize) -> Self {
        let conn = Url::parse(conn)?;
        let user = decode(conn.username().unwrap_or(""))?.into_owned();
        let password = decode(conn.password().unwrap_or(""))?.into_owned();
        let host = "//".to_owned()
            + decode(conn.host_str().unwrap_or("localhost"))?
                .into_owned()
                .as_str()
            + conn.path();

        let mut connector = oracle::Connector::new(user.as_str(), password.as_str(), host.as_str());

        if user.is_empty() && password.is_empty() && host == "//localhost" {
            debug!("No username or password provided, assuming system auth.");
            connector.external_auth(true);
        }

        let manager = OracleConnectionManager::from_connector(connector);
        let pool = r2d2::Pool::builder()
            .max_size(nconn as u32)
            .build(manager)?;

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

impl Source for OracleSource
where
    OracleSourcePartition:
        SourcePartition<TypeSystem = OracleTypeSystem, Error = OracleSourceError>,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = OracleSourcePartition;
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

        let conn = self.pool.get()?;
        for (i, query) in self.queries.iter().enumerate() {
            // assuming all the partition queries yield same schema
            match conn.query(limit1_query_oracle(query)?.as_str(), &[]) {
                Ok(rows) => {
                    let (names, types) = rows
                        .column_info()
                        .iter()
                        .map(|col| {
                            (
                                col.name().to_string(),
                                OracleTypeSystem::from(col.oracle_type()),
                            )
                        })
                        .unzip();
                    self.names = names;
                    self.schema = types;
                    return;
                }
                Err(e) if i == self.queries.len() - 1 => {
                    // tried the last query but still get an error
                    debug!("cannot get metadata for '{}': {}", query, e);
                    throw!(e);
                }
                Err(_) => {}
            }
        }
        // tried all queries but all get empty result set
        let iter = conn.query(self.queries[0].as_str(), &[])?;
        let (names, types) = iter
            .column_info()
            .iter()
            .map(|col| (col.name().to_string(), OracleTypeSystem::VarChar(false)))
            .unzip();
        self.names = names;
        self.schema = types;
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

pub struct OracleSourcePartition {
    conn: OracleConn,
    query: CXQuery<String>,
    schema: Vec<OracleTypeSystem>,
    nrows: usize,
    ncols: usize,
    buf_size: usize,
}

impl OracleSourcePartition {
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
        }
    }
}

impl SourcePartition for OracleSourcePartition {
    type TypeSystem = OracleTypeSystem;
    type Parser<'a> = OracleTextSourceParser<'a>;
    type Error = OracleSourceError;

    #[throws(OracleSourceError)]
    fn prepare(&mut self) {
        self.nrows = match get_limit(&self.query, &OracleDialect {})? {
            None => {
                let row = self.conn.query_row_as::<usize>(
                    &count_query(&self.query, &OracleDialect {})?.as_str(),
                    &[],
                )?;
                row
            }
            Some(n) => n,
        };
    }

    #[throws(OracleSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        let query = self.query.clone();
        let iter = self.conn.query(query.as_str(), &[])?;
        OracleTextSourceParser::new(iter, &self.schema, self.buf_size)
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

pub struct OracleTextSourceParser<'a> {
    iter: ResultSet<'a, Row>,
    buf_size: usize,
    rowbuf: Vec<Row>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
}

impl<'a> OracleTextSourceParser<'a> {
    pub fn new(iter: ResultSet<'a, Row>, schema: &[OracleTypeSystem], buf_size: usize) -> Self {
        Self {
            iter,
            buf_size,
            rowbuf: Vec::with_capacity(buf_size),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
        }
    }

    #[throws(OracleSourceError)]
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
                throw!(anyhow!("Oracle EOF"));
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
                    let res = self.rowbuf[ridx].get(cidx)?;
                    res
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for OracleTextSourceParser<'a> {
                type Error = OracleSourceError;

                #[throws(OracleSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let res = self.rowbuf[ridx].get(cidx)?;
                    res
                }
            }
        )+
    };
}

impl_produce_text!(i64, f64, String, NaiveDate, NaiveDateTime, DateTime<Utc>,);
