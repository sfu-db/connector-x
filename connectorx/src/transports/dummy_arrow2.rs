//! Transport from Dummy Source to Arrow2 Destination.

use crate::destinations::arrow2::{Arrow2Destination, Arrow2DestinationError, Arrow2TypeSystem};
use crate::sources::dummy::{DummySource, DummyTypeSystem};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use thiserror::Error;

/// Convert Dummy data types to Arrow2 data types.
pub struct DummyArrow2Transport;

#[derive(Error, Debug)]
pub enum DummyArrow2TransportError {
    #[error(transparent)]
    Destination(#[from] Arrow2DestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

impl_transport!(
    name = DummyArrow2Transport,
    error = DummyArrow2TransportError,
    systems = DummyTypeSystem => Arrow2TypeSystem,
    route = DummySource => Arrow2Destination,
    mappings = {
        { F64[f64]                => Float64[f64]               | conversion auto}
        { I64[i64]                => Int64[i64]                 | conversion auto}
        { Bool[bool]              => Boolean[bool]              | conversion auto}
        { String[String]          => LargeUtf8[String]          | conversion auto}
        { DateTime[DateTime<Utc>] => Date64[NaiveDateTime]      | conversion option}
    }
);

impl TypeConversion<DateTime<Utc>, NaiveDateTime> for DummyArrow2Transport {
    fn convert(val: DateTime<Utc>) -> NaiveDateTime {
        NaiveDateTime::from_timestamp_opt(val.timestamp(), val.timestamp_subsec_nanos())
            .unwrap_or_else(|| panic!("from_timestamp_opt return None"))
    }
}

impl TypeConversion<NaiveDateTime, DateTime<Utc>> for DummyArrow2Transport {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_naive_utc_and_offset(val, Utc)
    }
}

impl TypeConversion<NaiveDate, DateTime<Utc>> for DummyArrow2Transport {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_naive_utc_and_offset(
            val.and_hms_opt(0, 0, 0)
                .unwrap_or_else(|| panic!("from_hms_opt return None")),
            Utc,
        )
    }
}
