use crate::destinations::memory::MemoryDestination;
use crate::dummy_typesystem::DummyTypeSystem;
use crate::sources::dummy::DummySource;
use crate::typesystem::TypeConversion;
use crate::ConnectorAgentError;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

pub struct DummyMemoryTransport;

impl_transport!(
    name = DummyMemoryTransport,
    error = ConnectorAgentError,
    systems = DummyTypeSystem => DummyTypeSystem,
    route = DummySource => MemoryDestination,
    mappings = {
        { F64[f64]                => F64[f64]                | conversion all}
        { I64[i64]                => I64[i64]                | conversion all}
        { Bool[bool]              => Bool[bool]              | conversion all}
        { String[String]          => String[String]          | conversion all}
        { DateTime[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all}
    }
);

impl TypeConversion<NaiveDateTime, DateTime<Utc>> for DummyMemoryTransport {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl TypeConversion<NaiveDate, DateTime<Utc>> for DummyMemoryTransport {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}
