// Why we need to implement Transmit for TypeSystem? This is because only TypeSystem knows how to dispatch
// functions to it's native type N based on our defined type T. Remember, T is value and N is a type.

use crate::{
    data_sources::{DataSource, Parse},
    errors::{ConnectorAgentError, Result},
    writers::PartitionWriter,
};
use fehler::throws;

pub trait TypeSystem<T> {
    fn check(&self) -> Result<()>;
}

// Macro to implement type system and saves repetitive code.
macro_rules! impl_typesystem {
    ($ts:ty, $($variant:pat => $native_type:ty),+) => {
        $(
            impl TypeSystem<$native_type> for $ts {
                fn check(&self) -> $crate::errors::Result<()> {
                    if !matches!(self, $variant) {
                        fehler::throw!($crate::errors::ConnectorAgentError::UnexpectedType(*self, std::any::type_name::<$native_type>()))
                    } else {
                        Ok(())
                    }
                }
            }
        )+
    };
}

pub trait Transmit<S, W> {
    fn transmit(&self) -> fn(&mut S, &mut W, usize, usize) -> Result<()>;
    fn transmit_checked(&self) -> fn(&mut S, &mut W, usize, usize) -> Result<()>;
}

#[throws(ConnectorAgentError)]
pub fn transmit<'a, S, W, T>(source: &mut S, writer: &mut W, row: usize, col: usize)
where
    S: DataSource + Parse<T>,
    S::TypeSystem: TypeSystem<T>,
    W: PartitionWriter<'a>,
    W::TypeSystem: TypeSystem<T>,
{
    unsafe { writer.write::<T>(row, col, source.produce()?) }
}

#[throws(ConnectorAgentError)]
pub fn transmit_checked<'a, S, W, T>(source: &mut S, writer: &mut W, row: usize, col: usize)
where
    S: DataSource + Parse<T>,
    S::TypeSystem: TypeSystem<T>,
    W: PartitionWriter<'a>,
    W::TypeSystem: TypeSystem<T>,
{
    writer.write_checked::<T>(row, col, source.produce()?)?
}

macro_rules! impl_transmit {
    ($ts:ty, $($variant:pat => $native_type:ty),+) => {
        impl<'a, S, W> $crate::typesystem::Transmit<S, W> for $ts
        where
            S: $crate::data_sources::DataSource,
            $(S: $crate::data_sources::Parse<$native_type>,)+
            $(S::TypeSystem: $crate::typesystem::TypeSystem<$native_type>,)+
            W: PartitionWriter<'a>,
            $(W::TypeSystem: $crate::typesystem::TypeSystem<$native_type>,)+
        {
            fn transmit(&self) -> fn(&mut S, &mut W, usize, usize) -> Result<()> {
                match self {
                    $($variant => $crate::typesystem::transmit::<S, W, $native_type>),+
                }
            }

            fn transmit_checked(&self) -> fn(&mut S, &mut W, usize, usize) -> Result<()> {
                match self {
                    $($variant => $crate::typesystem::transmit_checked::<S, W, $native_type>),+
                }
            }
        }
    };
}
