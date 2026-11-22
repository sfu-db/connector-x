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

    /// Error from the `tiberius` client.
    #[cfg(feature = "src_mssql_tiberius")]
    #[error(transparent)]
    MsSQLError(#[from] tiberius::error::Error),

    /// Error while getting a connection from the pool.
    #[cfg(feature = "src_mssql_tiberius")]
    #[error(transparent)]
    MsSQLRuntimeError(#[from] bb8_07::RunError<bb8_tiberius::Error>),

    /// Error from the connection pool manager.
    #[cfg(feature = "src_mssql_tiberius")]
    #[error(transparent)]
    MsSQLPoolError(#[from] bb8_tiberius::Error),

    /// Error from the `mssql-tds` client.
    #[cfg(feature = "src_mssql_tds")]
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
