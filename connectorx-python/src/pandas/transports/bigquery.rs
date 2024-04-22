use crate::errors::ConnectorXPythonError;
use crate::pandas::destination::PandasDestination;
use crate::pandas::typesystem::PandasTypeSystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use connectorx::{
    impl_transport,
    sources::bigquery::{BigQuerySource, BigQueryTypeSystem},
    typesystem::TypeConversion,
};

#[allow(dead_code)]
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
        { Numeric[f64]               => F64[f64]                | conversion none }
        { Bignumeric[f64]            => F64[f64]                | conversion none }
        { String[String]             => String[String]          | conversion auto }
        { Bytes[String]              => String[String]          | conversion none }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion option }
        { Datetime[NaiveDateTime]    => DateTime[DateTime<Utc>] | conversion option }
        { Time[NaiveTime]            => String[String]          | conversion option }
        { Timestamp[DateTime<Utc>]   => DateTime[DateTime<Utc>] | conversion auto }
    }
);

impl<'py> TypeConversion<NaiveDate, DateTime<Utc>> for BigQueryPandasTransport<'py> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_naive_utc_and_offset(
            val.and_hms_opt(0, 0, 0)
                .unwrap_or_else(|| panic!("and_hms_opt got None from {:?}", val)),
            Utc,
        )
    }
}

impl<'py> TypeConversion<NaiveDateTime, DateTime<Utc>> for BigQueryPandasTransport<'py> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_naive_utc_and_offset(val, Utc)
    }
}

impl<'py> TypeConversion<NaiveTime, String> for BigQueryPandasTransport<'py> {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}
