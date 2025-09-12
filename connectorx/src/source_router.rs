use crate::constants::CONNECTORX_PROTOCOL;
use crate::errors::{ConnectorXError, Result};
use anyhow::anyhow;
use fehler::throws;
use std::convert::TryFrom;
use url::Url;

#[derive(Debug, Clone)]
pub enum SourceType {
    Postgres,
    SQLite,
    MySQL,
    MsSQL,
    Oracle,
    BigQuery,
    DuckDB,
    Trino,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct BigQueryAdcConfig {
    pub secret_path: String,
    pub project_id: String,
}

#[derive(Debug, Clone)]
pub struct SourceConn {
    pub ty: SourceType,
    pub conn: Url,
    pub proto: String,
    pub bq_adc_config: Option<BigQueryAdcConfig>,
}

impl TryFrom<&str> for SourceConn {
    type Error = ConnectorXError;

    fn try_from(conn: &str) -> Result<SourceConn> {
        let old_url = Url::parse(conn).map_err(|e| anyhow!("parse error: {}", e))?;

        // parse connectorx protocol
        let proto = match old_url.query_pairs().find(|p| p.0 == CONNECTORX_PROTOCOL) {
            Some((_, proto)) => proto.to_owned().to_string(),
            None => "binary".to_string(),
        };

        // create url by removing connectorx protocol
        let stripped_query: Vec<(_, _)> = old_url
            .query_pairs()
            .filter(|p| &*p.0 != CONNECTORX_PROTOCOL)
            .collect();
        let mut url = old_url.clone();
        url.set_query(None);
        for pair in stripped_query {
            url.query_pairs_mut()
                .append_pair(&pair.0.to_string()[..], &pair.1.to_string()[..]);
        }

        // users from sqlalchemy may set engine in connection url (e.g. mssql+pymssql://...)
        // only for compatablility, we don't use the same engine
        match url.scheme().split('+').collect::<Vec<&str>>()[0] {
            "postgres" | "postgresql" => Ok(SourceConn::new(SourceType::Postgres, url, proto)),
            "sqlite" => Ok(SourceConn::new(SourceType::SQLite, url, proto)),
            "mysql" => Ok(SourceConn::new(SourceType::MySQL, url, proto)),
            "mssql" => Ok(SourceConn::new(SourceType::MsSQL, url, proto)),
            "oracle" => Ok(SourceConn::new(SourceType::Oracle, url, proto)),
            "bigquery" => Ok(SourceConn::new(SourceType::BigQuery, url, proto)),
            "duckdb" => Ok(SourceConn::new(SourceType::DuckDB, url, proto)),
            "trino" => Ok(SourceConn::new(SourceType::Trino, url, proto)),
            _ => Ok(SourceConn::new(SourceType::Unknown, url, proto)),
        }
    }
}

impl SourceConn {
    pub fn new(ty: SourceType, conn: Url, proto: String) -> Self {
        Self {
            ty,
            conn,
            proto,
            bq_adc_config: None,
        }
    }
    pub fn set_protocol(&mut self, protocol: &str) {
        self.proto = protocol.to_string();
    }
    pub fn new_bq_user_adc(ty: SourceType, proto: String, config: BigQueryAdcConfig) -> Self {
        let secret_path_url = url::from_file_path(config.secret_path.clone());
        Self {
            ty,
            conn: secret_path_url,
            proto,
            bq_adc_config: Some(config),
        }
    }
}

#[throws(ConnectorXError)]
pub fn parse_source(conn: &str, protocol: Option<&str>) -> SourceConn {
    let mut source_conn = SourceConn::try_from(conn)?;
    match protocol {
        Some(p) => source_conn.set_protocol(p),
        None => {}
    }
    source_conn
}
