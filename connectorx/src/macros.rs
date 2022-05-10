/// Associate physical representations to a typesystem.
///
/// # Example Usage
/// ```ignore
/// pub enum ArrowTypeSystem {
///     Int32(bool),
///     Int64(bool),
///     UInt32(bool),
///     UInt64(bool),
///     Float32(bool),
///     Float64(bool),
///     Boolean(bool),
///     LargeUtf8(bool),
///     LargeBinary(bool),
///     Date32(bool),
///     Date64(bool),
///     Time64(bool),
///     DateTimeTz(bool),
/// }
///
/// impl_typesystem! {
///     system = ArrowTypeSystem,
///     mappings = {
///         { Int32      => i32           }
///         { Int64      => i64           }
///         { UInt32     => u32           }
///         { UInt64     => u64           }
///         { Float64    => f64           }
///         { Float32    => f32           }
///         { Boolean    => bool          }
///         { LargeUtf8  => String        }
///         { LargeBinary => Vec<u8>      }
///         { Date32     => NaiveDate     }
///         { Date64     => NaiveDateTime }
///         { Time64     => NaiveTime     }
///         { DateTimeTz => DateTime<Utc> }
///     }
/// }
/// ```
/// This means for the type system `ArrowTypeSystem`, it's variant `ArrowTypeSystem::Int32(false)` is corresponding to the physical type `i32` and
/// `ArrowTypeSystem::Int32(true)` is corresponding to the physical type `Option<i32>`.
#[macro_export]
macro_rules! impl_typesystem {
    (
        system = $TS:tt,
        mappings = {
            $(
                { $($V:tt)|+ => $NT:ty }
            )*
        }
    ) => {
        impl $crate::typesystem::TypeSystem for $TS {}

        $(
            impl_typesystem!(@typeassoc $TS [$($V)+], $NT);
        )+

        impl_typesystem!(@realize $TS $([ [$($V)+] => $NT ])+ );
    };

    (@typeassoc $TS:tt [$($V:tt)+], $NT:ty) => {
        impl<'r> $crate::typesystem::TypeAssoc<$TS> for $NT {
            fn check(ts: $TS) -> $crate::errors::Result<()> {
                match ts {
                    $(
                        $TS::$V(false) => Ok(()),
                    )+
                    _ => fehler::throw!($crate::errors::ConnectorXError::TypeCheckFailed(format!("{:?}", ts), std::any::type_name::<$NT>()))
                }
            }
        }

        impl<'r> $crate::typesystem::TypeAssoc<$TS> for Option<$NT> {
            fn check(ts: $TS) -> $crate::errors::Result<()> {
                match ts {
                    $(
                        $TS::$V(true) => Ok(()),
                    )+
                    _ => fehler::throw!($crate::errors::ConnectorXError::TypeCheckFailed(format!("{:?}", ts), std::any::type_name::<$NT>()))
                }
            }
        }
    };

    (@realize $TS:tt $([ [$($V:tt)+] => $NT:ty ])+) => {
        impl<'r, F> $crate::typesystem::Realize<F> for $TS
        where
            F: $crate::typesystem::ParameterizedFunc,
            $(F: $crate::typesystem::ParameterizedOn<$NT>,)+
            $(F: $crate::typesystem::ParameterizedOn<Option<$NT>>,)+
        {
            fn realize(self) -> $crate::errors::Result<F::Function> {
                match self {
                    $(
                        $(
                            $TS::$V(false) => Ok(F::realize::<$NT>()),
                        )+
                        $(
                            $TS::$V(true) => Ok(F::realize::<Option<$NT>>()),
                        )+
                    )+
                }
            }
        }
    };
}

