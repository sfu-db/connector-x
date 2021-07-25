use thiserror::Error;

#[derive(Error, Debug)]
pub enum CSVSourceError {
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    #[error(transparent)]
    RegexError(#[from] regex::Error),

    #[error(transparent)]
    CSVError(#[from] csv::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
