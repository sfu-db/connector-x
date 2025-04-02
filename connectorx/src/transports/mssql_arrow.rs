//! Transport from MsSQL Source to Arrow Destination.

use crate::destinations::arrow::{
    typesystem::{DateTimeWrapperMicro, NaiveDateTimeWrapperMicro, NaiveTimeWrapperMicro},
    ArrowDestination, ArrowDestinationError, ArrowTypeSystem,
};
use crate::sources::mssql::{FloatN, IntN, MsSQLSource, MsSQLSourceError, MsSQLTypeSystem};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use num_traits::ToPrimitive;
use rust_decimal::Decimal;
use thiserror::Error;
use uuid_old::Uuid;

/// Convert MsSQL data types to Arrow data types.
pub struct MsSQLArrowTransport;

#[derive(Error, Debug)]
pub enum MsSQLArrowTransportError {
    #[error(transparent)]
    Source(#[from] MsSQLSourceError),

    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

impl_transport!(
    name = MsSQLArrowTransport,
    error = MsSQLArrowTransportError,
    systems = MsSQLTypeSystem => ArrowTypeSystem,
    route = MsSQLSource => ArrowDestination,
    mappings = {
        { Tinyint[u8]                   => Int64[i64]                | conversion auto }
        { Smallint[i16]                 => Int64[i64]                | conversion auto }
        { Int[i32]                      => Int64[i64]                | conversion auto }
        { Bigint[i64]                   => Int64[i64]                | conversion auto }
        { Intn[IntN]                    => Int64[i64]                | conversion option }
        { Float24[f32]                  => Float32[f32]              | conversion auto }
        { Float53[f64]                  => Float64[f64]              | conversion auto }
        { Floatn[FloatN]                => Float64[f64]              | conversion option }
        { Bit[bool]                     => Boolean[bool]             | conversion auto  }
        { Nvarchar[&'r str]             => LargeUtf8[String]         | conversion owned }
        { Varchar[&'r str]              => LargeUtf8[String]         | conversion none }
        { Nchar[&'r str]                => LargeUtf8[String]         | conversion none }
        { Char[&'r str]                 => LargeUtf8[String]         | conversion none }
        { Text[&'r str]                 => LargeUtf8[String]         | conversion none }
        { Ntext[&'r str]                => LargeUtf8[String]         | conversion none }
        { Binary[&'r [u8]]              => LargeBinary[Vec<u8>]      | conversion owned }
        { Varbinary[&'r [u8]]           => LargeBinary[Vec<u8>]      | conversion none }
        { Image[&'r [u8]]               => LargeBinary[Vec<u8>]      | conversion none }
        { Numeric[Decimal]              => Float64[f64]              | conversion option }
        { Decimal[Decimal]              => Float64[f64]              | conversion none }
        { Datetime[NaiveDateTime]       => Date64Micro[NaiveDateTimeWrapperMicro]     | conversion option }
        { Datetime2[NaiveDateTime]      => Date64Micro[NaiveDateTimeWrapperMicro]     | conversion none }
        { Smalldatetime[NaiveDateTime]  => Date64Micro[NaiveDateTimeWrapperMicro]     | conversion none }
        { Date[NaiveDate]               => Date32[NaiveDate]         | conversion auto }
        { Datetimeoffset[DateTime<Utc>] => DateTimeTzMicro[DateTimeWrapperMicro] | conversion option }
        { Uniqueidentifier[Uuid]        => LargeUtf8[String]         | conversion option }
        { Time[NaiveTime]               => Time64Micro[NaiveTimeWrapperMicro]         | conversion option }
        { SmallMoney[f32]               => Float32[f32]              | conversion none }
        { Money[f64]                    => Float64[f64]              | conversion none }
    }
);

impl TypeConversion<NaiveTime, NaiveTimeWrapperMicro> for MsSQLArrowTransport {
    fn convert(val: NaiveTime) -> NaiveTimeWrapperMicro {
        NaiveTimeWrapperMicro(val)
    }
}

impl TypeConversion<NaiveDateTime, NaiveDateTimeWrapperMicro> for MsSQLArrowTransport {
    fn convert(val: NaiveDateTime) -> NaiveDateTimeWrapperMicro {
        NaiveDateTimeWrapperMicro(val)
    }
}

impl TypeConversion<DateTime<Utc>, DateTimeWrapperMicro> for MsSQLArrowTransport {
    fn convert(val: DateTime<Utc>) -> DateTimeWrapperMicro {
        DateTimeWrapperMicro(val)
    }
}

impl TypeConversion<Uuid, String> for MsSQLArrowTransport {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl TypeConversion<IntN, i64> for MsSQLArrowTransport {
    fn convert(val: IntN) -> i64 {
        val.0
    }
}

impl TypeConversion<FloatN, f64> for MsSQLArrowTransport {
    fn convert(val: FloatN) -> f64 {
        val.0
    }
}

impl TypeConversion<Decimal, f64> for MsSQLArrowTransport {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}
