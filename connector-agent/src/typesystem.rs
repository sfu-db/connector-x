// Why we need to implement Transmit for TypeSystem? This is because only TypeSystem knows how to dispatch
// functions to it's native type N based on our defined type T. Remember, T is value and N is a type.

use crate::errors::Result;
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

/// A macro to implement `TypeAssoc` and `Realize` which saves repetitive code.
///
/// # Example Usage
/// `associate_typesystem!(DataType, DataType::F64 => f64, DataType::U64 => u64);`
macro_rules! associate_typesystem {
    ($ts:ty, $($variant:pat => $native_type:ty),+) => {
        $(
            impl $crate::typesystem::TypeAssoc<$ts> for $native_type {
                fn check(ts: $ts) -> $crate::errors::Result<()> {
                    if !matches!(ts, $variant) {
                        fehler::throw!($crate::errors::ConnectorAgentError::UnexpectedType(ts, std::any::type_name::<$native_type>()))
                    } else {
                        Ok(())
                    }
                }
            }
        )+

        impl<F> $crate::typesystem::Realize<F> for $ts
        where
            F: $crate::typesystem::ParameterizedFunc,
            $(F: $crate::typesystem::ParameterizedOn<$native_type>),+
        {
            fn realize(self) -> F::Function {
                match self {
                    $($variant => F::realize::<$native_type>()),+
                }
            }
        }
    };
}

/// Realize means that a TypeSystem can realize a parameterized func F, based on its current variants.
pub trait Realize<F>
where
    F: ParameterizedFunc,
{
    /// realize a parameterized function with the type that self currently is.
    fn realize(self) -> F::Function;
}

/// A ParameterizedFunc refers to a function that is parameterized on a type T,
/// where type T is determined by the variant of a TypeSystem.
/// An example is the `transmit<S,W,T>` function since its type `T`
/// is determined by the schema.
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
