use thiserror::Error;

/// Errors raised by the ClickHouse source.
#[derive(Error, Debug)]
pub enum ClickHouseSourceError {
    /// Generic ConnectorX error.
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    /// The connection string is not a valid URL.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    /// The row count of a partition could not be read.
    #[error("Failed to get row count")]
    GetNRowsFailed,

    /// A ClickHouse column type could not be parsed.
    #[error("Failed to parse type: {0}")]
    TypeParseError(String),

    /// Any other error.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
