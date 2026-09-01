use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use fehler::{throw, throws};
use prusto::{auth::Auth, Client, ClientBuilder, DataSet, Presto, Row};
use serde_json::Value;
use sqlparser::dialect::{Dialect, GenericDialect};
use std::convert::TryFrom;
use tokio::runtime::Runtime;

use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::Produce,
    sql::{count_query, limit0_query, CXQuery},
};

pub use self::{errors::TrinoSourceError, typesystem::TrinoTypeSystem};
use urlencoding::decode;

use super::{PartitionParser, Source, SourcePartition};

use anyhow::anyhow;

pub mod errors;
pub mod typesystem;

#[throws(TrinoSourceError)]
fn get_total_rows(rt: Arc<Runtime>, client: Arc<Client>, query: &CXQuery<String>) -> usize {
    let cquery = count_query(query, &TrinoDialect {})?;

    let row = rt
        .block_on(client.get_all::<Row>(cquery.to_string()))
        .map_err(TrinoSourceError::PrustoError)?
        .split()
        .1[0]
        .clone();

    let value = row
        .value()
        .first()
        .ok_or_else(|| anyhow!("Trino count dataset is empty"))?;

    value
        .as_i64()
        .ok_or_else(|| anyhow!("Trino cannot parse i64"))? as usize
}

#[derive(Presto, Debug)]
pub struct TrinoPartitionQueryResult {
    pub _col0: i64,
    pub _col1: i64,
}

#[derive(Debug)]
pub struct TrinoDialect {}

// implementation copy from AnsiDialect
impl Dialect for TrinoDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ch.is_ascii_lowercase() || ch.is_ascii_uppercase()
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '_'
    }
}

pub struct TrinoSource {
    client: Arc<Client>,
    rt: Arc<Runtime>,
    origin_query: Option<String>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<TrinoTypeSystem>,
}

/// Build a prusto Client from a Trino connection URL, parsing all supported query parameters.
///
/// Supported URL params:
///   source, schema, client_tags (comma-separated), client_info, trace_token,
///   session.<key>=<value>, extra_credential.<key>=<value>, verify=false
#[throws(TrinoSourceError)]
pub fn build_client_from_url(url: &url::Url) -> Client {
    let username = match url.username() {
        "" => "connectorx",
        username => username,
    };

    let no_verify = url
        .query_pairs()
        .any(|(k, v)| k == "verify" && v == "false");

    let mut builder = ClientBuilder::new(username, url.host().unwrap().to_owned())
        .port(url.port().unwrap_or(8080))
        .ssl(prusto::ssl::Ssl { root_cert: None })
        .no_verify(no_verify)
        .secure(url.scheme() == "trino+https")
        .catalog(url.path_segments().unwrap().last().unwrap_or("hive"));

    let mut session_props: HashMap<String, String> = HashMap::new();
    let mut extra_creds: HashMap<String, String> = HashMap::new();

    for (key, value) in url.query_pairs() {
        match key.as_ref() {
            "source" => builder = builder.source(value.as_ref()),
            "schema" => builder = builder.schema(value.as_ref()),
            "client_tags" => {
                for tag in value.split(',') {
                    let tag = tag.trim();
                    if !tag.is_empty() {
                        builder = builder.client_tag(tag);
                    }
                }
            }
            "client_info" => builder = builder.client_info(value.as_ref()),
            "trace_token" => builder = builder.trace_token(value.as_ref()),
            k if k.starts_with("session.") => {
                if let Some(prop) = k.strip_prefix("session.").filter(|s| !s.is_empty()) {
                    session_props.insert(prop.to_string(), value.to_string());
                }
            }
            k if k.starts_with("extra_credential.") => {
                if let Some(cred) = k
                    .strip_prefix("extra_credential.")
                    .filter(|s| !s.is_empty())
                {
                    extra_creds.insert(cred.to_string(), value.to_string());
                }
            }
            _ => {}
        }
    }

    if !session_props.is_empty() {
        builder = builder.properties(session_props);
    }
    if !extra_creds.is_empty() {
        builder = builder.extra_credentials(extra_creds);
    }

    let builder = match url.password() {
        None => builder,
        Some(password) => builder.auth(Auth::Basic(username.to_owned(), Some(password.to_owned()))),
    };

    builder.build().map_err(TrinoSourceError::PrustoError)?
}

