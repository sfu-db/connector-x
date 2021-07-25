use thiserror::Error;

#[derive(Error, Debug)]
pub enum SQLiteSourceError {
    #[error(transparent)]
    ConnectorAgentError(#[from] crate::ConnectorAgentError),

    #[error(transparent)]
    SQLiteError(#[from] rusqlite::Error),

    #[error(transparent)]
    SQLitePoolError(#[from] r2d2::Error),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
