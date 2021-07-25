use crate::errors::ConnectorXPythonError;
use crate::pandas::destination::PandasDestination;
use crate::pandas::types::PandasTypeSystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use connectorx::{
    impl_transport,
    sources::postgres::{
        BinaryProtocol, CSVProtocol, CursorProtocol, PostgresSource, PostgresTypeSystem,
    },
    typesystem::TypeConversion,
};
use rust_decimal::prelude::*;
use serde_json::{to_string, Value};
use std::marker::PhantomData;
use uuid::Uuid;

pub struct PostgresPandasTransport<'py, P>(&'py (), PhantomData<P>);

impl_transport!(
    name = PostgresPandasTransport<'tp, BinaryProtocol>,
    error = ConnectorXPythonError,
    systems = PostgresTypeSystem => PandasTypeSystem,
    route = PostgresSource<BinaryProtocol> => PandasDestination<'tp>,
    mappings = {
        { Float4[f32]                => F64[f64]                | conversion all }
        { Float8[f64]                => F64[f64]                | conversion all }
        { Numeric[Decimal]           => F64[f64]                | conversion half }
        { Int2[i16]                  => I64[i64]                | conversion all }
        { Int4[i32]                  => I64[i64]                | conversion all }
        { Int8[i64]                  => I64[i64]                | conversion all }
        { Bool[bool]                 => Bool[bool]              | conversion all }
        { Char[i8]                   => Char[char]              | conversion half }
        { Text[&'r str]              => Str[&'r str]            | conversion all }
        { BpChar[&'r str]            => Str[&'r str]            | conversion none }
        { VarChar[&'r str]           => Str[&'r str]            | conversion none }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion half }
        { TimestampTz[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
        { UUID[Uuid]                 => String[String]          | conversion half }
        { JSON[Value]                => String[String]          | conversion half }
        { JSONB[Value]               => String[String]          | conversion none }
        { Time[NaiveTime]            => String[String]          | conversion half }
        { ByteA[Vec<u8>]             => Bytes[Vec<u8>]          | conversion all }
        { Enum[&'r str]              => Str[&'r str]            | conversion none }
    }
);

impl_transport!(
    name = PostgresPandasTransport<'tp, CSVProtocol>,
    error = ConnectorXPythonError,
    systems = PostgresTypeSystem => PandasTypeSystem,
    route = PostgresSource<CSVProtocol> => PandasDestination<'tp>,
    mappings = {
        { Float4[f32]                => F64[f64]                | conversion all }
        { Float8[f64]                => F64[f64]                | conversion all }
        { Numeric[Decimal]           => F64[f64]                | conversion half }
        { Int2[i16]                  => I64[i64]                | conversion all }
        { Int4[i32]                  => I64[i64]                | conversion all }
        { Int8[i64]                  => I64[i64]                | conversion all }
        { Bool[bool]                 => Bool[bool]              | conversion all }
        { Char[i8]                   => Char[char]              | conversion half }
        { Text[&'r str]              => Str[&'r str]            | conversion all }
        { BpChar[&'r str]            => Str[&'r str]            | conversion none }
        { VarChar[&'r str]           => Str[&'r str]            | conversion none }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion half }
        { TimestampTz[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
        { UUID[Uuid]                 => String[String]          | conversion half }
        { JSON[Value]                => String[String]          | conversion half }
        { JSONB[Value]               => String[String]          | conversion none }
        { Time[NaiveTime]            => String[String]          | conversion half }
        { ByteA[Vec<u8>]             => Bytes[Vec<u8>]          | conversion all }
        { Enum[&'r str]              => Str[&'r str]            | conversion none }
    }
);

impl_transport!(
    name = PostgresPandasTransport<'tp, CursorProtocol>,
    error = ConnectorXPythonError,
    systems = PostgresTypeSystem => PandasTypeSystem,
    route = PostgresSource<CursorProtocol> => PandasDestination<'tp>,
    mappings = {
        { Float4[f32]                => F64[f64]                | conversion all }
        { Float8[f64]                => F64[f64]                | conversion all }
        { Numeric[Decimal]           => F64[f64]                | conversion half }
        { Int2[i16]                  => I64[i64]                | conversion all }
        { Int4[i32]                  => I64[i64]                | conversion all }
        { Int8[i64]                  => I64[i64]                | conversion all }
        { Bool[bool]                 => Bool[bool]              | conversion all }
        { Char[i8]                   => Char[char]              | conversion half }
        { Text[&'r str]              => Str[&'r str]            | conversion all }
        { BpChar[&'r str]            => Str[&'r str]            | conversion none }
        { VarChar[&'r str]           => Str[&'r str]            | conversion none }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion half }
        { TimestampTz[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
        { UUID[Uuid]                 => String[String]          | conversion half }
        { JSON[Value]                => String[String]          | conversion half }
        { JSONB[Value]               => String[String]          | conversion none }
        { Time[NaiveTime]            => String[String]          | conversion half }
        { ByteA[Vec<u8>]             => Bytes[Vec<u8>]          | conversion all }
        { Enum[&'r str]              => Str[&'r str]            | conversion none }
    }
);

impl<'py, P> TypeConversion<Decimal, f64> for PostgresPandasTransport<'py, P> {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}

impl<'py, P> TypeConversion<NaiveTime, String> for PostgresPandasTransport<'py, P> {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}

impl<'py, P> TypeConversion<i8, char> for PostgresPandasTransport<'py, P> {
    fn convert(val: i8) -> char {
        val as u8 as char
    }
}

impl<'py, P> TypeConversion<NaiveDateTime, DateTime<Utc>> for PostgresPandasTransport<'py, P> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl<'py, P> TypeConversion<NaiveDate, DateTime<Utc>> for PostgresPandasTransport<'py, P> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}

impl<'py, P> TypeConversion<Uuid, String> for PostgresPandasTransport<'py, P> {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl<'py, P> TypeConversion<Value, String> for PostgresPandasTransport<'py, P> {
    fn convert(val: Value) -> String {
        to_string(&val).unwrap()
    }
}
