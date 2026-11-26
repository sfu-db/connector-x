use crate::constants::CONNECTORX_PROTOCOL;
use crate::errors::{ConnectorXError, Result};
use crate::utils::remove_query_params;
use anyhow::anyhow;
use fehler::throws;
#[cfg(feature = "src_redshift")]
use redshift_iam::redshift_to_postgres;
use std::convert::TryFrom;
use url::Url;

/// Database kind, derived from the connection string scheme.
#[derive(Debug, Clone)]
pub enum SourceType {
    /// `postgres://`, `postgresql://` or `redshift-iam://`.
    Postgres,
    /// `sqlite://`.
    SQLite,
    /// `mysql://`.
    MySQL,
    /// `mssql://` (SQL Server).
    MsSQL,
    /// `oracle://`.
    Oracle,
    /// `bigquery://`.
    BigQuery,
    /// `duckdb://`.
    DuckDB,
    /// `trino://`.
    Trino,
    /// `clickhouse://`.
    ClickHouse,
    /// Any other scheme.
    Unknown,
}

/// Parsed connection string.
#[derive(Debug, Clone)]
pub struct SourceConn {
    /// Database kind.
    pub ty: SourceType,
    /// Connection URL without the `cxprotocol` parameter.
    pub conn: Url,
    /// ConnectorX protocol to use, `binary` by default.
    pub proto: String,
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
            #[cfg(feature = "src_redshift")]
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
    /// Create a connection from its parts.
    pub fn new(ty: SourceType, conn: Url, proto: String) -> Self {
        Self { ty, conn, proto }
    }
    /// Override the protocol.
    pub fn set_protocol(&mut self, protocol: &str) {
        self.proto = protocol.to_string();
    }
}

/// Parse a connection string, overriding the protocol when `protocol` is given.
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

    /// Without `src_redshift`, `redshift-iam://` is an unknown scheme: `src_postgres` alone does not build the AWS SDK.
    #[test]
    #[cfg(not(feature = "src_redshift"))]
    fn redshift_iam_needs_its_feature() {
        let conn = SourceConn::try_from("redshift-iam://u:p@h:5439/db").unwrap();
        assert_eq!(format!("{:?}", conn.ty), "Unknown");
    }

    /// The scheme picks the source type; a `+driver` suffix (as SQLAlchemy
    /// writes it) is ignored, and an unknown scheme is kept as `Unknown`.
    #[test]
    fn maps_schemes_to_source_types() {
        use super::SourceType::*;
        for (url, ty) in [
            ("postgres://h/db", Postgres),
            ("postgresql+psycopg2://h/db", Postgres),
            ("sqlite:///tmp/x.db", SQLite),
            ("mysql+pymysql://h/db", MySQL),
            ("mssql+pymssql://h/db", MsSQL),
            ("oracle://h/db", Oracle),
            ("bigquery:///tmp/key.json", BigQuery),
            ("duckdb:///tmp/x.duckdb", DuckDB),
            ("trino://h/catalog", Trino),
            ("clickhouse://h/db", ClickHouse),
            ("nosuchdb://h/db", Unknown),
        ] {
            let conn = SourceConn::try_from(url).unwrap();
            assert_eq!(format!("{:?}", conn.ty), format!("{ty:?}"), "{url}");
            assert_eq!(conn.proto, "binary", "{url}");
        }
    }

    #[test]
    fn parse_source_overrides_the_protocol_only_when_given() {
        let conn = super::parse_source("mysql://h/db?cxprotocol=text", None).unwrap();
        assert_eq!(conn.proto, "text");
        let conn = super::parse_source("mysql://h/db?cxprotocol=text", Some("binary")).unwrap();
        assert_eq!(conn.proto, "binary");
        assert_eq!(conn.conn.as_str(), "mysql://h/db");
    }

    #[test]
    fn rejects_strings_that_are_not_urls() {
        let err = SourceConn::try_from("not a url").unwrap_err();
        assert!(err.to_string().contains("parse error"), "{err}");
        assert!(super::parse_source("", None).is_err());
    }
}
