//! Transport from BigQuery Source to Arrow Destination.

use crate::{
    destinations::arrow2::{
        typesystem::Arrow2TypeSystem, Arrow2Destination, Arrow2DestinationError,
    },
    impl_transport,
    sources::bigquery::{BigQuerySource, BigQuerySourceError, BigQueryTypeSystem},
    typesystem::TypeConversion,
};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BigQueryArrow2TransportError {
    #[error(transparent)]
    Source(#[from] BigQuerySourceError),

    #[error(transparent)]
    Destination(#[from] Arrow2DestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

/// Convert BigQuery data types to Arrow data types.
pub struct BigQueryArrow2Transport;

impl_transport!(
    name = BigQueryArrow2Transport,
    error = BigQueryArrow2TransportError,
    systems = BigQueryTypeSystem => Arrow2TypeSystem,
    route = BigQuerySource => Arrow2Destination,
    mappings = {
        { Bool[bool]                 => Boolean[bool]             | conversion auto }
        { Boolean[bool]              => Boolean[bool]             | conversion none }
        { Int64[i64]                 => Int64[i64]                | conversion auto }
        { Integer[i64]               => Int64[i64]                | conversion none }
        { Float64[f64]               => Float64[f64]              | conversion auto }
        { Float[f64]                 => Float64[f64]              | conversion none }
        { Numeric[f64]               => Float64[f64]              | conversion none }
        { Bignumeric[f64]            => Float64[f64]              | conversion none }
        { String[String]             => LargeUtf8[String]         | conversion auto }
        { Bytes[String]              => LargeUtf8[String]         | conversion none }
        { Date[NaiveDate]            => Date32[NaiveDate]         | conversion auto }
        { Datetime[NaiveDateTime]    => Date64[NaiveDateTime]     | conversion auto }
        { Time[NaiveTime]            => Time64[NaiveTime]         | conversion auto }
        { Timestamp[DateTime<Utc>]   => DateTimeTz[DateTime<Utc>] | conversion auto }
    }
);
