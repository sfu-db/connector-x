use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostgresSourceError {
    #[error(transparent)]
    ConnectorXError(#[from] crate::ConnectorXError),

    #[error(transparent)]
    PostgresPoolError(#[from] r2d2::Error),

    #[error(transparent)]
    PostgresError(#[from] postgres::Error),

    #[error(transparent)]
    CSVError(#[from] csv::Error),

    #[error(transparent)]
    HexError(#[from] hex::FromHexError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
