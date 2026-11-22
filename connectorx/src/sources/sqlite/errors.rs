use std::string::FromUtf8Error;
use thiserror::Error;

/// Errors raised by the SQLite source.
#[derive(Error, Debug)]
pub enum SQLiteSourceError {
    /// A column type could not be determined because its first value is `NULL`.
    #[error("Cannot infer type from null for SQLite")]
    InferTypeFromNull,

    /// Generic ConnectorX error.
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    /// Error from the `rusqlite` client.
    #[error(transparent)]
    SQLiteError(#[from] rusqlite::Error),

    /// Error from the connection pool.
    #[error(transparent)]
    SQLitePoolError(#[from] r2d2::Error),

    /// A percent-encoded part of the connection string is not valid UTF-8.
    #[error(transparent)]
    SQLiteUrlDecodeError(#[from] FromUtf8Error),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