impl TrinoSource {
    #[throws(TrinoSourceError)]
    pub fn new(rt: Arc<Runtime>, conn: &str) -> Self {
        let decoded_conn = decode(conn)?.into_owned();

        let url = decoded_conn
            .parse::<url::Url>()
            .map_err(TrinoSourceError::UrlParseError)?;

        let username = match url.username() {
            "" => "connectorx",
            username => username,
        };

        let no_verify = url
            .query_pairs()
            .any(|(k, v)| k == "verify" && v == "false");

        let builder = ClientBuilder::new(username, url.host().unwrap().to_owned())
            .port(url.port().unwrap_or(8080))
            .ssl(prusto::ssl::Ssl { root_cert: None })
            .no_verify(no_verify)
            .secure(url.scheme() == "trino+https")
            .catalog(url.path_segments().unwrap().last().unwrap_or("hive"));

        let builder = match url.password() {
            None => builder,
            Some(password) => {
                builder.auth(Auth::Basic(username.to_owned(), Some(password.to_owned())))
            }
        };

        let client = builder.build().map_err(TrinoSourceError::PrustoError)?;

        Self {
            client: Arc::new(client),
            rt,
            origin_query: None,
            queries: vec![],
            names: vec![],
            schema: vec![],
        }
    }
}

impl Source for TrinoSource
where
    TrinoSourcePartition: SourcePartition<TypeSystem = TrinoTypeSystem, Error = TrinoSourceError>,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type TypeSystem = TrinoTypeSystem;
    type Partition = TrinoSourcePartition;
    type Error = TrinoSourceError;

    #[throws(TrinoSourceError)]
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

    #[throws(TrinoSourceError)]
    fn fetch_metadata(&mut self) {
        assert!(!self.queries.is_empty());

        let first_query = &self.queries[0];
        let cxq = limit0_query(first_query, &GenericDialect {})?;

        let dataset: DataSet<Row> = self
            .rt
            .block_on(self.client.get_all::<Row>(cxq.to_string()))
            .map_err(TrinoSourceError::PrustoError)?;

        let schema = dataset.split().0;

        for (name, t) in schema {
            self.names.push(name.clone());
            self.schema.push(TrinoTypeSystem::try_from(t.clone())?);
        }
    }

    #[throws(TrinoSourceError)]
    fn result_rows(&mut self) -> Option<usize> {
        match &self.origin_query {
            Some(q) => {
                let cxq = CXQuery::Naked(q.clone());
                let nrows = get_total_rows(self.rt.clone(), self.client.clone(), &cxq)?;
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

    #[throws(TrinoSourceError)]
    fn partition(self) -> Vec<Self::Partition> {
        let mut ret = vec![];

        for query in self.queries {
            ret.push(TrinoSourcePartition::new(
                self.client.clone(),
                query,
                self.schema.clone(),
                self.rt.clone(),
            )?);
        }
        ret
    }
}

pub struct TrinoSourcePartition {
    client: Arc<Client>,
    query: CXQuery<String>,
    schema: Vec<TrinoTypeSystem>,
    rt: Arc<Runtime>,
    nrows: usize,
}

impl TrinoSourcePartition {
    #[throws(TrinoSourceError)]
    pub fn new(
        client: Arc<Client>,
        query: CXQuery<String>,
        schema: Vec<TrinoTypeSystem>,
        rt: Arc<Runtime>,
    ) -> Self {
        Self {
            client,
            query: query.clone(),
            schema: schema.to_vec(),
            rt,
            nrows: 0,
        }
    }
}

impl SourcePartition for TrinoSourcePartition {
    type TypeSystem = TrinoTypeSystem;
    type Parser<'a> = TrinoSourcePartitionParser<'a>;
    type Error = TrinoSourceError;

    #[throws(TrinoSourceError)]
    fn result_rows(&mut self) {
        self.nrows = get_total_rows(self.rt.clone(), self.client.clone(), &self.query)?;
    }

    #[throws(TrinoSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        TrinoSourcePartitionParser::new(
            self.rt.clone(),
            self.client.clone(),
            self.query.clone(),
            &self.schema,
        )?
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.schema.len()
    }
}

pub struct TrinoSourcePartitionParser<'a> {
    rt: Arc<Runtime>,
    client: Arc<Client>,
    next_uri: Option<String>,
    rows: Vec<Row>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
    _phantom: &'a PhantomData<DataSet<Row>>,
}

