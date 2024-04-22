//! Transport from Trino Source to Arrow Destination.

use crate::{
    destinations::arrowstream::{
        typesystem::ArrowTypeSystem, ArrowDestination, ArrowDestinationError,
    },
    impl_transport,
    sources::trino::{TrinoSource, TrinoSourceError, TrinoTypeSystem},
    typesystem::TypeConversion,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use num_traits::ToPrimitive;
use rust_decimal::Decimal;
use serde_json::{to_string, Value};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrinoArrowTransportError {
    #[error(transparent)]
    Source(#[from] TrinoSourceError),

    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

/// Convert Trino data types to Arrow data types.
pub struct TrinoArrowTransport();

impl_transport!(
    name = TrinoArrowTransport,
    error = TrinoArrowTransportError,
    systems = TrinoTypeSystem => ArrowTypeSystem,
    route = TrinoSource => ArrowDestination,
    mappings = {
        { Date[NaiveDate]            => Date32[NaiveDate]       | conversion auto }
        { Time[NaiveTime]            => Time64[NaiveTime]       | conversion auto }
        { Timestamp[NaiveDateTime]   => Date64[NaiveDateTime]   | conversion auto }
        { Boolean[bool]              => Boolean[bool]           | conversion auto }
        { Bigint[i32]                => Int64[i64]              | conversion auto }
        { Integer[i32]               => Int64[i64]              | conversion none }
        { Smallint[i16]              => Int64[i64]              | conversion auto }
        { Tinyint[i8]                => Int64[i64]              | conversion auto }
        { Double[f64]                => Float64[f64]            | conversion auto }
        { Real[f32]                  => Float64[f64]            | conversion auto }
        { Varchar[String]            => LargeUtf8[String]       | conversion auto }
        { Char[String]               => LargeUtf8[String]       | conversion none }
    }
);

impl TypeConversion<Decimal, f64> for TrinoArrowTransport {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}

impl TypeConversion<Value, String> for TrinoArrowTransport {
    fn convert(val: Value) -> String {
        to_string(&val).unwrap()
    }
}
