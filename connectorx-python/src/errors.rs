use pyo3::exceptions::PyRuntimeError;
use pyo3::PyErr;
use thiserror::Error;

#[allow(unused)]
pub type Result<T> = std::result::Result<T, ConnectorXPythonError>;

/// Errors that can be raised from this library.
#[derive(Error, Debug)]
pub enum ConnectorXPythonError {
    /// The required type does not same as the schema defined.
    #[error("Unknown pandas data type: {0}.")]
    UnknownPandasType(String),

    #[error("Python: {0}.")]
    PythonError(String),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    MsSQL(#[from] tiberius::error::Error),

    #[error(transparent)]
    MysqlError(#[from] r2d2_mysql::mysql::Error),

    #[error(transparent)]
    SQLiteError(#[from] rusqlite::Error),

    #[error(transparent)]
    PostgresError(#[from] postgres::Error),

    #[error(transparent)]
    OracleError(#[from] r2d2_oracle::oracle::Error),

    #[error(transparent)]
    NdArrayShapeError(#[from] ndarray::ShapeError),

    #[error(transparent)]
    ConnectorXError(#[from] connectorx::errors::ConnectorXError),

    #[error(transparent)]
    MsSQLSourceError(#[from] connectorx::sources::mssql::MsSQLSourceError),

    #[error(transparent)]
    PostgresSourceError(#[from] connectorx::sources::postgres::PostgresSourceError),

    #[error(transparent)]
    MySQLSourceError(#[from] connectorx::sources::mysql::MySQLSourceError),

    #[error(transparent)]
    SQLiteSourceError(#[from] connectorx::sources::sqlite::SQLiteSourceError),

    #[error(transparent)]
    OracleSourceError(#[from] connectorx::sources::oracle::OracleSourceError),

    #[error(transparent)]
    ArrowDestinationError(#[from] connectorx::destinations::arrow::ArrowDestinationError),

    #[error(transparent)]
    PostgresArrowTransportError(#[from] connectorx::transports::PostgresArrowTransportError),

    #[error(transparent)]
    MySQLArrowTransportError(#[from] connectorx::transports::MySQLArrowTransportError),

    #[error(transparent)]
    SQLiteArrowTransportError(#[from] connectorx::transports::SQLiteArrowTransportError),

    #[error(transparent)]
    MsSQLArrowTransportError(#[from] connectorx::transports::MsSQLArrowTransportError),

    #[error(transparent)]
    OracleArrowTransportError(#[from] connectorx::transports::OracleArrowTransportError),
    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<ConnectorXPythonError> for PyErr {
    fn from(e: ConnectorXPythonError) -> PyErr {
        PyRuntimeError::new_err(format!("{}", e))
    }
}

impl From<PyErr> for ConnectorXPythonError {
    fn from(e: PyErr) -> ConnectorXPythonError {
        ConnectorXPythonError::PythonError(format!("{}", e))
    }
}
