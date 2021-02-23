// Each variant in DataType represents a type that connector-agent currently
// supports to read from a data source and write into a writer.
// When adding a new supported type T and associate it to the native representation N, please do
// 1. Add a T variant to DataType.
// 2. Add `DataType::T => N` to the macro impl_typesystem!.
// 3. Add `DataType::T => N` to the macro impl_transmit!.
//

use crate::{
    data_sources::{DataSource, Produce},
    errors::{ConnectorAgentError, Result},
    typesystem::{ParameterizedFunc, ParameterizedOn, TypeAssoc, TypeSystem},
    writers::{Consume, PartitionWriter},
};
use fehler::throws;
use std::marker::PhantomData;
/// This is our intermediate type system used in this library.
/// For all the sources, their output values must be one of the types defined by DataType.
/// For all the writers, they must support writing any value whose type is defined by DataType.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DataType {
    F64(bool), // nullable
    U64(bool),
    Bool(bool),
    String(bool),
}

impl TypeSystem for DataType {}

associate_typesystem! {
    DataType,
    DataType::F64(false) => f64,
    DataType::U64(false) => u64,
    DataType::Bool(false) => bool,
    DataType::String(false) => String,
    DataType::F64(true) => Option<f64>,
    DataType::U64(true) => Option<u64>,
    DataType::Bool(true) => Option<bool>,
    DataType::String(true) => Option<String>
}

pub struct Transmit<'a, S, W>(PhantomData<(&'a S, W)>);

impl<'a, S, W> ParameterizedFunc for Transmit<'a, S, W> {
    type Function = fn(source: &mut S, writer: &mut W, row: usize, col: usize) -> Result<()>;
}

impl<'a, S, W, T> ParameterizedOn<T> for Transmit<'a, S, W>
where
    S: DataSource + Produce<T>,
    W: PartitionWriter<'a, TypeSystem = S::TypeSystem> + Consume<T>,
    T: TypeAssoc<S::TypeSystem> + 'static,
{
    fn parameterize() -> Self::Function {
        #[throws(ConnectorAgentError)]
        pub fn transmit<'a, S, W, T>(source: &mut S, writer: &mut W, row: usize, col: usize)
        where
            S: DataSource + Produce<T>,
            W: PartitionWriter<'a, TypeSystem = S::TypeSystem> + Consume<T>,
            T: TypeAssoc<S::TypeSystem> + 'static,
        {
            unsafe { writer.write::<T>(row, col, source.read()?) }
        }

        transmit::<S, W, T>
    }
}

pub struct TransmitChecked<'a, S, W>(PhantomData<(&'a S, W)>);

impl<'a, S, W> ParameterizedFunc for TransmitChecked<'a, S, W> {
    type Function = fn(source: &mut S, writer: &mut W, row: usize, col: usize) -> Result<()>;
}

impl<'a, S, W, T> ParameterizedOn<T> for TransmitChecked<'a, S, W>
where
    S: DataSource + Produce<T>,
    W: PartitionWriter<'a, TypeSystem = S::TypeSystem> + Consume<T>,
    T: TypeAssoc<S::TypeSystem> + 'static,
{
    fn parameterize() -> Self::Function {
        #[throws(ConnectorAgentError)]
        pub fn transmit_checked<'a, S, W, T>(source: &mut S, writer: &mut W, row: usize, col: usize)
        where
            S: DataSource + Produce<T>,
            W: PartitionWriter<'a, TypeSystem = S::TypeSystem> + Consume<T>,
            T: TypeAssoc<S::TypeSystem> + 'static,
        {
            writer.write_checked::<T>(row, col, source.read()?)?
        }
        transmit_checked::<S, W, T>
    }
}
