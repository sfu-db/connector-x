//! Each variant in DataType represents a type that connector-agent currently
//! supports to read from a data source and write into a writer.
//! When adding a variant into DataType, please also impl TypeSystem<corresponding native type>
//! to DataType. Additionally, add that to the producer requirement for DataSource.

use crate::{
    data_sources::{DataSource, Parse},
    errors::{ConnectorAgentError, Result},
    writers::PartitionWriter,
};
use fehler::{throw, throws};
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

pub trait Transmit<S, W> {
    fn transmit(&self) -> fn(&mut S, &mut W, usize, usize) -> Result<()>;
    fn transmit_checked(&self) -> fn(&mut S, &mut W, usize, usize) -> Result<()>;
}

impl<'a, S, W> Transmit<S, W> for DataType
where
    S: DataSource,
    S::TypeSystem: TypeSystem<u64> + TypeSystem<f64>,
    W: PartitionWriter<'a>,
    W::TypeSystem: TypeSystem<f64> + TypeSystem<u64>,
{
    fn transmit(&self) -> fn(&mut S, &mut W, usize, usize) -> Result<()> {
        #[throws(ConnectorAgentError)]
        fn imp<'a, S, W, T>(source: &mut S, writer: &mut W, row: usize, col: usize)
        where
            S: DataSource + Parse<T>,
            S::TypeSystem: TypeSystem<T>,
            W: PartitionWriter<'a>,
            W::TypeSystem: TypeSystem<T>,
        {
            unsafe { writer.write::<T>(row, col, source.produce()?) }
        }

        match self {
            DataType::U64 => imp::<S, W, u64>,
            DataType::F64 => imp::<S, W, f64>,
        }
    }

    fn transmit_checked(&self) -> fn(&mut S, &mut W, usize, usize) -> Result<()> {
        #[throws(ConnectorAgentError)]
        fn imp<'a, S, W, T>(source: &mut S, writer: &mut W, row: usize, col: usize)
        where
            S: DataSource + Parse<T>,
            S::TypeSystem: TypeSystem<T>,
            W: PartitionWriter<'a>,
            W::TypeSystem: TypeSystem<T>,
        {
            writer.write_checked::<T>(row, col, source.produce()?)?
        }

        match self {
            DataType::U64 => imp::<S, W, u64>,
            DataType::F64 => imp::<S, W, f64>,
        }
    }
}
