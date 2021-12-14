use crate::errors::ConnectorXPythonError;
use crate::pandas::destination::PandasDestination;
use crate::pandas::typesystem::PandasTypeSystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use connectorx::{
    impl_transport,
    sources::bigquery::{BigQuerySource, BigQueryTypeSystem},
    typesystem::TypeConversion,
};

pub struct BigQueryPandasTransport<'py>(&'py ());

impl_transport!(
    name = BigQueryPandasTransport<'tp>,
    error = ConnectorXPythonError,
    systems = BigQueryTypeSystem => PandasTypeSystem,
    route = BigQuerySource => PandasDestination<'tp>,
    mappings = {
        { Bool[bool]                 => Bool[bool]              | conversion auto }
        { Boolean[bool]              => Bool[bool]              | conversion none }
        { Int64[i64]                 => I64[i64]                | conversion auto }
        { Integer[i64]               => I64[i64]                | conversion none }
        { Float64[f64]               => F64[f64]                | conversion auto }
        { Float[f64]                 => F64[f64]                | conversion none }
        { String[String]             => String[String]          | conversion auto }
        // { Bytes[Vec<u8>]             => Bytes[Vec<u8>]          | conversion auto }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion option }
        { Datetime[NaiveDateTime]    => DateTime[DateTime<Utc>] | conversion option }
        { Time[NaiveTime]            => String[String]          | conversion option }
        // { Timestamp[DateTime<Utc>]   => DateTime[DateTime<Utc>] | conversion auto }
    }
);

impl<'py> TypeConversion<NaiveDate, DateTime<Utc>> for BigQueryPandasTransport<'py> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}

impl<'py> TypeConversion<NaiveDateTime, DateTime<Utc>> for BigQueryPandasTransport<'py> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl<'py> TypeConversion<NaiveTime, String> for BigQueryPandasTransport<'py> {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}
