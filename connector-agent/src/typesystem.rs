// Why we need to implement Transmit for TypeSystem? This is because only TypeSystem knows how to dispatch
// functions to it's native type N based on our defined type T. Remember, T is value and N is a type.

use crate::destinations::{Consume, Destination, DestinationPartition};
use crate::errors::Result;
use crate::sources::{PartitionParser, Produce, Source, SourcePartition};

/// `TypeSystem` describes all the types a source or destination support
/// using enum variants.
/// The variant can be used to type check with a static type `T` through the `check` method.
pub trait TypeSystem: Copy + Clone + Send + Sync {
    /// Check whether T is the same type as defined by self.
    fn check<T: TypeAssoc<Self>>(self) -> Result<()> {
        T::check(self)
    }
}

/// Associate a static type to a TypeSystem
pub trait TypeAssoc<TS: TypeSystem> {
    fn check(ts: TS) -> Result<()>;
}

/// Realize means that a TypeSystem can realize a parameterized func F, based on its current variants.
pub trait Realize<F>
where
    F: ParameterizedFunc,
{
    /// realize a parameterized function with the type that self currently is.
    fn realize(self) -> Result<F::Function>;
}

/// A ParameterizedFunc refers to a function that is parameterized on a type T,
/// where type T will be dynaically determined by the variant of a TypeSystem.
/// An example is the `transmit<S,W,T>` function. When piping values from a source
/// to the destination, its type `T` is determined by the schema at the runtime.
pub trait ParameterizedFunc {
    type Function;
    fn realize<T>() -> Self::Function
    where
        Self: ParameterizedOn<T>,
    {
        Self::parameterize()
    }
}

/// `ParameterizedOn` indicates a parameterized function `Self`
/// is parameterized on type `T`
pub trait ParameterizedOn<T>: ParameterizedFunc {
    fn parameterize() -> Self::Function;
}

pub trait TypeConversion<T, U> {
    fn convert(val: T) -> U;
}

/// Transport defines how to produce a value, do type conversion and then write
/// the value to a destination.
pub trait Transport {
    type TSS: TypeSystem;
    type TSD: TypeSystem;
    type S: Source;
    type D: Destination;

    /// convert_typesystem convert the source type system TSS to the destination
    /// type system TSD.
    fn convert_typesystem(ts: Self::TSS) -> Result<Self::TSD>;

    /// convert_type convert the type T1 associated with the source type system
    /// TSS to a type T2 which is associated with the destination type system TSD.
    fn convert_type<T1, T2>(val: T1) -> T2
    where
        Self: TypeConversion<T1, T2>,
    {
        <Self as TypeConversion<T1, T2>>::convert(val)
    }

    /// `process` will ask source to produce a value with type T1, based on TSS, and then do
    /// type conversion using `convert_type` to get value with type T2, which is associated to
    /// TSD. Finally, it will write the value with type T2 to the destination.
    fn process<'s, 'd, 'r>(
        ts1: Self::TSS,
        ts2: Self::TSD,
        src: &'r mut <<Self::S as Source>::Partition as SourcePartition>::Parser<'s>,
        dst: &'r mut <Self::D as Destination>::Partition<'d>,
    ) -> Result<()>;

    fn process_func<'s, 'd>(
        ts1: Self::TSS,
        ts2: Self::TSD,
    ) -> Result<
        for<'r> fn(
            src: &'r mut <<Self::S as Source>::Partition as SourcePartition>::Parser<'s>,
            dst: &'r mut <Self::D as Destination>::Partition<'d>,
        ) -> Result<()>,
    >;
}

pub fn process<'s, 'd, 'r, T1, T2, TP, S, D>(
    src: &'r mut <<S as Source>::Partition as SourcePartition>::Parser<'s>,
    dst: &'r mut <D as Destination>::Partition<'d>,
) -> crate::errors::Result<()>
where
    T1: TypeAssoc<<S as Source>::TypeSystem>,
    T2: TypeAssoc<<D as Destination>::TypeSystem>,
    TP: TypeConversion<T1, T2>,
    S: Source,
    D: Destination,
    <<S as Source>::Partition as SourcePartition>::Parser<'s>: Produce<'r, T1>,
    <D as Destination>::Partition<'d>: Consume<T2>,
{
    let val: T1 = PartitionParser::parse(src)?;
    let val: T2 = <TP as TypeConversion<T1, _>>::convert(val);
    DestinationPartition::write(dst, val)?;
    Ok(())
}
