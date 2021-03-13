use super::destination::PandasDestination;
use super::types::PandasTypeSystem;
use bytes::Bytes;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use connector_agent::{
    impl_transport,
    sources::postgres::{PostgresSource, PostgresTypeSystem},
    typesystem::TypeConversion,
};

pub struct PostgresPandasTransport<'py>(&'py ());

impl_transport! {
    ['py],
    PostgresPandasTransport<'py>,
    PostgresTypeSystem => PandasTypeSystem,
    PostgresSource => PandasDestination<'py>,
    ([PostgresTypeSystem::Float4], [PandasTypeSystem::F64]) => (f32, f64) conversion all,
    ([PostgresTypeSystem::Float8], [PandasTypeSystem::F64]) => (f64, f64) conversion all,
    ([PostgresTypeSystem::Int4], [PandasTypeSystem::I64]) => (i32, i64) conversion all,
    ([PostgresTypeSystem::Int8], [PandasTypeSystem::I64]) => (i64, i64) conversion all,
    ([PostgresTypeSystem::Bool], [PandasTypeSystem::Bool]) => (bool, bool) conversion all,
    ([PostgresTypeSystem::Text], [PandasTypeSystem::String]) | ([PostgresTypeSystem::BpChar], [PandasTypeSystem::String]) | ([PostgresTypeSystem::VarChar], [PandasTypeSystem::String]) => (Bytes, Bytes) conversion all,
    ([PostgresTypeSystem::Timestamp], [PandasTypeSystem::DateTime]) => (NaiveDateTime, DateTime<Utc>) conversion half,
    ([PostgresTypeSystem::TimestampTz], [PandasTypeSystem::DateTime]) => (DateTime<Utc>, DateTime<Utc>) conversion all,
    ([PostgresTypeSystem::Date], [PandasTypeSystem::DateTime]) => (NaiveDate, DateTime<Utc>) conversion half,
}

impl<'py> TypeConversion<NaiveDateTime, DateTime<Utc>> for PostgresPandasTransport<'py> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl<'py> TypeConversion<NaiveDate, DateTime<Utc>> for PostgresPandasTransport<'py> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}
