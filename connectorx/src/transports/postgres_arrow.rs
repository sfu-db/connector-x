use crate::destinations::arrow::ArrowDestination;
use crate::dummy_typesystem::DummyTypeSystem;
use crate::sources::postgres::{Binary, PostgresSource, PostgresTypeSystem};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use uuid::Uuid;

pub struct PostgresArrowTransport;

impl_transport!(
    name = PostgresArrowTransport,
    systems = PostgresTypeSystem => DummyTypeSystem,
    route = PostgresSource<Binary> => ArrowDestination,
    mappings = {
        { Float4[f32]                => F64[f64]                | conversion all }
        { Float8[f64]                => F64[f64]                | conversion all }
        { Int2[i16]                  => I64[i64]                | conversion all }
        { Int4[i32]                  => I64[i64]                | conversion all }
        { Int8[i64]                  => I64[i64]                | conversion all }
        { Bool[bool]                 => Bool[bool]              | conversion all  }
        { Text[&'r str]              => String[String]          | conversion half }
        { BpChar[&'r str]            => String[String]          | conversion none }
        { VarChar[&'r str]           => String[String]          | conversion none }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion half }
        { TimestampTz[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
        { UUID[Uuid]                 => String[String]          | conversion half }
        { Char[&'r str]              => String[String]          | conversion none}
        // { Time[NaiveTime]            => String[String]          | conversion half }
    }
);

impl TypeConversion<Uuid, String> for PostgresArrowTransport {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl TypeConversion<NaiveTime, String> for PostgresArrowTransport {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}

impl<'r> TypeConversion<&'r str, String> for PostgresArrowTransport {
    fn convert(val: &'r str) -> String {
        val.to_string()
    }
}

impl TypeConversion<NaiveDateTime, DateTime<Utc>> for PostgresArrowTransport {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl TypeConversion<NaiveDate, DateTime<Utc>> for PostgresArrowTransport {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}
