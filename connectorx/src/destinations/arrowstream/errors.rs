use thiserror::Error;

/// Result type of the streaming Arrow destination.
pub type Result<T> = std::result::Result<T, ArrowDestinationError>;

/// Errors raised by the streaming Arrow destination.
#[derive(Error, Debug)]
pub enum ArrowDestinationError {
    /// Error from the `arrow` crate while building arrays or batches.
    #[error(transparent)]
    ArrowError(#[from] arrow::error::ArrowError),

    /// Generic ConnectorX error.
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
