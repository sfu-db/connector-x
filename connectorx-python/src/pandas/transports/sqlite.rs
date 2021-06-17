use crate::pandas::destination::PandasDestination;
use crate::pandas::types::PandasTypeSystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use connectorx::{
    impl_transport,
    sources::sqlite::{SqliteSource, SqliteTypeSystem},
    typesystem::TypeConversion,
};

pub struct SqlitePandasTransport<'py>(&'py ());

impl_transport!(
    name = SqlitePandasTransport<'tp>,
    systems = SqliteTypeSystem => PandasTypeSystem,
    route = SqliteSource => PandasDestination<'tp>,
    mappings = {
        { Bool[bool]                 => Bool[bool]              | conversion all }
        { Integer[i64]               => I64[i64]                | conversion all }
        { Real[f64]                  => F64[f64]                | conversion all }
        { Text[Box<str>]             => BoxStr[Box<str>]        | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
        { Time[NaiveTime]            => String[String]          | conversion half }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion half }
    }
);

impl<'py> TypeConversion<NaiveDateTime, DateTime<Utc>> for SqlitePandasTransport<'py> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl<'py> TypeConversion<NaiveDate, DateTime<Utc>> for SqlitePandasTransport<'py> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}

impl<'py> TypeConversion<NaiveTime, String> for SqlitePandasTransport<'py> {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}
