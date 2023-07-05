//! Transport from Postgres Source to Arrow2 Destination.

use crate::destinations::arrow2::{
    typesystem::Arrow2TypeSystem, Arrow2Destination, Arrow2DestinationError,
};
use crate::sources::postgres::{
    BinaryProtocol, CSVProtocol, CursorProtocol, PostgresSource, PostgresSourceError,
    PostgresTypeSystem, SimpleProtocol,
};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use num_traits::ToPrimitive;
use postgres::NoTls;
use postgres_openssl::MakeTlsConnector;
use rust_decimal::Decimal;
use serde_json::Value;
use std::marker::PhantomData;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum PostgresArrow2TransportError {
    #[error(transparent)]
    Source(#[from] PostgresSourceError),

    #[error(transparent)]
    Destination(#[from] Arrow2DestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

/// Convert Postgres data types to Arrow2 data types.
pub struct PostgresArrow2Transport<P, C>(PhantomData<P>, PhantomData<C>);

macro_rules! impl_postgres_transport {
    ($proto:ty, $tls:ty) => {
        impl_transport!(
            name = PostgresArrow2Transport<$proto, $tls>,
            error = PostgresArrow2TransportError,
            systems = PostgresTypeSystem => Arrow2TypeSystem,
            route = PostgresSource<$proto, $tls> => Arrow2Destination,
            mappings = {
                { Float4[f32]                       => Float32[f32]                | conversion auto }
                { Float8[f64]                       => Float64[f64]                | conversion auto }
                { Numeric[Decimal]                  => Float64[f64]                | conversion option }
                { Int2[i16]                         => Int32[i32]                  | conversion auto }
                { Int4[i32]                         => Int32[i32]                  | conversion auto }
                { Int8[i64]                         => Int64[i64]                  | conversion auto }
                { Bool[bool]                        => Boolean[bool]               | conversion auto  }
                { Text[&'r str]                     => LargeUtf8[String]           | conversion owned }
                { BpChar[&'r str]                   => LargeUtf8[String]           | conversion none }
                { VarChar[&'r str]                  => LargeUtf8[String]           | conversion none }
                { Enum[&'r str]                     => LargeUtf8[String]           | conversion none }
                { Name[&'r str]                     => LargeUtf8[String]           | conversion none }
                { Timestamp[NaiveDateTime]          => Date64[NaiveDateTime]       | conversion auto }
                { Date[NaiveDate]                   => Date32[NaiveDate]           | conversion auto }
                { Time[NaiveTime]                   => Time64[NaiveTime]           | conversion auto }
                { TimestampTz[DateTime<Utc>]        => DateTimeTz[DateTime<Utc>]   | conversion auto }
                { UUID[Uuid]                        => LargeUtf8[String]           | conversion option }
                { Char[&'r str]                     => LargeUtf8[String]           | conversion none }
                { ByteA[Vec<u8>]                    => LargeBinary[Vec<u8>]        | conversion auto }
                { JSON[Value]                       => LargeUtf8[String]           | conversion option }
                { JSONB[Value]                      => LargeUtf8[String]           | conversion none }
                { BoolArray[Vec<bool>]              => BoolArray[Vec<bool>]        | conversion auto_vec }
                { Int2Array[Vec<i16>]               => Int64Array[Vec<i64>]        | conversion auto_vec }
                { Int4Array[Vec<i32>]               => Int64Array[Vec<i64>]        | conversion auto_vec }
                { Int8Array[Vec<i64>]               => Int64Array[Vec<i64>]        | conversion auto }
                { Float4Array[Vec<f32>]             => Float64Array[Vec<f64>]      | conversion auto_vec }
                { Float8Array[Vec<f64>]             => Float64Array[Vec<f64>]      | conversion auto }
                { NumericArray[Vec<Decimal>]        => Float64Array[Vec<f64>]      | conversion option }
                { VarcharArray[Vec<String>]        => Utf8Array[Vec<String>]      | conversion none }
                { TextArray[Vec<String>]        => Utf8Array[Vec<String>]      | conversion auto }

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

impl<P, C> TypeConversion<Uuid, String> for PostgresArrow2Transport<P, C> {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl<P, C> TypeConversion<Decimal, f64> for PostgresArrow2Transport<P, C> {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}

impl<P, C> TypeConversion<Vec<Decimal>, Vec<f64>> for PostgresArrow2Transport<P, C> {
    fn convert(val: Vec<Decimal>) -> Vec<f64> {
        val.into_iter()
            .map(|v| {
                v.to_f64()
                    .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", v))
            })
            .collect()
    }
}

impl<P, C> TypeConversion<Value, String> for PostgresArrow2Transport<P, C> {
    fn convert(val: Value) -> String {
        val.to_string()
    }
}
