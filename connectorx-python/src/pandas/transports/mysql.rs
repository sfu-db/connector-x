use crate::errors::ConnectorXPythonError;
use crate::pandas::destination::PandasDestination;
use crate::pandas::typesystem::PandasTypeSystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use connectorx::{
    impl_transport,
    sources::mysql::{BinaryProtocol, MySQLSource, MySQLTypeSystem, TextProtocol},
    typesystem::TypeConversion,
};
use rust_decimal::prelude::*;
use serde_json::{to_string, Value};
use std::marker::PhantomData;

#[allow(dead_code)]
pub struct MysqlPandasTransport<'py, P>(&'py (), PhantomData<P>);

impl_transport!(
    name = MysqlPandasTransport<'tp, BinaryProtocol>,
    error = ConnectorXPythonError,
    systems = MySQLTypeSystem => PandasTypeSystem,
    route = MySQLSource<BinaryProtocol> => PandasDestination<'tp>,
    mappings = {
        { Float[f32]                 => F64[f64]                | conversion auto }
        { Double[f64]                => F64[f64]                | conversion auto }
        { Tiny[i8]                   => I64[i64]                | conversion auto }
        { Short[i16]                 => I64[i64]                | conversion auto }
        { Long[i32]                  => I64[i64]                | conversion auto }
        { Int24[i32]                 => I64[i64]                | conversion none }
        { LongLong[i64]              => I64[i64]                | conversion auto }
        { UTiny[u8]                  => I64[i64]                | conversion auto }
        { UShort[u16]                => I64[i64]                | conversion auto }
        { ULong[u32]                 => I64[i64]                | conversion auto }
        { UInt24[u32]                => I64[i64]                | conversion none }
        { ULongLong[u64]             => F64[f64]                | conversion auto }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion option }
        { Time[NaiveTime]            => String[String]          | conversion option }
        { Year[i16]                  => I64[i64]                | conversion none}
        { Datetime[NaiveDateTime]    => DateTime[DateTime<Utc>] | conversion option }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion none }
        { Decimal[Decimal]           => F64[f64]                | conversion option }
        { VarChar[String]            => String[String]          | conversion auto }
        { Char[String]               => String[String]          | conversion none }
        { Enum[String]               => Str[String]             | conversion none }
        { TinyBlob[Vec<u8>]          => Bytes[Vec<u8>]          | conversion auto }
        { Blob[Vec<u8>]              => Bytes[Vec<u8>]          | conversion none }
        { MediumBlob[Vec<u8>]        => Bytes[Vec<u8>]          | conversion none }
        { LongBlob[Vec<u8>]          => Bytes[Vec<u8>]          | conversion none }
        { Json[Value]                => String[String]          | conversion option }
        { Bit[Vec<u8>]               => Bytes[Vec<u8>]          | conversion none }
    }
);

impl_transport!(
    name = MysqlPandasTransport<'tp, TextProtocol>,
    error = ConnectorXPythonError,
    systems = MySQLTypeSystem => PandasTypeSystem,
    route = MySQLSource<TextProtocol> => PandasDestination<'tp>,
    mappings = {
        { Float[f32]                 => F64[f64]                | conversion auto }
        { Double[f64]                => F64[f64]                | conversion auto }
        { Tiny[i8]                   => I64[i64]                | conversion auto }
        { Short[i16]                 => I64[i64]                | conversion auto }
        { Long[i32]                  => I64[i64]                | conversion auto }
        { Int24[i32]                 => I64[i64]                | conversion none }
        { LongLong[i64]              => I64[i64]                | conversion auto }
        { UTiny[u8]                  => I64[i64]                | conversion auto }
        { UShort[u16]                => I64[i64]                | conversion auto }
        { ULong[u32]                 => I64[i64]                | conversion auto }
        { UInt24[u32]                => I64[i64]                | conversion none }
        { ULongLong[u64]             => F64[f64]                | conversion auto }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion option }
        { Time[NaiveTime]            => String[String]          | conversion option }
        { Datetime[NaiveDateTime]    => DateTime[DateTime<Utc>] | conversion option }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion none }
        { Year[i16]                  => I64[i64]                | conversion none}
        { Decimal[Decimal]           => F64[f64]                | conversion option }
        { VarChar[String]            => String[String]          | conversion auto }
        { Char[String]               => String[String]          | conversion none }
        { Enum[String]               => Str[String]             | conversion none }
        { TinyBlob[Vec<u8>]          => Bytes[Vec<u8>]          | conversion auto }
        { Blob[Vec<u8>]              => Bytes[Vec<u8>]          | conversion none }
        { MediumBlob[Vec<u8>]        => Bytes[Vec<u8>]          | conversion none }
        { LongBlob[Vec<u8>]          => Bytes[Vec<u8>]          | conversion none }
        { Json[Value]                => String[String]          | conversion option }
        { Bit[Vec<u8>]               => Bytes[Vec<u8>]          | conversion none }
    }
);

impl<'py, P> TypeConversion<NaiveDate, DateTime<Utc>> for MysqlPandasTransport<'py, P> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_naive_utc_and_offset(
            val.and_hms_opt(0, 0, 0)
                .unwrap_or_else(|| panic!("and_hms_opt got None from {:?}", val)),
            Utc,
        )
    }
}

impl<'py, P> TypeConversion<NaiveTime, String> for MysqlPandasTransport<'py, P> {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}

impl<'py, P> TypeConversion<NaiveDateTime, DateTime<Utc>> for MysqlPandasTransport<'py, P> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_naive_utc_and_offset(val, Utc)
    }
}

impl<'py, P> TypeConversion<Decimal, f64> for MysqlPandasTransport<'py, P> {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}

impl<'py, P> TypeConversion<Value, String> for MysqlPandasTransport<'py, P> {
    fn convert(val: Value) -> String {
        to_string(&val).unwrap()
    }
}
