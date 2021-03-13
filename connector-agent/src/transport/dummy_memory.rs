use crate::data_sources::dummy::MixedSource;
use crate::types::DataType;
use crate::typesystem::TypeConversion;
use crate::writers::memory::MemoryWriter;
use bytes::Bytes;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use std::str::from_utf8;

impl TypeConversion<Bytes, String> for DummyMemoryTransport {
    fn convert(val: Bytes) -> String {
        from_utf8(&val[..]).unwrap().to_string()
    }
}

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

pub struct DummyMemoryTransport;

define_transport!(
    [],
    DummyMemoryTransport,
    DataType => DataType,
    MixedSource => MemoryWriter,
    ([DataType::F64], [DataType::F64]) => (f64, f64) conversion all,
    ([DataType::I64], [DataType::I64]) => (i64, i64) conversion all,
    ([DataType::Bool], [DataType::Bool]) => (bool, bool) conversion all,
    ([DataType::String], [DataType::String]) => (String, String) conversion all,
    ([DataType::DateTime], [DataType::DateTime]) => (DateTime<Utc>, DateTime<Utc>) conversion all,
);
