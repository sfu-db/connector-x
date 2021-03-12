use super::types::PandasTypes;
use super::writers::PandasWriter;
use bytes::Bytes;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use connector_agent::{
    create_transport,
    data_sources::postgres::{PostgresDTypes, PostgresSource},
    typesystem::TypeConversion,
};

pub struct PostgresPandasTransport<'py>(&'py ());

create_transport! {
    ['py],
    PostgresPandasTransport<'py>,
    PostgresDTypes => PandasTypes,
    PostgresSource => PandasWriter<'py>,
    ([PostgresDTypes::Float4], [PandasTypes::F64]) => (f32, f64) conversion all,
    ([PostgresDTypes::Float8], [PandasTypes::F64]) => (f64, f64) conversion all,
    ([PostgresDTypes::Int4], [PandasTypes::I64]) => (i32, i64) conversion all,
    ([PostgresDTypes::Int8], [PandasTypes::I64]) => (i64, i64) conversion all,
    ([PostgresDTypes::Bool], [PandasTypes::Bool]) => (bool, bool) conversion all,
    ([PostgresDTypes::Text], [PandasTypes::String]) | ([PostgresDTypes::BpChar], [PandasTypes::String]) | ([PostgresDTypes::VarChar], [PandasTypes::String]) => (Bytes, Bytes) conversion all,
    ([PostgresDTypes::Timestamp], [PandasTypes::DateTime]) => (NaiveDateTime, DateTime<Utc>) conversion half,
    ([PostgresDTypes::TimestampTz], [PandasTypes::DateTime]) => (DateTime<Utc>, DateTime<Utc>) conversion all,
    ([PostgresDTypes::Date], [PandasTypes::DateTime]) => (NaiveDate, DateTime<Utc>) conversion half,
}

impl<'py> TypeConversion<NaiveDateTime, DateTime<Utc>> for PostgresPandasTransport<'py> {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl<'py> TypeConversion<NaiveDate, DateTime<Utc>> for PostgresPandasTransport<'py> {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}
