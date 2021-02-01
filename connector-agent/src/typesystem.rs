// Why we need to implement Transmit for TypeSystem? This is because only TypeSystem knows how to dispatch
// functions to it's native type N based on our defined type T. Remember, T is value and N is a type.

use crate::{
    data_sources::{DataSource, Parse},
    errors::{ConnectorAgentError, Result},
    writers::PartitionWriter,
};
use fehler::throws;

/// `TypeSystem` describes a type system in a value type (e.g. enum variants),
/// which can be used to type check with a static type `T` through the `check` method.
pub trait TypeSystem: Copy + Clone {
    /// Check whether T is the same type as defined by self.
    fn check<T: TypeAssoc<Self>>(self) -> Result<()> {
        T::check(self)
    }
}

/// Associate a static type to a TypeSystem
pub trait TypeAssoc<TS: TypeSystem> {
    fn check(ts: TS) -> Result<()>;
}

/// A macro to implement `TypeAssoc` which saves repetitive code.
///
/// # Example Usage
/// `impl_type_assoc!(DataType, DataType::F64 => f64, DataType::U64 => u64);`
macro_rules! impl_type_assoc {
    ($ts:ty, $($variant:pat => $native_type:ty),+) => {
        $(
            impl TypeAssoc<$ts> for $native_type {
                fn check(ts: $ts) -> $crate::errors::Result<()> {
                    if !matches!(ts, $variant) {
                        fehler::throw!($crate::errors::ConnectorAgentError::UnexpectedType(ts, std::any::type_name::<$native_type>()))
                    } else {
                        Ok(())
                    }
                }
            }
        )+
    };
}

/// Transmit defines Self that can pull data from a data source S and push the data to the writer W.
pub trait Transmit<S, W> {
    fn transmit(&self) -> fn(&mut S, &mut W, usize, usize) -> Result<()>;
    fn transmit_checked(&self) -> fn(&mut S, &mut W, usize, usize) -> Result<()>;
}

#[throws(ConnectorAgentError)]
pub fn transmit<'a, S, W, T>(source: &mut S, writer: &mut W, row: usize, col: usize)
where
    S: DataSource + Parse<T>,
    W: PartitionWriter<'a, TypeSystem = S::TypeSystem>,
    T: TypeAssoc<S::TypeSystem> + 'static,
{
    unsafe { writer.write::<T>(row, col, source.produce()?) }
}

#[throws(ConnectorAgentError)]
pub fn transmit_checked<'a, S, W, T>(source: &mut S, writer: &mut W, row: usize, col: usize)
where
    S: DataSource + Parse<T>,
    W: PartitionWriter<'a, TypeSystem = S::TypeSystem>,
    T: TypeAssoc<S::TypeSystem> + 'static,
{
    writer.write_checked::<T>(row, col, source.produce()?)?
}

macro_rules! impl_transmit {
    ($ts:ty, $($variant:pat => $native_type:ty),+) => {
        impl<'a, S, W> $crate::typesystem::Transmit<S, W> for $ts
        where
            S: $crate::data_sources::DataSource,
            W: PartitionWriter<'a, TypeSystem=S::TypeSystem>,
            $(S: $crate::data_sources::Parse<$native_type>,)+
            $($native_type: $crate::typesystem::TypeAssoc<S::TypeSystem>,)+
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
