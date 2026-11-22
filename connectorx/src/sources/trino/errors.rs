use std::string::FromUtf8Error;

use thiserror::Error;

/// Errors raised by the Trino source.
#[derive(Error, Debug)]
pub enum TrinoSourceError {
    /// A column type could not be determined because its first value is `NULL`.
    #[error("Cannot infer type from null for Trino")]
    InferTypeFromNull,

    /// Generic ConnectorX error.
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    /// Error from the `prusto` client.
    #[error(transparent)]
    PrustoError(prusto::error::Error),

    /// The connection string is not a valid URL.
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    /// A percent-encoded part of the connection string is not valid UTF-8.
    #[error(transparent)]
    TrinoUrlDecodeError(#[from] FromUtf8Error),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
