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
macro_rules! associate_typesystem {
    ($ts:ty, $(/*multiple mapping*/$(/*multiple variant*/ [$($variant:tt)+])|+ => $native_type:ty,)+) => {
        associate_typesystem!(IMPL $ts, $(/*multiple mapping*/
            $(/*multiple variant*/$($variant)+ (false))+ => $native_type,
            $(/*multiple variant*/$($variant)+ (true))+ => Option<$native_type>,
        )+);
    };

    (IMPL $ts:ty, $($($variant:pat)+ => $native_type:ty,)+) => {
        $(
            impl $crate::typesystem::TypeAssoc<$ts> for $native_type {
                fn check(ts: $ts) -> $crate::errors::Result<()> {
                    match ts {
                        $(
                            $variant => Ok(()),
                        )+
                        _ => fehler::throw!($crate::errors::ConnectorAgentError::UnexpectedType(format!("{:?}", ts), std::any::type_name::<$native_type>()))
                    }
                }
            }
        )+

        impl<F> $crate::typesystem::Realize<F> for $ts
        where
            F: $crate::typesystem::ParameterizedFunc,
            $(F: $crate::typesystem::ParameterizedOn<$native_type>),+
        {
            fn realize(self) -> $crate::errors::Result<F::Function> {
                match self {
                    $(
                        $($variant)|+ => Ok(F::realize::<$native_type>()),
                    )+
                }
            }
        }

        // Always true for type system self conversion
        impl<F> $crate::typesystem::Realize<F> for ($ts, $ts)
        where
            F: $crate::typesystem::ParameterizedFunc,
            $(F: $crate::typesystem::ParameterizedOn<($native_type, $native_type)>),+
        {
            fn realize(self) -> $crate::errors::Result<F::Function> {
                match self {
                    $($(
                        ($variant, $variant) => Ok(F::realize::<($native_type,$native_type)>()),
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
            impl $crate::typesystem::TypeConversion<$native_type, $native_type> for ($ts, $ts) {
                fn convert(val: $native_type) -> $native_type { val }
            }
        )+

        impl $crate::typesystem::TypeSystemConversion<$ts> for $ts {
            fn from(dt: $ts) -> $crate::errors::Result<$ts> { Ok(dt) }
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
/// ```
/// associate_typesystems! {
///     (PostgresDTypes, DataType),
///     ([PostgresDTypes::Float4], [DataType::F64]) => (f32, f64) impl all,
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
macro_rules! associate_typesystems {
    (($ts1:ty, $ts2:ty), $($(([$($v1:tt)+], [$($v2:tt)+]))|+ => ($t1:ty, $t2:ty) conversion $cast:ident,)+) => {
        associate_typesystems!(Realize ($ts1, $ts2), $(
            $(($($v1)+ (false), $($v2)+ (false)))|+ => ($t1, $t2),
            $(($($v1)+ (true), $($v2)+ (true)))|+ => (Option<$t1>, Option<$t2>),
        )+);

        associate_typesystems!(Transmit ($ts1, $ts2), $(
            $(($($v1)+ (false), $($v2)+ (false)))|+ => ($t1, $t2),
            $(($($v1)+ (true), $($v2)+ (true)))|+ => (Option<$t1>, Option<$t2>),
        )+);

        associate_typesystems!(Conversion ($ts1, $ts2), $(
            $(($($v1)+ (false), $($v2)+ (false))),+,
            $(($($v1)+ (true), $($v2)+ (true))),+,
        )+);

        $(
            associate_typesystems!(TypeConversion $cast $ts1, $ts2, $t1, $t2);
        )+
    };

    (Realize ($ts1:ty, $ts2:ty), $($(($v1:pat, $v2:pat))|+ => ($t1:ty, $t2:ty),)+) => {
        impl<F> $crate::typesystem::Realize<F> for ($ts1, $ts2)
        where
            F: $crate::typesystem::ParameterizedFunc,
            $(F: $crate::typesystem::ParameterizedOn<($t1, $t2)>),+
        {
            fn realize(self) -> $crate::errors::Result<F::Function> {
                match self {
                    $(
                        $(($v1, $v2))|+ => Ok(F::realize::<($t1, $t2)>()),
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

    (Transmit ($TS1:ty, $TS2:ty), $($(($V1:pat, $V2:pat))|+ => ($T1:ty, $T2:ty),)+) => {
        impl<'s, 'w, S, W> $crate::typesystem::TransmitHack<S, W> for ($TS1, $TS2)
        where
            S: $crate::data_sources::Parser<'s, TypeSystem = $TS1>,
            $(S: $crate::data_sources::Produce<$T1>,)+
            W: $crate::writers::PartitionWriter<'w, TypeSystem = $TS2>,
            $(W: $crate::writers::Consume<$T2>,)+
            $($T1: $crate::typesystem::TypeAssoc<$TS1> + 'static,)+
            $($T2: $crate::typesystem::TypeAssoc<$TS2> + 'static,)+
            $(
                ($TS1, $TS2): $crate::typesystem::TypeConversion<$T1, $T2>,
            )+
        {
            #[fehler::throws($crate::errors::ConnectorAgentError)]
            fn transmit(self, source: &mut S, writer: &mut W, row: usize, col: usize) {
                match self {
                    $(
                        $(($V1, $V2))|+ => {
                            let val: $T1 = source.read()?;
                            let val =
                                <(S::TypeSystem, W::TypeSystem) as TypeConversion<$T1, $T2>>::convert(
                                    val,
                                );
                            unsafe { writer.write(row, col, val) }
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

    (Conversion ($ts1:ty, $ts2:ty), $(($v1:pat, $v2:expr),)+) => {
        impl $crate::typesystem::TypeSystemConversion<$ts1> for $ts2 {
            fn from(dt: $ts1) -> Result<Self> {
                match dt {
                    $(
                        $v1 => Ok($v2),
                    )+
                    #[allow(unreachable_patterns)]
                    _ => fehler::throw!($crate::errors::ConnectorAgentError::NoTypeSystemConversionRule(
                        format!("{:?}", dt), format!("{}", std::any::type_name::<$ts2>())
                    ))
                }
            }
        }
    };

    (TypeConversion all $ts1:ty, $ts2:ty, $t1:ty, $t2:ty) => {
        impl $crate::typesystem::TypeConversion<$t1, $t2> for ($ts1, $ts2) {
            fn convert(val: $t1) -> $t2 {
                val as _
            }
        }

        impl $crate::typesystem::TypeConversion<Option<$t1>, Option<$t2>> for ($ts1, $ts2) {
            fn convert(val: Option<$t1>) -> Option<$t2> {
                val.map(Self::convert)
            }
        }
    };

    (TypeConversion half $ts1:ty, $ts2:ty, $t1:ty, $t2:ty) => {
        impl $crate::typesystem::TypeConversion<Option<$t1>, Option<$t2>> for ($ts1, $ts2) {
            fn convert(val: Option<$t1>) -> Option<$t2> {
                val.map(Self::convert)
            }
        }
    };

    (TypeConversion none $ts1:ty, $ts2:ty, $t1:ty, $t2:ty) => {};
}

pub trait TransmitHack<S, W> {
    fn transmit(self, source: &mut S, writer: &mut W, row: usize, col: usize) -> Result<()>;
}
