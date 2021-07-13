use crate::destinations::arrow::{types::ArrowTypeSystem, ArrowDestination};
use crate::sources::postgres::{Binary, PostgresSource, PostgresTypeSystem};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use std::marker::PhantomData;
use uuid::Uuid;

pub struct PostgresArrowTransport<P>(PhantomData<P>);

impl_transport!(
    name = PostgresArrowTransport<Binary>,
    systems = PostgresTypeSystem => ArrowTypeSystem,
    route = PostgresSource<Binary> => ArrowDestination,
    mappings = {
        { Float4[f32]                => Float32[f32]            | conversion all }
        { Float8[f64]                => Float64[f64]            | conversion all }
        { Int2[i16]                  => Int32[i32]              | conversion all }
        { Int4[i32]                  => Int32[i32]              | conversion all }
        { Int8[i64]                  => Int64[i64]              | conversion all }
        { Bool[bool]                 => Boolean[bool]           | conversion all  }
        { Text[&'r str]              => LargeUtf8[String]       | conversion half }
        { BpChar[&'r str]            => LargeUtf8[String]       | conversion none }
        { VarChar[&'r str]           => LargeUtf8[String]       | conversion none }
        { Timestamp[NaiveDateTime]   => Date64[NaiveDateTime]   | conversion all }
        { Date[NaiveDate]            => Date32[NaiveDate]       | conversion all }
        { UUID[Uuid]                 => LargeUtf8[String]       | conversion half }
        { Char[&'r str]              => LargeUtf8[String]       | conversion none }
        // { Time[NaiveTime]            => String[String]       | conversion half }
    }
);

impl<P> TypeConversion<Uuid, String> for PostgresArrowTransport<P> {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl<P> TypeConversion<NaiveTime, String> for PostgresArrowTransport<P> {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}

impl<'r, P> TypeConversion<&'r str, String> for PostgresArrowTransport<P> {
    fn convert(val: &'r str) -> String {
        val.to_string()
    }
}

impl<P> TypeConversion<NaiveDateTime, DateTime<Utc>> for PostgresArrowTransport<P> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl<P> TypeConversion<NaiveDate, DateTime<Utc>> for PostgresArrowTransport<P> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}
