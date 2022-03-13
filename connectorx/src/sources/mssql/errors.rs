use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MsSQLSourceError {
    #[error("Cannot get # of rows in the partition")]
    GetNRowsFailed,

    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    #[error(transparent)]
    MsSQLError(#[from] tiberius::error::Error),

    #[error(transparent)]
    MsSQLRuntimeError(#[from] bb8::RunError<bb8_tiberius::Error>),

    #[error(transparent)]
    MsSQLPoolError(#[from] bb8_tiberius::Error),

    #[error(transparent)]
    MsSQLUrlError(#[from] url::ParseError),

    #[error(transparent)]
    MsSQLUrlDecodeError(#[from] FromUtf8Error),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
