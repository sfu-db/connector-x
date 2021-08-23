use crate::errors::ConnectorXPythonError;
use crate::pandas::destination::PandasDestination;
use crate::pandas::typesystem::PandasTypeSystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use connectorx::{
    impl_transport,
    sources::oracle::{OracleSource, OracleTypeSystem},
    typesystem::TypeConversion,
};

pub struct OraclePandasTransport<'py>(&'py ());

impl_transport!(
    name = OraclePandasTransport<'tp>,
    error = ConnectorXPythonError,
    systems = OracleTypeSystem => PandasTypeSystem,
    route = OracleSource => PandasDestination<'tp>,
    mappings = {
        { NumFloat[f64]              => F64[f64]                | conversion auto }
        { Float[f64]                 => F64[f64]                | conversion none }
        { NumInt[i64]                => I64[i64]                | conversion auto }
        { VarChar[String]            => String[String]          | conversion auto }
        { Char[String]               => String[String]          | conversion none }
        { NVarChar[String]           => String[String]          | conversion none }
        { NChar[String]              => String[String]          | conversion none }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion option }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion option }
    }
);

impl<'py> TypeConversion<NaiveDate, DateTime<Utc>> for OraclePandasTransport<'py> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}

impl<'py> TypeConversion<NaiveDateTime, DateTime<Utc>> for OraclePandasTransport<'py> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}
