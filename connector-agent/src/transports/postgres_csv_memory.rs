use crate::destinations::memory::MemoryDestination;
use crate::dummy_typesystem::DummyTypeSystem;
use crate::sources::postgres::{PostgresSourceCSV, PostgresTypeSystem};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

pub struct PostgresCSVMemoryTransport;

impl_transport!(
    [],
    PostgresCSVMemoryTransport,
    PostgresTypeSystem => DummyTypeSystem,
    PostgresSourceCSV => MemoryDestination,
    ([PostgresTypeSystem::Float4], [DummyTypeSystem::F64]) => (f32, f64) conversion all,
    ([PostgresTypeSystem::Float8], [DummyTypeSystem::F64]) => (f64, f64) conversion all,
    ([PostgresTypeSystem::Int4], [DummyTypeSystem::I64]) => (i32, i64) conversion all,
    ([PostgresTypeSystem::Int8], [DummyTypeSystem::I64]) => (i64, i64) conversion all,
    ([PostgresTypeSystem::Bool], [DummyTypeSystem::Bool]) => (bool, bool) conversion all,
    ([PostgresTypeSystem::Text], [DummyTypeSystem::String]) | ([PostgresTypeSystem::BpChar], [DummyTypeSystem::String]) | ([PostgresTypeSystem::VarChar], [DummyTypeSystem::String]) => ['r] (&'r str, String) conversion half,
    ([PostgresTypeSystem::Timestamp], [DummyTypeSystem::DateTime]) => (NaiveDateTime, DateTime<Utc>) conversion half,
    ([PostgresTypeSystem::TimestampTz], [DummyTypeSystem::DateTime]) => (DateTime<Utc>, DateTime<Utc>) conversion all,
    ([PostgresTypeSystem::Date], [DummyTypeSystem::DateTime]) => (NaiveDate, DateTime<Utc>) conversion half,
);

impl<'r> TypeConversion<&'r str, String> for PostgresCSVMemoryTransport {
    fn convert(val: &'r str) -> String {
        val.to_string()
    }
}

impl TypeConversion<NaiveDateTime, DateTime<Utc>> for PostgresCSVMemoryTransport {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl TypeConversion<NaiveDate, DateTime<Utc>> for PostgresCSVMemoryTransport {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}
