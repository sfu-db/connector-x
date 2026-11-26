use thiserror::Error;

/// Errors raised by the Postgres source.
#[derive(Error, Debug)]
pub enum PostgresSourceError {
    /// Generic ConnectorX error.
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    /// Error from the connection pool.
    #[error(transparent)]
    PostgresPoolError(#[from] r2d2::Error),

    /// Error from the `postgres` client.
    #[error(transparent)]
    PostgresError(#[from] postgres::Error),

    /// Error while reading `COPY ... CSV` output.
    #[error(transparent)]
    CSVError(#[from] csv::Error),

    /// A `bytea` value in CSV output is not valid hex.
    #[error(transparent)]
    HexError(#[from] hex::FromHexError),

    /// I/O error.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    /// Error while setting up TLS.
    #[error(transparent)]
    TlsError(#[from] openssl::error::ErrorStack),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
