use crate::data_order::DataOrder;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ConnectorAgentError>;

/// Errors that can be raised from this library.
#[derive(Error, Debug)]
pub enum ConnectorAgentError {
    /// The required type does not same as the schema defined.
    #[error("Data type unexpected: {0:?} expected, {1} found.")]
    UnexpectedType(String, &'static str),

    #[error("Index operation out of bound.")]
    OutOfBound,

    #[error("Data order not supported {0:?}.")]
    UnsupportedDataOrder(DataOrder),

    #[error("Cannot resolve data order: got {0:?} from source, {1:?} from destination.")]
    CannotResolveDataOrder(Vec<DataOrder>, Vec<DataOrder>),

    #[error("Cannot parse {0} from {1}.")]
    CannotParse(&'static str, String),

    #[error("Allocate is already called.")]
    DuplicatedAllocation,

    #[error("Writer has not been allocated yet.")]
    WriterNotAllocated,

    #[error("No type system conversion rule from {0} to {1}.")]
    NoTypeSystemConversionRule(String, String),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    PostgresPoolError(#[from] r2d2::Error),

    #[error(transparent)]
    PostgresError(#[from] postgres::Error),

    #[error(transparent)]
    CSVError(#[from] csv::Error),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
