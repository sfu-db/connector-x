//! Transport from SQLite Source to Arrow Destination.

use crate::{
    destinations::arrowstream::{
        typesystem::ArrowTypeSystem, ArrowDestination, ArrowDestinationError,
    },
    impl_transport,
    sources::sqlite::{SQLiteSource, SQLiteSourceError, SQLiteTypeSystem},
    typesystem::TypeConversion,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SQLiteArrowTransportError {
    #[error(transparent)]
    Source(#[from] SQLiteSourceError),

    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

/// Convert SQLite data types to Arrow data types.
pub struct SQLiteArrowTransport;

impl_transport!(
    name = SQLiteArrowTransport,
    error = SQLiteArrowTransportError,
    systems = SQLiteTypeSystem => ArrowTypeSystem,
    route = SQLiteSource => ArrowDestination,
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

impl TypeConversion<Box<str>, String> for SQLiteArrowTransport {
    fn convert(val: Box<str>) -> String {
        val.to_string()
    }
}
