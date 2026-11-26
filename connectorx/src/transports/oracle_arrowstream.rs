use crate::{
    destinations::arrowstream::{
        typesystem::{ArrowTypeSystem, DateTimeWrapperMicro, NaiveDateTimeWrapperMicro},
        ArrowDestination, ArrowDestinationError,
    },
    sources::oracle::{OracleSource, OracleSourceError, OracleTypeSystem},
    typesystem::TypeConversion,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use rust_decimal::Decimal;
use thiserror::Error;

/// Errors raised while moving data from the Oracle source to the streaming Arrow destination.
#[derive(Error, Debug)]
pub enum OracleArrowTransportError {
    /// Error from the Oracle source.
    #[error(transparent)]
    Source(#[from] OracleSourceError),

    /// Error from the streaming Arrow destination.
    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    /// Generic ConnectorX error.
    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

/// Transport from the Oracle source to the streaming Arrow destination.
pub struct OracleArrowTransport;

impl_transport!(
    name = OracleArrowTransport,
    error = OracleArrowTransportError,
    systems = OracleTypeSystem => ArrowTypeSystem,
    route = OracleSource => ArrowDestination,
    mappings = {
        { NumFloat[f64]                     => Float64[f64]                             | conversion auto }
        { NumDecimal[Decimal]               => Decimal[Decimal]                         | conversion auto }
        { Float[f64]                        => Float64[f64]                             | conversion none }
        { BinaryFloat[f64]                  => Float64[f64]                             | conversion none }
        { BinaryDouble[f64]                 => Float64[f64]                             | conversion none }
        { NumInt[i64]                       => Int64[i64]                               | conversion auto }
        { Blob[Vec<u8>]                     => LargeBinary[Vec<u8>]                     | conversion auto }
        { Clob[String]                      => LargeUtf8[String]                        | conversion none }
        { VarChar[String]                   => LargeUtf8[String]                        | conversion auto }
        { Char[String]                      => LargeUtf8[String]                        | conversion none }
        { NVarChar[String]                  => LargeUtf8[String]                        | conversion none }
        { NChar[String]                     => LargeUtf8[String]                        | conversion none }
        { Date[NaiveDateTime]               => Date64Micro[NaiveDateTimeWrapperMicro]   | conversion option }
        { Timestamp[NaiveDateTime]          => Date64Micro[NaiveDateTimeWrapperMicro]   | conversion none }
        { TimestampNano[NaiveDateTime]      => Date64[NaiveDateTime]                    | conversion auto }
        { TimestampTz[DateTime<Utc>]        => DateTimeTzMicro[DateTimeWrapperMicro]    | conversion option }
        { TimestampTzNano[DateTime<Utc>]    => DateTimeTz[DateTime<Utc>]                | conversion auto }
        { Boolean[bool]                     => Boolean[bool]                            | conversion auto }
    }
);

impl TypeConversion<NaiveDateTime, NaiveDateTimeWrapperMicro> for OracleArrowTransport {
    fn convert(val: NaiveDateTime) -> NaiveDateTimeWrapperMicro {
        NaiveDateTimeWrapperMicro(val)
    }
}

impl TypeConversion<DateTime<Utc>, DateTimeWrapperMicro> for OracleArrowTransport {
    fn convert(val: DateTime<Utc>) -> DateTimeWrapperMicro {
        DateTimeWrapperMicro(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typesystem::{Transport, TypeSystem};

    /// The destination type of each Oracle date and time type must hold the value the conversion produces: a
    /// mismatch is only caught at run time, on the first row ("Data type unexpected"). `TIMESTAMP WITH TIME ZONE`
    /// produced a `DateTimeWrapperMicro` into a `DateTimeTz` column, which holds a `DateTime<Utc>`.
    /// Checks that `ts` holds values of type `T` (`Option<T>` when nullable).
    fn holds<T>(ts: ArrowTypeSystem, nullable: bool) -> bool
    where
        T: crate::typesystem::TypeAssoc<ArrowTypeSystem>,
        Option<T>: crate::typesystem::TypeAssoc<ArrowTypeSystem>,
    {
        if nullable {
            ts.check::<Option<T>>().is_ok()
        } else {
            ts.check::<T>().is_ok()
        }
    }

    #[test]
    fn date_and_time_types_hold_the_converted_values() {
        let dst = |ts| OracleArrowTransport::convert_typesystem(ts).unwrap();
        for nullable in [true, false] {
            let tz = dst(OracleTypeSystem::TimestampTz(nullable));
            assert!(
                holds::<DateTimeWrapperMicro>(tz, nullable),
                "TIMESTAMP WITH TIME ZONE -> {tz:?}"
            );

            let tz_nano = dst(OracleTypeSystem::TimestampTzNano(nullable));
            assert!(
                holds::<DateTime<Utc>>(tz_nano, nullable),
                "TIMESTAMP(7..9) WITH TIME ZONE -> {tz_nano:?}"
            );

            for naive in [
                OracleTypeSystem::Date(nullable),
                OracleTypeSystem::Timestamp(nullable),
            ] {
                let ts = dst(naive);
                assert!(
                    holds::<NaiveDateTimeWrapperMicro>(ts, nullable),
                    "{naive:?} -> {ts:?}"
                );
            }
            let nano = dst(OracleTypeSystem::TimestampNano(nullable));
            assert!(
                holds::<NaiveDateTime>(nano, nullable),
                "TIMESTAMP(7..9) -> {nano:?}"
            );
        }
    }

    #[test]
    fn timestamp_with_time_zone_converts_at_microsecond_precision() {
        let utc = DateTime::from_timestamp_micros(944_074_800_123_456).unwrap();
        let converted: Option<DateTimeWrapperMicro> = <OracleArrowTransport as TypeConversion<
            Option<DateTime<Utc>>,
            Option<DateTimeWrapperMicro>,
        >>::convert(Some(utc));
        assert_eq!(converted.map(|w| w.0), Some(utc));
    }
}
