use crate::destinations::memory::MemoryDestination;
use crate::dummy_typesystem::DummyTypeSystem;
use crate::sources::postgres::{PostgresSource, PostgresTypeSystem};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

pub struct PostgresMemoryTransport;

impl_transport!(
    name = PostgresMemoryTransport,
    systems = PostgresTypeSystem => DummyTypeSystem,
    route = PostgresSource => MemoryDestination,
    mappings = {
        [Float4      => F64      | f32           => f64           | conversion all]
        [Float8      => F64      | f64           => f64           | conversion all]
        [Int4        => I64      | i32           => i64           | conversion all]
        [Int8        => I64      | i64           => i64           | conversion all]
        [Bool        => Bool     | bool          => bool          | conversion all]
        [Text        => String   | &'r str       => String        | conversion half]
        [BpChar      => String   | &'r str       => String        | conversion none]
        [VarChar     => String   | &'r str       => String        | conversion none]
        [Timestamp   => DateTime | NaiveDateTime => DateTime<Utc> | conversion half]
        [TimestampTz => DateTime | DateTime<Utc> => DateTime<Utc> | conversion all]
        [Date        => DateTime | NaiveDate     => DateTime<Utc> | conversion half]
    }
);

impl<'r> TypeConversion<&'r str, String> for PostgresMemoryTransport {
    fn convert(val: &'r str) -> String {
        val.to_string()
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
