//! Source implementation for Google BigQuery

mod errors;
mod typesystem;

pub use self::errors::BigQuerySourceError;
pub use typesystem::BigQueryTypeSystem;
use crate::constants::DB_BUFFER_SIZE;
use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::{PartitionParser, Produce, Source, SourcePartition},
    sql::{count_query, limit1_query, CXQuery},
};
use fehler::{throw, throws};
use gcp_bigquery_client::{
    model::{query_request::QueryRequest, table_row::TableRow},
    Client
};

use log::debug;
use sqlparser::dialect::Dialect;
use std::sync::Arc;
// use std::vec::IntoIter;
use std::slice::Iter;
use tokio::runtime::{Handle, Runtime};

#[derive(Debug)]
pub struct BigQueryDialect {}

impl Dialect for BigQueryDialect {
    // See https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical
    fn is_delimited_identifier_start(&self, ch: char) -> bool {
        ch == '`'
    }

    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_' || ch == '-'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        self.is_identifier_start(ch) || ('0'..='9').contains(&ch)
    }
}

pub struct BigQuerySource {
    rt: Arc<Runtime>,
    client: Arc<Client>,
    project_id: String,
    origin_query: Option<String>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<BigQueryTypeSystem>,
}

impl BigQuerySource {
    #[throws(BigQuerySourceError)]
    pub fn new(rt: Arc<Runtime>, conn: &str, nconn: usize) -> Self {
        let sa_key = "/home/jinze/dataprep-bigquery-d6514e01c1db.json"; // TODO: dynamic key
        let client = Arc::new(rt.block_on(gcp_bigquery_client::Client::from_service_account_key_file(
            sa_key,
        )));
        let project_id = "dataprep-bigquery".to_string(); // TODO: hard-code
        Self {
            rt,
            client,
            project_id,
            origin_query: None,
            queries: vec![],
            names: vec![],
            schema: vec![],
        }
    }
}

