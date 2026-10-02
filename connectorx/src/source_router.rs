use crate::constants::CONNECTORX_PROTOCOL;
use crate::errors::{ConnectorXError, Result};
use crate::utils::remove_query_params;
use anyhow::anyhow;
use fehler::{throw, throws};
#[cfg(feature = "src_postgres")]
use redshift_iam::redshift_to_postgres;
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
    ClickHouse,
    Unknown,
}

#[derive(Clone)]
pub struct SourceConn {
    pub ty: SourceType,
    pub conn: Url,
    pub proto: String,
    // Kept out of `conn` so the secret never appears in connection URLs, and
    // private so it can only be set through the validating setter.
    access_token: Option<String>,
}

// Hand-written so the access token is redacted from logs and error messages.
impl std::fmt::Debug for SourceConn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceConn")
            .field("ty", &self.ty)
            .field("conn", &self.conn)
            .field("proto", &self.proto)
            .field(
                "access_token",
                &self.access_token.as_ref().map(|_| "<redacted>"),
            )
            .finish()
    }
}

impl TryFrom<&str> for SourceConn {
    type Error = ConnectorXError;

    fn try_from(conn: &str) -> Result<SourceConn> {
        let old_url = Url::parse(conn).map_err(|e| anyhow!("parse error: {}", e))?;

        // parse connectorx protocol
        let proto = match old_url.query_pairs().find(|p| p.0 == CONNECTORX_PROTOCOL) {
            Some((_, proto)) => proto.to_string(),
            None => "binary".to_string(),
        };

        // create url by removing connectorx protocol
        let url = remove_query_params(&old_url, &[CONNECTORX_PROTOCOL]);

        // users from sqlalchemy may set engine in connection url (e.g. mssql+pymssql://...)
        // only for compatablility, we don't use the same engine
        match url.scheme().split('+').collect::<Vec<&str>>()[0] {
            "postgres" | "postgresql" => Ok(SourceConn::new(SourceType::Postgres, url, proto)),
            #[cfg(feature = "src_postgres")]
            "redshift-iam" => Ok(SourceConn::new(
                SourceType::Postgres,
                redshift_to_postgres(url),
                "cursor".to_string(),
            )),
            "sqlite" => Ok(SourceConn::new(SourceType::SQLite, url, proto)),
            "mysql" => Ok(SourceConn::new(SourceType::MySQL, url, proto)),
            "mssql" => Ok(SourceConn::new(SourceType::MsSQL, url, proto)),
            "oracle" => Ok(SourceConn::new(SourceType::Oracle, url, proto)),
            "bigquery" => Ok(SourceConn::new(SourceType::BigQuery, url, proto)),
            "duckdb" => Ok(SourceConn::new(SourceType::DuckDB, url, proto)),
            "trino" => Ok(SourceConn::new(SourceType::Trino, url, proto)),
            "clickhouse" => Ok(SourceConn::new(SourceType::ClickHouse, url, proto)),
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
            access_token: None,
        }
    }
    pub fn set_protocol(&mut self, protocol: &str) {
        self.proto = protocol.to_string();
    }

    /// Sets a Microsoft Entra ID access token (the raw JWT) used to
    /// authenticate instead of credentials in the connection URL.
    /// Only SQL Server sources accept a token.
    #[throws(ConnectorXError)]
    pub fn set_access_token(&mut self, token: &str) {
        if !matches!(self.ty, SourceType::MsSQL) {
            throw!(anyhow!(
                "access_token is only supported for SQL Server (mssql://) connections"
            ));
        }
        if token.trim().is_empty() {
            throw!(anyhow!("access_token must not be empty"));
        }
        self.access_token = Some(token.to_string());
    }

    pub fn access_token(&self) -> Option<&str> {
        self.access_token.as_deref()
    }
}

#[throws(ConnectorXError)]
pub fn parse_source(conn: &str, protocol: Option<&str>) -> SourceConn {
    let mut source_conn = SourceConn::try_from(conn)?;
    if let Some(p) = protocol {
        source_conn.set_protocol(p)
    }
    source_conn
}

#[cfg(test)]
mod tests {
    use super::SourceConn;
    use std::convert::TryFrom;

    /// Removing the connectorx protocol must not re-encode the remaining parameters:
    /// sources that percent-decode the query would otherwise receive a literal `+`
    /// wherever the caller wrote a space.
    #[test]
    fn keeps_remaining_query_params_verbatim() {
        let source_conn = SourceConn::try_from(
            "postgresql://u:p@host:5432/db?options=-c%20statement_timeout%3D1s&cxprotocol=cursor",
        )
        .unwrap();

        assert_eq!(
            source_conn.conn.query(),
            Some("options=-c%20statement_timeout%3D1s")
        );
        assert_eq!(source_conn.proto, "cursor");
    }

    /// The query is left alone even when there is no protocol parameter to remove.
    #[test]
    fn leaves_the_query_alone_when_there_is_nothing_to_remove() {
        let source_conn = SourceConn::try_from("mysql://host:3306/db?a=x%20y&b=%2Fz").unwrap();

        assert_eq!(source_conn.conn.query(), Some("a=x%20y&b=%2Fz"));
        assert_eq!(source_conn.proto, "binary");
    }

    #[test]
    fn access_token_is_only_accepted_for_mssql() {
        let mut mssql = SourceConn::try_from("mssql://host:1433/db").unwrap();
        assert_eq!(mssql.access_token(), None);
        mssql.set_access_token("eyJ.token.sig").unwrap();
        assert_eq!(mssql.access_token(), Some("eyJ.token.sig"));

        let mut postgres = SourceConn::try_from("postgresql://host:5432/db").unwrap();
        let err = postgres.set_access_token("eyJ.token.sig").unwrap_err();
        assert!(err.to_string().contains("only supported for SQL Server"));
        assert_eq!(postgres.access_token(), None);
    }

    #[test]
    fn empty_access_token_is_rejected() {
        let mut mssql = SourceConn::try_from("mssql://host:1433/db").unwrap();
        for token in ["", "  "] {
            let err = mssql.set_access_token(token).unwrap_err();
            assert!(err.to_string().contains("must not be empty"));
        }
        assert_eq!(mssql.access_token(), None);
    }

    #[test]
    fn debug_output_redacts_access_token() {
        let mut mssql = SourceConn::try_from("mssql://host:1433/db").unwrap();
        mssql.set_access_token("eyJ.secret-token.sig").unwrap();
        let debug = format!("{:?}", mssql);
        assert!(!debug.contains("secret-token"));
        assert!(debug.contains("<redacted>"));
    }
}
