use thiserror::Error;

/// Result type of the Arrow destination.
pub type Result<T> = std::result::Result<T, ArrowDestinationError>;

/// Errors raised by the Arrow destination.
#[derive(Error, Debug)]
pub enum ArrowDestinationError {
    /// Error from the `arrow` crate while building arrays or batches.
    #[error(transparent)]
    ArrowError(#[from] arrow::error::ArrowError),

    /// Generic ConnectorX error.
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    /// Error from Polars while converting the batches to a `DataFrame`.
    #[cfg(feature = "dst_polars")]
    #[error(transparent)]
    PolarsError(#[from] polars::error::PolarsError),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
