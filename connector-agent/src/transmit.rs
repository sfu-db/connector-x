use crate::data_sources::{Parser, Produce};
use crate::errors::{ConnectorAgentError, Result};
use crate::typesystem::{ParameterizedFunc, ParameterizedOn, TypeAssoc, TypeConversion};
use crate::writers::{Consume, PartitionWriter};
use fehler::throws;
use std::marker::PhantomData;

pub struct Transmit<S, W>(PhantomData<(S, W)>);

impl<S, W> ParameterizedFunc for Transmit<S, W> {
    type Function = for<'r> fn(source: &'r mut S, writer: &'r mut W) -> Result<()>;
}

impl<'s, 'w, 'r, S, W, T1, T2> ParameterizedOn<(T1, T2)> for Transmit<S, W>
where
    S: Parser<'s> + Produce<'r, T1> + 's,
    W: PartitionWriter<'w> + Consume<T2> + 'w,
    T1: TypeAssoc<S::TypeSystem>,
    T2: TypeAssoc<W::TypeSystem>,
    (S::TypeSystem, W::TypeSystem): TypeConversion<T1, T2>,
{
    fn parameterize() -> Self::Function {
        #[throws(ConnectorAgentError)]
        pub fn inner<'s: 'r, 'w: 'r, 'r, S, W, T1, T2>(source: &'r mut S, writer: &'r mut W)
        where
            S: Parser<'s> + Produce<'r, T1> + 'r,
            W: PartitionWriter<'w> + Consume<T2> + 'w,
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

pub struct TransmitChecked<'r, S, W>(PhantomData<&'r (S, W)>);

impl<'s: 'r, 'w: 'r, 'r, S: 's, W: 'w> ParameterizedFunc for TransmitChecked<'r, S, W> {
    type Function = fn(source: &'r mut S, writer: &'r mut W) -> Result<()>;
}

impl<'s: 'r, 'w: 'r, 'r, S, W, T1, T2> ParameterizedOn<(T1, T2)> for TransmitChecked<'r, S, W>
where
    S: Parser<'s> + Produce<'r, T1> + 's,
    W: PartitionWriter<'w> + Consume<T2> + 'w,
    T1: TypeAssoc<S::TypeSystem> + 'r,
    T2: TypeAssoc<W::TypeSystem>,
    (S::TypeSystem, W::TypeSystem): TypeConversion<T1, T2>,
{
    fn parameterize() -> Self::Function {
        #[throws(ConnectorAgentError)]
        pub fn inner<'s: 'r, 'w: 'r, 'r, S, W, T1, T2>(source: &'r mut S, writer: &'r mut W)
        where
            S: Parser<'s> + Produce<'r, T1> + 'r,
            W: PartitionWriter<'w> + Consume<T2> + 'w,
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
