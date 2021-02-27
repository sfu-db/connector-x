use crate::data_sources::{Parser, Produce};
use crate::errors::{ConnectorAgentError, Result};
use crate::typesystem::{ParameterizedFunc, ParameterizedOn, TypeAssoc};
use crate::writers::{Consume, PartitionWriter};
use fehler::throws;
use std::marker::PhantomData;

pub struct Transmit<S, W>(PhantomData<(S, W)>);

impl<S, W> ParameterizedFunc for Transmit<S, W> {
    type Function = fn(source: &mut S, writer: &mut W, row: usize, col: usize) -> Result<()>;
}

impl<'s, 'w, S, W, T> ParameterizedOn<T> for Transmit<S, W>
where
    S: Parser<'s> + Produce<T>,
    W: PartitionWriter<'w, TypeSystem = S::TypeSystem> + Consume<T>,
    T: TypeAssoc<S::TypeSystem> + 'static,
{
    fn parameterize() -> Self::Function {
        #[throws(ConnectorAgentError)]
        pub fn transmit<'s, 'w, S, W, T>(source: &mut S, writer: &mut W, row: usize, col: usize)
        where
            S: Parser<'s> + Produce<T>,
            W: PartitionWriter<'w, TypeSystem = S::TypeSystem> + Consume<T>,
            T: TypeAssoc<S::TypeSystem> + 'static,
        {
            unsafe { writer.write::<T>(row, col, source.read()?) }
        }

        transmit::<S, W, T>
    }
}

pub struct TransmitChecked<S, W>(PhantomData<(S, W)>);

impl<S, W> ParameterizedFunc for TransmitChecked<S, W> {
    type Function = fn(source: &mut S, writer: &mut W, row: usize, col: usize) -> Result<()>;
}

impl<'s, 'w, S, W, T> ParameterizedOn<T> for TransmitChecked<S, W>
where
    S: Parser<'s> + Produce<T>,
    W: PartitionWriter<'w, TypeSystem = S::TypeSystem> + Consume<T>,
    T: TypeAssoc<S::TypeSystem> + 'static,
{
    fn parameterize() -> Self::Function {
        #[throws(ConnectorAgentError)]
        pub fn transmit_checked<'s, 'w, S, W, T>(
            source: &mut S,
            writer: &mut W,
            row: usize,
            col: usize,
        ) where
            S: Parser<'s> + Produce<T>,
            W: PartitionWriter<'w, TypeSystem = S::TypeSystem> + Consume<T>,
            T: TypeAssoc<S::TypeSystem> + 'static,
        {
            writer.write_checked::<T>(row, col, source.read()?)?
        }
        transmit_checked::<S, W, T>
    }
}
