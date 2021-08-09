use crate::errors::ConnectorXPythonError;
use crate::pandas::destination::PandasDestination;
use crate::pandas::typesystem::PandasTypeSystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use connectorx::{
    impl_transport,
    sources::mysql::{BinaryProtocol, MySQLSource, MySQLTypeSystem, TextProtocol},
    typesystem::TypeConversion,
};
use rust_decimal::prelude::*;
use std::marker::PhantomData;
use serde_json::{to_string, Value};

pub struct MysqlPandasTransport<'py, P>(&'py (), PhantomData<P>);

impl_transport!(
    name = MysqlPandasTransport<'tp, BinaryProtocol>,
    error = ConnectorXPythonError,
    systems = MySQLTypeSystem => PandasTypeSystem,
    route = MySQLSource<BinaryProtocol> => PandasDestination<'tp>,
    mappings = {
        { Float[f32]                 => F64[f64]                | conversion auto }
        { Double[f64]                => F64[f64]                | conversion auto }
        { Long[i64]                  => I64[i64]                | conversion auto }
        { LongLong[i64]              => I64[i64]                | conversion none }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion option }
        { Time[NaiveTime]            => String[String]          | conversion option }
        { Datetime[NaiveDateTime]    => DateTime[DateTime<Utc>] | conversion option }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion option }
        { Decimal[Decimal]           => F64[f64]                | conversion option }
        { VarChar[String]            => String[String]          | conversion auto }
        { Char[String]               => String[String]          | conversion none }
        { Enum[&'r str]              => Str[&'r str]            | conversion none }
        { TinyBlob[Vec<u8>]          => Bytes[Vec<u8>]          | conversion auto }
        { Blob[Vec<u8>]              => Bytes[Vec<u8>]          | conversion auto }
        { MediumBlob[Vec<u8>]        => Bytes[Vec<u8>]          | conversion auto }
        { LongBlob[Vec<u8>]          => Bytes[Vec<u8>]          | conversion auto }
    }
);

impl_transport!(
    name = MysqlPandasTransport<'tp, TextProtocol>,
    error = ConnectorXPythonError,
    systems = MySQLTypeSystem => PandasTypeSystem,
    route = MySQLSource<TextProtocol> => PandasDestination<'tp>,
    mappings = {
        { Float[f32]                 => F64[f64]                | conversion auto }
        { Double[f64]                => F64[f64]                | conversion auto }
        { Long[i64]                  => I64[i64]                | conversion auto }
        { LongLong[i64]              => I64[i64]                | conversion none }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion option }
        { Time[NaiveTime]            => String[String]          | conversion option }
        { Datetime[NaiveDateTime]    => DateTime[DateTime<Utc>] | conversion option }
        { Timestamp[NaiveDateTime]   => DateTime[DateTime<Utc>] | conversion option }
        { Decimal[Decimal]           => F64[f64]                | conversion option }
        { VarChar[String]            => String[String]          | conversion auto }
        { Char[String]               => String[String]          | conversion none }
        { Enum[&'r str]              => Str[&'r str]            | conversion none }
        { TinyBlob[Vec<u8>]          => Bytes[Vec<u8>]          | conversion auto }
        { Blob[Vec<u8>]              => Bytes[Vec<u8>]          | conversion auto }
        { MediumBlob[Vec<u8>]        => Bytes[Vec<u8>]          | conversion auto }
        { LongBlob[Vec<u8>]          => Bytes[Vec<u8>]          | conversion auto }
    }
);

impl<'py, P> TypeConversion<NaiveDate, DateTime<Utc>> for MysqlPandasTransport<'py, P> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}

impl<'py, P> TypeConversion<NaiveTime, String> for MysqlPandasTransport<'py, P> {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}

impl<'py, P> TypeConversion<NaiveDateTime, DateTime<Utc>> for MysqlPandasTransport<'py, P> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl<'py, P> TypeConversion<Decimal, f64> for MysqlPandasTransport<'py, P> {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}