impl Source for BigQuerySource
where
    BigQuerySourcePartition:
        SourcePartition<TypeSystem = BigQueryTypeSystem, Error = BigQuerySourceError>,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = BigQuerySourcePartition;
    type TypeSystem = BigQueryTypeSystem;
    type Error = BigQuerySourceError;

    #[throws(BigQuerySourceError)]
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

    #[throws(BigQuerySourceError)]
    fn fetch_metadata(&mut self) {
        assert!(!self.queries.is_empty());
        let job = self.client.job();
        for (i, query) in self.queries.iter().enumerate() {
            let l1query = limit1_query(query, &BigQueryDialect {})?;
            let job = self.rt.block_on(job.query(
                self.project_id.as_str(),
                QueryRequest::new(l1query.as_str())
            ))?;
            let (names, types) = job.query_response().schema.as_ref().unwrap()
                .fields
                .as_ref()
                .unwrap()
                .iter()
                .map(|col| {
                    (
                        col.clone().name,
                        BigQueryTypeSystem::from(&col.clone().r#type),
                    )
                })
                .unzip();
            self.names = names;
            self.schema = types;
        }
    }

    // 1. BigQuerySource func new, add project_id
    // 2. finished fetch_meta based on project_id
    // 3. result_rows function
    // 4. partition part

    #[throws(BigQuerySourceError)]
    fn result_rows(&mut self) -> Option<usize> {
        match &self.origin_query {
            Some(q) => {
                let cxq = CXQuery::Naked(q.clone());
                let cquery = count_query(&cxq, &BigQueryDialect {})?;
                let job = self.client.job();
                let mut rs = self
                    .rt
                    .block_on(job.query(self.project_id.as_str(), QueryRequest::new(cquery.as_str()),))?;
                rs.next_row();
                let nrows = rs.get_i64(0)?.unwrap();
                Some(nrows as usize)
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

    #[throws(BigQuerySourceError)]
    fn partition(self) -> Vec<Self::Partition> {
        let mut ret = vec![];
        for query in self.queries {
            ret.push(BigQuerySourcePartition::new(
                self.rt.clone(),
                self.client.clone(),
                self.project_id.clone(),
                &query,
                &self.schema,
            ));
        }
        ret
    }
}

pub struct BigQuerySourcePartition {
    rt: Arc<Runtime>,
    client: Arc<Client>,
    project_id: String,
    query: CXQuery<String>,
    schema: Vec<BigQueryTypeSystem>,
    nrows: usize,
    ncols: usize,
}

impl BigQuerySourcePartition {
    pub fn new(
        handle: Arc<Runtime>,
        client: Arc<Client>,
        project_id: String,
        query: &CXQuery<String>,
        schema: &[BigQueryTypeSystem],
    ) -> Self {
        Self {
            rt: handle,
            client,
            project_id: project_id.clone(),
            query: query.clone(),
            schema: schema.to_vec(),
            nrows: 0,
            ncols: schema.len(),
        }
    }
}

impl SourcePartition for BigQuerySourcePartition {
    type TypeSystem = BigQueryTypeSystem;
    type Parser<'a> = BigQuerySourceParser<'a>;
    type Error = BigQuerySourceError;

    #[throws(BigQuerySourceError)]
    fn result_rows(&mut self) {
        let cquery = count_query(&self.query, &BigQueryDialect {})?;
        let job = self.client.job();
        let mut rs = self
            .rt
            .block_on(job.query(self.project_id.as_str(), QueryRequest::new(cquery.as_str())))?;
        rs.next_row();
        let nrows = rs.get_i64(0)?.unwrap();
        self.nrows = nrows as usize;
    }

    #[throws(BigQuerySourceError)]
    fn parser<'a>(&'a mut self) -> Self::Parser<'a> {
        let job = self.client.job();
        // let rs = self
        //     .rt
        //     .block_on(job.query(self.project_id.as_str(), QueryRequest::new(self.query.as_str(),)))?;
        //let iter = rs.query_response().rows.as_ref().unwrap().iter();
        let iter = self
        .rt
        .block_on(job.query(self.project_id.as_str(), QueryRequest::new(self.query.as_str(),)))?
        .query_response().clone().rows
        .as_ref()
        .unwrap()
        .iter();
        BigQuerySourceParser::new(self.rt.handle(), iter, &self.schema)
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

pub struct BigQuerySourceParser<'a> {
    rt: &'a Handle,
    iter: Iter<'a, TableRow>,
    rowbuf: Vec<TableRow>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
}

impl<'a> BigQuerySourceParser<'a> {
    fn new(rt: &'a Handle, iter: Iter<'a, TableRow>, schema: &[BigQueryTypeSystem]) -> Self {
        Self {
            rt,
            iter,
            rowbuf: Vec::with_capacity(DB_BUFFER_SIZE),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
        }
    }

    #[throws(BigQuerySourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        ret
    }
}

impl<'a> PartitionParser<'a> for BigQuerySourceParser<'a> {
    type TypeSystem = BigQueryTypeSystem;
    type Error = BigQuerySourceError;

    #[throws(BigQuerySourceError)]
    fn fetch_next(&mut self) -> (usize, bool) {
        if !self.rowbuf.is_empty() {
            self.rowbuf.drain(..);
        }
        for _ in 0..DB_BUFFER_SIZE {
            if let Some(tablerow) = self.iter.next() {
                self.rowbuf.push(tablerow.clone());
            } else {
                break;
            }
        }
        self.current_row = 0;
        self.current_col = 0;
        (self.rowbuf.len(), self.rowbuf.len() < DB_BUFFER_SIZE)
    }
}

macro_rules! impl_produce {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for BigQuerySourceParser<'a> {
                type Error = BigQuerySourceError;

                #[throws(BigQuerySourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = self.rowbuf[ridx].columns.as_ref().unwrap().get(cidx).unwrap().value.as_ref().unwrap().as_str().unwrap();
                    value.parse().map_err(|_| {
                        ConnectorXError::cannot_produce::<$t>(Some(value.into()))
                    })?
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for BigQuerySourceParser<'a> {
                type Error = BigQuerySourceError;

                #[throws(BigQuerySourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    let value = self.rowbuf[ridx].columns.as_ref().unwrap().get(cidx).unwrap().value.as_ref().unwrap().as_str().unwrap();
                    match &value[..] {
                        "" => None,
                        v => Some(v.parse().map_err(|_| {
                            ConnectorXError::cannot_produce::<$t>(Some(value.into()))
                        })?),
                    }
                }
            }
        )+
    };
}

impl_produce!(i64,);
