//! Transport from Postgres Source to Arrow Destination.

use crate::destinations::arrowstream::{
    typesystem::ArrowTypeSystem, ArrowDestination, ArrowDestinationError,
};
use crate::sources::postgres::{
    BinaryProtocol, CSVProtocol, CursorProtocol, PostgresSource, PostgresSourceError,
    PostgresTypeSystem, SimpleProtocol,
};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use cidr_02::IpInet;
use pgvector::{Bit, HalfVector, SparseVector, Vector};
use postgres::NoTls;
use postgres_openssl::MakeTlsConnector;
use rust_decimal::Decimal;
use serde_json::Value;
use std::marker::PhantomData;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum PostgresArrowTransportError {
    #[error(transparent)]
    Source(#[from] PostgresSourceError),

    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

/// Convert Postgres data types to Arrow data types.
pub struct PostgresArrowTransport<P, C>(PhantomData<P>, PhantomData<C>);

macro_rules! impl_postgres_transport {
    ($proto:ty, $tls:ty) => {
        impl_transport!(
            name = PostgresArrowTransport<$proto, $tls>,
            error = PostgresArrowTransportError,
            systems = PostgresTypeSystem => ArrowTypeSystem,
            route = PostgresSource<$proto, $tls> => ArrowDestination,
            mappings = {
                { Float4[f32]                => Float64[f64]                           | conversion auto   }
                { Float8[f64]                => Float64[f64]                           | conversion auto   }
                { Numeric[Decimal]           => Decimal[Decimal]                       | conversion auto   }
                { Int2[i16]                  => Int64[i64]                             | conversion auto   }
                { Int4[i32]                  => Int64[i64]                             | conversion auto   }
                { Int8[i64]                  => Int64[i64]                             | conversion auto   }
                { Bool[bool]                 => Boolean[bool]                          | conversion auto   }
                { Text[&'r str]              => LargeUtf8[String]                      | conversion owned  }
                { BpChar[&'r str]            => LargeUtf8[String]                      | conversion none   }
                { VarChar[&'r str]           => LargeUtf8[String]                      | conversion none   }
                { Name[&'r str]              => LargeUtf8[String]                      | conversion none   }
                { Timestamp[NaiveDateTime]   => Date64[NaiveDateTime]                  | conversion auto   }
                { Date[NaiveDate]            => Date32[NaiveDate]                      | conversion auto   }
                { Time[NaiveTime]            => Time64[NaiveTime]                      | conversion auto   }
                { TimestampTz[DateTime<Utc>] => DateTimeTz[DateTime<Utc>]              | conversion auto   }
                { UUID[Uuid]                 => LargeUtf8[String]                      | conversion option }
                { Char[&'r str]              => LargeUtf8[String]                      | conversion none   }
                { ByteA[Vec<u8>]             => LargeBinary[Vec<u8>]                   | conversion auto   }
                { JSON[Value]                => LargeUtf8[String]                      | conversion option }
                { JSONB[Value]               => LargeUtf8[String]                      | conversion none   }
                { Inet[IpInet]               => LargeUtf8[String]                      | conversion none   }
                { Vector[Vector]             => Float32Array[Vec<Option<f32>>]         | conversion none   }
                { HalfVec[HalfVector]        => Float32Array[Vec<Option<f32>>]         | conversion none   }
                { Bit[Bit]                   => LargeBinary[Vec<u8>]                   | conversion none   }
                { SparseVec[SparseVector]    => Float32Array[Vec<Option<f32>>]         | conversion none   }
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
impl_postgres_transport!(SimpleProtocol, NoTls);
impl_postgres_transport!(SimpleProtocol, MakeTlsConnector);

impl<P, C> TypeConversion<IpInet, String> for PostgresArrowTransport<P, C> {
    fn convert(val: IpInet) -> String {
        val.to_string()
    }
}

impl<P, C> TypeConversion<Option<IpInet>, Option<String>> for PostgresArrowTransport<P, C> {
    fn convert(val: Option<IpInet>) -> Option<String> {
        val.map(|val| val.to_string())
    }
}

impl<P, C> TypeConversion<Uuid, String> for PostgresArrowTransport<P, C> {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl<P, C> TypeConversion<Value, String> for PostgresArrowTransport<P, C> {
    fn convert(val: Value) -> String {
        val.to_string()
    }
}

impl<P, C> TypeConversion<Vector, Vec<Option<f32>>> for PostgresArrowTransport<P, C> {
    fn convert(val: Vector) -> Vec<Option<f32>> {
        val.to_vec().into_iter().map(Some).collect()
    }
}

impl<P, C> TypeConversion<Option<Vector>, Option<Vec<Option<f32>>>>
    for PostgresArrowTransport<P, C>
{
    fn convert(val: Option<Vector>) -> Option<Vec<Option<f32>>> {
        val.map(|val| val.to_vec().into_iter().map(Some).collect())
    }
}

impl<P, C> TypeConversion<HalfVector, Vec<Option<f32>>> for PostgresArrowTransport<P, C> {
    fn convert(val: HalfVector) -> Vec<Option<f32>> {
        val.to_vec().into_iter().map(|v| Some(v.to_f32())).collect()
    }
}

impl<P, C> TypeConversion<Option<HalfVector>, Option<Vec<Option<f32>>>>
    for PostgresArrowTransport<P, C>
{
    fn convert(val: Option<HalfVector>) -> Option<Vec<Option<f32>>> {
        val.map(|val| val.to_vec().into_iter().map(|v| Some(v.to_f32())).collect())
    }
}

impl<P, C> TypeConversion<Bit, Vec<u8>> for PostgresArrowTransport<P, C> {
    fn convert(val: Bit) -> Vec<u8> {
        val.as_bytes().into()
    }
}

impl<P, C> TypeConversion<Option<Bit>, Option<Vec<u8>>> for PostgresArrowTransport<P, C> {
    fn convert(val: Option<Bit>) -> Option<Vec<u8>> {
        val.map(|val| val.as_bytes().into())
    }
}

impl<P, C> TypeConversion<SparseVector, Vec<Option<f32>>> for PostgresArrowTransport<P, C> {
    fn convert(val: SparseVector) -> Vec<Option<f32>> {
        val.to_vec().into_iter().map(Some).collect()
    }
}

impl<P, C> TypeConversion<Option<SparseVector>, Option<Vec<Option<f32>>>>
    for PostgresArrowTransport<P, C>
{
    fn convert(val: Option<SparseVector>) -> Option<Vec<Option<f32>>> {
        val.map(|val| val.to_vec().into_iter().map(Some).collect())
    }
}
