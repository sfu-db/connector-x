//! Source implementation for SQL Server.

mod errors;
mod typesystem;

pub use self::errors::MsSQLSourceError;
pub use self::typesystem::{FloatN, IntN, MsSQLTypeSystem};
use crate::constants::DB_BUFFER_SIZE;
use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::{PartitionParser, Produce, Source, SourcePartition},
    sql::{count_query, CXQuery},
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
use std::collections::HashMap;
use std::sync::Arc;
use tiberius::{AuthMethod, Config, EncryptionLevel, QueryItem, QueryStream, Row};
use tokio::runtime::{Handle, Runtime};
use url::Url;
use urlencoding::decode;
use uuid_old::Uuid;

type Conn<'a> = PooledConnection<'a, ConnectionManager>;
pub struct MsSQLSource {
    rt: Arc<Runtime>,
    pool: Pool<ConnectionManager>,
    origin_query: Option<String>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<MsSQLTypeSystem>,
}

#[throws(MsSQLSourceError)]
pub fn mssql_config(url: &Url) -> Config {
    let mut config = Config::new();

    let host = decode(url.host_str().unwrap_or("localhost"))?.into_owned();
    let hosts: Vec<&str> = host.split('\\').collect();
    match hosts.len() {
        1 => config.host(host),
        2 => {
            // SQL Server support instance name: `server\instance:port`
            config.host(hosts[0]);
            config.instance_name(hosts[1]);
        }
        _ => throw!(anyhow!("MsSQL hostname parse error: {}", host)),
    }
    config.port(url.port().unwrap_or(1433));
    // remove the leading "/"
    config.database(decode(&url.path()[1..])?.to_owned());
    // Using SQL Server authentication.
    #[allow(unused)]
    let params: HashMap<String, String> = url.query_pairs().into_owned().collect();
    #[cfg(any(windows, feature = "integrated-auth-gssapi"))]
    match params.get("trusted_connection") {
        // pefer trusted_connection if set to true
        Some(v) if v == "true" => {
            debug!("mssql auth through trusted connection!");
            config.authentication(AuthMethod::Integrated);
        }
        _ => {
            debug!("mssql auth through sqlserver authentication");
            config.authentication(AuthMethod::sql_server(
                decode(url.username())?.to_owned(),
                decode(url.password().unwrap_or(""))?.to_owned(),
            ));
        }
    };
    #[cfg(all(not(windows), not(feature = "integrated-auth-gssapi")))]
    config.authentication(AuthMethod::sql_server(
        decode(url.username())?.to_owned(),
        decode(url.password().unwrap_or(""))?.to_owned(),
    ));

    match params.get("trust_server_certificate") {
        Some(v) if v.to_lowercase() == "true" => config.trust_cert(),
        _ => {}
    };

    match params.get("trust_server_certificate_ca") {
        Some(v) => config.trust_cert_ca(v),
        _ => {}
    };

    match params.get("encrypt") {
        Some(v) if v.to_lowercase() == "true" => config.encryption(EncryptionLevel::Required),
        Some(v) if v.to_lowercase() == "false" => config.encryption(EncryptionLevel::Off),
        _ => config.encryption(EncryptionLevel::NotSupported),
    };

    match params.get("appname") {
        Some(appname) => config.application_name(decode(appname)?.to_owned()),
        _ => {}
    };

    config
}

impl MsSQLSource {
    #[throws(MsSQLSourceError)]
    pub fn new(rt: Arc<Runtime>, conn: &str, nconn: usize) -> Self {
        let url = Url::parse(conn)?;
        let config = mssql_config(&url)?;
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
        let first_query = &self.queries[0];
        let (names, types) = match self.rt.block_on(conn.query(first_query.as_str(), &[])) {
            Ok(mut stream) => match self.rt.block_on(async { stream.columns().await }) {
                Ok(Some(columns)) => columns
                    .iter()
                    .map(|col| {
                        (
                            col.name().to_string(),
                            MsSQLTypeSystem::from(&col.column_type()),
                        )
                    })
                    .unzip(),
                Ok(None) => {
                    throw!(anyhow!(
                        "MsSQL returned no columns for query: {}",
                        first_query
                    ));
                }
                Err(e) => {
                    throw!(anyhow!("Error fetching columns: {}", e));
                }
            },
            Err(e) => {
                debug!(
                    "cannot get metadata for '{}', try next query: {}",
                    first_query, e
                );
                throw!(e);
            }
        };

        self.names = names;
        self.schema = types;
    }

    #[throws(MsSQLSourceError)]
    fn result_rows(&mut self) -> Option<usize> {
        match &self.origin_query {
            Some(q) => {
                let cxq = CXQuery::Naked(q.clone());
                let cquery = count_query(&cxq, &MsSqlDialect {})?;
                let mut conn = self.rt.block_on(self.pool.get())?;

                let stream = self.rt.block_on(conn.query(cquery.as_str(), &[]))?;
                let row = self
                    .rt
                    .block_on(stream.into_row())?
                    .ok_or_else(|| anyhow!("MsSQL failed to get the count of query: {}", q))?;

                let row: i32 = row.get(0).ok_or(MsSQLSourceError::GetNRowsFailed)?; // the count in mssql is i32
                Some(row as usize)
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
    fn result_rows(&mut self) {
        let cquery = count_query(&self.query, &MsSqlDialect {})?;
        let mut conn = self.rt.block_on(self.pool.get())?;

        let stream = self.rt.block_on(conn.query(cquery.as_str(), &[]))?;
        let row = self
            .rt
            .block_on(stream.into_row())?
            .ok_or_else(|| anyhow!("MsSQL failed to get the count of query: {}", self.query))?;

        let row: i32 = row.get(0).ok_or(MsSQLSourceError::GetNRowsFailed)?; // the count in mssql is i32
        self.nrows = row as usize;
    }

    #[throws(MsSQLSourceError)]
    fn parser<'a>(&'a mut self) -> Self::Parser<'a> {
        let conn = self.rt.block_on(self.pool.get())?;
        let rows: OwningHandle<Box<Conn<'a>>, DummyBox<QueryStream<'a>>> =
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
    iter: OwningHandle<Box<Conn<'a>>, DummyBox<QueryStream<'a>>>,
    rowbuf: Vec<Row>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
    is_finished: bool,
}

impl<'a> MsSQLSourceParser<'a> {
    fn new(
        rt: &'a Handle,
        iter: OwningHandle<Box<Conn<'a>>, DummyBox<QueryStream<'a>>>,
        schema: &[MsSQLTypeSystem],
    ) -> Self {
        Self {
            rt,
            iter,
            rowbuf: Vec::with_capacity(DB_BUFFER_SIZE),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
            is_finished: false,
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
            if let Some(item) = self.rt.block_on(self.iter.next()) {
                match item.map_err(MsSQLSourceError::MsSQLError)? {
                    QueryItem::Row(row) => self.rowbuf.push(row),
                    _ => continue,
                }
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
