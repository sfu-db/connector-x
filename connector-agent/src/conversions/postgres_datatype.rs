use crate::data_sources::postgres::{PostgresDTypes, PostgresSource};
use crate::types::DataType;
use crate::typesystem::TypeConversion;
use crate::writers::memory::MemoryWriter;
use bytes::Bytes;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use std::str::from_utf8;

impl TypeConversion<Bytes, String> for PostgresDataTypeTransport {
    fn convert(val: Bytes) -> String {
        from_utf8(&val[..]).unwrap().to_string()
    }
}

impl TypeConversion<NaiveDateTime, DateTime<Utc>> for PostgresDataTypeTransport {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl TypeConversion<NaiveDate, DateTime<Utc>> for PostgresDataTypeTransport {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}

pub struct PostgresDataTypeTransport;

create_transport!(
    [],
    PostgresDataTypeTransport,
    PostgresDTypes => DataType,
    PostgresSource => MemoryWriter,
    ([PostgresDTypes::Float4], [DataType::F64]) => (f32, f64) conversion all,
    ([PostgresDTypes::Float8], [DataType::F64]) => (f64, f64) conversion all,
    ([PostgresDTypes::Int4], [DataType::I64]) => (i32, i64) conversion all,
    ([PostgresDTypes::Int8], [DataType::I64]) => (i64, i64) conversion all,
    ([PostgresDTypes::Bool], [DataType::Bool]) => (bool, bool) conversion all,
    ([PostgresDTypes::Text], [DataType::String]) | ([PostgresDTypes::BpChar], [DataType::String]) | ([PostgresDTypes::VarChar], [DataType::String]) => (Bytes, String) conversion half,
    ([PostgresDTypes::Timestamp], [DataType::DateTime]) => (NaiveDateTime, DateTime<Utc>) conversion half,
    ([PostgresDTypes::TimestampTz], [DataType::DateTime]) => (DateTime<Utc>, DateTime<Utc>) conversion all,
    ([PostgresDTypes::Date], [DataType::DateTime]) => (NaiveDate, DateTime<Utc>) conversion half,
);