impl<'a> TrinoSourcePartitionParser<'a> {
    #[throws(TrinoSourceError)]
    pub fn new(
        rt: Arc<Runtime>,
        client: Arc<Client>,
        query: CXQuery,
        schema: &[TrinoTypeSystem],
    ) -> Self {
        let results = rt
            .block_on(client.get::<Row>(query.to_string()))
            .map_err(TrinoSourceError::PrustoError)?;

        let rows = match results.data_set {
            Some(x) => x.into_vec(),
            _ => vec![],
        };

        Self {
            rt,
            client,
            next_uri: results.next_uri,
            rows,
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
            _phantom: &PhantomData,
        }
    }

    #[throws(TrinoSourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        ret
    }
}

impl<'a> PartitionParser<'a> for TrinoSourcePartitionParser<'a> {
    type TypeSystem = TrinoTypeSystem;
    type Error = TrinoSourceError;

    #[throws(TrinoSourceError)]
    fn fetch_next(&mut self) -> (usize, bool) {
        assert!(self.current_col == 0);

        match self.next_uri.clone() {
            Some(uri) => {
                let results = self
                    .rt
                    .block_on(self.client.get_next::<Row>(&uri))
                    .map_err(TrinoSourceError::PrustoError)?;

                self.rows = match results.data_set {
                    Some(x) => x.into_vec(),
                    _ => vec![],
                };

                self.current_row = 0;
                self.next_uri = results.next_uri;

                (self.rows.len(), false)
            }
            None => return (self.rows.len(), true),
        }
    }
}

macro_rules! impl_produce_int {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for TrinoSourcePartitionParser<'a> {
                type Error = TrinoSourceError;

                #[throws(TrinoSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = &self.rows[ridx].value()[cidx];

                    match value {
                        Value::Number(x) => {
                            if (x.is_i64()) {
                                <$t>::try_from(x.as_i64().unwrap()).map_err(|_| anyhow!("Trino cannot parse i64 at position: ({}, {}) {:?}", ridx, cidx, value))?
                            } else {
                                throw!(anyhow!("Trino cannot parse Number at position: ({}, {}) {:?}", ridx, cidx, x))
                            }
                        }
                        _ => throw!(anyhow!("Trino cannot parse Number at position: ({}, {}) {:?}", ridx, cidx, value))
                    }
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for TrinoSourcePartitionParser<'a> {
                type Error = TrinoSourceError;

                #[throws(TrinoSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = &self.rows[ridx].value()[cidx];

                    match value {
                        Value::Null => None,
                        Value::Number(x) => {
                            if (x.is_i64()) {
                                Some(<$t>::try_from(x.as_i64().unwrap()).map_err(|_| anyhow!("Trino cannot parse i64 at position: ({}, {}) {:?}", ridx, cidx, value))?)
                            } else {
                                throw!(anyhow!("Trino cannot parse Number at position: ({}, {}) {:?}", ridx, cidx, x))
                            }
                        }
                        _ => throw!(anyhow!("Trino cannot parse Number at position: ({}, {}) {:?}", ridx, cidx, value))
                    }
                }
            }
        )+
    };
}

macro_rules! impl_produce_float {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for TrinoSourcePartitionParser<'a> {
                type Error = TrinoSourceError;

                #[throws(TrinoSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = &self.rows[ridx].value()[cidx];

                    match value {
                        Value::Number(x) => {
                            if (x.is_f64()) {
                                x.as_f64().unwrap() as $t
                            } else {
                                throw!(anyhow!("Trino cannot parse Number at position: ({}, {}) {:?}", ridx, cidx, x))
                            }
                        }
                        Value::String(x) => x.parse::<$t>().map_err(|_| anyhow!("Trino cannot parse String at position: ({}, {}) {:?}", ridx, cidx, value))?,
                        _ => throw!(anyhow!("Trino cannot parse Number at position: ({}, {}) {:?}", ridx, cidx, value))
                    }
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for TrinoSourcePartitionParser<'a> {
                type Error = TrinoSourceError;

                #[throws(TrinoSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = &self.rows[ridx].value()[cidx];

                    match value {
                        Value::Null => None,
                        Value::Number(x) => {
                            if (x.is_f64()) {
                                Some(x.as_f64().unwrap() as $t)
                            } else {
                                throw!(anyhow!("Trino cannot parse Number at position: ({}, {}) {:?}", ridx, cidx, x))
                            }
                        }
                        Value::String(x) => Some(x.parse::<$t>().map_err(|_| anyhow!("Trino cannot parse String at position: ({}, {}) {:?}", ridx, cidx, value))?),
                        _ => throw!(anyhow!("Trino cannot parse Number at position: ({}, {}) {:?}", ridx, cidx, value))
                    }
                }
            }
        )+
    };
}

