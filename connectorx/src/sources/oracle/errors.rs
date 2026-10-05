use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OracleSourceError {
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    #[error(transparent)]
    OracleError(#[from] r2d2_oracle::oracle::Error),

    #[error(transparent)]
    OraclePoolError(#[from] r2d2::Error),

    #[error(transparent)]
    OracleUrlError(#[from] url::ParseError),

    #[error(transparent)]
    OracleUrlDecodeError(#[from] FromUtf8Error),

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
