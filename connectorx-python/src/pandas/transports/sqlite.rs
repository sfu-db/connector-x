use crate::errors::ConnectorXPythonError;
use crate::pandas::destination::PandasDestination;
use crate::pandas::typesystem::PandasTypeSystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use connectorx::{
    impl_transport,
    sources::sqlite::{SQLiteSource, SQLiteTypeSystem},
    typesystem::TypeConversion,
};

#[allow(dead_code)]
pub struct SqlitePandasTransport<'py>(&'py ());

impl_transport!(
    name = SqlitePandasTransport<'tp>,
    error = ConnectorXPythonError,
    systems = SQLiteTypeSystem => PandasTypeSystem,
    route = SQLiteSource => PandasDestination<'tp>,
    mappings = {
        { Bool[bool]                 => Bool[bool]              | conversion auto }
        { Int8[i64]                  => I64[i64]                | conversion auto }
        { Int4[i32]                  => I64[i64]                | conversion auto }
        { Int2[i16]                  => I64[i64]                | conversion auto }
        { Real[f64]                  => F64[f64]                | conversion auto }
        { Text[Box<str>]             => BoxStr[Box<str>]        | conversion auto }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion option }
        { Time[NaiveTime]            => String[String]          | conversion option }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion option }
        { Blob[Vec<u8>]              => Bytes[Vec<u8>]          | conversion auto }
    }
);

impl<'py> TypeConversion<NaiveDateTime, DateTime<Utc>> for SqlitePandasTransport<'py> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_naive_utc_and_offset(val, Utc)
    }
}

impl<'py> TypeConversion<NaiveDate, DateTime<Utc>> for SqlitePandasTransport<'py> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_naive_utc_and_offset(
            val.and_hms_opt(0, 0, 0)
                .unwrap_or_else(|| panic!("and_hms_opt got None from {:?}", val)),
            Utc,
        )
    }
}

impl<'py> TypeConversion<NaiveTime, String> for SqlitePandasTransport<'py> {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}