macro_rules! impl_produce_text {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for TrinoSourcePartitionParser<'a> {
                type Error = TrinoSourceError;

                #[throws(TrinoSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = &self.rows[ridx].value()[cidx];

                    match value {
                        Value::String(x) => {
                            x.parse().map_err(|_| anyhow!("Trino cannot parse String at position: ({}, {}): {:?}", ridx, cidx, value))?
                        }
                        Value::Array(_) | Value::Object(_) | Value::Number(_) | Value::Bool(_) => {
                            serde_json::to_string(value)
                                .unwrap_or_else(|_| value.to_string())
                                .parse()
                                .map_err(|_| anyhow!("Trino cannot convert complex value at ({}, {}): {:?}", ridx, cidx, value))?
                        }
                        _ => throw!(anyhow!("Trino unknown value at position: ({}, {}): {:?}", ridx, cidx, value))
                    }
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for TrinoSourcePartitionParser<'a> {
                type Error = TrinoSourceError;

                #[throws(TrinoSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = &self.rows[ridx].value()[cidx];

                    match value {
                        Value::Null => None,
                        Value::String(x) => {
                            Some(x.parse().map_err(|_| anyhow!("Trino cannot parse String at position: ({}, {}): {:?}", ridx, cidx, value))?)
                        }
                        Value::Array(_) | Value::Object(_) | Value::Number(_) | Value::Bool(_) => {
                            Some(serde_json::to_string(value)
                                .unwrap_or_else(|_| value.to_string())
                                .parse()
                                .map_err(|_| anyhow!("Trino cannot convert complex value at ({}, {}): {:?}", ridx, cidx, value))?)
                        }
                        _ => throw!(anyhow!("Trino unknown value at position: ({}, {}): {:?}", ridx, cidx, value))
                    }
                }
            }
        )+
    };
}

macro_rules! impl_produce_timestamp {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for TrinoSourcePartitionParser<'a> {
                type Error = TrinoSourceError;

                #[throws(TrinoSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = &self.rows[ridx].value()[cidx];

                    match value {
                        Value::String(x) => NaiveDateTime::parse_from_str(x, "%Y-%m-%d %H:%M:%S%.f").map_err(|_| anyhow!("Trino cannot parse String at position: ({}, {}): {:?}", ridx, cidx, value))?,
                        _ => throw!(anyhow!("Trino unknown value at position: ({}, {}): {:?}", ridx, cidx, value))
                    }
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for TrinoSourcePartitionParser<'a> {
                type Error = TrinoSourceError;

                #[throws(TrinoSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = &self.rows[ridx].value()[cidx];

                    match value {
                        Value::Null => None,
                        Value::String(x) => Some(NaiveDateTime::parse_from_str(x, "%Y-%m-%d %H:%M:%S%.f").map_err(|_| anyhow!("Trino cannot parse String at position: ({}, {}): {:?}", ridx, cidx, value))?),
                        _ => throw!(anyhow!("Trino unknown value at position: ({}, {}): {:?}", ridx, cidx, value))
                    }
                }
            }
        )+
    };
}

macro_rules! impl_produce_bool {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for TrinoSourcePartitionParser<'a> {
                type Error = TrinoSourceError;

                #[throws(TrinoSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = &self.rows[ridx].value()[cidx];

                    match value {
                        Value::Bool(x) => *x,
                        _ => throw!(anyhow!("Trino unknown value at position: ({}, {}): {:?}", ridx, cidx, value))
                    }
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for TrinoSourcePartitionParser<'a> {
                type Error = TrinoSourceError;

                #[throws(TrinoSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = &self.rows[ridx].value()[cidx];

                    match value {
                        Value::Null => None,
                        Value::Bool(x) => Some(*x),
                        _ => throw!(anyhow!("Trino unknown value at position: ({}, {}): {:?}", ridx, cidx, value))
                    }
                }
            }
        )+
    };
}

impl_produce_bool!(bool,);
impl_produce_int!(i8, i16, i32, i64,);
impl_produce_float!(f32, f64,);
impl_produce_timestamp!(NaiveDateTime,);
impl_produce_text!(String, char,);

