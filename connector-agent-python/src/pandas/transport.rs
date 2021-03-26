use super::destination::PandasDestination;
use super::types::PandasTypeSystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use connector_agent::{
    impl_transport,
    sources::postgres::{PostgresBinarySource, PostgresCSVSource, PostgresTypeSystem},
    typesystem::TypeConversion,
};

pub struct PostgresPandasTransport<'py>(&'py ());

impl_transport!(
    name = PostgresPandasTransport<'tp>,
    systems = PostgresTypeSystem => PandasTypeSystem,
    route = PostgresBinarySource => PandasDestination<'tp>,
    mappings = {
        { Float4[f32]                => F64[f64]                | conversion all }
        { Float8[f64]                => F64[f64]                | conversion all }
        { Int4[i32]                  => I64[i64]                | conversion all }
        { Int8[i64]                  => I64[i64]                | conversion all }
        { Bool[bool]                 => Bool[bool]              | conversion all }
        { Text[&'r str]              => String[&'r str]         | conversion all }
        { BpChar[&'r str]            => String[&'r str]         | conversion none }
        { VarChar[&'r str]           => String[&'r str]         | conversion none }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion half }
        { TimestampTz[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
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
    route = PostgresCSVSource => PandasDestination<'tp>,
    mappings = {
        { Float4[f32]                => F64[f64]                | conversion all }
        { Float8[f64]                => F64[f64]                | conversion all }
        { Int4[i32]                  => I64[i64]                | conversion all }
        { Int8[i64]                  => I64[i64]                | conversion all }
        { Bool[bool]                 => Bool[bool]              | conversion all }
        { Text[&'r str]              => String[&'r str]         | conversion all }
        { BpChar[&'r str]            => String[&'r str]         | conversion none }
        { VarChar[&'r str]           => String[&'r str]         | conversion none }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion half }
        { TimestampTz[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
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
