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

pub trait TypeSystem<T> {
    fn check(&self) -> Result<()>;
}

// Macro to implement type system and saves repetitive code.
macro_rules! impl_typesystem {
    ($ts:ty, $variant:pat, $native_type:ty) => {
        impl TypeSystem<$native_type> for $ts {
            fn check(&self) -> Result<()> {
                if !matches!(self, $variant) {
                    throw!(ConnectorAgentError::UnexpectedType(*self, type_name::<$native_type>()))
                } else {
                    Ok(())
                }
            }
        }
    };
}

impl_typesystem!(DataType, DataType::F64, f64);
impl_typesystem!(DataType, DataType::U64, u64);
