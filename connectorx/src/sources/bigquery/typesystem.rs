use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use gcp_bigquery_client::model::field_type::FieldType;

#[derive(Copy, Clone, Debug)]
pub enum BigQueryTypeSystem {
    Bool(bool),
    Boolean(bool),
    Int64(bool),
    Integer(bool),
    Float(bool),
    Float64(bool),
    Numeric(bool),
    Bignumeric(bool),
    String(bool),
    Bytes(bool),
    Date(bool),
    Datetime(bool),
    Time(bool),
    Timestamp(bool),
}

impl_typesystem! {
    system = BigQueryTypeSystem,
    mappings = {
        { Bool | Boolean => bool }
        { Int64 | Integer  =>  i64 }
        { Float64 | Float | Numeric | Bignumeric  =>  f64 }
        { String | Bytes  =>  String }
        { Date => NaiveDate }
        { Datetime => NaiveDateTime }
        { Time => NaiveTime }
        { Timestamp => DateTime<Utc> }
    }
}

impl<'a> From<&'a FieldType> for BigQueryTypeSystem {
    fn from(ty: &'a FieldType) -> BigQueryTypeSystem {
        use BigQueryTypeSystem::*;
        match ty {
            FieldType::Bool => Bool(true),
            FieldType::Boolean => Boolean(true),
            FieldType::Int64 => Int64(true),
            FieldType::Integer => Integer(true),
            FieldType::Float => Float(true),
            FieldType::Float64 => Float64(true),
            FieldType::Numeric => Numeric(true),
            FieldType::Bignumeric => Bignumeric(true),
            FieldType::String => String(true),
            FieldType::Bytes => Bytes(true),
            FieldType::Date => Date(true),
            FieldType::Datetime => Datetime(true),
            FieldType::Time => Time(true),
            FieldType::Timestamp => Timestamp(true),
            _ => unimplemented!("{}", format!("{:?}", ty)),
        }
    }
}

impl<'a> From<BigQueryTypeSystem> for FieldType {
    fn from(ty: BigQueryTypeSystem) -> FieldType {
        use BigQueryTypeSystem::*;
        match ty {
            Bool(_) => FieldType::Bool,
            Boolean(_) => FieldType::Boolean,
            Int64(_) => FieldType::Int64,
            Integer(_) => FieldType::Integer,
            Float64(_) => FieldType::Float64,
            Float(_) => FieldType::Float,
            Numeric(_) => FieldType::Numeric,
            Bignumeric(_) => FieldType::Bignumeric,
            String(_) => FieldType::String,
            Bytes(_) => FieldType::Bytes,
            Date(_) => FieldType::Date,
            Datetime(_) => FieldType::Datetime,
            Time(_) => FieldType::Time,
            Timestamp(_) => FieldType::Timestamp,
        }
    }
}