/// A macro to help define a Transport.
///
/// # Example Usage
/// ```ignore
/// impl_transport!(
///     name = MsSQLArrowTransport,
///     error = MsSQLArrowTransportError,
///     systems = MsSQLTypeSystem => ArrowTypeSystem,
///     route = MsSQLSource => ArrowDestination,
///     mappings = {
///         { Tinyint[u8]                   => Int32[i32]                | conversion auto }
///         { Smallint[i16]                 => Int32[i32]                | conversion auto }
///         { Int[i32]                      => Int32[i32]                | conversion auto }
///         { Bigint[i64]                   => Int64[i64]                | conversion auto }
///         { Intn[IntN]                    => Int64[i64]                | conversion option }
///         { Float24[f32]                  => Float32[f32]              | conversion auto }
///         { Float53[f64]                  => Float64[f64]              | conversion auto }
///         { Floatn[FloatN]                => Float64[f64]              | conversion option }
///         { Bit[bool]                     => Boolean[bool]             | conversion auto  }
///         { Nvarchar[&'r str]             => LargeUtf8[String]         | conversion owned }
///         { Varchar[&'r str]              => LargeUtf8[String]         | conversion none }
///         { Nchar[&'r str]                => LargeUtf8[String]         | conversion none }
///         { Char[&'r str]                 => LargeUtf8[String]         | conversion none }
///         { Text[&'r str]                 => LargeUtf8[String]         | conversion none }
///         { Ntext[&'r str]                => LargeUtf8[String]         | conversion none }
///         { Binary[&'r [u8]]              => LargeBinary[Vec<u8>]      | conversion owned }
///         { Varbinary[&'r [u8]]           => LargeBinary[Vec<u8>]      | conversion none }
///         { Image[&'r [u8]]               => LargeBinary[Vec<u8>]      | conversion none }
///         { Numeric[Decimal]              => Float64[f64]              | conversion option }
///         { Decimal[Decimal]              => Float64[f64]              | conversion none }
///         { Datetime[NaiveDateTime]       => Date64[NaiveDateTime]     | conversion auto }
///         { Datetime2[NaiveDateTime]      => Date64[NaiveDateTime]     | conversion none }
///         { Smalldatetime[NaiveDateTime]  => Date64[NaiveDateTime]     | conversion none }
///         { Date[NaiveDate]               => Date32[NaiveDate]         | conversion auto }
///         { Datetimeoffset[DateTime<Utc>] => DateTimeTz[DateTime<Utc>] | conversion auto }
///         { Uniqueidentifier[Uuid]        => LargeUtf8[String]         | conversion option }
///     }
/// );
/// ```
/// This implements a `Transport` called `MsSQLArrowTransport` that can convert types from MsSQL to Arrow.
#[macro_export]
macro_rules! impl_transport {
    (
        name = $TP:ty,
        error = $ET:ty,
        systems = $TSS:tt => $TSD:tt,
        route = $S:ty => $D:ty,
        mappings = {
            $(
                { $($TOKENS:tt)+ }
            )*
        }
    ) => {
        $(
            impl_transport!(@cvt $TP, $($TOKENS)+);
        )*

        impl_transport!(@transport $TP, $ET [$TSS, $TSD] [$S, $D] $([ $($TOKENS)+ ])*);
    };

    // transport
    (@transport $TP:ty, $ET:ty [$TSS:tt, $TSD:tt] [$S:ty, $D:ty] $([ $($TOKENS:tt)+ ])*) => {
        impl <'tp> $crate::typesystem::Transport for $TP {
            type TSS = $TSS;
            type TSD = $TSD;
            type S = $S;
            type D = $D;
            type Error = $ET;

            impl_transport!(@cvtts [$TSS, $TSD] $([ $($TOKENS)+ ])*);
            impl_transport!(@process [$TSS, $TSD] $([ $($TOKENS)+ ])*);
            impl_transport!(@processor [$TSS, $TSD] $([ $($TOKENS)+ ])*, $([ $($TOKENS)+ ])*);
        }
    };

    (@cvtts [$TSS:tt, $TSD:tt] $( [$V1:tt [$T1:ty] => $V2:tt [$T2:ty] | conversion $HOW:ident] )*) => {
        fn convert_typesystem(ts: Self::TSS) -> $crate::errors::Result<Self::TSD> {
            match ts {
                $(
                    $TSS::$V1(true) => Ok($TSD::$V2(true)),
                    $TSS::$V1(false) => Ok($TSD::$V2(false)),
                )*
                #[allow(unreachable_patterns)]
                _ => fehler::throw!($crate::errors::ConnectorXError::NoConversionRule(
                    format!("{:?}", ts), format!("{}", std::any::type_name::<Self::TSD>())
                ))
            }
        }
    };

    (@process [$TSS:tt, $TSD:tt] $([ $V1:tt [$T1:ty] => $V2:tt [$T2:ty] | conversion $HOW:ident ])*) => {
        fn process<'s, 'd, 'r>(
            ts1: Self::TSS,
            ts2: Self::TSD,
            src: &'r mut <<Self::S as $crate::sources::Source>::Partition as $crate::sources::SourcePartition>::Parser<'s>,
            dst: &'r mut <Self::D as $crate::destinations::Destination>::Partition<'d>,
        ) -> Result<(), Self::Error> where Self: 'd {
            match (ts1, ts2) {
                $(
                    ($TSS::$V1(true), $TSD::$V2(true)) => {
                        let val: Option<$T1> = $crate::sources::PartitionParser::parse(src)?;
                        let val: Option<$T2> = <Self as TypeConversion<Option<$T1>, _>>::convert(val);
                        $crate::destinations::DestinationPartition::write(dst, val)?;
                        Ok(())
                    }

                    ($TSS::$V1(false), $TSD::$V2(false)) => {
                        let val: $T1 = $crate::sources::PartitionParser::parse(src)?;
                        let val: $T2 = <Self as TypeConversion<$T1, _>>::convert(val);
                        $crate::destinations::DestinationPartition::write(dst, val)?;
                        Ok(())
                    }
                )*
                #[allow(unreachable_patterns)]
                _ => fehler::throw!($crate::errors::ConnectorXError::NoConversionRule(
                    format!("{:?}", ts1), format!("{:?}", ts1))
                )
            }

        }
    };

    (@processor [$TSS:tt, $TSD:tt] $([ $V1:tt [$T1:ty] => $V2:tt [$T2:ty] | conversion $HOW:ident ])*, $([ $($TOKENS:tt)+ ])*) => {
        fn processor<'s, 'd>(
            ts1: Self::TSS,
            ts2: Self::TSD,
        ) -> $crate::errors::Result<
            fn(
                src: &mut <<Self::S as $crate::sources::Source>::Partition as $crate::sources::SourcePartition>::Parser<'s>,
                dst: &mut <Self::D as $crate::destinations::Destination>::Partition<'d>,
            ) -> Result<(), Self::Error>
        > where Self: 'd {
            match (ts1, ts2) {
                $(
                    ($TSS::$V1(true), $TSD::$V2(true)) => {
                        impl_transport!(@process_func_branch true [ $($TOKENS)+ ])
                    }

                    ($TSS::$V1(false), $TSD::$V2(false)) => {
                        impl_transport!(@process_func_branch false [ $($TOKENS)+ ])
                    }
                )*
                #[allow(unreachable_patterns)]
                _ => fehler::throw!($crate::errors::ConnectorXError::NoConversionRule(
                    format!("{:?}", ts1), format!("{:?}", ts1))
                )
            }

        }
    };

    (@process_func_branch $OPT:ident [ $V1:tt [&$L1:lifetime $T1:ty] => $V2:tt [&$L2:lifetime $T2:ty] | conversion $HOW:ident ]) => {
        impl_transport!(@process_func_branch $OPT &$T1, &$T2)
    };
    (@process_func_branch $OPT:ident [ $V1:tt [$T1:ty] => $V2:tt [&$L2:lifetime $T2:ty] | conversion $HOW:ident ]) => {
        impl_transport!(@process_func_branch $OPT $T1, &$T2)
    };
    (@process_func_branch $OPT:ident [ $V1:tt [&$L1:lifetime $T1:ty] => $V2:tt [$T2:ty] | conversion $HOW:ident ]) => {
        impl_transport!(@process_func_branch $OPT &$T1, $T2)
    };
    (@process_func_branch $OPT:ident [ $V1:tt [$T1:ty] => $V2:tt [$T2:ty] | conversion $HOW:ident ]) => {
        impl_transport!(@process_func_branch $OPT $T1, $T2)
    };
    (@process_func_branch true $T1:ty, $T2:ty) => {
        Ok(
            |s: &mut _, d: &mut _| $crate::typesystem::process::<Option<$T1>, Option<$T2>, Self, Self::S, Self::D, <Self::S as $crate::sources::Source>::Error, <Self::D as $crate::destinations::Destination>::Error, Self::Error>(s, d)
        )
    };
    (@process_func_branch false $T1:ty, $T2:ty) => {
        Ok(
            |s: &mut _, d: &mut _| $crate::typesystem::process::<$T1, $T2, Self, Self::S, Self::D, <Self::S as $crate::sources::Source>::Error, <Self::D as $crate::destinations::Destination>::Error, Self::Error>(s, d)
        )
    };

    // TypeConversion
    (@cvt $TP:ty, $V1:tt [$T1:ty] => $V2:tt [$T2:ty] | conversion $HOW:ident) => {
        impl_transport!(@cvt $HOW $TP, $T1, $T2);
    };
    (@cvt auto $TP:ty, $T1:ty, $T2:ty) => {
        impl<'tp, 'r> $crate::typesystem::TypeConversion<$T1, $T2> for $TP {
            fn convert(val: $T1) -> $T2 {
                val as _
            }
        }

        impl_transport!(@cvt option $TP, $T1, $T2);
    };
    (@cvt auto_vec $TP:ty, $T1:ty, $T2:ty) => {
        impl<'tp, 'r> $crate::typesystem::TypeConversion<$T1, $T2> for $TP {
            fn convert(val: $T1) -> $T2 {
                val.into_iter().map(|v| v as _).collect()
            }
        }

        impl_transport!(@cvt option $TP, $T1, $T2);
    };
    (@cvt owned $TP:ty, $T1:ty, $T2:ty) => {
        impl<'tp, 'r> $crate::typesystem::TypeConversion<$T1, $T2> for $TP {
            fn convert(val: $T1) -> $T2 {
                val.to_owned()
            }
        }

        impl_transport!(@cvt option $TP, $T1, $T2);
    };
    (@cvt option $TP:ty, $T1:ty, $T2:ty) => {
        impl<'tp, 'r> $crate::typesystem::TypeConversion<Option<$T1>, Option<$T2>> for $TP {
            fn convert(val: Option<$T1>) -> Option<$T2> {
                val.map(Self::convert)
            }
        }
    };
    (@cvt none $TP:ty, $T1:ty, $T2:ty) => {};
}
