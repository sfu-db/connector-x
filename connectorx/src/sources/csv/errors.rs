use thiserror::Error;

/// Errors raised by the CSV source.
#[derive(Error, Debug)]
pub enum CSVSourceError {
    /// Generic ConnectorX error.
    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),

    /// A type inference regular expression failed to build.
    #[error(transparent)]
    RegexError(#[from] regex::Error),

    /// The file could not be read as CSV.
    #[error(transparent)]
    CSVError(#[from] csv::Error),

    /// I/O error while opening or reading a file.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
