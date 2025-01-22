use crate::data_order::DataOrder;
use std::any::type_name;
use std::fmt;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ConnectorXError>;
pub type OutResult<T> = std::result::Result<T, ConnectorXOutError>;

#[derive(Error, Debug)]
pub enum ConnectorXOutError {
    #[error("File {0} not found.")]
    FileNotFoundError(String),

    #[error("Source {0} not supported.")]
    SourceNotSupport(String),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[cfg(feature = "federation")]
    #[error(transparent)]
    J4RSError(#[from] j4rs::errors::J4RsError),

    #[cfg(feature = "fed_exec")]
    #[error(transparent)]
    DataFusionError(#[from] datafusion::error::DataFusionError),

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    ConnectorXInternalError(#[from] ConnectorXError),

    #[cfg(feature = "src_postgres")]
    #[error(transparent)]
    PostgresSourceError(#[from] crate::sources::postgres::PostgresSourceError),

    #[cfg(feature = "src_postgres")]
    #[error(transparent)]
    PostgresError(#[from] postgres::Error),

    #[cfg(feature = "src_mysql")]
    #[error(transparent)]
    MySQLSourceError(#[from] crate::sources::mysql::MySQLSourceError),

    #[cfg(feature = "src_mysql")]
    #[error(transparent)]
    MysqlError(#[from] r2d2_mysql::mysql::Error),

    #[cfg(feature = "src_mssql")]
    #[error(transparent)]
    MsSQLSourceError(#[from] crate::sources::mssql::MsSQLSourceError),

    #[cfg(feature = "src_mssql")]
    #[error(transparent)]
    MsSQL(#[from] tiberius::error::Error),

    #[cfg(feature = "src_sqlite")]
    #[error(transparent)]
    SQLiteSourceError(#[from] crate::sources::sqlite::SQLiteSourceError),

    #[cfg(feature = "src_sqlite")]
    #[error(transparent)]
    SQLiteError(#[from] rusqlite::Error),

    #[cfg(feature = "src_oracle")]
    #[error(transparent)]
    OracleSourceError(#[from] crate::sources::oracle::OracleSourceError),

    #[cfg(feature = "src_oracle")]
    #[error(transparent)]
    OracleError(#[from] r2d2_oracle::oracle::Error),

    #[cfg(feature = "src_bigquery")]
    #[error(transparent)]
    BigQuerySourceError(#[from] crate::sources::bigquery::BigQuerySourceError),

    #[cfg(feature = "src_bigquery")]
    #[error(transparent)]
    BigQueryError(#[from] gcp_bigquery_client::error::BQError),

    #[cfg(feature = "src_trino")]
    #[error(transparent)]
    TrinoSourceError(#[from] crate::sources::trino::TrinoSourceError),

    #[cfg(feature = "dst_arrow")]
    #[error(transparent)]
    ArrowError(#[from] crate::destinations::arrow::ArrowDestinationError),

    #[cfg(feature = "dst_arrow")]
    #[error(transparent)]
    ArrowStreamError(#[from] crate::destinations::arrowstream::ArrowDestinationError),

    #[cfg(all(feature = "src_postgres", feature = "dst_arrow"))]
    #[error(transparent)]
    PostgresArrowTransportError(#[from] crate::transports::PostgresArrowTransportError),

    #[cfg(all(feature = "src_mysql", feature = "dst_arrow"))]
    #[error(transparent)]
    MySQLArrowTransportError(#[from] crate::transports::MySQLArrowTransportError),

    #[cfg(all(feature = "src_sqlite", feature = "dst_arrow"))]
    #[error(transparent)]
    SQLiteArrowTransportError(#[from] crate::transports::SQLiteArrowTransportError),

    #[cfg(all(feature = "src_mssql", feature = "dst_arrow"))]
    #[error(transparent)]
    MsSQLArrowTransportError(#[from] crate::transports::MsSQLArrowTransportError),

    #[cfg(all(feature = "src_oracle", feature = "dst_arrow"))]
    #[error(transparent)]
    OracleArrowTransportError(#[from] crate::transports::OracleArrowTransportError),

    #[cfg(all(feature = "src_bigquery", feature = "dst_arrow"))]
    #[error(transparent)]
    BigqueryArrowTransportError(#[from] crate::transports::BigQueryArrowTransportError),

    #[cfg(all(feature = "src_trino", feature = "dst_arrow"))]
    #[error(transparent)]
    TrinoArrowTransportError(#[from] crate::transports::TrinoArrowTransportError),

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

    #[error("Data order not supported {0:?}.")]
    UnsupportedDataOrder(DataOrder),

    #[error("Cannot resolve data order: got {0:?} from source, {1:?} from destination.")]
    CannotResolveDataOrder(Vec<DataOrder>, Vec<DataOrder>),

    #[error("Cannot produce a {0}, context: {1}.")]
    CannotProduce(&'static str, ProduceContext),

    #[error("No conversion rule from {0} to {1}.")]
    NoConversionRule(String, String),

    #[error("Only support single query with SELECT statement, got {0}.")]
    SqlQueryNotSupported(String),

    #[error("Cannot get total number of rows in advance.")]
    CountError(),

    #[error(transparent)]
    SQLParserError(#[from] sqlparser::parser::ParserError),

    #[error(transparent)]
    StdIOError(#[from] std::io::Error),

    #[error(transparent)]
    StdVarError(#[from] std::env::VarError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ConnectorXError {
    pub fn cannot_produce<T>(context: Option<String>) -> Self {
        ConnectorXError::CannotProduce(type_name::<T>(), context.into())
    }
}

#[derive(Debug)]
pub enum ProduceContext {
    NoContext,
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