impl<'r, 'a> Produce<'r, NaiveTime> for TrinoSourcePartitionParser<'a> {
    type Error = TrinoSourceError;

    #[throws(TrinoSourceError)]
    fn produce(&'r mut self) -> NaiveTime {
        let (ridx, cidx) = self.next_loc()?;
        let value = &self.rows[ridx].value()[cidx];

        match value {
            Value::String(x) => NaiveTime::parse_from_str(x, "%H:%M:%S%.f").map_err(|_| {
                anyhow!(
                    "Trino cannot parse String at position: ({}, {}): {:?}",
                    ridx,
                    cidx,
                    value
                )
            })?,
            _ => throw!(anyhow!(
                "Trino unknown value at position: ({}, {}): {:?}",
                ridx,
                cidx,
                value
            )),
        }
    }
}

impl<'r, 'a> Produce<'r, Option<NaiveTime>> for TrinoSourcePartitionParser<'a> {
    type Error = TrinoSourceError;

    #[throws(TrinoSourceError)]
    fn produce(&'r mut self) -> Option<NaiveTime> {
        let (ridx, cidx) = self.next_loc()?;
        let value = &self.rows[ridx].value()[cidx];

        match value {
            Value::Null => None,
            Value::String(x) => {
                Some(NaiveTime::parse_from_str(x, "%H:%M:%S%.f").map_err(|_| {
                    anyhow!(
                        "Trino cannot parse Time at position: ({}, {}): {:?}",
                        ridx,
                        cidx,
                        value
                    )
                })?)
            }
            _ => throw!(anyhow!(
                "Trino unknown value at position: ({}, {}): {:?}",
                ridx,
                cidx,
                value
            )),
        }
    }
}

impl<'r, 'a> Produce<'r, NaiveDate> for TrinoSourcePartitionParser<'a> {
    type Error = TrinoSourceError;

    #[throws(TrinoSourceError)]
    fn produce(&'r mut self) -> NaiveDate {
        let (ridx, cidx) = self.next_loc()?;
        let value = &self.rows[ridx].value()[cidx];

        match value {
            Value::String(x) => NaiveDate::parse_from_str(x, "%Y-%m-%d").map_err(|_| {
                anyhow!(
                    "Trino cannot parse Date at position: ({}, {}): {:?}",
                    ridx,
                    cidx,
                    value
                )
            })?,
            _ => throw!(anyhow!(
                "Trino unknown value at position: ({}, {}): {:?}",
                ridx,
                cidx,
                value
            )),
        }
    }
}

impl<'r, 'a> Produce<'r, Option<NaiveDate>> for TrinoSourcePartitionParser<'a> {
    type Error = TrinoSourceError;

    #[throws(TrinoSourceError)]
    fn produce(&'r mut self) -> Option<NaiveDate> {
        let (ridx, cidx) = self.next_loc()?;
        let value = &self.rows[ridx].value()[cidx];

        match value {
            Value::Null => None,
            Value::String(x) => Some(NaiveDate::parse_from_str(x, "%Y-%m-%d").map_err(|_| {
                anyhow!(
                    "Trino cannot parse Date at position: ({}, {}): {:?}",
                    ridx,
                    cidx,
                    value
                )
            })?),
            _ => throw!(anyhow!(
                "Trino unknown value at position: ({}, {}): {:?}",
                ridx,
                cidx,
                value
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_all_params() {
        let rt = Arc::new(Runtime::new().unwrap());
        let conn = "trino+https://myuser:mypass@localhost:8443/mycatalog?\
            source=connectorx-test&schema=myschema&\
            client_tags=tag1,tag2,tag3&client_info=test-run&\
            trace_token=abc123&\
            session.query_max_execution_time=10m&\
            extra_credential.token=secret123&verify=false";
        assert!(TrinoSource::new(rt, conn).is_ok());
    }

    #[test]
    fn test_new_minimal_url() {
        let rt = Arc::new(Runtime::new().unwrap());
        assert!(TrinoSource::new(rt, "trino://test@localhost:8080/memory").is_ok());
    }

    #[test]
    fn test_new_ignores_empty_keys() {
        let rt = Arc::new(Runtime::new().unwrap());
        let conn = "trino://test@localhost:8080/memory?session.=val&extra_credential.=x";
        assert!(TrinoSource::new(rt, conn).is_ok());
    }
}
