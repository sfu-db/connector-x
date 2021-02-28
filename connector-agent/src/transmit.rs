use crate::data_sources::{Parser, Produce};
use crate::errors::{ConnectorAgentError, Result};
use crate::typesystem::{ParameterizedFunc, ParameterizedOn, TypeAssoc, TypeConversion};
use crate::writers::{Consume, PartitionWriter};
use fehler::throws;
use std::marker::PhantomData;

pub struct Transmit<S, W>(PhantomData<(S, W)>);

impl<S, W> ParameterizedFunc for Transmit<S, W> {
    type Function = fn(source: &mut S, writer: &mut W, row: usize, col: usize) -> Result<()>;
}

impl<'s, 'w, S, W, T1, T2> ParameterizedOn<(T1, T2)> for Transmit<S, W>
where
    S: Parser<'s> + Produce<T1>,
    W: PartitionWriter<'w> + Consume<T2>,
    T1: TypeAssoc<S::TypeSystem> + 'static,
    T2: TypeAssoc<W::TypeSystem> + 'static,
    (S::TypeSystem, W::TypeSystem): TypeConversion<T1, T2>,
{
    fn parameterize() -> Self::Function {
        #[throws(ConnectorAgentError)]
        pub fn transmit<'s, 'w, S, W, T1, T2>(
            source: &mut S,
            writer: &mut W,
            row: usize,
            col: usize,
        ) where
            S: Parser<'s> + Produce<T1>,
            W: PartitionWriter<'w> + Consume<T2>,
            T1: TypeAssoc<S::TypeSystem> + 'static,
            T2: TypeAssoc<W::TypeSystem> + 'static,
            (S::TypeSystem, W::TypeSystem): TypeConversion<T1, T2>,
        {
            let val: T1 = source.read()?;
            let val = <(S::TypeSystem, W::TypeSystem) as TypeConversion<T1, T2>>::convert(val);
            unsafe { writer.write(row, col, val) }
        }

        transmit::<S, W, T1, T2>
    }
}

pub struct TransmitChecked<S, W>(PhantomData<(S, W)>);

impl<S, W> ParameterizedFunc for TransmitChecked<S, W> {
    type Function = fn(source: &mut S, writer: &mut W, row: usize, col: usize) -> Result<()>;
}

impl<'s, 'w, S, W, T1, T2> ParameterizedOn<(T1, T2)> for TransmitChecked<S, W>
where
    S: Parser<'s> + Produce<T1>,
    W: PartitionWriter<'w> + Consume<T2>,
    T1: TypeAssoc<S::TypeSystem> + 'static,
    T2: TypeAssoc<W::TypeSystem> + 'static,
    (S::TypeSystem, W::TypeSystem): TypeConversion<T1, T2>,
{
    fn parameterize() -> Self::Function {
        #[throws(ConnectorAgentError)]
        pub fn transmit_checked<'s, 'w, S, W, T1, T2>(
            source: &mut S,
            writer: &mut W,
            row: usize,
            col: usize,
        ) where
            S: Parser<'s> + Produce<T1>,
            W: PartitionWriter<'w> + Consume<T2>,
            T1: TypeAssoc<S::TypeSystem> + 'static,
            T2: TypeAssoc<W::TypeSystem> + 'static,
            (S::TypeSystem, W::TypeSystem): TypeConversion<T1, T2>,
        {
            let val: T1 = source.read()?;
            let val = <(S::TypeSystem, W::TypeSystem) as TypeConversion<T1, T2>>::convert(val);
            writer.write_checked(row, col, val)?
        }
        transmit_checked::<S, W, T1, T2>
    }
}
