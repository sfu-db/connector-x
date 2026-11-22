use gcp_bigquery_client::error::BQError;
use thiserror::Error;
use url;

/// Errors raised by the BigQuery source.
#[derive(Error, Debug)]
pub enum BigQuerySourceError {
    /// Generic ConnectorX error.
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    /// Error from the BigQuery client.
    #[error(transparent)]
    BQError(#[from] BQError),

    /// The connection string is not a valid URL.
    #[error(transparent)]
    BigQueryUrlError(#[from] url::ParseError),

    /// I/O error, e.g. while reading the service account key.
    #[error(transparent)]
    BigQueryStdError(#[from] std::io::Error),

    /// JSON error, e.g. in the service account key.
    #[error(transparent)]
    BigQueryJsonError(#[from] serde_json::Error),

    /// A returned value could not be parsed as a float.
    #[error(transparent)]
    BigQueryParseFloatError(#[from] std::num::ParseFloatError),

    /// A returned value could not be parsed as an integer.
    #[error(transparent)]
    BigQueryParseIntError(#[from] std::num::ParseIntError),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
