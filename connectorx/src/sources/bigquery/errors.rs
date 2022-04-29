use gcp_bigquery_client::error::BQError;
use thiserror::Error;
use url;

#[derive(Error, Debug)]
pub enum BigQuerySourceError {
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    #[error(transparent)]
    BQError(#[from] BQError),

    #[error(transparent)]
    BigQueryUrlError(#[from] url::ParseError),

    #[error(transparent)]
    BigQueryStdError(#[from] std::io::Error),

    #[error(transparent)]
    BigQueryJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    BigQueryParseFloatError(#[from] std::num::ParseFloatError),

    #[error(transparent)]
    BigQueryParseIntError(#[from] std::num::ParseIntError),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
