/// A macro to implement `TypeAssoc` and `Realize` which saves repetitive code.
///
/// # Example Usage
/// `impl_typesystem!(DataType, [DataType::F64] => f64, [DataType::I64] => i64);`
/// This means for the type system `DataType`, it's variant `DataType::F64(false)` is corresponding to the physical type f64 and
/// `DataType::F64(true)` is corresponding to the physical type Option<f64>. Same for I64 and i64
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
        impl $crate::TypeSystem for $TS {}

        $(
            impl_typesystem!(@typeassoc $TS [$($V)+], $NT);
        )+

        impl_typesystem!(@realize $TS $([ [$($V)+] => $NT ])+ );
    };

    (@typeassoc $TS:tt [$($V:tt)+], $NT:ty) => {
        impl<'r> $crate::TypeAssoc<$TS> for $NT {
            fn check(ts: $TS) -> $crate::Result<()> {
                match ts {
                    $(
                        $TS::$V(false) => Ok(()),
                    )+
                    _ => fehler::throw!($crate::ConnectorAgentError::UnexpectedType(format!("{:?}", ts), std::any::type_name::<$NT>()))
                }
            }
        }

        impl<'r> $crate::TypeAssoc<$TS> for Option<$NT> {
            fn check(ts: $TS) -> $crate::Result<()> {
                match ts {
                    $(
                        $TS::$V(true) => Ok(()),
                    )+
                    _ => fehler::throw!($crate::ConnectorAgentError::UnexpectedType(format!("{:?}", ts), std::any::type_name::<$NT>()))
                }
            }
        }
    };

    (@realize $TS:tt $([ [$($V:tt)+] => $NT:ty ])+) => {
        impl<'r, F> $crate::Realize<F> for $TS
        where
            F: $crate::ParameterizedFunc,
            $(F: $crate::ParameterizedOn<$NT>,)+
            $(F: $crate::ParameterizedOn<Option<$NT>>,)+
        {
            fn realize(self) -> $crate::Result<F::Function> {
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

/// A macro to help define Transport.
///
/// # Example Usage
/// ```ignore
/// impl_transport! {
///    ['py],
///    PostgresPandasTransport<'py>,
///    PostgresDTypes => PandasTypes,
///    PostgresSource => PandasDestination<'py>,
///    ([PostgresDTypes::Float4], [PandasTypes::F64]) => (f32, f64) conversion all
/// }
/// ```
/// This implements `Transport` to `PostgresPandasTransport<'py>`.
/// The lifetime used must be declare in the first argument in the bracket.
#[macro_export]
macro_rules! impl_transport {
    (
        name = $TP:ty,
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

        impl_transport!(@transport $TP [$TSS, $TSD] [$S, $D] $([ $($TOKENS)+ ])*);
    };

    // transport
    (@transport $TP:ty [$TSS:tt, $TSD:tt] [$S:ty, $D:ty] $([ $($TOKENS:tt)+ ])*) => {
        impl <'tp> $crate::Transport for $TP {
            type TSS = $TSS;
            type TSD = $TSD;
            type S = $S;
            type D = $D;

            impl_transport!(@cvtts [$TSS, $TSD] $([ $($TOKENS)+ ])*);
            impl_transport!(@process [$TSS, $TSD] $([ $($TOKENS)+ ])*);
            impl_transport!(@processor [$TSS, $TSD] $([ $($TOKENS)+ ])*, $([ $($TOKENS)+ ])*);
        }
    };

    (@cvtts [$TSS:tt, $TSD:tt] $( [$V1:tt [$T1:ty] => $V2:tt [$T2:ty] | conversion $HOW:ident] )*) => {
        fn convert_typesystem(ts: Self::TSS) -> $crate::Result<Self::TSD> {
            match ts {
                $(
                    $TSS::$V1(true) => Ok($TSD::$V2(true)),
                    $TSS::$V1(false) => Ok($TSD::$V2(false)),
                )*
                #[allow(unreachable_patterns)]
                _ => fehler::throw!($crate::ConnectorAgentError::NoConversionRule(
                    format!("{:?}", ts), format!("{}", std::any::type_name::<Self::TSD>())
                ))
            }
        }
    };

    (@process [$TSS:tt, $TSD:tt] $([ $V1:tt [$T1:ty] => $V2:tt [$T2:ty] | conversion $HOW:ident ])*) => {
        fn process<'s, 'd, 'r>(
            ts1: Self::TSS,
            ts2: Self::TSD,
            src: &'r mut <<Self::S as $crate::Source>::Partition as $crate::SourcePartition>::Parser<'s>,
            dst: &'r mut <Self::D as $crate::Destination>::Partition<'d>,
        ) -> $crate::Result<()> {
            match (ts1, ts2) {
                $(
                    ($TSS::$V1(true), $TSD::$V2(true)) => {
                        let val: Option<$T1> = $crate::PartitionParser::parse(src)?;
                        let val: Option<$T2> = <Self as TypeConversion<Option<$T1>, _>>::convert(val);
                        $crate::DestinationPartition::write(dst, val)?;
                        Ok(())
                    }

                    ($TSS::$V1(false), $TSD::$V2(false)) => {
                        let val: $T1 = $crate::PartitionParser::parse(src)?;
                        let val: $T2 = <Self as TypeConversion<$T1, _>>::convert(val);
                        $crate::DestinationPartition::write(dst, val)?;
                        Ok(())
                    }
                )*
                #[allow(unreachable_patterns)]
                _ => fehler::throw!($crate::ConnectorAgentError::NoConversionRule(
                    format!("{:?}", ts1), format!("{:?}", ts1))
                )
            }

        }
    };

    (@processor [$TSS:tt, $TSD:tt] $([ $V1:tt [$T1:ty] => $V2:tt [$T2:ty] | conversion $HOW:ident ])*, $([ $($TOKENS:tt)+ ])*) => {
        fn processor<'s, 'd>(
            ts1: Self::TSS,
            ts2: Self::TSD,
        ) -> $crate::Result<
            fn(
                src: &mut <<Self::S as $crate::Source>::Partition as $crate::SourcePartition>::Parser<'s>,
                dst: &mut <Self::D as $crate::Destination>::Partition<'d>,
            ) -> $crate::Result<()>,
        > {
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
                _ => fehler::throw!($crate::ConnectorAgentError::NoConversionRule(
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
            |s: &mut _, d: &mut _| $crate::typesystem::process::<Option<$T1>, Option<$T2>, Self, Self::S, Self::D>(s, d)
        )
    };
    (@process_func_branch false $T1:ty, $T2:ty) => {
        Ok(
            |s: &mut _, d: &mut _| $crate::typesystem::process::<$T1, $T2, Self, Self::S, Self::D>(s, d)
        )
    };

    // TypeConversion
    (@cvt $TP:ty, $V1:tt [$T1:ty] => $V2:tt [$T2:ty] | conversion $HOW:ident) => {
        impl_transport!(@cvt $HOW $TP, $T1, $T2);
    };
    (@cvt all $TP:ty, $T1:ty, $T2:ty) => {
        impl<'tp, 'r> $crate::TypeConversion<$T1, $T2> for $TP {
            fn convert(val: $T1) -> $T2 {
                val as _
            }
        }

        impl<'tp, 'r> $crate::TypeConversion<Option<$T1>, Option<$T2>> for $TP {
            fn convert(val: Option<$T1>) -> Option<$T2> {
                val.map(Self::convert)
            }
        }
    };


    (@cvt half $TP:ty, $T1:ty, $T2:ty) => {
        impl<'tp, 'r> $crate::TypeConversion<Option<$T1>, Option<$T2>> for $TP {
            fn convert(val: Option<$T1>) -> Option<$T2> {
                val.map(Self::convert)
            }
        }
    };

    (@cvt none $TP:ty, $T1:ty, $T2:ty) => {};
}
