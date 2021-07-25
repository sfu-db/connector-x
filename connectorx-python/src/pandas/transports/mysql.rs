use crate::errors::ConnectorAgentPythonError;
use crate::pandas::destination::PandasDestination;
use crate::pandas::types::PandasTypeSystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use connectorx::{
    impl_transport,
    sources::mysql::{BinaryProtocol, MysqlSource, MysqlTypeSystem, TextProtocol},
    typesystem::TypeConversion,
};
use rust_decimal::prelude::*;
use std::marker::PhantomData;

pub struct MysqlPandasTransport<'py, P>(&'py (), PhantomData<P>);

impl_transport!(
    name = MysqlPandasTransport<'tp, BinaryProtocol>,
    error = ConnectorAgentPythonError,
    systems = MysqlTypeSystem => PandasTypeSystem,
    route = MysqlSource<BinaryProtocol> => PandasDestination<'tp>,
    mappings = {
        { Double[f64]                => F64[f64]                | conversion all }
        { Long[i64]                  => I64[i64]                | conversion all }
        { LongLong[i64]              => I64[i64]                | conversion none }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
        { Time[NaiveTime]            => String[String]          | conversion half }
        { Datetime[NaiveDateTime]    => DateTime[DateTime<Utc>] | conversion half }
        { Decimal[Decimal]           => F64[f64]                | conversion half }
        { VarChar[String]            => String[String]          | conversion all }
        { Char[String]               => String[String]          | conversion none }
    }
);

impl_transport!(
    name = MysqlPandasTransport<'tp, TextProtocol>,
    error = ConnectorAgentPythonError,
    systems = MysqlTypeSystem => PandasTypeSystem,
    route = MysqlSource<TextProtocol> => PandasDestination<'tp>,
    mappings = {
        { Double[f64]                => F64[f64]                | conversion all }
        { Long[i64]                  => I64[i64]                | conversion all }
        { LongLong[i64]              => I64[i64]                | conversion none }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
        { Time[NaiveTime]            => String[String]          | conversion half }
        { Datetime[NaiveDateTime]    => DateTime[DateTime<Utc>] | conversion half }
        { Decimal[Decimal]           => F64[f64]                | conversion half }
        { VarChar[String]            => String[String]          | conversion all }
        { Char[String]               => String[String]          | conversion none }
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
