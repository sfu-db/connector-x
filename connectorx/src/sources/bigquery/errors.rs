use thiserror::Error;
use gcp_bigquery_client::error::BQError;
#[derive(Error, Debug)]
pub enum BigQuerySourceError {
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    #[error(transparent)]
    BQError(#[from] BQError),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
