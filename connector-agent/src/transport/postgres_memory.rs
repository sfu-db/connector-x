use crate::data_sources::postgres::{PostgresSource, PostgresTypeSystem};
use crate::dummy_typesystem::DummyTypeSystem;
use crate::typesystem::TypeConversion;
use crate::writers::memory::MemoryWriter;
use bytes::Bytes;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use std::str::from_utf8;

impl TypeConversion<Bytes, String> for PostgresMemoryTransport {
    fn convert(val: Bytes) -> String {
        from_utf8(&val[..]).unwrap().to_string()
    }
}

impl TypeConversion<NaiveDateTime, DateTime<Utc>> for PostgresMemoryTransport {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl TypeConversion<NaiveDate, DateTime<Utc>> for PostgresMemoryTransport {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}

pub struct PostgresMemoryTransport;

impl_transport!(
    [],
    PostgresMemoryTransport,
    PostgresTypeSystem => DummyTypeSystem,
    PostgresSource => MemoryWriter,
    ([PostgresTypeSystem::Float4], [DummyTypeSystem::F64]) => (f32, f64) conversion all,
    ([PostgresTypeSystem::Float8], [DummyTypeSystem::F64]) => (f64, f64) conversion all,
    ([PostgresTypeSystem::Int4], [DummyTypeSystem::I64]) => (i32, i64) conversion all,
    ([PostgresTypeSystem::Int8], [DummyTypeSystem::I64]) => (i64, i64) conversion all,
    ([PostgresTypeSystem::Bool], [DummyTypeSystem::Bool]) => (bool, bool) conversion all,
    ([PostgresTypeSystem::Text], [DummyTypeSystem::String]) | ([PostgresTypeSystem::BpChar], [DummyTypeSystem::String]) | ([PostgresTypeSystem::VarChar], [DummyTypeSystem::String]) => (Bytes, String) conversion half,
    ([PostgresTypeSystem::Timestamp], [DummyTypeSystem::DateTime]) => (NaiveDateTime, DateTime<Utc>) conversion half,
    ([PostgresTypeSystem::TimestampTz], [DummyTypeSystem::DateTime]) => (DateTime<Utc>, DateTime<Utc>) conversion all,
    ([PostgresTypeSystem::Date], [DummyTypeSystem::DateTime]) => (NaiveDate, DateTime<Utc>) conversion half,
);
