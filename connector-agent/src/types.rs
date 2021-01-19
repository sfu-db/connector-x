//! Each variant in DataType represents a type that connector-agent currently
//! supports to read from a data source and write into a writer.
//! When adding a variant into DataType, please also impl TypeInfo to
//! the native type. Additionally, add that to the producer requirement for DataSource.

use crate::errors::{ConnectorAgentError, Result};
use fehler::throw;
use std::any::type_name;

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    F64,
    U64,
}

pub trait TypeInfo {
    fn check(dt: DataType) -> Result<()> {
        unimplemented!("TypeInfo not implemented for {:?}, this is a bug.", dt)
    }
    fn name() -> &'static str {
        type_name::<Self>()
    }
}

impl TypeInfo for f64 {
    fn check(dt: DataType) -> Result<()> {
        if !matches!(dt, DataType::F64) {
            throw!(ConnectorAgentError::UnexpectedType(dt, Self::name()))
        } else {
            Ok(())
        }
    }
}

impl TypeInfo for u64 {
    fn check(dt: DataType) -> Result<()> {
        if !matches!(dt, DataType::U64) {
            throw!(ConnectorAgentError::UnexpectedType(dt, Self::name()))
        } else {
            Ok(())
        }
    }
}
