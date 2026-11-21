use crate::sources::postgres::IpInet;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use postgres::types::Type;
use rust_decimal::Decimal;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

use pgvector::{Bit, HalfVector, SparseVector, Vector};

/// Column types of the Postgres source. The `bool` is `true` when the column is nullable.
#[derive(Copy, Clone, Debug)]
pub enum PostgresTypeSystem {
    /// Postgres `bool`.
    Bool(bool),
    /// Postgres `float4` (32-bit float).
    Float4(bool),
    /// Postgres `float8` (64-bit float).
    Float8(bool),
    /// Postgres `numeric`, read as a `Decimal`.
    Numeric(bool),
    /// Postgres `int2` (16-bit integer).
    Int2(bool),
    /// Postgres `int4` (32-bit integer).
    Int4(bool),
    /// Postgres `int8` (64-bit integer).
    Int8(bool),
    /// Postgres `oid` (unsigned 32-bit integer).
    UInt4(bool),
    /// Postgres `float4[]`.
    Float4Array(bool),
    /// Postgres `float8[]`.
    Float8Array(bool),
    /// Postgres `numeric[]`.
    NumericArray(bool),
    /// Postgres `bool[]`.
    BoolArray(bool),
    /// Postgres `int2[]`.
    Int2Array(bool),
    /// Postgres `int4[]`.
    Int4Array(bool),
    /// Postgres `int8[]`.
    Int8Array(bool),
    /// Postgres `varchar[]`.
    VarcharArray(bool),
    /// Postgres `text[]`.
    TextArray(bool),
    /// Postgres `date`.
    Date(bool),
    /// Postgres `"char"` (single byte), read as `i8`.
    Char(bool),
    /// Postgres `bpchar` (`char(n)`).
    BpChar(bool),
    /// Postgres `varchar`.
    VarChar(bool),
    /// Postgres `text`, also used for `citext`, `ltree`, `lquery`, `ltxtquery` and `name`.
    Text(bool),
    /// Postgres `bytea`.
    ByteA(bool),
    /// Postgres `time`.
    Time(bool),
    /// Postgres `timestamp` (without time zone).
    Timestamp(bool),
    /// Postgres `timestamptz`, read as UTC.
    TimestampTz(bool),
    /// Postgres `uuid`.
    UUID(bool),
    /// Postgres `json`.
    JSON(bool),
    /// Postgres `jsonb`.
    JSONB(bool),
    /// Postgres enum type, read as text.
    Enum(bool),
    /// Postgres `hstore`, read as a string map.
    HSTORE(bool),
    /// Postgres `name`, read as text. Not produced by type detection, which maps `name` to `Text`.
    Name(bool),
    /// Postgres `inet`.
    Inet(bool),
    /// Postgres range types (`int4range`, `numrange`, `tsrange`, ...), read as text.
    Range(bool),
    /// Postgres `tsvector`, read as text.
    TsVector(bool),
    /// pgvector `vector`.
    Vector(bool),
    /// pgvector `halfvec`.
    HalfVec(bool),
    /// Postgres `bit`, read as a pgvector `Bit`.
    Bit(bool),
    /// pgvector `sparsevec`.
    SparseVec(bool),
}

impl_typesystem! {
    system = PostgresTypeSystem,
    mappings = {
        { Int2 => i16 }
        { Int4 => i32 }
        { Int8 => i64 }
        { UInt4 => u32 }
        { Float4 => f32 }
        { Float8 => f64 }
        { Numeric => Decimal }
        { BoolArray => Vec<Option<bool>> }
        { Int2Array => Vec<Option<i16>> }
        { Int4Array => Vec<Option<i32>> }
        { Int8Array => Vec<Option<i64>> }
        { Float4Array => Vec<Option<f32>> }
        { Float8Array => Vec<Option<f64>> }
        { NumericArray => Vec<Option<Decimal>> }
        { VarcharArray | TextArray => Vec<Option<String>>}
        { Bool => bool }
        { Char => i8 }
        { Text | BpChar | VarChar | Enum | Name | Range | TsVector => &'r str }
        { ByteA => Vec<u8> }
        { Time => NaiveTime }
        { Timestamp => NaiveDateTime }
        { TimestampTz => DateTime<Utc> }
        { Date => NaiveDate }
        { UUID => Uuid }
        { JSON | JSONB => Value }
        { HSTORE => HashMap<String, Option<String>> }
        { Inet => IpInet }
        { Vector => Vector }
        { HalfVec => HalfVector }
        { Bit => Bit }
        { SparseVec => SparseVector }
    }
}

impl<'a> From<&'a Type> for PostgresTypeSystem {
    fn from(ty: &'a Type) -> PostgresTypeSystem {
        use PostgresTypeSystem::*;
        match ty.name() {
            "int2" => Int2(true),
            "int4" => Int4(true),
            "int8" => Int8(true),
            "oid" => UInt4(true),
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
            "inet" => Inet(true),
            "tsvector" => TsVector(true),
            "int4range" | "int8range" | "numrange" | "tsrange" | "tstzrange" | "daterange" => {
                Range(true)
            }
            "vector" => Vector(true),
            "halfvec" => HalfVec(true),
            "bit" => Bit(true),
            "sparsevec" => SparseVec(true),
            _ => match ty.kind() {
                postgres::types::Kind::Enum(_) => Enum(true),
                _ => unimplemented!("{}", ty.name()),
            },
        }
    }
}

/// A Postgres column type paired with the ConnectorX type chosen for it; converts back to the `Type` to request on the wire.
pub struct PostgresTypePairs<'a>(pub &'a Type, pub &'a PostgresTypeSystem);

// Link (postgres::Type, connectorx::PostgresTypes) back to the one defined by the postgres crate.
impl From<PostgresTypePairs<'_>> for Type {
    fn from(ty: PostgresTypePairs) -> Type {
        use PostgresTypeSystem::*;
        match ty.1 {
            Enum(_) => Type::TEXT,
            HSTORE(_) => Type::TEXT, // hstore is not supported in binary protocol (since no corresponding inner TYPE)
            Range(_) | TsVector(_) => Type::TEXT,
            _ => ty.0.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PostgresTypePairs, PostgresTypeSystem};
    use postgres::types::Type;

    #[test]
    fn tsvector_uses_text_wire_type() {
        let schema = PostgresTypeSystem::from(&Type::TS_VECTOR);
        assert!(matches!(schema, PostgresTypeSystem::TsVector(true)));
        for nullable in [false, true] {
            assert_eq!(
                Type::from(PostgresTypePairs(
                    &Type::TS_VECTOR,
                    &PostgresTypeSystem::TsVector(nullable),
                )),
                Type::TEXT
            );
        }
    }
}
