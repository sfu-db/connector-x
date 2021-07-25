use pyo3::exceptions::PyRuntimeError;
use pyo3::PyErr;
use thiserror::Error;

#[allow(unused)]
pub type Result<T> = std::result::Result<T, ConnectorAgentPythonError>;

/// Errors that can be raised from this library.
#[derive(Error, Debug)]
pub enum ConnectorAgentPythonError {
    /// The required type does not same as the schema defined.
    #[error("Unknown pandas data type: {0}.")]
    UnknownPandasType(String),

    #[error("Python: {0}.")]
    PythonError(String),

    #[error(transparent)]
    MysqlError(#[from] r2d2_mysql::mysql::Error),

    #[error(transparent)]
    SQLiteError(#[from] rusqlite::Error),

    #[error(transparent)]
    PostgresError(#[from] postgres::Error),

    #[error(transparent)]
    NdArrayShapeError(#[from] ndarray::ShapeError),

    #[error(transparent)]
    ConnectorAgentError(#[from] connectorx::ConnectorAgentError),

    #[error(transparent)]
    PostgresSourceError(#[from] connectorx::sources::postgres::PostgresSourceError),

    #[error(transparent)]
    MySQLSourceError(#[from] connectorx::sources::mysql::MySQLSourceError),

    #[error(transparent)]
    SQLiteSourceError(#[from] connectorx::sources::sqlite::SQLiteSourceError),

    #[error(transparent)]
    PostgresArrowTransportError(#[from] connectorx::transports::PostgresArrowTransportError),

    #[error(transparent)]
    MySQLArrowTransportError(#[from] connectorx::transports::MySQLArrowTransportError),

    #[error(transparent)]
    SQLiteArrowTransportError(#[from] connectorx::transports::SQLiteArrowTransportError),

    #[error(transparent)]
    ArrowDestinationError(#[from] connectorx::destinations::arrow::ArrowDestinationError),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<ConnectorAgentPythonError> for PyErr {
    fn from(e: ConnectorAgentPythonError) -> PyErr {
        PyRuntimeError::new_err(format!("{}", e))
    }
}

impl From<PyErr> for ConnectorAgentPythonError {
    fn from(e: PyErr) -> ConnectorAgentPythonError {
        ConnectorAgentPythonError::PythonError(format!("{}", e))
    }
}
