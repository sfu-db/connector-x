use std::{marker::PhantomData, sync::Arc};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use fehler::{throw, throws};
use prusto::{auth::Auth, Client, ClientBuilder, DataSet, Presto, Row};
use serde_json::Value;
use sqlparser::dialect::GenericDialect;
use std::convert::TryFrom;
use tokio::runtime::Runtime;

use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::Produce,
    sql::{count_query, limit1_query, CXQuery},
};

pub use self::{errors::TrinoSourceError, typesystem::TrinoTypeSystem};
use urlencoding::decode;

use super::{PartitionParser, Source, SourcePartition};

use anyhow::anyhow;

pub mod errors;
pub mod typesystem;

#[throws(TrinoSourceError)]
async fn get_total_rows(client: Arc<Client>, query: &CXQuery<String>) -> usize {
    let result = client
        .get_all::<u16>(count_query(query, &GenericDialect {})?.to_string())
        .await
        .map_err(TrinoSourceError::PrustoError)?;

    usize::from(result.as_slice()[0])
}

pub struct TrinoSource {
    client: Arc<Client>,
    rt: Arc<Runtime>,
    origin_query: Option<String>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<TrinoTypeSystem>,
}

impl TrinoSource {
    #[throws(TrinoSourceError)]
    pub fn new(rt: Arc<Runtime>, conn: &str) -> Self {
        let decoded_conn = decode(conn)?.into_owned();

        let url = decoded_conn
            .parse::<url::Url>()
            .map_err(TrinoSourceError::UrlParseError)?;

        let client = ClientBuilder::new(url.username(), url.host().unwrap().to_owned())
            .port(url.port().unwrap_or(8080))
            .auth(Auth::Basic(
                url.username().to_owned(),
                url.password().map(|x| x.to_owned()),
            ))
            .ssl(prusto::ssl::Ssl { root_cert: None })
            .secure(url.scheme() == "trino+https")
            .catalog(url.path_segments().unwrap().last().unwrap_or("hive"))
            .build()
            .map_err(TrinoSourceError::PrustoError)?;

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

        match &self.origin_query {
            Some(q) => {
                /*let cxq = CXQuery::Naked(q.clone());
                let cxq = limit1_query(&cxq, &GenericDialect {})?;
                let data_set: DataSet<_> = self
                    .rt
                    .block_on(self.client.get_all::<Row>(cxq.to_string()))
                    .map_err(TrinoSourceError::PrustoError)?;

                let x = data_set.into_vec().first().unwrap();
                let ncols = x.value().to_vec().len();

                let mut parser =
                    TrinoSourceParser::new(self.rt.clone(), self.client.clone(), cxq, ncols)?;

                // produce the first row
                for x in 0..ncols {
                    let x: TrinoTypeSystem = parser.produce()?;
                }

                data_set.into_vec().iter().for_each(|row| {
                    row.value().iter().for_each(|x| {
                        println!("{:?}", x);
                    });

                    println!("{:?}", row);
                });*/

                // TODO: remove hard-coded
                self.schema = vec![
                    TrinoTypeSystem::Integer(true),
                    TrinoTypeSystem::Double(true),
                    TrinoTypeSystem::Varchar(true),
                ];
                self.names = vec!["a".to_string(), "b".to_string(), "c".to_string()];
            }
            None => {}
        }
    }

    #[throws(TrinoSourceError)]
    fn result_rows(&mut self) -> Option<usize> {
        match &self.origin_query {
            Some(q) => {
                let cxq = CXQuery::Naked(q.clone());
                let client = self.client.clone();
                let nrows = self.rt.block_on(get_total_rows(client, &cxq))?;
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
            ret.push(
                TrinoSourcePartition::new(
                    self.client.clone(),
                    query,
                    self.schema.clone(),
                    self.rt.clone(),
                )
                .unwrap(), // TODO: handle error
            );
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
    ncols: usize,
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
            query,
            schema: schema.clone(),
            rt,
            nrows: 0,
            ncols: schema.len(),
        }
    }
}

impl SourcePartition for TrinoSourcePartition {
    type TypeSystem = TrinoTypeSystem;
    type Parser<'a> = TrinoSourceParser<'a>;
    type Error = TrinoSourceError;

    #[throws(TrinoSourceError)]
    fn result_rows(&mut self) {
        self.nrows = self
            .rt
            .block_on(get_total_rows(self.client.clone(), &self.query))?
    }

