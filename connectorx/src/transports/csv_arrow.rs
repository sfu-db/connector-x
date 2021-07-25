use crate::destinations::arrow::{ArrowDestination, ArrowDestinationError};
use crate::dummy_typesystem::DummyTypeSystem;
use crate::sources::csv::{CSVSource, CSVSourceError};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use thiserror::Error;

pub struct CSVArrowTransport;

#[derive(Error, Debug)]
pub enum CSVArrowTransportError {
    #[error(transparent)]
    CSVSourceError(#[from] CSVSourceError),

    #[error(transparent)]
    ArrowDestinationError(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorXError(#[from] crate::ConnectorXError),
}

impl_transport!(
    name = CSVArrowTransport,
    error = CSVArrowTransportError,
    systems = DummyTypeSystem => DummyTypeSystem,
    route = CSVSource => ArrowDestination,
    mappings = {
        { F64[f64]                => F64[f64]                | conversion all}
        { I64[i64]                => I64[i64]                | conversion all}
        { Bool[bool]              => Bool[bool]              | conversion all}
        { String[String]          => String[String]          | conversion all}
        { DateTime[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all}
    }
);

impl TypeConversion<NaiveDateTime, DateTime<Utc>> for CSVArrowTransport {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl TypeConversion<NaiveDate, DateTime<Utc>> for CSVArrowTransport {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}
