use thiserror::Error;

pub type Result<T> = std::result::Result<T, ArrowDestinationError>;

#[derive(Error, Debug)]
pub enum ArrowDestinationError {
    #[error(transparent)]
    ArrowError(#[from] arrow::error::ArrowError),

    #[cfg(feature = "polars")]
    #[error(transparent)]
    PolarsError(#[from] polars::error::PolarsError),

    #[error(transparent)]
    ConnectorAgentError(#[from] crate::ConnectorAgentError),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
