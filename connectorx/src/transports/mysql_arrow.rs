use crate::{
    destinations::arrow::{types::ArrowTypeSystem, ArrowDestination, ArrowDestinationError},
    impl_transport,
    sources::mysql::{
        BinaryProtocol, MySQLSource, MySQLSourceError, MySQLTypeSystem, TextProtocol,
    },
    typesystem::TypeConversion,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use num_traits::ToPrimitive;
use rust_decimal::Decimal;
use std::marker::PhantomData;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MySQLArrowTransportError {
    #[error(transparent)]
    MySQLSourceError(#[from] MySQLSourceError),

    #[error(transparent)]
    ArrowDestinationError(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorXError(#[from] crate::ConnectorXError),
}

pub struct MySQLArrowTransport<P>(PhantomData<P>);

impl_transport!(
    name = MySQLArrowTransport<BinaryProtocol>,
    error = MySQLArrowTransportError,
    systems = MySQLTypeSystem => ArrowTypeSystem,
    route = MySQLSource<BinaryProtocol> => ArrowDestination,
    mappings = {
        { Double[f64]                => Float64[f64]            | conversion all }
        { Long[i64]                  => Int64[i64]              | conversion all }
        { LongLong[i64]              => Int64[i64]              | conversion none }
        { Date[NaiveDate]            => Date32[NaiveDate]       | conversion all }
        { Time[NaiveTime]            => Time64[NaiveTime]       | conversion all }
        { Datetime[NaiveDateTime]    => Date64[NaiveDateTime]   | conversion all }
        { Decimal[Decimal]           => Float64[f64]            | conversion half }
        { VarChar[String]            => LargeUtf8[String]       | conversion all }
        { Char[String]               => LargeUtf8[String]       | conversion none }
    }
);

impl_transport!(
    name = MySQLArrowTransport<TextProtocol>,
    error = MySQLArrowTransportError,
    systems = MySQLTypeSystem => ArrowTypeSystem,
    route = MySQLSource<TextProtocol> => ArrowDestination,
    mappings = {
        { Double[f64]                => Float64[f64]            | conversion all }
        { Long[i64]                  => Int64[i64]              | conversion all }
        { LongLong[i64]              => Int64[i64]              | conversion none }
        { Date[NaiveDate]            => Date32[NaiveDate]       | conversion all }
        { Time[NaiveTime]            => Time64[NaiveTime]       | conversion all }
        { Datetime[NaiveDateTime]    => Date64[NaiveDateTime]   | conversion all }
        { Decimal[Decimal]           => Float64[f64]            | conversion half }
        { VarChar[String]            => LargeUtf8[String]       | conversion all }
        { Char[String]               => LargeUtf8[String]       | conversion none }
    }
);

impl<P> TypeConversion<Decimal, f64> for MySQLArrowTransport<P> {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}
