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

    /// Every variant realizes to the Rust type named in `impl_typesystem!`.
    #[test]
    fn every_variant_realizes_to_its_rust_type() {
        use crate::typesystem::testing::assert_maps;
        use PostgresTypeSystem as P;
        type Variant = fn(bool) -> PostgresTypeSystem;
        let cases: &[(Variant, &str)] = &[
            (P::Int2, "i16"),
            (P::Int4, "i32"),
            (P::Int8, "i64"),
            (P::UInt4, "u32"),
            (P::Float4, "f32"),
            (P::Float8, "f64"),
            (P::Numeric, "::Decimal"),
            (P::BoolArray, "Vec<core::option::Option<bool>>"),
            (P::Int2Array, "Vec<core::option::Option<i16>>"),
            (P::Int4Array, "Vec<core::option::Option<i32>>"),
            (P::Int8Array, "Vec<core::option::Option<i64>>"),
            (P::Float4Array, "Vec<core::option::Option<f32>>"),
            (P::Float8Array, "Vec<core::option::Option<f64>>"),
            (P::NumericArray, "::Decimal>>"),
            (
                P::VarcharArray,
                "Vec<core::option::Option<alloc::string::String>>",
            ),
            (
                P::TextArray,
                "Vec<core::option::Option<alloc::string::String>>",
            ),
            (P::Bool, "bool"),
            (P::Char, "i8"),
            (P::Text, "&str"),
            (P::BpChar, "&str"),
            (P::VarChar, "&str"),
            (P::Enum, "&str"),
            (P::Name, "&str"),
            (P::Range, "&str"),
            (P::TsVector, "&str"),
            (P::ByteA, "Vec<u8>"),
            (P::Time, "::NaiveTime"),
            (P::Timestamp, "::NaiveDateTime"),
            (P::TimestampTz, "DateTime<chrono::offset::utc::Utc>"),
            (P::Date, "::NaiveDate"),
            (P::UUID, "::Uuid"),
            (P::JSON, "::Value"),
            (P::JSONB, "::Value"),
            (
                P::HSTORE,
                "HashMap<alloc::string::String, core::option::Option<alloc::string::String>>",
            ),
            (P::Inet, "::IpInet"),
            (P::Vector, "::Vector"),
            (P::HalfVec, "::HalfVector"),
            (P::Bit, "::Bit"),
            (P::SparseVec, "::SparseVector"),
        ];
        for (ts, rust_type) in cases {
            assert_maps(*ts, rust_type);
        }
    }

    /// Built-in and extension types map by name; every column is nullable.
    #[test]
    fn maps_postgres_types_by_name() {
        use postgres::types::Kind;
        use PostgresTypeSystem as P;
        let ext = |name: &str| Type::new(name.into(), 900_000, Kind::Simple, "public".into());
        let cases: Vec<(Type, PostgresTypeSystem)> = vec![
            (Type::INT2, P::Int2(true)),
            (Type::INT4, P::Int4(true)),
            (Type::INT8, P::Int8(true)),
            (Type::OID, P::UInt4(true)),
            (Type::FLOAT4, P::Float4(true)),
            (Type::FLOAT8, P::Float8(true)),
            (Type::NUMERIC, P::Numeric(true)),
            (Type::BOOL_ARRAY, P::BoolArray(true)),
            (Type::INT2_ARRAY, P::Int2Array(true)),
            (Type::INT4_ARRAY, P::Int4Array(true)),
            (Type::INT8_ARRAY, P::Int8Array(true)),
            (Type::FLOAT4_ARRAY, P::Float4Array(true)),
            (Type::FLOAT8_ARRAY, P::Float8Array(true)),
            (Type::NUMERIC_ARRAY, P::NumericArray(true)),
            (Type::VARCHAR_ARRAY, P::VarcharArray(true)),
            (Type::TEXT_ARRAY, P::TextArray(true)),
            (Type::BOOL, P::Bool(true)),
            (Type::CHAR, P::Char(true)),
            (Type::TEXT, P::Text(true)),
            (Type::NAME, P::Text(true)),
            (ext("citext"), P::Text(true)),
            (ext("ltree"), P::Text(true)),
            (ext("lquery"), P::Text(true)),
            (ext("ltxtquery"), P::Text(true)),
            (Type::BPCHAR, P::BpChar(true)),
            (Type::VARCHAR, P::VarChar(true)),
            (Type::BYTEA, P::ByteA(true)),
            (Type::TIME, P::Time(true)),
            (Type::TIMESTAMP, P::Timestamp(true)),
            (Type::TIMESTAMPTZ, P::TimestampTz(true)),
            (Type::DATE, P::Date(true)),
            (Type::UUID, P::UUID(true)),
            (Type::JSON, P::JSON(true)),
            (Type::JSONB, P::JSONB(true)),
            (ext("hstore"), P::HSTORE(true)),
            (Type::INET, P::Inet(true)),
            (Type::INT4_RANGE, P::Range(true)),
            (Type::INT8_RANGE, P::Range(true)),
            (Type::NUM_RANGE, P::Range(true)),
            (Type::TS_RANGE, P::Range(true)),
            (Type::TSTZ_RANGE, P::Range(true)),
            (Type::DATE_RANGE, P::Range(true)),
            (ext("vector"), P::Vector(true)),
            (ext("halfvec"), P::HalfVec(true)),
            (Type::BIT, P::Bit(true)),
            (ext("sparsevec"), P::SparseVec(true)),
            (
                Type::new(
                    "mood".into(),
                    900_001,
                    Kind::Enum(vec!["ok".into()]),
                    "public".into(),
                ),
                P::Enum(true),
            ),
        ];
        for (ty, expected) in cases {
            let got = PostgresTypeSystem::from(&ty);
            assert_eq!(format!("{got:?}"), format!("{expected:?}"), "{}", ty.name());
        }
    }

    /// Enum, hstore, range and tsvector columns are requested as text on the
    /// wire; every other type keeps its own.
    #[test]
    fn requests_text_for_types_without_a_binary_decoder() {
        use PostgresTypeSystem as P;
        for ts in [
            P::Enum(true),
            P::HSTORE(false),
            P::Range(true),
            P::TsVector(false),
        ] {
            assert_eq!(
                Type::from(PostgresTypePairs(&Type::INT4, &ts)),
                Type::TEXT,
                "{ts:?}"
            );
        }
        for (ty, ts) in [(Type::INT4, P::Int4(true)), (Type::JSONB, P::JSONB(false))] {
            assert_eq!(Type::from(PostgresTypePairs(&ty, &ts)), ty, "{ts:?}");
        }
    }
}
