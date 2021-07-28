use crate::errors::ConnectorXPythonError;
use crate::pandas::{destination::PandasDestination, typesystem::PandasTypeSystem};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use connectorx::{
    impl_transport,
    sources::postgres::{
        BinaryProtocol, CSVProtocol, CursorProtocol, PostgresSource, PostgresTypeSystem,
    },
    typesystem::TypeConversion,
};
use postgres::NoTls;
use postgres_native_tls::MakeTlsConnector;
use rust_decimal::prelude::*;
use serde_json::{to_string, Value};
use std::marker::PhantomData;
use uuid::Uuid;

pub struct PostgresPandasTransport<'py, P, C>(&'py (), PhantomData<P>, PhantomData<C>);

macro_rules! impl_postgres_transport {
    ($proto:ty, $tls:ty) => {
        impl_transport!(
            name = PostgresPandasTransport<'tp, $proto, $tls>,
            error = ConnectorXPythonError,
            systems = PostgresTypeSystem => PandasTypeSystem,
            route = PostgresSource<$proto, $tls> => PandasDestination<'tp>,
            mappings = {
                { Float4[f32]                => F64[f64]                | conversion auto }
                { Float8[f64]                => F64[f64]                | conversion auto }
                { Numeric[Decimal]           => F64[f64]                | conversion option }
                { Int2[i16]                  => I64[i64]                | conversion auto }
                { Int4[i32]                  => I64[i64]                | conversion auto }
                { Int8[i64]                  => I64[i64]                | conversion auto }
                { Bool[bool]                 => Bool[bool]              | conversion auto }
                { Char[i8]                   => Char[char]              | conversion option }
                { Text[&'r str]              => Str[&'r str]            | conversion auto }
                { BpChar[&'r str]            => Str[&'r str]            | conversion none }
                { VarChar[&'r str]           => Str[&'r str]            | conversion none }
                { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion option }
                { TimestampTz[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion auto }
                { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion option }
                { UUID[Uuid]                 => String[String]          | conversion option }
                { JSON[Value]                => String[String]          | conversion option }
                { JSONB[Value]               => String[String]          | conversion none }
                { Time[NaiveTime]            => String[String]          | conversion option }
                { ByteA[Vec<u8>]             => Bytes[Vec<u8>]          | conversion auto }
                { Enum[&'r str]              => Str[&'r str]            | conversion none }
            }
        );
    }
}

impl_postgres_transport!(BinaryProtocol, NoTls);
impl_postgres_transport!(BinaryProtocol, MakeTlsConnector);
impl_postgres_transport!(CSVProtocol, NoTls);
impl_postgres_transport!(CSVProtocol, MakeTlsConnector);
impl_postgres_transport!(CursorProtocol, NoTls);
impl_postgres_transport!(CursorProtocol, MakeTlsConnector);

impl<'py, P, C> TypeConversion<Decimal, f64> for PostgresPandasTransport<'py, P, C> {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}

impl<'py, P, C> TypeConversion<NaiveTime, String> for PostgresPandasTransport<'py, P, C> {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}

impl<'py, P, C> TypeConversion<i8, char> for PostgresPandasTransport<'py, P, C> {
    fn convert(val: i8) -> char {
        val as u8 as char
    }
}

impl<'py, P, C> TypeConversion<NaiveDateTime, DateTime<Utc>>
    for PostgresPandasTransport<'py, P, C>
{
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl<'py, P, C> TypeConversion<NaiveDate, DateTime<Utc>> for PostgresPandasTransport<'py, P, C> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}

impl<'py, P, C> TypeConversion<Uuid, String> for PostgresPandasTransport<'py, P, C> {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl<'py, P, C> TypeConversion<Value, String> for PostgresPandasTransport<'py, P, C> {
    fn convert(val: Value) -> String {
        to_string(&val).unwrap()
    }
}
