//! Transport from BigQuery Source to Arrow Destination.

use crate::{
    destinations::arrow::{
        typesystem::{
            ArrowTypeSystem, DateTimeWrapperMicro, NaiveDateTimeWrapperMicro, NaiveTimeWrapperMicro,
        },
        ArrowDestination, ArrowDestinationError,
    },
    impl_transport,
    sources::bigquery::{BigQuerySource, BigQuerySourceError, BigQueryTypeSystem},
    typesystem::TypeConversion,
};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BigQueryArrowTransportError {
    #[error(transparent)]
    Source(#[from] BigQuerySourceError),

    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

/// Convert BigQuery data types to Arrow data types.
pub struct BigQueryArrowTransport;

impl_transport!(
    name = BigQueryArrowTransport,
    error = BigQueryArrowTransportError,
    systems = BigQueryTypeSystem => ArrowTypeSystem,
    route = BigQuerySource => ArrowDestination,
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
        { Datetime[NaiveDateTime]    => Date64Micro[NaiveDateTimeWrapperMicro] | conversion option }
        { Time[NaiveTime]            => Time64Micro[NaiveTimeWrapperMicro]     | conversion option }
        { Timestamp[DateTime<Utc>]   => DateTimeTzMicro[DateTimeWrapperMicro]  | conversion option }
    }
);

impl TypeConversion<NaiveDateTime, NaiveDateTimeWrapperMicro> for BigQueryArrowTransport {
    fn convert(val: NaiveDateTime) -> NaiveDateTimeWrapperMicro {
        NaiveDateTimeWrapperMicro(val)
    }
}

impl TypeConversion<NaiveTime, NaiveTimeWrapperMicro> for BigQueryArrowTransport {
    fn convert(val: NaiveTime) -> NaiveTimeWrapperMicro {
        NaiveTimeWrapperMicro(val)
    }
}

impl TypeConversion<DateTime<Utc>, DateTimeWrapperMicro> for BigQueryArrowTransport {
    fn convert(val: DateTime<Utc>) -> DateTimeWrapperMicro {
        DateTimeWrapperMicro(val)
    }
}
