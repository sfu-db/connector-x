// Why we need to implement Transmit for TypeSystem? This is because only TypeSystem knows how to dispatch
// functions to it's native type N based on our defined type T. Remember, T is value and N is a type.

use crate::errors::Result;
/// `TypeSystem` describes a type system in a value type (e.g. enum variants),
/// which can be used to type check with a static type `T` through the `check` method.
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
/// `associate_typesystem!(DataType, [DataType::F64] => f64, [DataType::I64] => i64);`
/// This means for the type system `DataType`, it's variant `DataType::F64(false)` is corresponding to the physical type f64 and
/// `DataType::F64(true)` is corresponding to the physical type Option<f64>. Same for I64 and i64
#[macro_export]
macro_rules! define_typesystem {
    ($TS:ty, $(<$LT0:lifetime>,)? $(/*multiple mapping*/$(/*multiple variant*/ [$($V:tt)+])|+ => $(<$LT:lifetime>)? [$NT:ty],)+) => {
        define_typesystem!(IMPL $TS, [$($LT0)?] $(/*multiple mapping*/
            $(/*multiple variant*/$($V)+ (false))+ => $($LT)? [$NT],
            $(/*multiple variant*/$($V)+ (true))+ => $($LT)? [Option<$NT>],
        )+);
    };

    (IMPL $TS:ty, [$($LT0:lifetime)?] $($($V:pat)+ => $($LT:lifetime)? [$NT:ty],)+) => {
        impl TypeSystem for $TS {}

        $(
            impl $(<$LT>)? $crate::typesystem::TypeAssoc<$TS> for $NT {
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

        impl<'hlt, $($LT0,)? F> $crate::typesystem::Realize<F> for $TS
        where
            F: $crate::typesystem::ParameterizedFunc,
            $(F: $crate::typesystem::ParameterizedOn<'hlt, $NT>),+
        {
            fn realize(self) -> $crate::errors::Result<F::Function> {
                match self {
                    $(
                        $($V)|+ => Ok(F::realize::<$NT>()),
                    )+
                }
            }
        }

        // Always true for type system self conversion
        impl<'hlt, $($LT0,)? F> $crate::typesystem::Realize<F> for ($TS, $TS)
        where
            F: $crate::typesystem::ParameterizedFunc,
            $(F: $crate::typesystem::ParameterizedOn<'hlt, ($NT, $NT)>),+
        {
            fn realize(self) -> $crate::errors::Result<F::Function> {
                match self {
                    $($(
                        ($V, $V) => Ok(F::realize::<($NT, $NT)>()),
                    )+)+
                    (v1, v2) => {
                        fehler::throw!($crate::errors::ConnectorAgentError::NoTypeSystemConversionRule(
                            format!("{:?}", v1), format!("{:?}", v2)
                        ))
                    }
                }
            }
        }

        $(
            impl <$($LT,)?> $crate::typesystem::TypeConversion<$NT, $NT> for ($TS, $TS) {
                fn convert(val: $NT) -> $NT { val }
            }
        )+

        impl $crate::typesystem::TypeSystemConversion<$TS> for $TS {
            fn from(dt: $TS) -> $crate::errors::Result<$TS> { Ok(dt) }
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
    fn realize<'a, T>() -> Self::Function
    where
        Self: ParameterizedOn<'a, T>,
    {
        Self::parameterize()
    }
}

/// `ParameterizedOn` indicates a parameterized function `Self`
/// is parameterized on type `T`
pub trait ParameterizedOn<'r, T>: ParameterizedFunc {
    fn parameterize() -> Self::Function;
}

pub trait TypeConversion<T, U> {
    fn convert(val: T) -> U;
}

pub trait TypeSystemConversion<T>
where
    T: TypeSystem,
    Self: TypeSystem,
{
    fn from(dt: T) -> Result<Self>;
}

/// A macro to define how to convert between one type system to another, in terms
/// of both their enum variants and the physical types
///
/// # Example Usage
/// ```ignore
/// associate_typesystems! {
///     (PostgresDTypes, DataType),
///     ([PostgresDTypes::Float4], [DataType::F64]) => (f32, f64) conversion all,
/// }
/// ```
/// This means for the type system `PostgresDTypes`, is can be converted to another type system DataType.
/// Specifically, the variant `PostgresDTypes::Float4(false)` is corresponding to the variant `DataType::F64(false)` and
/// the variant `PostgresDTypes::Float4(true)` is corresponding to the variant `DataType::F64(true)`.
/// Also, the physical type `f32` is corresponding to the physical type `f64` and the physical type `Option<f32>` is
/// corresponding to the type `Option<f64>`.
/// The last piece of `conversion all` means auto implementing the physical type conversion trait `TypeConversion<f32, f64>` and
/// conversion trait `TypeConversion<Option<f32>, Option<f64>>` for `(PostgresDTypes, DataType)` by using
/// casting rule (v as f64). If this is set to `half`, you need to manually write an `TypeConversion` implementation for `(f32,f64)`,
/// but the conversion rule for `(Option<f32>, Option<f64>)` is still generated. `conversion none` means generate nothing.
#[macro_export]
macro_rules! associate_typesystems {
    (($TS1:ty, $TS2:ty), $(<$LT0:lifetime>,)? $($(([$($v1:tt)+], [$($v2:tt)+]))|+ => $(<$LT:lifetime>)? [$T1:ty, $T2:ty] conversion $cast:ident,)+) => {
        associate_typesystems!(Transmit ($TS1, $TS2), [$($LT0)?] $(
            $(($($v1)+ (false), $($v2)+ (false)))|+ => [$($LT)?] ($T1, $T2),
            $(($($v1)+ (true), $($v2)+ (true)))|+ => [$($LT)?] (Option<$T1>, Option<$T2>),
        )+);

        associate_typesystems!(Conversion ($TS1, $TS2), $(
            $(($($v1)+ (false), $($v2)+ (false))),+,
            $(($($v1)+ (true), $($v2)+ (true))),+,
        )+);

        $(
            associate_typesystems!(TypeConversion $cast [$($LT)?] $TS1, $TS2, $T1, $T2);
        )+
    };

    (Transmit ($TS1:ty, $TS2:ty), [$($LT0:lifetime)?] $($(($v1:pat, $v2:pat))|+ => [$($LT:lifetime)?] ($T1:ty, $T2:ty),)+) => {
        impl<'a, S, W> $crate::typesystem::Transmit<S, W> for ($TS1, $TS2)
        where
            S: $crate::data_sources::Parser<'a, TypeSystem = $TS1>,
            W: $crate::writers::PartitionWriter<'a, TypeSystem = $TS1>,
            $(S: $crate::data_sources::Produce<'a, $T1>,)+
            $(W: $crate::writers::Consume<$T2>,)+
            $($T1: $crate::typesystem::TypeAssoc<S::TypeSystem>,)+
            $($T2: $crate::typesystem::TypeAssoc<W::TypeSystem>,)+
            $((S::TypeSystem, W::TypeSystem): TypeConversion<$T1, $T2>,)+
        {
            fn transmit_checked(self, source: &mut S, writer: &mut W) -> $crate::errors::Result<()> {
                match self {
                    $(
                        $(($v1, $v2))|+ => {
                            let val: $T1 = source.read()?;
                            let val = <(S::TypeSystem, W::TypeSystem) as TypeConversion<$T1, $T2>>::convert(val);
                            writer.write_checked(val)
                        }
                    )+
                    (v1, v2) => {
                        fehler::throw!($crate::errors::ConnectorAgentError::NoTypeSystemConversionRule(
                            format!("{:?}", v1), format!("{:?}", v2)
                        ))
                    }
                }
            }

            unsafe fn transmit(self, source: &'a mut S, writer: &mut W) -> $crate::errors::Result<()> {
                match self {
                    $(
                        $(($v1, $v2))|+ => {
                            let val: $T1 = source.read()?;
                            let val = <(S::TypeSystem, W::TypeSystem) as TypeConversion<$T1, $T2>>::convert(val);
                            unsafe { writer.write(val) };
                            Ok(())
                        }
                    )+
                    (v1, v2) => {
                        fehler::throw!($crate::errors::ConnectorAgentError::NoTypeSystemConversionRule(
                            format!("{:?}", v1), format!("{:?}", v2)
                        ))
                    }
                }
            }
        }
    };

    (Conversion ($TS1:ty, $TS2:ty), $(($v1:pat, $v2:expr),)+) => {
        impl $crate::typesystem::TypeSystemConversion<$TS1> for $TS2 {
            fn from(dt: $TS1) -> $crate::errors::Result<Self> {
                match dt {
                    $(
                        $v1 => Ok($v2),
                    )+
                    #[allow(unreachable_patterns)]
                    _ => fehler::throw!($crate::errors::ConnectorAgentError::NoTypeSystemConversionRule(
                        format!("{:?}", dt), format!("{}", std::any::type_name::<$TS2>())
                    ))
                }
            }
        }
    };

    (TypeConversion all [$($LT:lifetime)?] $TS1:ty, $TS2:ty, $T1:ty, $T2:ty) => {
        impl <$($LT,)?> $crate::typesystem::TypeConversion<$T1, $T2> for ($TS1, $TS2) {
            fn convert(val: $T1) -> $T2 {
                val as _
            }
        }

        associate_typesystems!(TypeConversion half [$($LT)?] $TS1, $TS2, $T1, $T2);
    };

    (TypeConversion half [$($LT:lifetime)?] $TS1:ty, $TS2:ty, $T1:ty, $T2:ty) => {
        impl <$($LT,)?> $crate::typesystem::TypeConversion<Option<$T1>, Option<$T2>> for ($TS1, $TS2) {
            fn convert(val: Option<$T1>) -> Option<$T2> {
                val.map(Self::convert)
            }
        }
    };

    (TypeConversion none [$($LT:lifetime)?] $TS1:ty, $TS2:ty, $T1:ty, $T2:ty) => {};
}

pub trait Transmit<S, W> {
    unsafe fn transmit(self, source: &mut S, writer: &mut W) -> Result<()>;

    fn transmit_checked(self, source: &mut S, writer: &mut W) -> Result<()>;
}

// #[throws(ConnectorAgentError)]
// pub fn inner<'s, 'w, 'r, S, W, T1, T2>(source: &'r mut S, writer: &mut W)
// where
//     S: Parser<'s> + Produce<'r, T1>,
//     W: PartitionWriter<'w> + Consume<T2>,
//     T1: TypeAssoc<S::TypeSystem> + 'r,
//     T2: TypeAssoc<W::TypeSystem>,
//     (S::TypeSystem, W::TypeSystem): TypeConversion<T1, T2>,
// {
//     let val: T1 = source.read()?;
//     let val = <(S::TypeSystem, W::TypeSystem) as TypeConversion<T1, T2>>::convert(val);
//     unsafe { writer.write(val) }
// }
