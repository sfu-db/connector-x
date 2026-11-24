use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use gcp_bigquery_client::model::field_type::FieldType;

/// Column types of the BigQuery source. The `bool` is `true` when the column is nullable.
#[derive(Copy, Clone, Debug)]
pub enum BigQueryTypeSystem {
    /// BigQuery `BOOL`, read as `bool`.
    Bool(bool),
    /// BigQuery `BOOLEAN` (legacy name for `BOOL`), read as `bool`.
    Boolean(bool),
    /// BigQuery `INT64`, read as `i64`.
    Int64(bool),
    /// BigQuery `INTEGER` (legacy name for `INT64`), read as `i64`.
    Integer(bool),
    /// BigQuery `FLOAT` (legacy name for `FLOAT64`), read as `f64`.
    Float(bool),
    /// BigQuery `FLOAT64`, read as `f64`.
    Float64(bool),
    /// BigQuery `NUMERIC`, read as `f64`, so precision beyond `f64` is lost.
    Numeric(bool),
    /// BigQuery `BIGNUMERIC`, read as `f64`, so precision beyond `f64` is lost.
    Bignumeric(bool),
    /// BigQuery `STRING`, read as `String`.
    String(bool),
    /// BigQuery `BYTES`, read as `String` (the base64 text BigQuery returns).
    Bytes(bool),
    /// BigQuery `DATE`, read as `NaiveDate`.
    Date(bool),
    /// BigQuery `DATETIME`, read as `NaiveDateTime`.
    Datetime(bool),
    /// BigQuery `TIME`, read as `NaiveTime`.
    Time(bool),
    /// BigQuery `TIMESTAMP`, read as `DateTime<Utc>`.
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

impl From<BigQueryTypeSystem> for FieldType {
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

#[cfg(test)]
mod tests {
    use super::BigQueryTypeSystem;
    use gcp_bigquery_client::model::field_type::FieldType;

    #[test]
    fn maps_bigquery_field_types_in_both_directions() {
        let fields = [
            (FieldType::Bool, "Bool"),
            (FieldType::Boolean, "Boolean"),
            (FieldType::Int64, "Int64"),
            (FieldType::Integer, "Integer"),
            (FieldType::Float, "Float"),
            (FieldType::Float64, "Float64"),
            (FieldType::Numeric, "Numeric"),
            (FieldType::Bignumeric, "Bignumeric"),
            (FieldType::String, "String"),
            (FieldType::Bytes, "Bytes"),
            (FieldType::Date, "Date"),
            (FieldType::Datetime, "Datetime"),
            (FieldType::Time, "Time"),
            (FieldType::Timestamp, "Timestamp"),
        ];
        for (field, expected) in fields {
            assert_eq!(
                format!("{:?}", BigQueryTypeSystem::from(&field)),
                format!("{expected}(true)")
            );
            let system = BigQueryTypeSystem::from(&field);
            assert_eq!(
                format!("{:?}", FieldType::from(system)),
                format!("{field:?}")
            );
        }
    }
}
