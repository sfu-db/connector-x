use super::destination::PandasDestination;
use super::types::PandasTypeSystem;
use connectorx::{
    impl_transport,
    sources::mysql::{MysqlSource, MysqlTypeSystem},
    typesystem::TypeConversion,
};
use rust_decimal::prelude::*;
// use uuid::Uuid;
use chrono::{DateTime, NaiveDate, NaiveTime, NaiveDateTime, Utc};

pub struct MysqlPandasTransport<'py>(&'py ());

impl_transport!(
    name = MysqlPandasTransport<'tp>,
    systems = MysqlTypeSystem => PandasTypeSystem,
    route = MysqlSource => PandasDestination<'tp>,
    mappings = {
        { Double[f64]                => F64[f64]                | conversion all }
        { Long[i64]                  => I64[i64]                | conversion all }
        { Date[NaiveDate]            => DateTime[DateTime<Utc>] | conversion half }
        { Time[NaiveTime]            => String[String]          | conversion half }
        { Datetime[NaiveDateTime]    => DateTime[DateTime<Utc>] | conversion half }
        { Decimal[Decimal]           => F64[f64]                | conversion half }
        { VarChar[String]            => String[String]          | conversion all }
        { Char[String]               => String[String]          | conversion none }
    }
);

impl<'py> TypeConversion<NaiveDate, DateTime<Utc>> for MysqlPandasTransport<'py> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}

impl<'py> TypeConversion<NaiveTime, String> for MysqlPandasTransport<'py> {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}

impl<'py> TypeConversion<NaiveDateTime, DateTime<Utc>> for MysqlPandasTransport<'py> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl<'py> TypeConversion<Decimal, f64> for MysqlPandasTransport<'py> {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}

