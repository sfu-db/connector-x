use crate::destinations::arrow::ArrowDestination;
use crate::dummy_typesystem::DummyTypeSystem;
use crate::sources::postgres::{PostgresSource, PostgresTypeSystem};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

pub struct PostgresArrowTransport;

impl_transport!(
    name = PostgresArrowTransport,
    systems = PostgresTypeSystem => DummyTypeSystem,
    route = PostgresSource => ArrowDestination,
    mappings = {
        { Float4[f32]                => F64[f64]                | conversion all }
        { Float8[f64]                => F64[f64]                | conversion all }
        { Int4[i32]                  => I64[i64]                | conversion all }
        { Int8[i64]                  => I64[i64]                | conversion all }
        { Bool[bool]                 => Bool[bool]              | conversion all  }
        { Text[&'r str]              => String[String]          | conversion half }
        { BpChar[&'r str]            => String[String]          | conversion none }
        { VarChar[&'r str]           => String[String]          | conversion none }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion half }
        { TimestampTz[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
    }
);

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
