//! Transport from SQLite Source to Arrow2 Destination.

use crate::{
    destinations::arrow2::{
        typesystem::Arrow2TypeSystem, Arrow2Destination, Arrow2DestinationError,
    },
    impl_transport,
    sources::sqlite::{SQLiteSource, SQLiteSourceError, SQLiteTypeSystem},
    typesystem::TypeConversion,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SQLiteArrow2TransportError {
    #[error(transparent)]
    Source(#[from] SQLiteSourceError),

    #[error(transparent)]
    Destination(#[from] Arrow2DestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

/// Convert SQLite data types to Arrow2 data types.
pub struct SQLiteArrow2Transport;

impl_transport!(
    name = SQLiteArrow2Transport,
    error = SQLiteArrow2TransportError,
    systems = SQLiteTypeSystem => Arrow2TypeSystem,
    route = SQLiteSource => Arrow2Destination,
    mappings = {
        { Bool[bool]                 => Boolean[bool]           | conversion auto }
        { Int8[i64]                  => Int64[i64]              | conversion auto }
        { Int4[i32]                  => Int64[i64]              | conversion auto }
        { Int2[i16]                  => Int64[i64]              | conversion auto }
        { Real[f64]                  => Float64[f64]            | conversion auto }
        { Text[Box<str>]             => LargeUtf8[String]       | conversion option }
        { Blob[Vec<u8>]              => LargeBinary[Vec<u8>]    | conversion auto }
        { Date[NaiveDate]            => Date32[NaiveDate]       | conversion auto }
        { Time[NaiveTime]            => Time64[NaiveTime]       | conversion auto }
        { Timestamp[NaiveDateTime]   => Date64[NaiveDateTime]   | conversion auto }
    }
);

impl TypeConversion<Box<str>, String> for SQLiteArrow2Transport {
    fn convert(val: Box<str>) -> String {
        val.to_string()
    }
}
