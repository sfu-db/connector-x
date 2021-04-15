use super::destination::PandasDestination;
use super::types::PandasTypeSystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use connectorx::{
    impl_transport,
    sources::postgres::{Binary, PostgresSource, PostgresTypeSystem, CSV},
    typesystem::TypeConversion,
};
use rust_decimal::prelude::*;
use std::marker::PhantomData;

pub struct PostgresPandasTransport<'py, P>(&'py (), PhantomData<P>);

impl_transport!(
    name = PostgresPandasTransport<'tp, Binary>,
    systems = PostgresTypeSystem => PandasTypeSystem,
    route = PostgresSource<Binary> => PandasDestination<'tp>,
    mappings = {
        { Float4[f32]                => F64[f64]                | conversion all }
        { Float8[f64]                => F64[f64]                | conversion all }
        { Numeric[Decimal]           => F64[f64]                | conversion half }
        { Int2[i16]                  => I64[i64]                | conversion all }
        { Int4[i32]                  => I64[i64]                | conversion all }
        { Int8[i64]                  => I64[i64]                | conversion all }
        { Bool[bool]                 => Bool[bool]              | conversion all }
        { Text[&'r str]              => String[&'r str]         | conversion all }
        { BpChar[&'r str]            => String[&'r str]         | conversion none }
        { VarChar[&'r str]           => String[&'r str]         | conversion none }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion half }
        { TimestampTz[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
        { Char[&'r str]              => String[&'r str]         | conversion none }
    }
);

impl_transport!(
    name = PostgresPandasTransport<'tp, CSV>,
    systems = PostgresTypeSystem => PandasTypeSystem,
    route = PostgresSource<CSV> => PandasDestination<'tp>,
    mappings = {
        { Float4[f32]                => F64[f64]                | conversion all }
        { Float8[f64]                => F64[f64]                | conversion all }
        { Numeric[Decimal]           => F64[f64]                | conversion half }
        { Int2[i16]                  => I64[i64]                | conversion all }
        { Int4[i32]                  => I64[i64]                | conversion all }
        { Int8[i64]                  => I64[i64]                | conversion all }
        { Bool[bool]                 => Bool[bool]              | conversion all }
        { Text[&'r str]              => String[&'r str]         | conversion all }
        { BpChar[&'r str]            => String[&'r str]         | conversion none }
        { VarChar[&'r str]           => String[&'r str]         | conversion none }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion half }
        { TimestampTz[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
        { Char[&'r str]              => String[&'r str]         | conversion none }
    }
);

impl<'py, P> TypeConversion<Decimal, f64> for PostgresPandasTransport<'py, P> {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}

impl<'py, P> TypeConversion<NaiveDateTime, DateTime<Utc>> for PostgresPandasTransport<'py, P> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl<'py, P> TypeConversion<NaiveDate, DateTime<Utc>> for PostgresPandasTransport<'py, P> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}
