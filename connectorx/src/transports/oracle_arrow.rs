use crate::{
    destinations::arrow::{typesystem::ArrowTypeSystem, ArrowDestination, ArrowDestinationError},
    impl_transport,
    sources::oracle::{OracleSource, OracleSourceError, OracleTypeSystem, TextProtocol},
    typesystem::TypeConversion,
};
// use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::marker::PhantomData;
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

pub struct OracleArrowTransport<P>(PhantomData<P>);

impl_transport!(
    name = OracleArrowTransport<TextProtocol>,
    error = OracleArrowTransportError,
    systems = OracleTypeSystem => ArrowTypeSystem,
    route = OracleSource<TextProtocol> => ArrowDestination,
    mappings = {
        { Float[f64]                => Float64[f64]            | conversion auto }
        { Int[i64]                  => Int64[i64]              | conversion auto }
        { VarChar[String]           => LargeUtf8[String]       | conversion auto }
    }
);
