use bytes::Bytes;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use postgres::types::Type;

#[derive(Copy, Clone, Debug)]
pub enum PostgresTypeSystem {
    Bool(bool),
    Float4(bool),
    Float8(bool),
    Int4(bool),
    Int8(bool),
    Date(bool),
    BpChar(bool),
    VarChar(bool),
    Text(bool),
    Timestamp(bool),
    TimestampTz(bool),
}

impl_typesystem! {
    PostgresTypeSystem,
    [PostgresTypeSystem::Int4] => i32,
    [PostgresTypeSystem::Int8] => i64,
    [PostgresTypeSystem::Float4] => f32,
    [PostgresTypeSystem::Float8] => f64,
    [PostgresTypeSystem::Bool] => bool,
    [PostgresTypeSystem::Text] | [PostgresTypeSystem::BpChar] | [PostgresTypeSystem::VarChar] => Bytes,
    [PostgresTypeSystem::Timestamp] => NaiveDateTime,
    [PostgresTypeSystem::TimestampTz] => DateTime<Utc>,
    [PostgresTypeSystem::Date] => NaiveDate,
}

impl<'a> From<&'a Type> for PostgresTypeSystem {
    fn from(ty: &'a Type) -> PostgresTypeSystem {
        use PostgresTypeSystem::*;
        match ty.name() {
            "int4" => Int4(true),
            "int8" => Int8(true),
            "float4" => Float4(true),
            "float8" => Float8(true),
            "bool" => Bool(true),
            "text" => Text(true),
            "bpchar" => BpChar(true),
            "varchar" => VarChar(true),
            "timestamp" => TimestampTz(true),
            "timestamptz" => Timestamp(true),
            "date" => Date(true),
            ty => unimplemented!("{}", ty),
        }
    }
}

// Link PostgresDTypes back to the one defiend by the postgres crate.
impl<'a> From<PostgresTypeSystem> for Type {
    fn from(ty: PostgresTypeSystem) -> Type {
        use PostgresTypeSystem::*;
        match ty {
            Int4(_) => Type::INT4,
            Int8(_) => Type::INT8,
            Float4(_) => Type::FLOAT4,
            Float8(_) => Type::FLOAT8,
            Bool(_) => Type::BOOL,
            Text(_) => Type::TEXT,
            BpChar(_) => Type::BPCHAR,
            VarChar(_) => Type::VARCHAR,
            Timestamp(_) => Type::TIMESTAMP,
            TimestampTz(_) => Type::TIMESTAMPTZ,
            Date(_) => Type::DATE,
        }
    }
}
