use crate::{
    destinations::arrow::{typesystem::ArrowTypeSystem, ArrowDestination, ArrowDestinationError},
    impl_transport,
    sources::oracle::{OracleSource, OracleSourceError, OracleTypeSystem},
    typesystem::TypeConversion,
};
// use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OracleArrowTransportError {
    #[error(transparent)]
    Source(#[from] OracleSourceError),

    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

pub struct OracleArrowTransport;

impl_transport!(
    name = OracleArrowTransport,
    error = OracleArrowTransportError,
    systems = OracleTypeSystem => ArrowTypeSystem,
    route = OracleSource => ArrowDestination,
    mappings = {
        { Float[f64]                => Float64[f64]            | conversion auto }
        { Int[i64]                  => Int64[i64]              | conversion auto }
        { VarChar[String]           => LargeUtf8[String]       | conversion auto }
    }
);
