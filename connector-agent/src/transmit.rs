use crate::data_sources::{Parser, Produce};
use crate::errors::{ConnectorAgentError, Result};
use crate::typesystem::{ParameterizedFunc, ParameterizedOn, TypeAssoc, TypeConversion};
use crate::writers::{Consume, PartitionWriter};
use fehler::throws;
use std::marker::PhantomData;

pub struct Transmit<S, W>(PhantomData<(S, W)>);

impl<S, W> ParameterizedFunc for Transmit<S, W> {
    type Function = fn(source: &mut S, writer: &mut W) -> Result<()>;
}

impl<'s, 'w, 'r, S, W, T1, T2> ParameterizedOn<'r, (T1, T2)> for Transmit<S, W>
where
    S: Parser<'s> + Produce<'r, T1>,
    W: PartitionWriter<'w> + Consume<T2>,
    T1: TypeAssoc<S::TypeSystem>,
    T2: TypeAssoc<W::TypeSystem>,
    (S::TypeSystem, W::TypeSystem): TypeConversion<T1, T2>,
{
    fn parameterize() -> Self::Function {
        #[throws(ConnectorAgentError)]
        pub fn inner<'s, 'w, 'r, S, W, T1, T2>(source: &'r mut S, writer: &mut W)
        where
            S: Parser<'s> + Produce<'r, T1>,
            W: PartitionWriter<'w> + Consume<T2>,
            T1: TypeAssoc<S::TypeSystem> + 'r,
            T2: TypeAssoc<W::TypeSystem>,
            (S::TypeSystem, W::TypeSystem): TypeConversion<T1, T2>,
        {
            let val: T1 = source.read()?;
            let val = <(S::TypeSystem, W::TypeSystem) as TypeConversion<T1, T2>>::convert(val);
            unsafe { writer.write(val) }
        }

        inner::<S, W, T1, T2>
    }
}

pub struct TransmitChecked<S, W>(PhantomData<(S, W)>);

impl<S, W> ParameterizedFunc for TransmitChecked<S, W> {
    type Function = fn(source: &mut S, writer: &mut W) -> Result<()>;
}

impl<'r, S, W, T1, T2> ParameterizedOn<'r, (T1, T2)> for TransmitChecked<S, W>
where
    S: Parser<'r> + Produce<'r, T1>,
    W: PartitionWriter<'r> + Consume<T2>,
    T1: TypeAssoc<S::TypeSystem> + 'r,
    T2: TypeAssoc<W::TypeSystem>,
    (S::TypeSystem, W::TypeSystem): TypeConversion<T1, T2>,
{
    fn parameterize() -> Self::Function {
        #[throws(ConnectorAgentError)]
        pub fn inner<'r, S, W, T1, T2>(source: &'r mut S, writer: &mut W)
        where
            S: Parser<'r> + Produce<'r, T1>,
            W: PartitionWriter<'r> + Consume<T2>,
            T1: TypeAssoc<S::TypeSystem> + 'r,
            T2: TypeAssoc<W::TypeSystem>,
            (S::TypeSystem, W::TypeSystem): TypeConversion<T1, T2>,
        {
            let val: T1 = source.read()?;
            let val = <(S::TypeSystem, W::TypeSystem) as TypeConversion<T1, T2>>::convert(val);
            writer.write_checked(val)?
        }
        inner::<S, W, T1, T2>
    }
}