    #[throws(TrinoSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        let query = self.query.clone();
        TrinoSourceParser::new(self.rt.clone(), self.client.clone(), query, &self.schema)?
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

pub struct TrinoSourceParser<'a> {
    rows: Vec<Row>,
    nrows: usize,
    ncols: usize,
    current_col: usize,
    current_row: usize,
    _phantom: &'a PhantomData<DataSet<Row>>,
}

impl<'a> TrinoSourceParser<'a> {
    #[throws(TrinoSourceError)]
    pub fn new(
        rt: Arc<Runtime>,
        client: Arc<Client>,
        query: CXQuery,
        schema: &[TrinoTypeSystem],
    ) -> Self {
        let rows = client.get_all::<Row>(query.to_string());
        let data = rt.block_on(rows).map_err(TrinoSourceError::PrustoError)?;
        let rows = data.clone().into_vec();

        Self {
            rows,
            nrows: data.len(),
            ncols: schema.len(),
            current_col: 0,
            current_row: 0,
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

impl<'a> PartitionParser<'a> for TrinoSourceParser<'a> {
    type TypeSystem = TrinoTypeSystem;
    type Error = TrinoSourceError;

    #[throws(TrinoSourceError)]
    fn fetch_next(&mut self) -> (usize, bool) {
        assert!(self.current_col == 0);

        (self.nrows, true)
    }
}

macro_rules! impl_produce_int {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for TrinoSourceParser<'a> {
                type Error = TrinoSourceError;

                #[throws(TrinoSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = &self.rows[ridx].value()[cidx];

                    match value {
                        Value::Number(x) => {
                            if (x.is_i64()) {
                                <$t>::try_from(x.as_i64().unwrap()).map_err(|_| anyhow!("Trino cannot parse Number at position: ({}, {})", ridx, cidx))?
                            } else {
                                throw!(anyhow!("Trino cannot parse Number at position: ({}, {})", ridx, cidx))
                            }
                        }
                        _ => throw!(anyhow!("Trino cannot parse Number at position: ({}, {})", ridx, cidx))
                    }
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for TrinoSourceParser<'a> {
                type Error = TrinoSourceError;

                #[throws(TrinoSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = &self.rows[ridx].value()[cidx];

                    match value {
                        Value::Null => None,
                        Value::Number(x) => {
                            if (x.is_i64()) {
                                Some(<$t>::try_from(x.as_i64().unwrap()).map_err(|_| anyhow!("Trino cannot parse Number at position: ({}, {})", ridx, cidx))?)
                            } else {
                                throw!(anyhow!("Trino cannot parse Number at position: ({}, {})", ridx, cidx))
                            }
                        }
                        _ => throw!(anyhow!("Trino cannot parse Number at position: ({}, {})", ridx, cidx))
                    }
                }
            }
        )+
    };
}

macro_rules! impl_produce_float {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for TrinoSourceParser<'a> {
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
                                throw!(anyhow!("Trino cannot parse Number at position: ({}, {})", ridx, cidx))
                            }
                        }
                        _ => throw!(anyhow!("Trino cannot parse Number at position: ({}, {})", ridx, cidx))
                    }
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for TrinoSourceParser<'a> {
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
                                throw!(anyhow!("Trino cannot parse Number at position: ({}, {})", ridx, cidx))
                            }
                        }
                        _ => throw!(anyhow!("Trino cannot parse Number at position: ({}, {})", ridx, cidx))
                    }
                }
            }
        )+
    };
}

macro_rules! impl_produce_text {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for TrinoSourceParser<'a> {
                type Error = TrinoSourceError;

                #[throws(TrinoSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = &self.rows[ridx].value()[cidx];

                    match value {
                        Value::String(x) => {
                            x.parse().map_err(|_| anyhow!("Trino cannot parse String at position: ({}, {})", ridx, cidx))?
                        }
                        _ => throw!(anyhow!("Trino cannot parse String at position: ({}, {})", ridx, cidx))
                    }
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for TrinoSourceParser<'a> {
                type Error = TrinoSourceError;

                #[throws(TrinoSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = &self.rows[ridx].value()[cidx];

                    match value {
                        Value::Null => None,
                        Value::String(x) => {
                            Some(x.parse().map_err(|_| anyhow!("Trino cannot parse String at position: ({}, {})", ridx, cidx))?)
                        }
                        _ => throw!(anyhow!("Trino cannot parse String at position: ({}, {})", ridx, cidx))
                    }
                }
            }
        )+
    };
}

impl_produce_int!(i8, i16, i32, i64,);
impl_produce_float!(f32, f64,);
impl_produce_text!(NaiveDate, NaiveTime, NaiveDateTime, String, bool, char,);
