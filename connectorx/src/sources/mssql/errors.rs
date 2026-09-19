use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MsSQLSourceError {
    #[error("Cannot get # of rows in the partition")]
    GetNRowsFailed,

    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    #[error(transparent)]
    MsSQLTdsError(#[from] mssql_tds::error::Error),

    #[error(transparent)]
    MsSQLUrlError(#[from] url::ParseError),

    #[error(transparent)]
    MsSQLUrlDecodeError(#[from] FromUtf8Error),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
