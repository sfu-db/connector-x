use crate::types::DataType;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ConnectorAgentError>;

#[derive(Error, Debug)]
pub enum ConnectorAgentError {
    #[error("Data type unexpected: {0:?} expected, {1} found.")]
    UnexpectedType(DataType, &'static str),

    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}
