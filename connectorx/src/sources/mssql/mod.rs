//! Source implementation for SQL Server.

mod errors;
mod typesystem;

pub use self::errors::MsSQLSourceError;
pub use self::typesystem::{FloatN, IntN, MsSQLTypeSystem};
use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::{PartitionParser, Produce, Source, SourcePartition},
    sql::{count_query, get_limit_mssql, limit1_query_mssql, CXQuery},
    utils::DummyBox,
};
use anyhow::anyhow;
use bb8::{Pool, PooledConnection};
use bb8_tiberius::ConnectionManager;
use chrono::{DateTime, Utc};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use fehler::{throw, throws};
use futures::StreamExt;
use log::debug;
use owning_ref::OwningHandle;
use rust_decimal::Decimal;
use sqlparser::dialect::MsSqlDialect;
use std::sync::Arc;
use tiberius::{AuthMethod, Config, EncryptionLevel, QueryResult, Row};
use tokio::runtime::{Handle, Runtime};
use url::Url;
use urlencoding::decode;
use uuid::Uuid;

type Conn<'a> = PooledConnection<'a, ConnectionManager>;

const BUF_SIZE: usize = 32;

pub struct MsSQLSource {
    rt: Arc<Runtime>,
    pool: Pool<ConnectionManager>,
    origin_query: Option<String>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<MsSQLTypeSystem>,
}

impl MsSQLSource {
    #[throws(MsSQLSourceError)]
    pub fn new(rt: Arc<Runtime>, conn: &str, nconn: usize) -> Self {
        let mut config = Config::new();
        let url = Url::parse(conn)?;

        config.host(decode(url.host_str().unwrap_or("localhost"))?.into_owned());
        config.port(url.port().unwrap_or(1433));

        // Using SQL Server authentication.
        config.authentication(AuthMethod::sql_server(
            decode(url.username())?.to_owned(),
            decode(url.password().unwrap_or(""))?.to_owned(),
        ));

        config.database(&url.path()[1..]); // remove the leading "/"
        config.encryption(EncryptionLevel::NotSupported);

        let manager = bb8_tiberius::ConnectionManager::new(config);
        let pool = rt.block_on(Pool::builder().max_size(nconn as u32).build(manager))?;

        Self {
            rt,
            pool,
            origin_query: None,
            queries: vec![],
            names: vec![],
            schema: vec![],
        }
    }
}

