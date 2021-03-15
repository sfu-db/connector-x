use super::destination::PandasDestination;
use super::types::PandasTypeSystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use connector_agent::{
    impl_transport,
    sources::postgres::{PostgresSource, PostgresSourceCSV, PostgresTypeSystem},
    typesystem::TypeConversion,
};

pub struct PostgresPandasTransport<'py>(&'py ());

impl_transport!(
    name = PostgresPandasTransport<'tp>,
    systems = PostgresTypeSystem => PandasTypeSystem,
    route = PostgresSource => PandasDestination<'tp>,
    mappings = {
        [Float4      => F64      | f32           => f64           | conversion all]
        [Float8      => F64      | f64           => f64           | conversion all]
        [Int4        => I64      | i32           => i64           | conversion all]
        [Int8        => I64      | i64           => i64           | conversion all]
        [Bool        => Bool     | bool          => bool          | conversion all]
        [Text        => String   | &'r str       => &'r str       | conversion all]
        [BpChar      => String   | &'r str       => &'r str       | conversion none]
        [VarChar     => String   | &'r str       => &'r str       | conversion none]
        [Timestamp   => DateTime | NaiveDateTime => DateTime<Utc> | conversion half]
        [TimestampTz => DateTime | DateTime<Utc> => DateTime<Utc> | conversion all]
        [Date        => DateTime | NaiveDate     => DateTime<Utc> | conversion half]
    }
);

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

pub struct PostgresCSVPandasTransport<'py>(&'py ());

impl_transport!(
    name = PostgresCSVPandasTransport<'tp>,
    systems = PostgresTypeSystem => PandasTypeSystem,
    route = PostgresSourceCSV => PandasDestination<'tp>,
    mappings = {
        [Float4      => F64      | f32           => f64           | conversion all]
        [Float8      => F64      | f64           => f64           | conversion all]
        [Int4        => I64      | i32           => i64           | conversion all]
        [Int8        => I64      | i64           => i64           | conversion all]
        [Bool        => Bool     | bool          => bool          | conversion all]
        [Text        => String   | &'r str       => &'r str       | conversion all]
        [BpChar      => String   | &'r str       => &'r str       | conversion none]
        [VarChar     => String   | &'r str       => &'r str       | conversion none]
        [Timestamp   => DateTime | NaiveDateTime => DateTime<Utc> | conversion half]
        [TimestampTz => DateTime | DateTime<Utc> => DateTime<Utc> | conversion all]
        [Date        => DateTime | NaiveDate     => DateTime<Utc> | conversion half]
    }
);

impl<'py> TypeConversion<NaiveDateTime, DateTime<Utc>> for PostgresCSVPandasTransport<'py> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl<'py> TypeConversion<NaiveDate, DateTime<Utc>> for PostgresCSVPandasTransport<'py> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}
