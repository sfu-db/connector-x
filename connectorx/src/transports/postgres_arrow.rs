use crate::destinations::arrow::{
    typesystem::ArrowTypeSystem, ArrowDestination, ArrowDestinationError,
};
use crate::sources::postgres::{
    BinaryProtocol, CSVProtocol, CursorProtocol, PostgresSource, PostgresSourceError,
    PostgresTypeSystem,
};
use crate::typesystem::TypeConversion;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use num_traits::ToPrimitive;
use postgres::NoTls;
use postgres_native_tls::MakeTlsConnector;
use rust_decimal::Decimal;
use std::marker::PhantomData;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum PostgresArrowTransportError {
    #[error(transparent)]
    PostgresSourceError(#[from] PostgresSourceError),

    #[error(transparent)]
    ArrowDestinationError(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),
}

pub struct PostgresArrowTransport<P, C>(PhantomData<P>, PhantomData<C>);

macro_rules! impl_postgres_transport {
    ($proto:ty, $tls:ty) => {
        impl_transport!(
            name = PostgresArrowTransport<$proto, $tls>,
            error = PostgresArrowTransportError,
            systems = PostgresTypeSystem => ArrowTypeSystem,
            route = PostgresSource<$proto, $tls> => ArrowDestination,
            mappings = {
                { Float4[f32]                => Float32[f32]            | conversion all }
                { Float8[f64]                => Float64[f64]            | conversion all }
                { Numeric[Decimal]           => Float64[f64]            | conversion half }
                { Int2[i16]                  => Int32[i32]              | conversion all }
                { Int4[i32]                  => Int32[i32]              | conversion all }
                { Int8[i64]                  => Int64[i64]              | conversion all }
                { Bool[bool]                 => Boolean[bool]           | conversion all  }
                { Text[&'r str]              => LargeUtf8[String]       | conversion half }
                { BpChar[&'r str]            => LargeUtf8[String]       | conversion none }
                { VarChar[&'r str]           => LargeUtf8[String]       | conversion none }
                { Timestamp[NaiveDateTime]   => Date64[NaiveDateTime]   | conversion all }
                { Date[NaiveDate]            => Date32[NaiveDate]       | conversion all }
                { Time[NaiveTime]            => Time64[NaiveTime]       | conversion all }
                { UUID[Uuid]                 => LargeUtf8[String]       | conversion half }
                { Char[&'r str]              => LargeUtf8[String]       | conversion none }
            }
        );
    }
}

impl_postgres_transport!(BinaryProtocol, NoTls);
impl_postgres_transport!(BinaryProtocol, MakeTlsConnector);
impl_postgres_transport!(CSVProtocol, NoTls);
impl_postgres_transport!(CSVProtocol, MakeTlsConnector);
impl_postgres_transport!(CursorProtocol, NoTls);
impl_postgres_transport!(CursorProtocol, MakeTlsConnector);

impl<P, C> TypeConversion<Uuid, String> for PostgresArrowTransport<P, C> {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl<'r, P, C> TypeConversion<&'r str, String> for PostgresArrowTransport<P, C> {
    fn convert(val: &'r str) -> String {
        val.to_string()
    }
}

impl<P, C> TypeConversion<Decimal, f64> for PostgresArrowTransport<P, C> {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}
