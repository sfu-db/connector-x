//! This module defines traits that required to define a typesystem.
//!
//! A typesystem is an enum that describes what types can be produced by a source and accepted by a destination.
//! A typesystem also needs to implement [`TypeAssoc`] to associate the enum variants to the physical representation
//! of the types in the typesystem.

use crate::destinations::{Consume, Destination, DestinationPartition};
use crate::errors::{ConnectorXError, Result as CXResult};
use crate::sources::{PartitionParser, Produce, Source, SourcePartition};

#[doc(hidden)]
/// `TypeSystem` describes all the types a source or destination support
/// using enum variants.
/// The variant can be used to type check with a static type `T` through the `check` method.
pub trait TypeSystem: Copy + Clone + Send + Sync {
    /// Check whether T is the same type as defined by self.
    fn check<T: TypeAssoc<Self>>(self) -> CXResult<()> {
        T::check(self)
    }
}

#[doc(hidden)]
/// Associate a static type to a TypeSystem
pub trait TypeAssoc<TS: TypeSystem> {
    fn check(ts: TS) -> CXResult<()>;
}

#[doc(hidden)]
/// Realize means that a TypeSystem can realize a parameterized func F, based on its current variants.
pub trait Realize<F>
where
    F: ParameterizedFunc,
{
    /// realize a parameterized function with the type that self currently is.
    fn realize(self) -> CXResult<F::Function>;
}

#[doc(hidden)]
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

#[doc(hidden)]
/// `ParameterizedOn` indicates a parameterized function `Self`
/// is parameterized on type `T`
pub trait ParameterizedOn<T>: ParameterizedFunc {
    fn parameterize() -> Self::Function;
}

/// Defines a rule to convert a type `T` to a type `U`.
pub trait TypeConversion<T, U> {
    /// Convert `val` from `T` to `U`.
    fn convert(val: T) -> U;
}

/// Transport asks the source to produce a value, do type conversion and then write
/// the value to a destination. Do not manually implement this trait for types.
/// Use [`impl_transport!`](crate::impl_transport) to create a struct that implements this trait instead.
pub trait Transport {
    /// Type system of the source.
    type TSS: TypeSystem;
    /// Type system of the destination.
    type TSD: TypeSystem;
    /// Source this transport reads from.
    type S: Source;
    /// Destination this transport writes to.
    type D: Destination;
    /// Error type, convertible from the source, destination and ConnectorX errors.
    type Error: From<ConnectorXError>
        + From<<Self::S as Source>::Error>
        + From<<Self::D as Destination>::Error>
        + Send
        + std::fmt::Debug;

    /// convert_typesystem convert the source type system TSS to the destination
    /// type system TSD.
    fn convert_typesystem(ts: Self::TSS) -> CXResult<Self::TSD>;

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
    ) -> Result<(), Self::Error>
    where
        Self: 'd;

    /// Return a function pointer that moves one value of source type `ts1` into the
    /// destination as type `ts2`, or an error if no conversion rule exists.
    #[allow(clippy::type_complexity)]
    fn processor<'s, 'd>(
        ts1: Self::TSS,
        ts2: Self::TSD,
    ) -> CXResult<
        fn(
            src: &mut <<Self::S as Source>::Partition as SourcePartition>::Parser<'s>,
            dst: &mut <Self::D as Destination>::Partition<'d>,
        ) -> Result<(), Self::Error>,
    >
    where
        Self: 'd;
}

#[doc(hidden)]
pub fn process<'s, 'd, 'r, T1, T2, TP, S, D, ES, ED, ET>(
    src: &'r mut <<S as Source>::Partition as SourcePartition>::Parser<'s>,
    dst: &'r mut <D as Destination>::Partition<'d>,
) -> Result<(), ET>
where
    T1: TypeAssoc<<S as Source>::TypeSystem>,
    S: Source<Error = ES>,
    <S as Source>::Partition: SourcePartition<Error = ES>,

    <<S as Source>::Partition as SourcePartition>::Parser<'s>: Produce<'r, T1, Error = ES>,
    ES: From<ConnectorXError> + Send,

    T2: TypeAssoc<<D as Destination>::TypeSystem>,
    D: Destination<Error = ED>,
    <D as Destination>::Partition<'d>: Consume<T2, Error = ED>,
    ED: From<ConnectorXError> + Send,

    TP: TypeConversion<T1, T2>,
    ET: From<ES> + From<ED>,
{
    let val: T1 = PartitionParser::parse(src)?;
    let val: T2 = <TP as TypeConversion<T1, _>>::convert(val);
    DestinationPartition::write(dst, val)?;
    Ok(())
}

/// Test helpers for the mappings that `impl_typesystem!` generates.
// Only the MySQL and Postgres type-system tests use these.
#[cfg(all(test, any(feature = "src_mysql", feature = "src_postgres")))]
pub(crate) mod testing {
    use super::*;
    use std::marker::PhantomData;

    /// Realizes to the realized type's `TypeAssoc::check`.
    pub(crate) struct Check<TS>(PhantomData<TS>);

    impl<TS> ParameterizedFunc for Check<TS> {
        type Function = fn(TS) -> CXResult<()>;
    }

    impl<TS: TypeSystem, T: TypeAssoc<TS>> ParameterizedOn<T> for Check<TS> {
        fn parameterize() -> Self::Function {
            T::check
        }
    }

    /// Realizes to the realized type's name.
    pub(crate) struct Name;

    impl ParameterizedFunc for Name {
        type Function = fn() -> &'static str;
    }

    impl<T> ParameterizedOn<T> for Name {
        fn parameterize() -> Self::Function {
            std::any::type_name::<T>
        }
    }

    /// Asserts that the variant `ts(false)` realizes to the Rust type whose
    /// name ends in `rust_type`, `ts(true)` to `Option` of it, and that each
    /// realized type type-checks against its own variant and not the other.
    pub(crate) fn assert_maps<TS>(ts: fn(bool) -> TS, rust_type: &str)
    where
        TS: TypeSystem + Realize<Check<TS>> + Realize<Name> + std::fmt::Debug,
    {
        let (non_null, nullable) = (ts(false), ts(true));

        let name = Realize::<Name>::realize(non_null).unwrap()();
        assert!(
            name.ends_with(rust_type) && !name.starts_with("core::option::Option"),
            "{non_null:?} realizes to {name}, expected {rust_type}"
        );
        let name = Realize::<Name>::realize(nullable).unwrap()();
        assert!(
            name.starts_with("core::option::Option<") && name.ends_with(&format!("{rust_type}>")),
            "{nullable:?} realizes to {name}, expected Option<{rust_type}>"
        );

        for (ts, other) in [(non_null, nullable), (nullable, non_null)] {
            let check = Realize::<Check<TS>>::realize(ts).unwrap();
            assert!(check(ts).is_ok(), "{ts:?} does not check against itself");
            let err = check(other).unwrap_err();
            assert!(
                matches!(err, crate::errors::ConnectorXError::TypeCheckFailed(..)),
                "{err}"
            );
        }
    }
}
