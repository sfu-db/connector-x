use crate::errors::ConnectorXPythonError;
use crate::pandas::{destination::PandasDestination, typesystem::PandasTypeSystem};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use connectorx::{
    impl_transport,
    sources::postgres::{
        BinaryProtocol, CSVProtocol, CursorProtocol, PostgresSource, PostgresTypeSystem,
        SimpleProtocol,
    },
    typesystem::TypeConversion,
};
use postgres::NoTls;
use postgres_openssl::MakeTlsConnector;
use rust_decimal::prelude::*;
use serde_json::{to_string, Value};
use std::collections::HashMap;
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
                { Float4[f32]                                   => F64[f64]                 | conversion auto }
                { Float8[f64]                                   => F64[f64]                 | conversion auto }
                { Numeric[Decimal]                              => F64[f64]                 | conversion option }
                { Int2[i16]                                     => I64[i64]                 | conversion auto }
                { Int4[i32]                                     => I64[i64]                 | conversion auto }
                { Int8[i64]                                     => I64[i64]                 | conversion auto }
                { BoolArray[Vec<bool>]                          => BoolArray[Vec<bool>]     | conversion auto_vec }
                { Int2Array[Vec<i16>]                           => I64Array[Vec<i64>]       | conversion auto_vec }
                { Int4Array[Vec<i32>]                           => I64Array[Vec<i64>]       | conversion auto_vec }
                { Int8Array[Vec<i64>]                           => I64Array[Vec<i64>]       | conversion auto }
                { Float4Array[Vec<f32>]                         => F64Array[Vec<f64>]       | conversion auto_vec }
                { Float8Array[Vec<f64>]                         => F64Array[Vec<f64>]       | conversion auto }
                { NumericArray[Vec<Decimal>]                    => F64Array[Vec<f64>]       | conversion option }
                { Bool[bool]                                    => Bool[bool]               | conversion auto }
                { Char[i8]                                      => Char[char]               | conversion option }
                { Text[&'r str]                                 => Str[&'r str]             | conversion auto }
                { BpChar[&'r str]                               => Str[&'r str]             | conversion none }
                { VarChar[&'r str]                              => Str[&'r str]             | conversion none }
                { Name[&'r str]                                 => Str[&'r str]             | conversion none }
                { Timestamp[NaiveDateTime]                      => DateTime[DateTime<Utc>]  | conversion option }
                { TimestampTz[DateTime<Utc>]                    => DateTime[DateTime<Utc>]  | conversion auto }
                { Date[NaiveDate]                               => DateTime[DateTime<Utc>]  | conversion option }
                { UUID[Uuid]                                    => String[String]           | conversion option }
                { JSON[Value]                                   => String[String]           | conversion option }
                { JSONB[Value]                                  => String[String]           | conversion none }
                { Time[NaiveTime]                               => String[String]           | conversion option }
                { ByteA[Vec<u8>]                                => Bytes[Vec<u8>]           | conversion auto }
                { Enum[&'r str]                                 => Str[&'r str]             | conversion none }
                { HSTORE[HashMap<String, Option<String>>]       => String[String]           | conversion option }
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
impl_postgres_transport!(SimpleProtocol, NoTls);
impl_postgres_transport!(SimpleProtocol, MakeTlsConnector);

impl<'py, P, C> TypeConversion<HashMap<String, Option<String>>, String>
    for PostgresPandasTransport<'py, P, C>
{
    fn convert(val: HashMap<String, Option<String>>) -> String {
        to_string(&val).unwrap()
    }
}

impl<'py, P, C> TypeConversion<Vec<Decimal>, Vec<f64>> for PostgresPandasTransport<'py, P, C> {
    fn convert(val: Vec<Decimal>) -> Vec<f64> {
        val.into_iter()
            .map(|v| {
                v.to_f64()
                    .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", v))
            })
            .collect()
    }
}

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
        DateTime::from_naive_utc_and_offset(val, Utc)
    }
}

impl<'py, P, C> TypeConversion<NaiveDate, DateTime<Utc>> for PostgresPandasTransport<'py, P, C> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_naive_utc_and_offset(
            val.and_hms_opt(0, 0, 0)
                .unwrap_or_else(|| panic!("and_hms_opt got None from {:?}", val)),
            Utc,
        )
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
