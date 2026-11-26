use std::string::FromUtf8Error;
use thiserror::Error;

/// Errors raised by the Oracle source.
#[derive(Error, Debug)]
pub enum OracleSourceError {
    /// Generic ConnectorX error.
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    /// Error from the `oracle` client.
    #[error(transparent)]
    OracleError(#[from] r2d2_oracle::oracle::Error),

    /// Error from the connection pool.
    #[error(transparent)]
    OraclePoolError(#[from] r2d2::Error),

    /// The connection string is not a valid URL.
    #[error(transparent)]
    OracleUrlError(#[from] url::ParseError),

    /// A percent-encoded part of the connection string is not valid UTF-8.
    #[error(transparent)]
    OracleUrlDecodeError(#[from] FromUtf8Error),

    /// A `NUMBER` value could not be converted to a decimal.
    #[error(transparent)]
    DecimalError(#[from] rust_decimal::Error),

    /// The query returns a native `BOOLEAN` column, but the loaded Oracle Instant Client
    /// cannot be trusted to read it.
    #[error(
        "column {column} is a native BOOLEAN, but Oracle Instant Client {client} can return \
         wrong BOOLEAN values without raising an error. Upgrade Instant Client to 23.5 or \
         later, or convert the column in SQL, e.g. CAST({column} AS NUMBER)"
    )]
    UnreliableBooleanClient {
        /// Name of the first `BOOLEAN` column in the result.
        column: String,
        /// Instant Client version reported by `oracle::Version::client()`.
        client: String,
    },

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
