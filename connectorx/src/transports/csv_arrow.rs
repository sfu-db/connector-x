//! Transport from CSV Source to Arrow Destination.

use crate::destinations::arrow::{
    typesystem::{ArrowTypeSystem, DateTimeWrapperMicro},
    ArrowDestination, ArrowDestinationError,
};
use crate::sources::csv::{CSVSource, CSVSourceError, CSVTypeSystem};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, Utc};
use thiserror::Error;

/// Convert CSV data types to Arrow data types.
pub struct CSVArrowTransport;

#[derive(Error, Debug)]
pub enum CSVArrowTransportError {
    #[error(transparent)]
    Source(#[from] CSVSourceError),

    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

impl_transport!(
    name = CSVArrowTransport,
    error = CSVArrowTransportError,
    systems = CSVTypeSystem => ArrowTypeSystem,
    route = CSVSource => ArrowDestination,
    mappings = {
        { F64[f64]                => Float64[f64]              | conversion auto}
        { I64[i64]                => Int64[i64]                | conversion auto}
        { Bool[bool]              => Boolean[bool]             | conversion auto}
        { String[String]          => LargeUtf8[String]         | conversion auto}
        { DateTime[DateTime<Utc>] => DateTimeTzMicro[DateTimeWrapperMicro] | conversion option}
    }
);

impl TypeConversion<DateTime<Utc>, DateTimeWrapperMicro> for CSVArrowTransport {
    fn convert(val: DateTime<Utc>) -> DateTimeWrapperMicro {
        DateTimeWrapperMicro(val)
    }
}
