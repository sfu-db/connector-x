//! Transport from Postgres Source to Arrow Destination.

use crate::destinations::arrow::{
    typesystem::{
        ArrowTypeSystem, DateTimeWrapperMicro, NaiveDateTimeWrapperMicro, NaiveTimeWrapperMicro,
    },
    ArrowDestination, ArrowDestinationError,
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
pub enum PostgresArrowTransportError {
    #[error(transparent)]
    Source(#[from] PostgresSourceError),

    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

/// Convert Postgres data types to Arrow data types.
pub struct PostgresArrowTransport<P, C>(PhantomData<P>, PhantomData<C>);

macro_rules! impl_postgres_transport {
    ($proto:ty, $tls:ty) => {
        impl_transport!(
            name = PostgresArrowTransport<$proto, $tls>,
            error = PostgresArrowTransportError,
            systems = PostgresTypeSystem => ArrowTypeSystem,
            route = PostgresSource<$proto, $tls> => ArrowDestination,
            mappings = {
                { Float4[f32]                => Float32[f32]                           | conversion auto   }
                { Float8[f64]                => Float64[f64]                           | conversion auto   }
                { Numeric[Decimal]           => Float64[f64]                           | conversion option }
                { Int2[i16]                  => Int16[i16]                             | conversion auto   }
                { Int4[i32]                  => Int32[i32]                             | conversion auto   }
                { Int8[i64]                  => Int64[i64]                             | conversion auto   }
                { Bool[bool]                 => Boolean[bool]                          | conversion auto   }
                { Text[&'r str]              => LargeUtf8[String]                      | conversion owned  }
                { BpChar[&'r str]            => LargeUtf8[String]                      | conversion none   }
                { VarChar[&'r str]           => LargeUtf8[String]                      | conversion none   }
                { Name[&'r str]              => LargeUtf8[String]                      | conversion none   }
                { Enum[&'r str]              => LargeUtf8[String]                      | conversion none   }
                { Timestamp[NaiveDateTime]   => Date64Micro[NaiveDateTimeWrapperMicro] | conversion option }
                { Date[NaiveDate]            => Date32[NaiveDate]                      | conversion auto   }
                { Time[NaiveTime]            => Time64Micro[NaiveTimeWrapperMicro]     | conversion option }
                { TimestampTz[DateTime<Utc>] => DateTimeTzMicro[DateTimeWrapperMicro]  | conversion option }
                { UUID[Uuid]                 => LargeUtf8[String]                      | conversion option }
                { Char[&'r str]              => LargeUtf8[String]                      | conversion none   }
                { ByteA[Vec<u8>]             => LargeBinary[Vec<u8>]                   | conversion auto   }
                { JSON[Value]                => LargeUtf8[String]                      | conversion option }
                { JSONB[Value]               => LargeUtf8[String]                      | conversion none   }
                { BoolArray[Vec<bool>]       => BoolArray[Vec<Option<bool>>]           | conversion option }
                { VarcharArray[Vec<String>]  => Utf8Array[Vec<Option<String>>]         | conversion option }
                { TextArray[Vec<String>]     => Utf8Array[Vec<Option<String>>]         | conversion none   }
                { Int2Array[Vec<i16>]        => Int16Array[Vec<Option<i16>>]           | conversion option }
                { Int4Array[Vec<i32>]        => Int32Array[Vec<Option<i32>>]           | conversion option }
                { Int8Array[Vec<i64>]        => Int64Array[Vec<Option<i64>>]           | conversion option }
                { Float4Array[Vec<f32>]      => Float32Array[Vec<Option<f32>>]         | conversion option }
                { Float8Array[Vec<f64>]      => Float64Array[Vec<Option<f64>>]         | conversion option }
                { NumericArray[Vec<Decimal>] => Float64Array[Vec<Option<f64>>]         | conversion option }
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

impl<P, C> TypeConversion<NaiveTime, NaiveTimeWrapperMicro> for PostgresArrowTransport<P, C> {
    fn convert(val: NaiveTime) -> NaiveTimeWrapperMicro {
        NaiveTimeWrapperMicro(val)
    }
}

impl<P, C> TypeConversion<NaiveDateTime, NaiveDateTimeWrapperMicro>
    for PostgresArrowTransport<P, C>
{
    fn convert(val: NaiveDateTime) -> NaiveDateTimeWrapperMicro {
        NaiveDateTimeWrapperMicro(val)
    }
}

impl<P, C> TypeConversion<DateTime<Utc>, DateTimeWrapperMicro> for PostgresArrowTransport<P, C> {
    fn convert(val: DateTime<Utc>) -> DateTimeWrapperMicro {
        DateTimeWrapperMicro(val)
    }
}

impl<P, C> TypeConversion<Uuid, String> for PostgresArrowTransport<P, C> {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl<P, C> TypeConversion<Decimal, f64> for PostgresArrowTransport<P, C> {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}

impl<P, C> TypeConversion<Value, String> for PostgresArrowTransport<P, C> {
    fn convert(val: Value) -> String {
        val.to_string()
    }
}

impl<P, C> TypeConversion<Vec<bool>, Vec<Option<bool>>> for PostgresArrowTransport<P, C> {
    fn convert(val: Vec<bool>) -> Vec<Option<bool>> {
        val.into_iter().map(Some).collect()
    }
}

impl<P, C> TypeConversion<Vec<String>, Vec<Option<String>>> for PostgresArrowTransport<P, C> {
    fn convert(val: Vec<String>) -> Vec<Option<String>> {
        val.into_iter().map(Some).collect()
    }
}

impl<P, C> TypeConversion<Vec<i16>, Vec<Option<i16>>> for PostgresArrowTransport<P, C> {
    fn convert(val: Vec<i16>) -> Vec<Option<i16>> {
        val.into_iter().map(Some).collect()
    }
}

impl<P, C> TypeConversion<Vec<i32>, Vec<Option<i32>>> for PostgresArrowTransport<P, C> {
    fn convert(val: Vec<i32>) -> Vec<Option<i32>> {
        val.into_iter().map(Some).collect()
    }
}

impl<P, C> TypeConversion<Vec<i64>, Vec<Option<i64>>> for PostgresArrowTransport<P, C> {
    fn convert(val: Vec<i64>) -> Vec<Option<i64>> {
        val.into_iter().map(Some).collect()
    }
}

impl<P, C> TypeConversion<Vec<Decimal>, Vec<Option<f64>>> for PostgresArrowTransport<P, C> {
    fn convert(val: Vec<Decimal>) -> Vec<Option<f64>> {
        val.into_iter()
            .map(|v| {
                Some(
                    v.to_f64()
                        .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", v)),
                )
            })
            .collect()
    }
}

impl<P, C> TypeConversion<Vec<f32>, Vec<Option<f32>>> for PostgresArrowTransport<P, C> {
    fn convert(val: Vec<f32>) -> Vec<Option<f32>> {
        val.into_iter().map(Some).collect()
    }
}

impl<P, C> TypeConversion<Vec<f64>, Vec<Option<f64>>> for PostgresArrowTransport<P, C> {
    fn convert(val: Vec<f64>) -> Vec<Option<f64>> {
        val.into_iter().map(Some).collect()
    }
}
