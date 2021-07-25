use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemoryDestinationError {
    #[error("Index operation out of bound.")]
    OutOfBound,

    #[error(transparent)]
    ConnectorAgentError(#[from] crate::ConnectorAgentError),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
