// Why we need to implement Transmit for TypeSystem? This is because only TypeSystem knows how to dispatch
// functions to it's native type N based on our defined type T. Remember, T is value and N is a type.

use crate::data_sources::{PartitionedSource, Source};
use crate::errors::Result;
use crate::writers::Writer;

/// `TypeSystem` describes all the types a source or writer support
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

/// A macro to implement `TypeAssoc` and `Realize` which saves repetitive code.
///
/// # Example Usage
/// `impl_typesystem!(DataType, [DataType::F64] => f64, [DataType::I64] => i64);`
/// This means for the type system `DataType`, it's variant `DataType::F64(false)` is corresponding to the physical type f64 and
/// `DataType::F64(true)` is corresponding to the physical type Option<f64>. Same for I64 and i64
#[macro_export]
macro_rules! impl_typesystem {
    ($TS:ty, $(/*multiple mapping*/$(/*multiple variant*/ [$($V:tt)+])|+ => $NT:ty,)+) => {
        impl $crate::typesystem::TypeSystem for $TS {}

        impl_typesystem!(IMPL $TS, $(/*multiple mapping*/
            $(/*multiple variant*/$($V)+ (false))+ => $NT,
            $(/*multiple variant*/$($V)+ (true))+ => Option<$NT>,
        )+);
    };

    (IMPL $TS:ty, $($($V:pat)+ => $NT:ty,)+) => {
        $(
            impl $crate::typesystem::TypeAssoc<$TS> for $NT {
                fn check(ts: $TS) -> $crate::errors::Result<()> {
                    match ts {
                        $(
                            $V => Ok(()),
                        )+
                        _ => fehler::throw!($crate::errors::ConnectorAgentError::UnexpectedType(format!("{:?}", ts), std::any::type_name::<$NT>()))
                    }
                }
            }
        )+

        impl<F> $crate::typesystem::Realize<F> for $TS
        where
            F: $crate::typesystem::ParameterizedFunc,
            $(F: $crate::typesystem::ParameterizedOn<$NT>),+
        {
            fn realize(self) -> $crate::errors::Result<F::Function> {
                match self {
                    $(
                        $($V)|+ => Ok(F::realize::<$NT>()),
                    )+
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
    fn realize(self) -> Result<F::Function>;
}

/// A ParameterizedFunc refers to a function that is parameterized on a type T,
/// where type T will be dynaically determined by the variant of a TypeSystem.
/// An example is the `transmit<S,W,T>` function. When piping values from a source
/// to the writer, its type `T` is determined by the schema at the runtime.
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
/// the value to a writer.
pub trait Transport {
    type TS1: TypeSystem;
    type TS2: TypeSystem;
    type S: Source;
    type W: Writer;

    /// convert_typesystem convert the source type system TS1 to the writer
    /// type system TS2.
    fn convert_typesystem(ts: Self::TS1) -> Result<Self::TS2>;

    /// convert_type convert the type T1 associated with the source type system
    /// TS1 to a type T2 which is associated with the writer type system TS2.
    fn convert_type<T1, T2>(val: T1) -> T2
    where
        Self: TypeConversion<T1, T2>,
    {
        <Self as TypeConversion<T1, T2>>::convert(val)
    }

    /// `process` will ask source to produce a value with type T1, based on TS1, and then do
    /// type conversion using `convert_type` to get value with type T2, which is associated to
    /// TS2. Finally, it will write the value with type T2 to the writer.
    fn process<'s, 'w>(
        ts1: Self::TS1,
        ts2: Self::TS2,
        source: &mut <<Self::S as Source>::Partition as PartitionedSource>::Parser<'s>,
        writer: &mut <Self::W as Writer>::PartitionWriter<'w>,
    ) -> Result<()>;
}

/// A macro to help define Transport.
///
/// # Example Usage
/// ```ignore
/// impl_transport! {
///    ['py],
///    PostgresPandasTransport<'py>,
///    PostgresDTypes => PandasTypes,
///    PostgresSource => PandasWriter<'py>,
///    ([PostgresDTypes::Float4], [PandasTypes::F64]) => (f32, f64) conversion all
/// }
/// ```
/// This implements `Transport` to `PostgresPandasTransport<'py>`.
/// The lifetime used must be declare in the first argument in the bracket.
#[macro_export]
macro_rules! impl_transport {
    ([$($LT:lifetime)?], $TP:ty, $TS1:ty => $TS2:ty, $S:ty => $W:ty, $($(([$($V1:tt)+], [$($V2:tt)+]))|+ => ($T1:ty, $T2:ty) conversion $cast:ident,)+) => {
        impl_transport! (
            Transport [$($LT)?], $TP, ($TS1, $TS2) ($S => $W) $(
                $([$($V1)+ (false), $($V2)+ (false)] => [$T1, $T2])+
                $([$($V1)+ (true), $($V2)+ (true)] => [Option<$T1>, Option<$T2>])+
            )+
        );

        impl_transport!(TypeConversionDispatch [$($LT)?] $TP, $($cast $T1 => $T2)+);
    };

    (Transport [$($LT:lifetime)?], $TP:ty, ($TS1:ty, $TS2:ty) ($S:ty => $W:ty) $([$V1:pat, $($V2:tt)+] => [$T1:ty, $T2:ty])+ ) => {
        impl <$($LT)?> $crate::typesystem::Transport for $TP {
            type TS1 = $TS1;
            type TS2 = $TS2;
            type S = $S;
            type W = $W;

            fn convert_typesystem(ts: Self::TS1) -> $crate::errors::Result<Self::TS2> {
                match ts {
                    $(
                        $V1 => Ok($crate::cvt!(Expr $($V2)+)),
                    )+
                    #[allow(unreachable_patterns)]
                    _ => fehler::throw!($crate::errors::ConnectorAgentError::NoTypeSystemConversionRule(
                        format!("{:?}", ts), format!("{}", std::any::type_name::<Self::TS2>())
                    ))
                }
            }

            fn process<'s, 'w>(
                ts1: Self::TS1,
                ts2: Self::TS2,
                source: &mut <<Self::S as $crate::data_sources::Source>::Partition as $crate::data_sources::PartitionedSource>::Parser<'s>,
                writer: &mut <Self::W as $crate::writers::Writer>::PartitionWriter<'w>,
            ) -> $crate::errors::Result<()> {
                match (ts1, ts2) {
                    $(
                        ($V1, $crate::cvt!(Pat $($V2)+)) => {
                            let val: $T1 = $crate::data_sources::Parser::read(source)?;
                            let val = <Self as TypeConversion<$T1, $T2>>::convert(val);
                            $crate::writers::PartitionWriter::write(writer, val)?;
                            Ok(())
                        }
                    )+
                    #[allow(unreachable_patterns)]
                    _ => fehler::throw!($crate::errors::ConnectorAgentError::NoTypeSystemConversionRule(
                        format!("{:?}", ts1), format!("{:?}", ts1))
                    )
                }

            }
        }
    };

    (TypeConversionDispatch [$LT:lifetime] $TP:ty, $($cast:ident $T1:ty => $T2:ty)+) => {
        $(
            impl_transport!(TypeConversion $cast [$LT] $TP, $T1 => $T2);
        )+
    };

    (TypeConversionDispatch [] $TP:ty, $($cast:ident $T1:ty => $T2:ty)+) => {
        $(
            impl_transport!(TypeConversion $cast [] $TP, $T1 => $T2);
        )+
    };

    (TypeConversion all [$($LT:lifetime)?] $TP:ty, $T1:ty => $T2:ty) => {
        impl <$($LT)?> $crate::typesystem::TypeConversion<$T1, $T2> for $TP {
            fn convert(val: $T1) -> $T2 {
                val as _
            }
        }

        impl <$($LT)?> $crate::typesystem::TypeConversion<Option<$T1>, Option<$T2>> for $TP {
            fn convert(val: Option<$T1>) -> Option<$T2> {
                val.map(Self::convert)
            }
        }
    };


    (TypeConversion half [$($LT:lifetime)?] $TP:ty, $T1:ty => $T2:ty) => {
        impl <$($LT)?> $crate::typesystem::TypeConversion<Option<$T1>, Option<$T2>> for $TP {
            fn convert(val: Option<$T1>) -> Option<$T2> {
                val.map(Self::convert)
            }
        }
    };

    (TypeConversion none [$($LT:lifetime)?] $TP:ty, $T1:ty => $T2:ty) => {};
}

#[macro_export]
macro_rules! cvt {
    (Pat $V:pat) => {
        $V
    };

    (Expr $V:expr) => {
        $V
    };

    (Ident $V:ident) => {
        $V
    };

    (Ty $V:ty) => {
        $V
    };
}
