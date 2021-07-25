use crate::{
    destinations::arrow::{types::ArrowTypeSystem, ArrowDestination, ArrowDestinationError},
    impl_transport,
    sources::sqlite::{SQLiteSource, SQLiteSourceError, SQLiteTypeSystem},
    typesystem::TypeConversion,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SQLiteArrowTransportError {
    #[error(transparent)]
    SQLiteSourceError(#[from] SQLiteSourceError),

    #[error(transparent)]
    ArrowDestinationError(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),
}

pub struct SQLiteArrowTransport;

impl_transport!(
    name = SQLiteArrowTransport,
    error = SQLiteArrowTransportError,
    systems = SQLiteTypeSystem => ArrowTypeSystem,
    route = SQLiteSource => ArrowDestination,
    mappings = {
        { Bool[bool]                 => Boolean[bool]           | conversion all }
        { Int8[i64]                  => Int64[i64]              | conversion all }
        { Int4[i32]                  => Int64[i64]              | conversion all }
        { Int2[i16]                  => Int64[i64]              | conversion all }
        { Real[f64]                  => Float64[f64]            | conversion all }
        { Text[Box<str>]             => LargeUtf8[String]       | conversion half }
        { Blob[Vec<u8>]              => LargeBinary[Vec<u8>]    | conversion all }
        { Date[NaiveDate]            => Date32[NaiveDate]       | conversion all }
        { Time[NaiveTime]            => Time64[NaiveTime]       | conversion all }
        { Timestamp[NaiveDateTime]   => Date64[NaiveDateTime]   | conversion all }
    }
);

impl TypeConversion<Box<str>, String> for SQLiteArrowTransport {
    fn convert(val: Box<str>) -> String {
        val.to_string()
    }
}
