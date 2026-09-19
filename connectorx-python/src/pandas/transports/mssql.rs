use crate::errors::ConnectorXPythonError;
use crate::pandas::{
    destination::PandasDestination,
    typesystem::{DateTimeWrapperMicro, PandasTypeSystem},
};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use connectorx::{
    impl_transport,
    sources::mssql::{FloatN, IntN, MsSQLSource, MsSQLTypeSystem},
    typesystem::TypeConversion,
};
use rust_decimal::prelude::*;
use uuid_old::Uuid;

#[allow(dead_code)]
pub struct MsSQLPandasTransport<'py>(&'py ());

impl_transport!(
    name = MsSQLPandasTransport<'tp>,
    error = ConnectorXPythonError,
    systems = MsSQLTypeSystem => PandasTypeSystem,
    route = MsSQLSource => PandasDestination<'tp>,
    mappings = {
        { Tinyint[u8]                   => I64[i64]                | conversion auto }
        { Smallint[i16]                 => I64[i64]                | conversion auto }
        { Int[i32]                      => I64[i64]                | conversion auto }
        { Bigint[i64]                   => I64[i64]                | conversion auto }
        { Intn[IntN]                    => I64[i64]                | conversion option }
        { Float24[f32]                  => F64[f64]                | conversion auto }
        { Float53[f64]                  => F64[f64]                | conversion auto }
        { Floatn[FloatN]                => F64[f64]                | conversion option }
        { Bit[bool]                     => Bool[bool]              | conversion auto  }
        { Nvarchar[&'r str]             => Str[&'r str]            | conversion auto }
        { Varchar[&'r str]              => Str[&'r str]            | conversion none }
        { Nchar[&'r str]                => Str[&'r str]            | conversion none }
        { Char[&'r str]                 => Str[&'r str]            | conversion none }
        { Text[&'r str]                 => Str[&'r str]            | conversion none }
        { Ntext[&'r str]                => Str[&'r str]            | conversion none }
        { Binary[&'r [u8]]              => ByteSlice[&'r [u8]]     | conversion auto }
        { Varbinary[&'r [u8]]           => ByteSlice[&'r [u8]]     | conversion none }
        { Image[&'r [u8]]               => ByteSlice[&'r [u8]]     | conversion none }
        { Numeric[Decimal]              => F64[f64]                | conversion option }
        { Decimal[Decimal]              => F64[f64]                | conversion none }
        { Datetime[NaiveDateTime]       => DateTimeMicro[DateTimeWrapperMicro] | conversion option }
        { Datetime2[NaiveDateTime]      => DateTimeMicro[DateTimeWrapperMicro] | conversion none }
        { Smalldatetime[NaiveDateTime]  => DateTimeMicro[DateTimeWrapperMicro] | conversion none }
        { Date[NaiveDate]               => DateTimeMicro[DateTimeWrapperMicro] | conversion option }
        { Datetimeoffset[DateTime<Utc>] => DateTimeMicro[DateTimeWrapperMicro] | conversion option }
        { Uniqueidentifier[Uuid]        => String[String]          | conversion option }
        { Time[NaiveTime]               => String[String]          | conversion option }
        { SmallMoney[f32]               => F64[f64]                | conversion none }
        { Money[f64]                    => F64[f64]                | conversion none }
    }
);

impl<'py> TypeConversion<IntN, i64> for MsSQLPandasTransport<'py> {
    fn convert(val: IntN) -> i64 {
        val.0
    }
}

impl<'py> TypeConversion<FloatN, f64> for MsSQLPandasTransport<'py> {
    fn convert(val: FloatN) -> f64 {
        val.0
    }
}

impl<'py> TypeConversion<NaiveDateTime, DateTimeWrapperMicro> for MsSQLPandasTransport<'py> {
    fn convert(val: NaiveDateTime) -> DateTimeWrapperMicro {
        DateTimeWrapperMicro(DateTime::from_naive_utc_and_offset(val, Utc))
    }
}

impl<'py> TypeConversion<NaiveDate, DateTimeWrapperMicro> for MsSQLPandasTransport<'py> {
    fn convert(val: NaiveDate) -> DateTimeWrapperMicro {
        DateTimeWrapperMicro(DateTime::from_naive_utc_and_offset(
            val.and_hms_opt(0, 0, 0)
                .unwrap_or_else(|| panic!("and_hms_opt got None from {:?}", val)),
            Utc,
        ))
    }
}

impl<'py> TypeConversion<DateTime<Utc>, DateTimeWrapperMicro> for MsSQLPandasTransport<'py> {
    fn convert(val: DateTime<Utc>) -> DateTimeWrapperMicro {
        DateTimeWrapperMicro(val)
    }
}

impl<'py> TypeConversion<Uuid, String> for MsSQLPandasTransport<'py> {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl<'py> TypeConversion<Decimal, f64> for MsSQLPandasTransport<'py> {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}

impl<'py> TypeConversion<NaiveTime, String> for MsSQLPandasTransport<'py> {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}
