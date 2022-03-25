use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SQLiteSourceError {
    #[error("Cannot infer type from null for SQLite")]
    InferTypeFromNull,

    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    #[error(transparent)]
    SQLiteError(#[from] rusqlite::Error),

    #[error(transparent)]
    SQLitePoolError(#[from] r2d2::Error),

    #[error(transparent)]
    SQLiteUrlDecodeError(#[from] FromUtf8Error),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
