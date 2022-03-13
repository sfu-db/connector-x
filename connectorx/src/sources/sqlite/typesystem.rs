use super::errors::SQLiteSourceError;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use fehler::{throw, throws};
use rusqlite::types::Type;
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SQLiteTypeSystem {
    Bool(bool),
    Int8(bool),
    Int4(bool),
    Int2(bool),
    Real(bool),
    Text(bool),
    Date(bool),
    Time(bool),
    Timestamp(bool),
    Blob(bool),
}

impl_typesystem! {
    system = SQLiteTypeSystem,
    mappings = {
        { Bool => bool }
        { Int8 => i64 }
        { Int4 => i32 }
        { Int2 => i16 }
        { Real => f64 }
        { Text => Box<str> }
        { Date => NaiveDate}
        { Time => NaiveTime}
        { Timestamp => NaiveDateTime}
        { Blob => Vec<u8>}
    }
}

impl TryFrom<Type> for SQLiteTypeSystem {
    type Error = SQLiteSourceError;

    #[throws(SQLiteSourceError)]
    fn try_from(ty: Type) -> Self {
        use SQLiteTypeSystem::*;
        match ty {
            Type::Integer => Int8(true),
            Type::Real => Real(true),
            Type::Text => Text(true),
            Type::Blob => Blob(true),
            Type::Null => throw!(SQLiteSourceError::InferTypeFromNull),
        }
    }
}

impl TryFrom<(Option<&str>, Type)> for SQLiteTypeSystem {
    type Error = SQLiteSourceError;

    #[throws(SQLiteSourceError)]
    fn try_from(types: (Option<&str>, Type)) -> Self {
        use SQLiteTypeSystem::*;
        match types {
            // derive from column's declare type, some rules refer to:
            // https://www.sqlite.org/datatype3.html#affname
            (Some(decl_type), ty) => {
                let decl_type = decl_type.to_lowercase();
                match decl_type.as_str() {
                    "int4" => Int4(true),
                    "int2" => Int2(true),
                    "boolean" | "bool" => Bool(true),
                    "date" => Date(true),
                    "time" => Time(true),
                    "datetime" | "timestamp" => Timestamp(true),
                    _ if decl_type.contains("int") => Int8(true),
                    _ if decl_type.contains("char")
                        || decl_type.contains("clob")
                        || decl_type.contains("text") =>
                    {
                        Text(true)
                    }
                    _ if decl_type.contains("real")
                        || decl_type.contains("floa")
                        || decl_type.contains("doub") =>
                    {
                        Real(true)
                    }
                    _ if decl_type.contains("blob") => Blob(true),
                    _ => SQLiteTypeSystem::try_from(ty)?,
                }
            }
            // derive from value type directly if no declare type available
            (None, ty) => SQLiteTypeSystem::try_from(ty)?,
        }
    }
}
