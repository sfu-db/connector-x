use crate::types::DataType;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ConnectorAgentError>;

#[derive(Error, Debug)]
pub enum ConnectorAgentError {
    #[error("Wrong type for field.")]
    WrongTypeForField,

    #[error("Data type check failed, {0:?} expected, {1} found.")]
    DataTypeCheckFailed(DataType, String),
}
