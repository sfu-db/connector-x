//! Transport from BigQuery Source to Arrow Destination.

use crate::{
    destinations::arrow::{typesystem::ArrowTypeSystem, ArrowDestination, ArrowDestinationError},
    impl_transport,
    sources::bigquery::{BigQuerySource, BigQuerySourceError, BigQueryTypeSystem},
    typesystem::TypeConversion,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
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
        { Int64[i64]                 => Int64[i64]              | conversion auto }
        { Integer[i64]               => Int64[i64]              | conversion none }
        { Float64[f64]               => Float64[i64]            | conversion auto }
        { Float[f64]                 => Float64[i64]            | conversion none }
        { String[String]             => LargeUtf8[String]       | conversion auto }
        { Date[NaiveDate]            => Date32[NaiveDate]       | conversion auto }
        // { String[String]             => LargeUtf8[String]       | conversion auto }
    }
);
