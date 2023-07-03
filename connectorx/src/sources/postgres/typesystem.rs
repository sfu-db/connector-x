use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use postgres::types::Type;
use rust_decimal::Decimal;
use serde_json::Value;
use std::collections::HashMap;
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
    Float4Array(bool),
    Float8Array(bool),
    NumericArray(bool),
    BoolArray(bool),
    Int2Array(bool),
    Int4Array(bool),
    Int8Array(bool),
    VarcharArray(bool),
    TextArray(bool),
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
    HSTORE(bool),
    Name(bool),
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
        { BoolArray => Vec<bool> }
        { Int2Array => Vec<i16> }
        { Int4Array => Vec<i32> }
        { Int8Array => Vec<i64> }
        { Float4Array => Vec<f32> }
        { Float8Array => Vec<f64> }
        { NumericArray => Vec<Decimal> }
        { VarcharArray | TextArray => Vec<String>}
        { Bool => bool }
        { Char => i8 }
        { Text | BpChar | VarChar | Enum | Name => &'r str }        { ByteA => Vec<u8> }
        { Time => NaiveTime }
        { Timestamp => NaiveDateTime }
        { TimestampTz => DateTime<Utc> }
        { Date => NaiveDate }
        { UUID => Uuid }
        { JSON | JSONB => Value }
        { HSTORE => HashMap<String, Option<String>> }
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
            "_bool" => BoolArray(true),
            "_int2" => Int2Array(true),
            "_int4" => Int4Array(true),
            "_int8" => Int8Array(true),
            "_float4" => Float4Array(true),
            "_float8" => Float8Array(true),
            "_numeric" => NumericArray(true),
            "_varchar" => VarcharArray(true),
            "_text" => TextArray(true),
            "bool" => Bool(true),
            "char" => Char(true),
            "text" | "citext" | "ltree" | "lquery" | "ltxtquery" | "name" => Text(true),
            "bpchar" => BpChar(true),
            "varchar" => VarChar(true),
            "bytea" => ByteA(true),
            "time" => Time(true),
            "timestamp" => Timestamp(true),
            "timestamptz" => TimestampTz(true),
            "date" => Date(true),
            "uuid" => UUID(true),
            "json" => JSON(true),
            "jsonb" => JSONB(true),
            "hstore" => HSTORE(true),
            _ => match ty.kind() {
                postgres::types::Kind::Enum(_) => Enum(true),
                _ => unimplemented!("{}", ty.name()),
            },
        }
    }
}

pub struct PostgresTypePairs<'a>(pub &'a Type, pub &'a PostgresTypeSystem);

// Link (postgres::Type, connectorx::PostgresTypes) back to the one defiend by the postgres crate.
impl<'a> From<PostgresTypePairs<'a>> for Type {
    fn from(ty: PostgresTypePairs) -> Type {
        use PostgresTypeSystem::*;
        match ty.1 {
            Enum(_) => Type::TEXT,
            HSTORE(_) => Type::TEXT, // hstore is not supported in binary protocol (since no corresponding inner TYPE)
            _ => ty.0.clone(),
        }
    }
}
