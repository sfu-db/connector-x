use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrinoSourceError {
    #[error("Cannot infer type from null for Trino")]
    InferTypeFromNull,

    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    #[error(transparent)]
    PrustoError(prusto::error::Error),

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    TrinoUrlDecodeError(#[from] FromUtf8Error),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
