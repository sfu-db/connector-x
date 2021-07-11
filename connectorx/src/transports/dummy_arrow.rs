use crate::destinations::arrow::types::ArrowTypeSystem;
use crate::destinations::arrow::ArrowDestination;
use crate::dummy_typesystem::DummyTypeSystem;
use crate::sources::dummy::DummySource;
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

pub struct DummyArrowTransport;

impl_transport!(
    name = DummyArrowTransport,
    systems = DummyTypeSystem => ArrowTypeSystem,
    route = DummySource => ArrowDestination,
    mappings = {
        { F64[f64]                => Float64[f64]               | conversion all}
        { I64[i64]                => Int64[i64]                 | conversion all}
        { Bool[bool]              => Boolean[bool]              | conversion all}
        { String[String]          => LargeUtf8[String]          | conversion half}
        { DateTime[DateTime<Utc>] => Date64[NaiveDateTime]      | conversion half}
    }
);

impl TypeConversion<DateTime<Utc>, NaiveDateTime> for DummyArrowTransport {
    fn convert(val: DateTime<Utc>) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(val.timestamp(), val.timestamp_subsec_nanos())
    }
}

impl TypeConversion<NaiveDateTime, DateTime<Utc>> for DummyArrowTransport {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl TypeConversion<NaiveDate, DateTime<Utc>> for DummyArrowTransport {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}

impl TypeConversion<String, String> for DummyArrowTransport {
    fn convert(val: String) -> String {
        val
    }
}
