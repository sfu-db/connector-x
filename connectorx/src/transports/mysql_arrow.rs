use crate::{
    destinations::arrow::{types::ArrowTypeSystem, ArrowDestination},
    impl_transport,
    sources::mysql::{BinaryProtocol, MysqlSource, MysqlTypeSystem, TextProtocol},
    typesystem::TypeConversion,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use num_traits::ToPrimitive;
use rust_decimal::Decimal;
use std::marker::PhantomData;

pub struct MysqlArrowTransport<P>(PhantomData<P>);

impl_transport!(
    name = MysqlArrowTransport<BinaryProtocol>,
    systems = MysqlTypeSystem => ArrowTypeSystem,
    route = MysqlSource<BinaryProtocol> => ArrowDestination,
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
    name = MysqlArrowTransport<TextProtocol>,
    systems = MysqlTypeSystem => ArrowTypeSystem,
    route = MysqlSource<TextProtocol> => ArrowDestination,
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

impl<P> TypeConversion<Decimal, f64> for MysqlArrowTransport<P> {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}
