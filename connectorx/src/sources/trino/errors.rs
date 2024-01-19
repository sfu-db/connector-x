use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrinoSourceError {
    #[error("Cannot infer type from null for SQLite")]
    InferTypeFromNull,

    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
