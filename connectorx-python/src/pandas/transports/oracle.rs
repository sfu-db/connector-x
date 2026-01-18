use crate::errors::ConnectorXPythonError;
use crate::pandas::destination::PandasDestination;
use crate::pandas::typesystem::PandasTypeSystem;
use chrono::{DateTime, NaiveDateTime, Utc};
use rust_decimal::prelude::*;
use connectorx::{
    impl_transport,
    sources::oracle::{OracleSource, OracleTypeSystem},
    typesystem::TypeConversion,
};

#[allow(dead_code)]
pub struct OraclePandasTransport<'py>(&'py ());

impl_transport!(
    name = OraclePandasTransport<'tp>,
    error = ConnectorXPythonError,
    systems = OracleTypeSystem => PandasTypeSystem,
    route = OracleSource => PandasDestination<'tp>,
    mappings = {
        { NumFloat[f64]                     => F64[f64]                     | conversion auto }
        { Float[f64]                        => F64[f64]                     | conversion none }
        { BinaryFloat[f64]                  => F64[f64]                     | conversion none }
        { BinaryDouble[f64]                 => F64[f64]                     | conversion none }
        { NumInt[i64]                       => I64[i64]                     | conversion auto }
        { Blob[Vec<u8>]                     => Bytes[Vec<u8>]               | conversion auto }
        { Clob[String]                      => String[String]               | conversion none }
        { VarChar[String]                   => String[String]               | conversion auto }
        { Char[String]                      => String[String]               | conversion none }
        { NVarChar[String]                  => String[String]               | conversion none }
        { NChar[String]                     => String[String]               | conversion none }
        { Date[NaiveDateTime]               => DateTime[DateTime<Utc>]      | conversion option }
        { Timestamp[NaiveDateTime]          => DateTime[DateTime<Utc>]      | conversion none }
        { TimestampNano[NaiveDateTime]      => DateTime[DateTime<Utc>]      | conversion none }
        { TimestampTz[DateTime<Utc>]        => DateTime[DateTime<Utc>]      | conversion auto }
        { TimestampTzNano[DateTime<Utc>]    => DateTime[DateTime<Utc>]      | conversion none }
        { NumDecimal[Decimal]               => F64[f64]                     | conversion option }
    }
);

impl<'py> TypeConversion<NaiveDateTime, DateTime<Utc>> for OraclePandasTransport<'py> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_naive_utc_and_offset(val, Utc)
    }
}

impl<'py> TypeConversion<Decimal, f64> for OraclePandasTransport<'py> {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}