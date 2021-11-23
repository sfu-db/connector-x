use chrono::{NaiveDate};
use gcp_bigquery_client::model::field_type::FieldType;

#[derive(Copy, Clone, Debug)]
pub enum BigQueryTypeSystem {
    Int64(bool),
    Date(bool),
    String(bool),
}

impl_typesystem! {
    system = BigQueryTypeSystem,
    mappings = {
        { Int64 => i64 }
        { Date => NaiveDate }
        { String => String }
    }
}

impl<'a> From<&'a FieldType> for BigQueryTypeSystem {
    fn from(ty: &'a FieldType) -> BigQueryTypeSystem {
        use BigQueryTypeSystem::*;
        match ty {
            FieldType::Int64 => Int64(true),
            FieldType::Date => Date(true),
            FieldType::String => String(true),
            _ => unimplemented!("{}", format!("{:?}", ty)),
        }
    }
}

impl<'a> From<BigQueryTypeSystem> for FieldType {
    fn from(ty: BigQueryTypeSystem) -> FieldType {
        use BigQueryTypeSystem::*;
        match ty {
            Int64(_) => FieldType::Int64,
            Date(_) => FieldType::Date,
            String(_) => FieldType::String,
        }
    }
}
