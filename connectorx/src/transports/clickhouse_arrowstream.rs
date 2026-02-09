//! Transport from ClickHouse Source to Arrow Stream Destination.

use crate::{
    destinations::arrowstream::{
        typesystem::ArrowTypeSystem, ArrowDestination, ArrowDestinationError,
    },
    impl_transport,
    sources::clickhouse::{ClickHouseSource, ClickHouseSourceError, ClickHouseTypeSystem},
    typesystem::TypeConversion,
};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use rust_decimal::Decimal;
use std::net::IpAddr;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ClickHouseArrowTransportError {
    #[error(transparent)]
    Source(#[from] ClickHouseSourceError),

    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

/// Convert ClickHouse data types to Arrow Stream data types.
pub struct ClickHouseArrowStreamTransport;

impl_transport!(
    name = ClickHouseArrowStreamTransport,
    error = ClickHouseArrowTransportError,
    systems = ClickHouseTypeSystem => ArrowTypeSystem,
    route = ClickHouseSource => ArrowDestination,
    mappings = {
        { Int8[i8]                   => Int16[i16]                              | conversion auto }
        { Int16[i16]                 => Int16[i16]                              | conversion auto }
        { Int32[i32]                 => Int32[i32]                              | conversion auto }
        { Int64[i64]                 => Int64[i64]                              | conversion auto }

        { UInt8[u8]                  => UInt16[u16]                             | conversion auto }
        { UInt16[u16]                => UInt16[u16]                             | conversion auto }
        { UInt32[u32]                => UInt32[u32]                             | conversion auto }
        { UInt64[u64]                => UInt64[u64]                             | conversion auto }

        { Float32[f32]               => Float32[f32]                            | conversion auto }
        { Float64[f64]               => Float64[f64]                            | conversion auto }

        { Decimal[Decimal]           => Decimal[Decimal]                        | conversion auto }

        { String[String]             => LargeUtf8[String]                       | conversion auto }
        { FixedString[Vec<u8>]       => LargeBinary[Vec<u8>]                    | conversion auto }

        { Enum8[String]              => LargeUtf8[String]                       | conversion none }
        { Enum16[String]             => LargeUtf8[String]                       | conversion none }

        { Date[NaiveDate]            => Date32[NaiveDate]                       | conversion auto }
        { Date32[NaiveDate]          => Date32[NaiveDate]                       | conversion none }
        { DateTime[DateTime<Utc>]    => DateTimeTz[DateTime<Utc>]               | conversion auto }
        { DateTime64[DateTime<Utc>]  => DateTimeTz[DateTime<Utc>]               | conversion none }
        { Time[NaiveTime]            => Time64[NaiveTime]                       | conversion auto }
        { Time64[NaiveTime]          => Time64[NaiveTime]                       | conversion none }

        { UUID[Uuid]                 => LargeUtf8[String]                       | conversion option }
        { IPv4[IpAddr]               => LargeUtf8[String]                       | conversion option }
        { IPv6[IpAddr]               => LargeUtf8[String]                       | conversion none }
        { Bool[bool]                 => Boolean[bool]                           | conversion auto }

        { ArrayBool[Vec<Option<bool>>]         => BoolArray[Vec<Option<bool>>]           | conversion auto }
        { ArrayString[Vec<Option<String>>]     => Utf8Array[Vec<Option<String>>]         | conversion auto }
        { ArrayInt8[Vec<Option<i8>>]           => Int16Array[Vec<Option<i16>>]           | conversion none }
        { ArrayInt16[Vec<Option<i16>>]         => Int16Array[Vec<Option<i16>>]           | conversion auto }
        { ArrayInt32[Vec<Option<i32>>]         => Int32Array[Vec<Option<i32>>]           | conversion auto }
        { ArrayInt64[Vec<Option<i64>>]         => Int64Array[Vec<Option<i64>>]           | conversion auto }
        { ArrayUInt8[Vec<Option<u8>>]          => UInt16Array[Vec<Option<u16>>]          | conversion none }
        { ArrayUInt16[Vec<Option<u16>>]        => UInt16Array[Vec<Option<u16>>]          | conversion auto }
        { ArrayUInt32[Vec<Option<u32>>]        => UInt32Array[Vec<Option<u32>>]          | conversion auto }
        { ArrayUInt64[Vec<Option<u64>>]        => UInt64Array[Vec<Option<u64>>]          | conversion auto }
        { ArrayFloat32[Vec<Option<f32>>]       => Float32Array[Vec<Option<f32>>]         | conversion auto }
        { ArrayFloat64[Vec<Option<f64>>]       => Float64Array[Vec<Option<f64>>]         | conversion auto }
        { ArrayDecimal[Vec<Option<Decimal>>]   => DecimalArray[Vec<Option<Decimal>>]     | conversion auto }
    }
);

impl TypeConversion<Uuid, String> for ClickHouseArrowStreamTransport {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl TypeConversion<IpAddr, String> for ClickHouseArrowStreamTransport {
    fn convert(val: IpAddr) -> String {
        val.to_string()
    }
}

impl TypeConversion<Vec<Option<i8>>, Vec<Option<i16>>> for ClickHouseArrowStreamTransport {
    fn convert(val: Vec<Option<i8>>) -> Vec<Option<i16>> {
        val.into_iter().map(|opt| opt.map(|v| v as i16)).collect()
    }
}

impl TypeConversion<Vec<Option<u8>>, Vec<Option<u16>>> for ClickHouseArrowStreamTransport {
    fn convert(val: Vec<Option<u8>>) -> Vec<Option<u16>> {
        val.into_iter().map(|opt| opt.map(|v| v as u16)).collect()
    }
}

impl TypeConversion<Option<Vec<Option<i8>>>, Option<Vec<Option<i16>>>>
    for ClickHouseArrowStreamTransport
{
    fn convert(val: Option<Vec<Option<i8>>>) -> Option<Vec<Option<i16>>> {
        val.map(Self::convert)
    }
}

impl TypeConversion<Option<Vec<Option<u8>>>, Option<Vec<Option<u16>>>>
    for ClickHouseArrowStreamTransport
{
    fn convert(val: Option<Vec<Option<u8>>>) -> Option<Vec<Option<u16>>> {
        val.map(Self::convert)
    }
}