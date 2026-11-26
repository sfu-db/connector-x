use std::string::FromUtf8Error;
use thiserror::Error;

/// Errors raised by the SQL Server source.
#[derive(Error, Debug)]
pub enum MsSQLSourceError {
    /// The row count of a partition could not be read.
    #[error("Cannot get # of rows in the partition")]
    GetNRowsFailed,

    /// Generic ConnectorX error.
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    /// Error from the `mssql-tds` client.
    #[error(transparent)]
    MsSQLTdsError(#[from] mssql_tds::error::Error),

    /// The connection string is not a valid URL.
    #[error(transparent)]
    MsSQLUrlError(#[from] url::ParseError),

    /// A percent-encoded part of the connection string is not valid UTF-8.
    #[error(transparent)]
    MsSQLUrlDecodeError(#[from] FromUtf8Error),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
