use crate::{data_order::DataOrder, types::DataType};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ConnectorAgentError>;

/// Errors that can be raised from this library.
#[derive(Error, Debug)]
pub enum ConnectorAgentError {
    /// The required type does not same as the schema defined.
    #[error("Data type unexpected: {0:?} expected, {1} found.")]
    UnexpectedType(DataType, &'static str),

    #[error("Index operation out of bound.")]
    OutOfBound,

    #[error("Data order not supported {0:?}.")]
    UnsupportedDataOrder(DataOrder),

    #[error("Cannot resolve data order: got {0:?} from source, {1:?} from destination.")]
    CannotResolveDataOrder(Vec<DataOrder>, Vec<DataOrder>),

    #[error("Unexpected value, expect t or f, found {0}")]
    CannotParsePostgresBool(String),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    PostgresError(#[from] postgres::Error),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
