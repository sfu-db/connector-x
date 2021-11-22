//! Source implementation for Google BigQuery

mod errors;
mod typesystem;

pub use self::errors::BigQuerySourceError;
use crate::constants::DB_BUFFER_SIZE;
use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::{PartitionParser, Produce, Source, SourcePartition},
    sql::{count_query, get_limit, CXQuery},
};
use log::debug;
use anyhow::anyhow;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use fehler::{throw, throws};
use tokio::runtime::{Handle, Runtime};
use bigquery_storage::{Client, Table};

#[derive(Debug)]
pub struct BigQueryDialect {}

impl Dialect for BigQueryDialect {
    // See https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical
    fn is_delimited_identifier_start(&self, ch: char) -> bool {
        ch == '`'
    }

    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch)
        || ('A'..='Z').contains(&ch)
        || ch == '_'
        || ch == '-'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        self.is_identifier_start(ch) || ('0'..='9').contains(&ch)
    }
}

pub struct BigQuerySource {
    client: Client,
    origin_query: Option<String>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<BigQueryTypeSystem>,
}

impl BigQuerySource {
    #[throws(BigQuerySourceError)]
    pub fn new(rt: Arc<Runtime>, conn: &str, nconn: usize) -> Self {
        let sa_key = "/home/jinze/dataprep-bigquery-d6514e01c1db.json"; // TODO: dynamic key
        let client = rt.block_on(gcp_bigquery_client::Client::from_service_account_key_file(sa_key));
        let project_id = "dataprep-bigquery";  // TODO: hard-code
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
        SourcePartition<TypeSystem = BigQuerySystem, Error =BigQuerySourceError>,
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

            match self.rt.block_on(job.query(self.project_id, QueryRequest::new(l1query.as_str()).query_response().schema.as_ref().unwrap())?{
                Ok(table_schema) => {
                    let (names, types) = table_schema.fields.as_ref().unwrap().iter().map(
                        |col| {
                            (
                                col.clone().name,
                                BigQueryTypeSystem::from(col.clone().r#type)
                            )
                        }
                    ).unzip();
                    let self.names = names;
                    let self.schema = types;
                }
            }
        }
    }
    
    // 1. BigQuerySource func new, add project_id
    // 2. finished fetch_meta based on project_id
    // 3. result_rows function
    // 4. partition part
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
                self.client.clone(), 
                self.rt.clone(), 
                &query, 
                &self.schema
            ));
        }
        ret
    }

}

pub struct BigQuerySourcePartition {
    client: Client,
    rt: Arc<Runtime>,
    query: CXQuery<String>,
    schema: Vec<BigQueryTypeSystem>,
    nrows: usize,
    ncols: usize,
}

impl BigQuerySourcePartition {
    pub fn new(
        client: Client,
        handle: Arc<Runtime>,
        query: &CXQuery<String>,
        schema: &[BigQueryTypeSystem],
    ) -> Self {
        Self {
            client,
            rt: handle,
            query: query.clone(),
            shcema: schema.to_vec(),
            nrows: 0,
            ncols: schema.len(),
        }
    }
}

impl SourcePartition for BigQuerySourcePartition {
    type TypeSystem = BigQuerySystem;
    type Parser<'a> = BigQuerySourceParser<'a>;
    type Error = BigQuerySourceError;

}

pub struct BigQuerySourceParser<'a> {
    fn new(
        rt: &‘a Handle,
        iter:
        rowbuf: Vec<Row>,
        ncols: usize,
        current_col: usize,
        current_row: usize,
    )
}

impl<'a> BigQuerySourceParser<'a> {
    fn new(
        rt: &'a Handle,
        iter:
        schema: &[BigQueryTypeSystem],
    ) -> Self {
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
    type TypeSystem = OracleTypeSystem;
    type Error = OracleSourceError;

    #[throws(BigQuerySourceError)]
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
