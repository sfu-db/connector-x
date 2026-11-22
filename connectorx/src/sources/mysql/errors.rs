use thiserror::Error;

/// Errors raised by the MySQL source.
#[derive(Error, Debug)]
pub enum MySQLSourceError {
    /// Generic ConnectorX error.
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    /// Error from the `mysql` client.
    #[error(transparent)]
    MySQLError(#[from] r2d2_mysql::mysql::Error),

    /// The connection string is not valid for the `mysql` client.
    #[error(transparent)]
    MySQLUrlError(#[from] r2d2_mysql::mysql::UrlError),

    /// Error from the connection pool.
    #[error(transparent)]
    MySQLPoolError(#[from] r2d2::Error),

    /// The connection string is not a valid URL.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    /// The `ssl-mode` parameter has an unknown value.
    #[error("unsupported ssl-mode value: {0}")]
    InvalidSslMode(String),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
