mod errors;
mod typesystem;

use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::{PartitionParser, Produce, Source, SourcePartition},
    sql::{count_query, limit1_query_oracle, CXQuery},
};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use fehler::{throw, throws};
use log::debug;
use r2d2::{Pool, PooledConnection};
use r2d2_oracle::{oracle::Row, OracleConnectionManager};
use url::Url;
type OracleManager = OracleConnectionManager;
type OracleConn = PooledConnection<OracleManager>;

pub use self::errors::OracleSourceError;
use crate::constants::DB_BUFFER_SIZE;
use r2d2_oracle::oracle::ResultSet;
use sqlparser::dialect::Dialect;
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
    origin_query: Option<String>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<OracleTypeSystem>,
}

impl OracleSource {
    #[throws(OracleSourceError)]
    pub fn new(conn: &str, nconn: usize) -> Self {
        let conn = Url::parse(conn)?;
        let user = decode(conn.username())?.into_owned();
        let password = decode(conn.password().unwrap_or(""))?.into_owned();
        let host = "//".to_owned()
            + decode(conn.host_str().unwrap_or("localhost"))?
                .into_owned()
                .as_str()
            + conn.path();
        let manager = OracleConnectionManager::new(user.as_str(), password.as_str(), host.as_str());
        let pool = r2d2::Pool::builder()
            .max_size(nconn as u32)
            .build(manager)?;

        Self {
            pool,
            origin_query: None,
            queries: vec![],
            names: vec![],
            schema: vec![],
        }
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

    fn set_origin_query(&mut self, query: Option<String>) {
        self.origin_query = query;
    }

    #[throws(OracleSourceError)]
    fn fetch_metadata(&mut self) {
        assert!(!self.queries.is_empty());

        let conn = self.pool.get()?;
        for (i, query) in self.queries.iter().enumerate() {
            // assuming all the partition queries yield same schema
            // without rownum = 1, derived type might be wrong
            // example: select avg(test_int), test_char from test_table group by test_char
            // -> (NumInt, Char) instead of (NumtFloat, Char)
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

    #[throws(OracleSourceError)]
    fn result_rows(&mut self) -> Option<usize> {
        match &self.origin_query {
            Some(q) => {
                let cxq = CXQuery::Naked(q.clone());
                let conn = self.pool.get()?;

                let nrows = conn
                    .query_row_as::<usize>(count_query(&cxq, &OracleDialect {})?.as_str(), &[])?;
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

    #[throws(OracleSourceError)]
    fn partition(self) -> Vec<Self::Partition> {
        let mut ret = vec![];
        for query in self.queries {
            let conn = self.pool.get()?;
            ret.push(OracleSourcePartition::new(conn, &query, &self.schema));
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
}

impl OracleSourcePartition {
    pub fn new(conn: OracleConn, query: &CXQuery<String>, schema: &[OracleTypeSystem]) -> Self {
        Self {
            conn,
            query: query.clone(),
            schema: schema.to_vec(),
            nrows: 0,
            ncols: schema.len(),
        }
    }
}

impl SourcePartition for OracleSourcePartition {
    type TypeSystem = OracleTypeSystem;
    type Parser<'a> = OracleTextSourceParser<'a>;
    type Error = OracleSourceError;

    #[throws(OracleSourceError)]
    fn result_rows(&mut self) {
        self.nrows = self
            .conn
            .query_row_as::<usize>(count_query(&self.query, &OracleDialect {})?.as_str(), &[])?;

    }

    #[throws(OracleSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        let query = self.query.clone();
        let iter = self.conn.query(query.as_str(), &[])?;
        OracleTextSourceParser::new(iter, &self.schema)
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
    rowbuf: Vec<Row>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
}

impl<'a> OracleTextSourceParser<'a> {
    pub fn new(iter: ResultSet<'a, Row>, schema: &[OracleTypeSystem]) -> Self {
        Self {
            iter,
            rowbuf: Vec::with_capacity(DB_BUFFER_SIZE),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
        }
    }

    #[throws(OracleSourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        ret
    }
}

impl<'a> PartitionParser<'a> for OracleTextSourceParser<'a> {
    type TypeSystem = OracleTypeSystem;
    type Error = OracleSourceError;

    #[throws(OracleSourceError)]
    fn fetch_next(&mut self) -> (usize, bool) {
        if !self.rowbuf.is_empty() {
            self.rowbuf.drain(..);
        }
        for _ in 0..DB_BUFFER_SIZE {
            if let Some(item) = self.iter.next() {
                self.rowbuf.push(item?);
            } else {
                break;
            }
        }
        self.current_row = 0;
        self.current_col = 0;
        (self.rowbuf.len(), self.rowbuf.len() < DB_BUFFER_SIZE)
    }
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
