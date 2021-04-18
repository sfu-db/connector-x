use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use postgres::types::Type;
use rust_decimal::Decimal;
use serde_json::Value;
use uuid::Uuid;

#[derive(Copy, Clone, Debug)]
pub enum PostgresTypeSystem {
    Bool(bool),
    Float4(bool),
    Float8(bool),
    Numeric(bool),
    Int2(bool),
    Int4(bool),
    Int8(bool),
    Date(bool),
    Char(bool),
    BpChar(bool),
    VarChar(bool),
    Text(bool),
    ByteA(bool),
    Time(bool),
    Timestamp(bool),
    TimestampTz(bool),
    UUID(bool),
    JSON(bool),
    JSONB(bool),
    Enum(bool),
}

impl_typesystem! {
    system = PostgresTypeSystem,
    mappings = {
        { Int2 => i16 }
        { Int4 => i32 }
        { Int8 => i64 }
        { Float4 => f32 }
        { Float8 => f64 }
        { Numeric => Decimal }
        { Bool => bool }
        { Char => i8 }
        { Text | BpChar | VarChar | Enum => &'r str }
        { ByteA => Vec<u8> }
        { Time => NaiveTime }
        { Timestamp => NaiveDateTime }
        { TimestampTz => DateTime<Utc> }
        { Date => NaiveDate }
        { UUID => Uuid }
        { JSON | JSONB => Value }
    }
}

impl<'a> From<&'a Type> for PostgresTypeSystem {
    fn from(ty: &'a Type) -> PostgresTypeSystem {
        use PostgresTypeSystem::*;
        match ty.name() {
            "int2" => Int2(true),
            "int4" => Int4(true),
            "int8" => Int8(true),
            "float4" => Float4(true),
            "float8" => Float8(true),
            "numeric" => Numeric(true),
            "bool" => Bool(true),
            "char" => Char(true),
            "text" => Text(true),
            "bpchar" => BpChar(true),
            "varchar" => VarChar(true),
            "bytea" => ByteA(true),
            "time" => Time(true),
            "timestamp" => TimestampTz(true),
            "timestamptz" => Timestamp(true),
            "date" => Date(true),
            "uuid" => UUID(true),
            "json" => JSON(true),
            "jsonb" => JSONB(true),
            _ => match ty.kind() {
                postgres::types::Kind::Enum(_) => Enum(true),
                _ => unimplemented!("{}", ty.name()),
            },
        }
    }
}

// Link PostgresDTypes back to the one defiend by the postgres crate.
impl<'a> From<PostgresTypeSystem> for Type {
    fn from(ty: PostgresTypeSystem) -> Type {
        use PostgresTypeSystem::*;
        match ty {
            Int2(_) => Type::INT2,
            Int4(_) => Type::INT4,
            Int8(_) => Type::INT8,
            Float4(_) => Type::FLOAT4,
            Float8(_) => Type::FLOAT8,
            Numeric(_) => Type::NUMERIC,
            Bool(_) => Type::BOOL,
            Text(_) => Type::TEXT,
            BpChar(_) => Type::BPCHAR,
            VarChar(_) => Type::VARCHAR,
            Char(_) => Type::CHAR,
            ByteA(_) => Type::BYTEA,
            Date(_) => Type::DATE,
            Time(_) => Type::TIME,
            Timestamp(_) => Type::TIMESTAMP,
            TimestampTz(_) => Type::TIMESTAMPTZ,
            UUID(_) => Type::UUID,
            JSON(_) => Type::JSON,
            JSONB(_) => Type::JSONB,
            Enum(_) => Type::TEXT,
        }
    }
}
