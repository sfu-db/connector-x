use thiserror::Error;

#[derive(Error, Debug)]
pub enum MySQLSourceError {
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    #[error(transparent)]
    MySQLError(#[from] r2d2_mysql::mysql::Error),

    #[error(transparent)]
    MySQLUrlError(#[from] r2d2_mysql::mysql::UrlError),

    #[error(transparent)]
    MySQLPoolError(#[from] r2d2::Error),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
