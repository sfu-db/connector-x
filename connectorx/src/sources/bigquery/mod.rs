//! Source implementation for Google BigQuery

mod errors;
mod typesystem;

pub use self::errors::BigQuerySourceError;
use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::{PartitionParser, Produce, Source, SourcePartition},
    sql::{count_query, limit1_query, CXQuery},
};
<<<<<<< HEAD
=======
use hex::decode;
>>>>>>> d537310fd4dc103c9651db6f7ee0d0050c6281f9
use url::Url;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use fehler::{throw, throws};
use gcp_bigquery_client::{
    model::{query_request::QueryRequest, query_response::ResultSet},
    Client,
};
pub use typesystem::BigQueryTypeSystem;

use sqlparser::dialect::Dialect;
use std::sync::Arc;
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
    pub fn new(rt: Arc<Runtime>, conn: &str) -> Self {
        let url = Url::parse(conn)?;
        let sa_key_path = url.path();
        let client = Arc::new(rt.block_on(
            gcp_bigquery_client::Client::from_service_account_key_file(sa_key_path),
        ));
        let auth_data = std::fs::read_to_string(sa_key_path)?;
        let auth_json: serde_json::Value = serde_json::from_str(&auth_data)?;
        let project_id = auth_json
            .get("project_id")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
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
        for (_, query) in self.queries.iter().enumerate() {
            let l1query = limit1_query(query, &BigQueryDialect {})?;
            let rs = self.rt.block_on(job.query(
                self.project_id.as_str(),
                QueryRequest::new(l1query.as_str()),
            ))?;
            let (names, types) = rs
                .query_response()
                .schema
                .as_ref()
                .unwrap()
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

    #[throws(BigQuerySourceError)]
    fn result_rows(&mut self) -> Option<usize> {
        match &self.origin_query {
            Some(q) => {
                let cxq = CXQuery::Naked(q.clone());
                let cquery = count_query(&cxq, &BigQueryDialect {})?;
                let job = self.client.job();
                let mut rs = self.rt.block_on(
                    job.query(self.project_id.as_str(), QueryRequest::new(cquery.as_str())),
                )?;
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
        let rs = self.rt.block_on(job.query(
            self.project_id.as_str(),
            QueryRequest::new(self.query.as_str()),
        ))?;
        BigQuerySourceParser::new(self.rt.handle(), rs, &self.schema)
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
    iter: ResultSet,
    ncols: usize,
    current_col: usize,
    current_row: usize,
}

impl<'a> BigQuerySourceParser<'a> {
    fn new(rt: &'a Handle, iter: ResultSet, schema: &[BigQueryTypeSystem]) -> Self {
        Self {
            rt,
            iter,
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
        (self.iter.row_count(), true)
    }
}

macro_rules! impl_produce {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for BigQuerySourceParser<'a> {
                type Error = BigQuerySourceError;

                #[throws(BigQuerySourceError)]
                fn produce(&'r mut self) -> $t {
                    let (_, cidx) = self.next_loc()?;
                    if cidx == 0 {
                        self.iter.next_row();
                    }
<<<<<<< HEAD
                    self.iter.get_json_value(cidx).unwrap().unwrap().as_str().unwrap().parse().map_err(|_| {
=======
                    self.iter.get_json_value(cidx)?.unwrap().as_str().unwrap().parse().map_err(|_| {
>>>>>>> d537310fd4dc103c9651db6f7ee0d0050c6281f9
                        ConnectorXError::cannot_produce::<$t>(Some(self.iter.get_json_value(cidx).unwrap().unwrap().as_str().unwrap().into()))
                    })?
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for BigQuerySourceParser<'a> {
                type Error = BigQuerySourceError;

                #[throws(BigQuerySourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (_, cidx) = self.next_loc()?;
                    if cidx == 0 {
                        self.iter.next_row();
                    }
<<<<<<< HEAD
                    match &self.iter.get_json_value(cidx).unwrap() {
=======
                    match &self.iter.get_json_value(cidx)? {
>>>>>>> d537310fd4dc103c9651db6f7ee0d0050c6281f9
                        None => None,
                        v => Some(v.as_ref().unwrap().as_str().unwrap().parse().map_err(|_| {
                            ConnectorXError::cannot_produce::<$t>(Some(self.iter.get_json_value(cidx).unwrap().unwrap().as_str().unwrap().into()))
                        })?),
                    }
                }
            }
        )+
    };
}

impl_produce!(i64, f64, String,);

<<<<<<< HEAD
=======

macro_rules! impl_vec_produce {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, Vec<$t>> for BigQuerySourceParser<'a> {
                type Error = BigQuerySourceError;

                #[throws(BigQuerySourceError)]
                fn produce(&mut self) -> Vec<$t> {
                    let (_, cidx) = self.next_loc()?;
                    if cidx == 0 {
                        self.iter.next_row();
                    }
                    let value = self.iter.get_json_value(cidx)?.unwrap();
                    let s = value.as_str().unwrap();
                    match s {
                        "{}" => vec![],
                        _ if s.len() < 3 => throw!(ConnectorXError::cannot_produce::<$t>(Some(s.into()))),
                        s => s[1..s.len() - 1]
                            .split(",")
                            .map(|v| {
                                v.parse()
                                    .map_err(|_| ConnectorXError::cannot_produce::<$t>(Some(s.into())))
                            })
                            .collect::<Result<Vec<$t>, ConnectorXError>>()?,
                    }
                }
            }

            impl<'r, 'a> Produce<'r, Option<Vec<$t>>> for BigQuerySourceParser<'a> {
                type Error = BigQuerySourceError;

                #[throws(BigQuerySourceError)]
                fn produce(&mut self) -> Option<Vec<$t>> {
                    let (_, cidx) = self.next_loc()?;
                    if cidx == 0 {
                        self.iter.next_row();
                    }
                    let jsvalue = self.iter.get_json_value(cidx)?;
                    match jsvalue {
                        None => None,
                        Some(value) => {
                            let s = value.as_str().unwrap();
                            match s {
                                "{}" => Some(vec![]),
                                _ if s.len() < 3 => throw!(ConnectorXError::cannot_produce::<$t>(Some(s.into()))),
                                s => Some(
                                    s[1..s.len() - 1]
                                        .split(",")
                                        .map(|v| {
                                            v.parse()
                                                .map_err(|_| ConnectorXError::cannot_produce::<$t>(Some(s.into())))
                                        })
                                        .collect::<Result<Vec<$t>, ConnectorXError>>()?,
                                ),
                            }
                        }
                    }
                }
            }
        )+
    };
}

impl_vec_produce!(i64, f64, );

>>>>>>> d537310fd4dc103c9651db6f7ee0d0050c6281f9
impl<'r, 'a> Produce<'r, bool> for BigQuerySourceParser<'a> {
    type Error = BigQuerySourceError;

    #[throws(BigQuerySourceError)]
    fn produce(&mut self) -> bool {
        let (_, cidx) = self.next_loc()?;
        if cidx == 0 {
            self.iter.next_row();
        }
<<<<<<< HEAD
        let ret = match &self
            .iter
            .get_json_value(cidx)
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap()[..]
        {
            "TRUE" => true,
            "FALSE" => false,
            _ => throw!(ConnectorXError::cannot_produce::<bool>(Some(
                self.iter
                    .get_json_value(cidx)
                    .unwrap()
=======
        let ret = match self.iter.get_json_value(cidx)?
        .unwrap()
        .as_str()
        .unwrap() {
            "true" => true,
            "false" => false,
            _ => throw!(ConnectorXError::cannot_produce::<bool>(Some(
                self.iter
                    .get_json_value(cidx)?
>>>>>>> d537310fd4dc103c9651db6f7ee0d0050c6281f9
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .into()
            ))),
        };
        ret
    }
}

impl<'r, 'a> Produce<'r, Option<bool>> for BigQuerySourceParser<'a> {
    type Error = BigQuerySourceError;

    #[throws(BigQuerySourceError)]
    fn produce(&mut self) -> Option<bool> {
        let (_, cidx) = self.next_loc()?;
        if cidx == 0 {
            self.iter.next_row();
        }
<<<<<<< HEAD
        let ret = match &self
            .iter
            .get_json_value(cidx)
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap()[..]
        {
            "" => None,
            "TRUE" => Some(true),
            "FALSE" => Some(false),
            _ => throw!(ConnectorXError::cannot_produce::<bool>(Some(
                self.iter
                    .get_json_value(cidx)
                    .unwrap()
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .into()
            ))),
=======
        let ret = match self
            .iter
            .get_json_value(cidx)?
        {
            None => None,
            Some(v) => {
                match v.as_str().unwrap() {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => throw!(ConnectorXError::cannot_produce::<bool>(Some(
                        self.iter
                            .get_json_value(cidx)
                            .unwrap()
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .into()
                    ))),
                }
            }
            
>>>>>>> d537310fd4dc103c9651db6f7ee0d0050c6281f9
        };
        ret
    }
}

impl<'r, 'a> Produce<'r, NaiveDate> for BigQuerySourceParser<'a> {
    type Error = BigQuerySourceError;

    #[throws(BigQuerySourceError)]
    fn produce(&mut self) -> NaiveDate {
        let (_, cidx) = self.next_loc()?;
        if cidx == 0 {
            self.iter.next_row();
        }
        NaiveDate::parse_from_str(
            &self
                .iter
                .get_json_value(cidx)
                .unwrap()
                .unwrap()
                .as_str()
                .unwrap(),
            "%Y-%m-%d",
        )
        .map_err(|_| {
            ConnectorXError::cannot_produce::<NaiveDate>(Some(
                self.iter
                    .get_json_value(cidx)
                    .unwrap()
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .into(),
            ))
        })?
    }
}

impl<'r, 'a> Produce<'r, Option<NaiveDate>> for BigQuerySourceParser<'a> {
    type Error = BigQuerySourceError;

    #[throws(BigQuerySourceError)]
    fn produce(&mut self) -> Option<NaiveDate> {
        let (_, cidx) = self.next_loc()?;
        if cidx == 0 {
            self.iter.next_row();
        }
<<<<<<< HEAD
        match &self
            .iter
            .get_json_value(cidx)
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap()[..]
        {
            "" => None,
            v => Some(
                NaiveDate::parse_from_str(v, "%Y-%m-%d")
                    .map_err(|_| ConnectorXError::cannot_produce::<NaiveDate>(Some(v.into())))?,
=======
        match self
            .iter
            .get_json_value(cidx)?
        {
            None => None,
            v => Some(
                NaiveDate::parse_from_str(v.as_ref().unwrap().as_str().unwrap(), "%Y-%m-%d")
                    .map_err(|_| ConnectorXError::cannot_produce::<NaiveDate>(Some(v.as_ref().unwrap().as_str().unwrap().into())))?,
>>>>>>> d537310fd4dc103c9651db6f7ee0d0050c6281f9
            ),
        }
    }
}

impl<'r, 'a> Produce<'r, NaiveDateTime> for BigQuerySourceParser<'a> {
    type Error = BigQuerySourceError;

    #[throws(BigQuerySourceError)]
    fn produce(&mut self) -> NaiveDateTime {
        let (_, cidx) = self.next_loc()?;
        if cidx == 0 {
            self.iter.next_row();
        }
        NaiveDateTime::parse_from_str(
            &self
                .iter
                .get_json_value(cidx)
                .unwrap()
                .unwrap()
                .as_str()
                .unwrap(),
            "%Y-%m-%dT%H:%M:%S",
        )
        .map_err(|_| {
            ConnectorXError::cannot_produce::<NaiveDateTime>(Some(
                self.iter
                    .get_json_value(cidx)
                    .unwrap()
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .into(),
            ))
        })?
    }
}

impl<'r, 'a> Produce<'r, Option<NaiveDateTime>> for BigQuerySourceParser<'a> {
    type Error = BigQuerySourceError;

    #[throws(BigQuerySourceError)]
    fn produce(&mut self) -> Option<NaiveDateTime> {
        let (_, cidx) = self.next_loc()?;
        if cidx == 0 {
            self.iter.next_row();
        }
        match self
            .iter
            .get_json_value(cidx)?
        {
            None => None,
            v => Some(
                NaiveDateTime::parse_from_str(v.as_ref().unwrap().as_str().unwrap(), "%Y-%m-%dT%H:%M:%S").map_err(|_| {
                    ConnectorXError::cannot_produce::<NaiveDateTime>(Some(v.as_ref().unwrap().as_str().unwrap().into()))
                })?,
            ),
        }
    }
}

impl<'r, 'a> Produce<'r, NaiveTime> for BigQuerySourceParser<'a> {
    type Error = BigQuerySourceError;

    #[throws(BigQuerySourceError)]
    fn produce(&mut self) -> NaiveTime {
        let (_, cidx) = self.next_loc()?;
        if cidx == 0 {
            self.iter.next_row();
        }
        NaiveTime::parse_from_str(
            &self
                .iter
                .get_json_value(cidx)
                .unwrap()
                .unwrap()
                .as_str()
                .unwrap(),
            "%H:%M:%S",
        )
        .map_err(|_| {
            ConnectorXError::cannot_produce::<NaiveTime>(Some(
                self.iter
                    .get_json_value(cidx)
                    .unwrap()
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .into(),
            ))
        })?
    }
}

impl<'r, 'a> Produce<'r, Option<NaiveTime>> for BigQuerySourceParser<'a> {
    type Error = BigQuerySourceError;

    #[throws(BigQuerySourceError)]
    fn produce(&mut self) -> Option<NaiveTime> {
        let (_, cidx) = self.next_loc()?;
        if cidx == 0 {
            self.iter.next_row();
        }
        match self
            .iter
            .get_json_value(cidx)?
        {
            None => None,
            v => Some(
                NaiveTime::parse_from_str(v.as_ref().unwrap().as_str().unwrap(), "%H:%M:%S")
                    .map_err(|_| ConnectorXError::cannot_produce::<NaiveTime>(Some(v.as_ref().unwrap().as_str().unwrap().into())))?,
            ),
        }
    }
}

impl<'r, 'a> Produce<'r, DateTime<Utc>> for BigQuerySourceParser<'a> {
    type Error = BigQuerySourceError;

    #[throws(BigQuerySourceError)]
    fn produce(&mut self) -> DateTime<Utc> {
        let (_, cidx) = self.next_loc()?;
        if cidx == 0 {
            self.iter.next_row();
        }
        format!(
            "{}:00",
            &self
                .iter
                .get_json_value(cidx)
                .unwrap()
                .unwrap()
                .as_str()
                .unwrap()[..]
        )
        .parse()
        .map_err(|_| {
            ConnectorXError::cannot_produce::<DateTime<Utc>>(Some(
                self.iter
                    .get_json_value(cidx)
                    .unwrap()
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .into(),
            ))
        })?
    }
}

impl<'r, 'a> Produce<'r, Option<DateTime<Utc>>> for BigQuerySourceParser<'a> {
    type Error = BigQuerySourceError;

    #[throws(BigQuerySourceError)]
    fn produce(&mut self) -> Option<DateTime<Utc>> {
        let (_, cidx) = self.next_loc()?;
        if cidx == 0 {
            self.iter.next_row();
        }
        match &self
            .iter
            .get_json_value(cidx)
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap()[..]
        {
            "" => None,
            v => {
                Some(format!("{}:00", v).parse().map_err(|_| {
                    ConnectorXError::cannot_produce::<DateTime<Utc>>(Some(v.into()))
                })?)
            }
        }
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
