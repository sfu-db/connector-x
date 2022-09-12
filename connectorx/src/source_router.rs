use crate::constants::CONNECTORX_PROTOCOL;
use crate::errors::{ConnectorXError, Result};
use anyhow::anyhow;
use std::convert::TryFrom;
use url::Url;

pub enum SourceType {
    Postgres,
    SQLite,
    MySQL,
    MsSQL,
    Oracle,
    BigQuery,
}

pub struct SourceConn {
    pub ty: SourceType,
    pub conn: Url,
}

impl TryFrom<&str> for SourceConn {
    type Error = ConnectorXError;

    fn try_from(conn: &str) -> Result<SourceConn> {
        let url = Url::parse(conn).map_err(|e| anyhow!("parse error: {}", e))?;
        // users from sqlalchemy may set engine in connection url (e.g. mssql+pymssql://...)
        // only for compatablility, we don't use the same engine
        match url.scheme().split('+').collect::<Vec<&str>>()[0] {
            "postgres" | "postgresql" => Ok(SourceConn {
                ty: SourceType::Postgres,
                conn: url,
            }),
            "sqlite" => Ok(SourceConn {
                ty: SourceType::SQLite,
                conn: url,
            }),
            "mysql" => Ok(SourceConn {
                ty: SourceType::MySQL,
                conn: url,
            }),
            "mssql" => Ok(SourceConn {
                ty: SourceType::MsSQL,
                conn: url,
            }),
            "oracle" => Ok(SourceConn {
                ty: SourceType::Oracle,
                conn: url,
            }),
            "bigquery" => Ok(SourceConn {
                ty: SourceType::BigQuery,
                conn: url,
            }),
            _ => unimplemented!("Connection: {} not supported!", conn),
        }
    }
}

impl SourceConn {
    pub fn set_protocol(&mut self, protocol: &str) {
        self.conn
            .query_pairs_mut()
            .append_pair(CONNECTORX_PROTOCOL, protocol);
    }

    pub fn get_protocol(&self) -> String {
        match self.conn.query_pairs().find(|p| p.0 == CONNECTORX_PROTOCOL) {
            Some((_, proto)) => proto.to_owned().to_string(),
            None => "binary".to_string(),
        }
    }
}