impl Source for MsSQLSource
where
    MsSQLSourcePartition: SourcePartition<TypeSystem = MsSQLTypeSystem, Error = MsSQLSourceError>,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = MsSQLSourcePartition;
    type TypeSystem = MsSQLTypeSystem;
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
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

    #[throws(MsSQLSourceError)]
    fn fetch_metadata(&mut self) {
        assert!(!self.queries.is_empty());

        let mut conn = self.rt.block_on(self.pool.get())?;

        for (i, query) in self.queries.iter().enumerate() {
            // assuming all the partition queries yield same schema
            let l1query = limit1_query_mssql(query)?;

            match self.rt.block_on(conn.query(l1query.as_str(), &[])) {
                Ok(stream) => {
                    let row = match self.rt.block_on(stream.into_row())? {
                        Some(row) => row,
                        None => continue, // this partition is empty.
                    };

                    let columns = row.columns();

                    let (names, types) = columns
                        .iter()
                        .map(|col| {
                            (
                                col.name().to_string(),
                                MsSQLTypeSystem::from(&col.column_type()),
                            )
                        })
                        .unzip();
                    self.names = names;
                    self.schema = types;
                    return;
                }
                Err(e) if i == self.queries.len() - 1 => {
                    // tried the last query but still get an error
                    debug!("cannot get metadata for '{}', try next query: {}", query, e);
                    throw!(e);
                }
                Err(_) => {}
            }
        }

        // tried all queries but all get empty result set
        let stream = self
            .rt
            .block_on(conn.query(self.queries[0].as_str(), &[]))?;

        let columns = stream.columns().expect("cannot get column information");

        let (names, types) = columns
            .iter()
            .map(|col| {
                (
                    col.name().to_string(),
                    MsSQLTypeSystem::from(&col.column_type()),
                )
            })
            .unzip();

        self.names = names;
        self.schema = types;
    }

    #[throws(MsSQLSourceError)]
    fn result_rows(&mut self) -> Option<usize> {
        match &self.origin_query {
            Some(q) => {
                let cxq = CXQuery::Wrapped(q.clone());
                let dialect = MsSqlDialect {};
                let nrows = match get_limit_mssql(&cxq)? {
                    None => {
                        let mut conn = self.rt.block_on(self.pool.get())?;
                        let cquery = count_query(&cxq, &dialect)?;
                        let stream = self.rt.block_on(conn.query(cquery.as_str(), &[]))?;
                        let row = self.rt.block_on(stream.into_row())?.ok_or_else(|| {
                            anyhow!("MsSQL failed to get the count of query: {}", q)
                        })?;

                        let row: i32 = row.get(0).ok_or(MsSQLSourceError::GetNRowsFailed)?; // the count in mssql is i32
                        row as usize
                    }
                    Some(n) => n,
                };
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

    #[throws(MsSQLSourceError)]
    fn partition(self) -> Vec<Self::Partition> {
        let mut ret = vec![];
        for query in self.queries {
            ret.push(MsSQLSourcePartition::new(
                self.pool.clone(),
                self.rt.clone(),
                &query,
                &self.schema,
            ));
        }
        ret
    }
}

pub struct MsSQLSourcePartition {
    pool: Pool<ConnectionManager>,
    rt: Arc<Runtime>,
    query: CXQuery<String>,
    schema: Vec<MsSQLTypeSystem>,
    nrows: usize,
    ncols: usize,
}

impl MsSQLSourcePartition {
    pub fn new(
        pool: Pool<ConnectionManager>,
        handle: Arc<Runtime>,
        query: &CXQuery<String>,
        schema: &[MsSQLTypeSystem],
    ) -> Self {
        Self {
            rt: handle,
            pool,
            query: query.clone(),
            schema: schema.to_vec(),
            nrows: 0,
            ncols: schema.len(),
        }
    }
}

impl SourcePartition for MsSQLSourcePartition {
    type TypeSystem = MsSQLTypeSystem;
    type Parser<'a> = MsSQLSourceParser<'a>;
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn prepare(&mut self) {
        self.nrows = match get_limit_mssql(&self.query)? {
            None => {
                let mut conn = self.rt.block_on(self.pool.get())?;
                let cquery = count_query(&self.query, &MsSqlDialect {})?;

                let stream = self.rt.block_on(conn.query(cquery.as_str(), &[]))?;
                let row = self.rt.block_on(stream.into_row())?.ok_or_else(|| {
                    anyhow!("MsSQL failed to get the count of query: {}", self.query)
                })?;

                let row: i32 = row.get(0).ok_or(MsSQLSourceError::GetNRowsFailed)?; // the count in mssql is i32
                row as usize
            }
            Some(n) => n,
        };
    }

    #[throws(MsSQLSourceError)]
    fn parser<'a>(&'a mut self) -> Self::Parser<'a> {
        let conn = self.rt.block_on(self.pool.get())?;
        let rows: OwningHandle<Box<Conn<'a>>, DummyBox<QueryResult<'a>>> =
            OwningHandle::new_with_fn(Box::new(conn), |conn: *const Conn<'a>| unsafe {
                let conn = &mut *(conn as *mut Conn<'a>);

                DummyBox(
                    self.rt
                        .block_on(conn.query(self.query.as_str(), &[]))
                        .unwrap(),
                )
            });

        MsSQLSourceParser::new(self.rt.handle(), rows, &self.schema)
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

pub struct MsSQLSourceParser<'a> {
    rt: &'a Handle,
    iter: OwningHandle<Box<Conn<'a>>, DummyBox<QueryResult<'a>>>,
    rowbuf: Vec<Row>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
}

impl<'a> MsSQLSourceParser<'a> {
    fn new(
        rt: &'a Handle,
        iter: OwningHandle<Box<Conn<'a>>, DummyBox<QueryResult<'a>>>,
        schema: &[MsSQLTypeSystem],
    ) -> Self {
        Self {
            rt,
            iter,
            rowbuf: Vec::with_capacity(BUF_SIZE),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
        }
    }

    #[throws(MsSQLSourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        ret
    }
}

impl<'a> PartitionParser<'a> for MsSQLSourceParser<'a> {
    type TypeSystem = MsSQLTypeSystem;
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn fetch_next(&mut self) -> (usize, bool) {
        if !self.rowbuf.is_empty() {
            self.rowbuf.drain(..);
        }

        for _ in 0..BUF_SIZE {
            if let Some(item) = self.rt.block_on(self.iter.next()) {
                self.rowbuf.push(item?);
            } else {
                break;
            }
        }
        self.current_row = 0;
        self.current_col = 0;
        (self.rowbuf.len(), self.rowbuf.len() < BUF_SIZE)
    }
}

macro_rules! impl_produce {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for MsSQLSourceParser<'a> {
                type Error = MsSQLSourceError;

                #[throws(MsSQLSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    let res = self.rowbuf[ridx].get(cidx).ok_or_else(|| anyhow!("MsSQL get None at position: ({}, {})", ridx, cidx))?;
                    res
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for MsSQLSourceParser<'a> {
                type Error = MsSQLSourceError;

                #[throws(MsSQLSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let res = self.rowbuf[ridx].get(cidx);
                    res
                }
            }
        )+
    };
}

impl_produce!(
    u8,
    i16,
    i32,
    i64,
    IntN,
    f32,
    f64,
    FloatN,
    bool,
    &'r str,
    &'r [u8],
    Uuid,
    Decimal,
    NaiveDateTime,
    NaiveDate,
    NaiveTime,
    DateTime<Utc>,
);
