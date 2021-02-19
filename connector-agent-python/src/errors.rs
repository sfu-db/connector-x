use pyo3::exceptions::PyRuntimeError;
use pyo3::PyErr;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ConnectorAgentPythonError>;

/// Errors that can be raised from this library.
#[derive(Error, Debug)]
pub enum ConnectorAgentPythonError {
    /// The required type does not same as the schema defined.
    #[error("Unknown pandas data type: {0}.")]
    UnknownPandasType(String),

    #[error(transparent)]
    ConnectorAgentError(#[from] connector_agent::ConnectorAgentError),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<ConnectorAgentPythonError> for PyErr {
    fn from(e: ConnectorAgentPythonError) -> PyErr {
        PyRuntimeError::new_err(format!("{}", e))
    }
}
