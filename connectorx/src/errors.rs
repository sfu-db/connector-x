use crate::data_order::DataOrder;
use std::any::type_name;
use std::fmt;
use thiserror::Error;

/// Result type with [`ConnectorXError`] as the error.
pub type Result<T> = std::result::Result<T, ConnectorXError>;
/// Result type with [`ConnectorXOutError`] as the error.
pub type OutResult<T> = std::result::Result<T, ConnectorXOutError>;

/// Errors returned by the public entry points such as [`get_arrow`](crate::get_arrow::get_arrow). Wraps the errors of each source, destination and transport.
#[derive(Error, Debug)]
pub enum ConnectorXOutError {
    /// A file in the request does not exist.
    #[error("File {0} not found.")]
    FileNotFoundError(String),

    /// The connection string names a source that is not supported or not enabled.
    #[error("Source {0} not supported.")]
    SourceNotSupport(String),

    /// The source does not support the requested `protocol`.
    #[error("Protocol {protocol:?} is not supported for {db}; supported protocols: {supported}.")]
    ProtocolNotSupported {
        /// The protocol that was asked for.
        protocol: String,
        /// The source, e.g. `Postgres`.
        db: String,
        /// The protocols the source accepts, comma-separated.
        supported: String,
    },

    /// I/O error.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    /// JSON (de)serialization error.
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    /// Error from the JVM bridge used by the federated rewriter.
    #[cfg(feature = "federation")]
    #[error(transparent)]
    J4RSError(#[from] j4rs::errors::J4RsError),

    /// Error from DataFusion while running a federated query.
    #[cfg(feature = "fed_exec")]
    #[error(transparent)]
    DataFusionError(#[from] datafusion::error::DataFusionError),

    /// The connection string is not a valid URL.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    /// Internal ConnectorX error.
    #[error(transparent)]
    ConnectorXInternalError(#[from] ConnectorXError),

    /// Error from the Postgres source.
    #[cfg(feature = "src_postgres")]
    #[error(transparent)]
    PostgresSourceError(#[from] crate::sources::postgres::PostgresSourceError),

    /// Error from the `postgres` client.
    #[cfg(feature = "src_postgres")]
    #[error(transparent)]
    PostgresError(#[from] postgres::Error),

    /// Error from the MySQL source.
    #[cfg(feature = "src_mysql")]
    #[error(transparent)]
    MySQLSourceError(#[from] crate::sources::mysql::MySQLSourceError),

    /// Error from the `mysql` client.
    #[cfg(feature = "src_mysql")]
    #[error(transparent)]
    MysqlError(#[from] r2d2_mysql::mysql::Error),

    /// Error from the SQL Server source.
    #[cfg(feature = "src_mssql")]
    #[error(transparent)]
    MsSQLSourceError(#[from] crate::sources::mssql::MsSQLSourceError),

    /// Error from the SQLite source.
    #[cfg(feature = "src_sqlite")]
    #[error(transparent)]
    SQLiteSourceError(#[from] crate::sources::sqlite::SQLiteSourceError),

    /// Error from the `rusqlite` client.
    #[cfg(feature = "src_sqlite")]
    #[error(transparent)]
    SQLiteError(#[from] rusqlite::Error),

    /// Error from the Oracle source.
    #[cfg(feature = "src_oracle")]
    #[error(transparent)]
    OracleSourceError(#[from] crate::sources::oracle::OracleSourceError),

    /// Error from the `oracle` client.
    #[cfg(feature = "src_oracle")]
    #[error(transparent)]
    OracleError(#[from] r2d2_oracle::oracle::Error),

    /// Error from the BigQuery source.
    #[cfg(feature = "src_bigquery")]
    #[error(transparent)]
    BigQuerySourceError(#[from] crate::sources::bigquery::BigQuerySourceError),

    /// Error from the BigQuery client.
    #[cfg(feature = "src_bigquery")]
    #[error(transparent)]
    BigQueryError(#[from] gcp_bigquery_client::error::BQError),

    /// Error from the Trino source.
    #[cfg(feature = "src_trino")]
    #[error(transparent)]
    TrinoSourceError(#[from] crate::sources::trino::TrinoSourceError),

    /// Error from the Arrow destination.
    #[cfg(feature = "dst_arrow")]
    #[error(transparent)]
    ArrowError(#[from] crate::destinations::arrow::ArrowDestinationError),

    /// Error from the streaming Arrow destination.
    #[cfg(feature = "dst_arrow")]
    #[error(transparent)]
    ArrowStreamError(#[from] crate::destinations::arrowstream::ArrowDestinationError),

    /// Error from the Postgres to Arrow transport.
    #[cfg(all(feature = "src_postgres", feature = "dst_arrow"))]
    #[error(transparent)]
    PostgresArrowTransportError(#[from] crate::transports::PostgresArrowTransportError),

    /// Error from the Postgres to streaming Arrow transport.
    #[cfg(all(feature = "src_postgres", feature = "dst_arrow"))]
    #[error(transparent)]
    PostgresArrowStreamTransportError(#[from] crate::transports::PostgresArrowStreamTransportError),

    /// Error from the MySQL to Arrow transport.
    #[cfg(all(feature = "src_mysql", feature = "dst_arrow"))]
    #[error(transparent)]
    MySQLArrowTransportError(#[from] crate::transports::MySQLArrowTransportError),

    /// Error from the MySQL to streaming Arrow transport.
    #[cfg(all(feature = "src_mysql", feature = "dst_arrow"))]
    #[error(transparent)]
    MySQLArrowStreamTransportError(#[from] crate::transports::MySQLArrowStreamTransportError),

    /// Error from the SQLite to Arrow transport.
    #[cfg(all(feature = "src_sqlite", feature = "dst_arrow"))]
    #[error(transparent)]
    SQLiteArrowTransportError(#[from] crate::transports::SQLiteArrowTransportError),

    /// Error from the SQLite to streaming Arrow transport.
    #[cfg(all(feature = "src_sqlite", feature = "dst_arrow"))]
    #[error(transparent)]
    SQLiteArrowStreamTransportError(#[from] crate::transports::SQLiteArrowStreamTransportError),

    /// Error from the SQL Server to Arrow transport.
    #[cfg(all(feature = "src_mssql", feature = "dst_arrow"))]
    #[error(transparent)]
    MsSQLArrowTransportError(#[from] crate::transports::MsSQLArrowTransportError),

    /// Error from the SQL Server to streaming Arrow transport.
    #[cfg(all(feature = "src_mssql", feature = "dst_arrow"))]
    #[error(transparent)]
    MsSQLArrowStreamTransportError(#[from] crate::transports::MsSQLArrowStreamTransportError),

    /// Error from the Oracle to Arrow transport.
    #[cfg(all(feature = "src_oracle", feature = "dst_arrow"))]
    #[error(transparent)]
    OracleArrowTransportError(#[from] crate::transports::OracleArrowTransportError),

    /// Error from the Oracle to streaming Arrow transport.
    #[cfg(all(feature = "src_oracle", feature = "dst_arrow"))]
    #[error(transparent)]
    OracleArrowStreamTransportError(#[from] crate::transports::OracleArrowStreamTransportError),

    /// Error from the BigQuery to Arrow transport.
    #[cfg(all(feature = "src_bigquery", feature = "dst_arrow"))]
    #[error(transparent)]
    BigqueryArrowTransportError(#[from] crate::transports::BigQueryArrowTransportError),

    /// Error from the BigQuery to streaming Arrow transport.
    #[cfg(all(feature = "src_bigquery", feature = "dst_arrow"))]
    #[error(transparent)]
    BigqueryArrowStreamTransportError(#[from] crate::transports::BigQueryArrowStreamTransportError),

    /// Error from the Trino to Arrow transport.
    #[cfg(all(feature = "src_trino", feature = "dst_arrow"))]
    #[error(transparent)]
    TrinoArrowTransportError(#[from] crate::transports::TrinoArrowTransportError),

    /// Error from the Trino to streaming Arrow transport.
    #[cfg(all(feature = "src_trino", feature = "dst_arrow"))]
    #[error(transparent)]
    TrinoArrowStreamTransportError(#[from] crate::transports::TrinoArrowStreamTransportError),

    /// Error from the ClickHouse source.
    #[cfg(feature = "src_clickhouse")]
    #[error(transparent)]
    ClickHouseSourceError(#[from] crate::sources::clickhouse::ClickHouseSourceError),

    /// Error from the ClickHouse to Arrow transport.
    #[cfg(all(feature = "src_clickhouse", feature = "dst_arrow"))]
    #[error(transparent)]
    ClickHouseArrowTransportError(#[from] crate::transports::ClickHouseArrowTransportError),

    /// Error from the ClickHouse to streaming Arrow transport.
    #[cfg(all(feature = "src_clickhouse", feature = "dst_arrow"))]
    #[error(transparent)]
    ClickHouseArrowStreamTransportError(
        #[from] crate::transports::ClickHouseArrowStreamTransportError,
    ),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Errors that can be raised from this library.
#[derive(Error, Debug)]
pub enum ConnectorXError {
    /// The required type does not same as the schema defined.
    #[error("Data type unexpected: {0:?} expected, {1} found.")]
    TypeCheckFailed(String, &'static str),

    /// The destination does not support the data order the source asked for.
    #[error("Data order not supported {0:?}.")]
    UnsupportedDataOrder(DataOrder),

    /// Source and destination have no data order in common.
    #[error("Cannot resolve data order: got {0:?} from source, {1:?} from destination.")]
    CannotResolveDataOrder(Vec<DataOrder>, Vec<DataOrder>),

    /// A parser could not produce a value of the requested type.
    #[error("Cannot produce a {0}, context: {1}.")]
    CannotProduce(&'static str, ProduceContext),

    /// The transport has no rule to convert the source type into the destination type.
    #[error("No conversion rule from {0} to {1}.")]
    NoConversionRule(String, String),

    /// The query is not a single `SELECT` statement.
    #[error("Only support single query with SELECT statement, got {0}.")]
    SqlQueryNotSupported(String),

    /// The row count of a query could not be determined in advance.
    #[error("Cannot get total number of rows in advance.")]
    CountError(),

    /// The query could not be parsed.
    #[error(transparent)]
    SQLParserError(#[from] sqlparser::parser::ParserError),

    /// I/O error.
    #[error(transparent)]
    StdIOError(#[from] std::io::Error),

    /// An environment variable is missing or not valid Unicode.
    #[error(transparent)]
    StdVarError(#[from] std::env::VarError),

    /// Any other error.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ConnectorXOutError {
    /// The error for a `protocol` that source `db` (e.g. `SourceType::Postgres`)
    /// does not accept; `supported` lists the protocols it does.
    pub fn protocol_not_supported(protocol: &str, db: impl fmt::Debug, supported: &[&str]) -> Self {
        ConnectorXOutError::ProtocolNotSupported {
            protocol: protocol.to_string(),
            db: format!("{db:?}"),
            supported: supported.join(", "),
        }
    }
}

impl ConnectorXError {
    /// Build a [`ConnectorXError::CannotProduce`] for type `T`.
    pub fn cannot_produce<T>(context: Option<String>) -> Self {
        ConnectorXError::CannotProduce(type_name::<T>(), context.into())
    }
}

/// Optional context attached to [`ConnectorXError::CannotProduce`].
#[derive(Debug)]
pub enum ProduceContext {
    /// No context was given.
    NoContext,
    /// Free-form description of where the value came from.
    Context(String),
}

impl From<Option<String>> for ProduceContext {
    fn from(val: Option<String>) -> Self {
        match val {
            Some(c) => ProduceContext::Context(c),
            None => ProduceContext::NoContext,
        }
    }
}

impl fmt::Display for ProduceContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProduceContext::NoContext => write!(f, "No Context"),
            ProduceContext::Context(s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ConnectorXError, ProduceContext};

    #[test]
    fn formats_produce_contexts() {
        assert_eq!(
            ProduceContext::from(None).to_string(),
            ProduceContext::NoContext.to_string()
        );
        assert_eq!(
            ProduceContext::from(Some("column 1".to_string())).to_string(),
            "column 1"
        );
        assert_eq!(ProduceContext::NoContext.to_string(), "No Context");
    }

    #[test]
    fn creates_type_specific_production_errors() {
        let error = ConnectorXError::cannot_produce::<u32>(Some("value".to_string()));
        assert_eq!(error.to_string(), "Cannot produce a u32, context: value.");
    }
}
