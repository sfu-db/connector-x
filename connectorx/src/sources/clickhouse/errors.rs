use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClickHouseSourceError {
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error("Failed to get row count")]
    GetNRowsFailed,

    #[error("Failed to parse type: {0}")]
    TypeParseError(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
