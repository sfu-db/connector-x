use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rusqlite::types::Type;

#[derive(Copy, Clone, Debug)]
pub enum SqliteTypeSystem {
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
    system = SqliteTypeSystem,
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

impl From<Type> for SqliteTypeSystem {
    fn from(ty: Type) -> SqliteTypeSystem {
        use SqliteTypeSystem::*;
        match ty {
            Type::Integer => Int8(true),
            Type::Real => Real(true),
            Type::Text => Text(true),
            Type::Blob => Blob(true),
            _ => unimplemented!("{}", ty),
        }
    }
}

impl From<(Option<&str>, Type)> for SqliteTypeSystem {
    fn from(types: (Option<&str>, Type)) -> SqliteTypeSystem {
        use SqliteTypeSystem::*;
        match types {
            // derive from column's declare type, some rules refer to:
            // https://www.sqlite.org/datatype3.html#affname
            (Some(decl_type), ty) => {
                let s = decl_type.to_lowercase();
                match s.as_str() {
                    "int4" => Int4(true),
                    "int2" => Int2(true),
                    "boolean" | "bool" => Bool(true),
                    "date" => Date(true),
                    "time" => Time(true),
                    "datetime" | "timestamp" => Timestamp(true),
                    _ if s.contains("int") => Int8(true),
                    _ if s.contains("char") || s.contains("clob") || s.contains("text") => {
                        Text(true)
                    }
                    _ if s.contains("real") || s.contains("floa") || s.contains("doub") => {
                        Real(true)
                    }
                    _ if s.contains("blob") => Blob(true),
                    _ => ty.into(),
                }
            }
            // derive from value type directly if no declare type available
            (None, ty) => ty.into(),
        }
    }
}
