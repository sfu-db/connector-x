use crate::destinations::memory::{MemoryDestination, MemoryDestinationError};
use crate::dummy_typesystem::DummyTypeSystem;
use crate::sources::postgres::{
    BinaryProtocol, CSVProtocol, PostgresSource, PostgresSourceError, PostgresTypeSystem,
};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use postgres::NoTls;
use postgres_native_tls::MakeTlsConnector;
use std::marker::PhantomData;
use thiserror::Error;
use uuid::Uuid;

pub struct PostgresMemoryTransport<P, C>(PhantomData<P>, PhantomData<C>);

#[derive(Error, Debug)]
pub enum PostgresMemoryTransportError {
    #[error(transparent)]
    PostgresSourceError(#[from] PostgresSourceError),

    #[error(transparent)]
    MemoryDestinationError(#[from] MemoryDestinationError),

    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),
}

impl_transport!(
    name = PostgresMemoryTransport<CSVProtocol, NoTls>,
    error = PostgresMemoryTransportError,
    systems = PostgresTypeSystem => DummyTypeSystem,
    route = PostgresSource<CSVProtocol, NoTls> => MemoryDestination,
    mappings = {
        { Float4[f32]                => F64[f64]                | conversion all }
        { Float8[f64]                => F64[f64]                | conversion all }
        { Int2[i16]                  => I64[i64]                | conversion all }
        { Int4[i32]                  => I64[i64]                | conversion all }
        { Int8[i64]                  => I64[i64]                | conversion all }
        { Bool[bool]                 => Bool[bool]              | conversion all  }
        { Text[&'r str]              => String[String]          | conversion half }
        { BpChar[&'r str]            => String[String]          | conversion none }
        { VarChar[&'r str]           => String[String]          | conversion none }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion half }
        { TimestampTz[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
        { UUID[Uuid]                 => String[String]          | conversion half }
        { Char[&'r str]              => String[String]          | conversion none }
        // { Time[NaiveTime]            => String[String]          | conversion half }
    }
);

impl_transport!(
    name = PostgresMemoryTransport<CSVProtocol, MakeTlsConnector>,
    error = PostgresMemoryTransportError,
    systems = PostgresTypeSystem => DummyTypeSystem,
    route = PostgresSource<CSVProtocol, MakeTlsConnector> => MemoryDestination,
    mappings = {
        { Float4[f32]                => F64[f64]                | conversion all }
        { Float8[f64]                => F64[f64]                | conversion all }
        { Int2[i16]                  => I64[i64]                | conversion all }
        { Int4[i32]                  => I64[i64]                | conversion all }
        { Int8[i64]                  => I64[i64]                | conversion all }
        { Bool[bool]                 => Bool[bool]              | conversion all  }
        { Text[&'r str]              => String[String]          | conversion half }
        { BpChar[&'r str]            => String[String]          | conversion none }
        { VarChar[&'r str]           => String[String]          | conversion none }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion half }
        { TimestampTz[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
        { UUID[Uuid]                 => String[String]          | conversion half }
        { Char[&'r str]              => String[String]          | conversion none }
        // { Time[NaiveTime]            => String[String]          | conversion half }
    }
);

impl_transport!(
    name = PostgresMemoryTransport<BinaryProtocol, NoTls>,
    error = PostgresMemoryTransportError,
    systems = PostgresTypeSystem => DummyTypeSystem,
    route = PostgresSource<BinaryProtocol, NoTls> => MemoryDestination,
    mappings = {
        { Float4[f32]                => F64[f64]                | conversion all }
        { Float8[f64]                => F64[f64]                | conversion all }
        { Int2[i16]                  => I64[i64]                | conversion all }
        { Int4[i32]                  => I64[i64]                | conversion all }
        { Int8[i64]                  => I64[i64]                | conversion all }
        { Bool[bool]                 => Bool[bool]              | conversion all  }
        { Text[&'r str]              => String[String]          | conversion half }
        { BpChar[&'r str]            => String[String]          | conversion none }
        { VarChar[&'r str]           => String[String]          | conversion none }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion half }
        { TimestampTz[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
        { UUID[Uuid]                 => String[String]          | conversion half }
        { Char[&'r str]              => String[String]          | conversion none }
        // { Time[NaiveTime]            => String[String]          | conversion half }
    }
);

impl_transport!(
    name = PostgresMemoryTransport<BinaryProtocol, MakeTlsConnector>,
    error = PostgresMemoryTransportError,
    systems = PostgresTypeSystem => DummyTypeSystem,
    route = PostgresSource<BinaryProtocol, MakeTlsConnector> => MemoryDestination,
    mappings = {
        { Float4[f32]                => F64[f64]                | conversion all }
        { Float8[f64]                => F64[f64]                | conversion all }
        { Int2[i16]                  => I64[i64]                | conversion all }
        { Int4[i32]                  => I64[i64]                | conversion all }
        { Int8[i64]                  => I64[i64]                | conversion all }
        { Bool[bool]                 => Bool[bool]              | conversion all  }
        { Text[&'r str]              => String[String]          | conversion half }
        { BpChar[&'r str]            => String[String]          | conversion none }
        { VarChar[&'r str]           => String[String]          | conversion none }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion half }
        { TimestampTz[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
        { UUID[Uuid]                 => String[String]          | conversion half }
        { Char[&'r str]              => String[String]          | conversion none }
        // { Time[NaiveTime]            => String[String]          | conversion half }
    }
);

impl<P, C> TypeConversion<Uuid, String> for PostgresMemoryTransport<P, C> {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl<P, C> TypeConversion<NaiveTime, String> for PostgresMemoryTransport<P, C> {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}

impl<'r, P, C> TypeConversion<&'r str, String> for PostgresMemoryTransport<P, C> {
    fn convert(val: &'r str) -> String {
        val.to_string()
    }
}

impl<P, C> TypeConversion<NaiveDateTime, DateTime<Utc>> for PostgresMemoryTransport<P, C> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl<P, C> TypeConversion<NaiveDate, DateTime<Utc>> for PostgresMemoryTransport<P, C> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}
