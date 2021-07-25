use crate::errors::ConnectorAgentPythonError;
use crate::pandas::destination::PandasDestination;
use crate::pandas::types::PandasTypeSystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use connectorx::{
    impl_transport,
    sources::sqlite::{SQLiteSource, SQLiteTypeSystem},
    typesystem::TypeConversion,
};

pub struct SqlitePandasTransport<'py>(&'py ());

impl_transport!(
    name = SqlitePandasTransport<'tp>,
    error = ConnectorAgentPythonError,
    systems = SQLiteTypeSystem => PandasTypeSystem,
    route = SQLiteSource => PandasDestination<'tp>,
    mappings = {
        { Bool[bool]                 => Bool[bool]              | conversion all }
        { Int8[i64]                  => I64[i64]                | conversion all }
        { Int4[i32]                  => I64[i64]                | conversion all }
        { Int2[i16]                  => I64[i64]                | conversion all }
        { Real[f64]                  => F64[f64]                | conversion all }
        { Text[Box<str>]             => BoxStr[Box<str>]        | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
        { Time[NaiveTime]            => String[String]          | conversion half }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion half }
        { Blob[Vec<u8>]              => Bytes[Vec<u8>]          | conversion all }
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
