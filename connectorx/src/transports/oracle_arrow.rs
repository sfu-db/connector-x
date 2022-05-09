use crate::{
    destinations::arrow::{typesystem::ArrowTypeSystem, ArrowDestination, ArrowDestinationError},
    impl_transport,
    sources::oracle::{OracleSource, OracleSourceError, OracleTypeSystem},
    typesystem::TypeConversion,
};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OracleArrowTransportError {
    #[error(transparent)]
    Source(#[from] OracleSourceError),

    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

pub struct OracleArrowTransport;

impl_transport!(
    name = OracleArrowTransport,
    error = OracleArrowTransportError,
    systems = OracleTypeSystem => ArrowTypeSystem,
    route = OracleSource => ArrowDestination,
    mappings = {
        { NumFloat[f64]              => Float64[f64]               | conversion auto }
        { Float[f64]                 => Float64[f64]               | conversion none }
        { BinaryFloat[f64]           => Float64[f64]               | conversion none }
        { BinaryDouble[f64]          => Float64[f64]               | conversion none }
        { NumInt[i64]                => Int64[i64]                 | conversion auto }
        { Blob[Vec<u8>]              => LargeBinary[Vec<u8>]       | conversion none }
        { Clob[String]               => String[String]             | conversion none }
        { VarChar[String]            => LargeUtf8[String]          | conversion auto }
        { Char[String]               => LargeUtf8[String]          | conversion none }
        { NVarChar[String]           => LargeUtf8[String]          | conversion none }
        { NChar[String]              => LargeUtf8[String]          | conversion none }
        { Date[NaiveDate]            => Date32[NaiveDate]          | conversion auto }
        { Timestamp[NaiveDateTime]   => Date64[NaiveDateTime]      | conversion auto }
        { TimestampTz[DateTime<Utc>] => DateTimeTz[DateTime<Utc>]  | conversion auto }
    }
);
