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

#[cfg(test)]
mod tests {
    use super::SQLiteTypeSystem;
    use rusqlite::types::Type;
    use std::convert::TryFrom;

    #[test]
    fn maps_value_types() {
        assert!(matches!(
            SQLiteTypeSystem::try_from(Type::Integer).unwrap(),
            SQLiteTypeSystem::Int8(true)
        ));
        assert!(matches!(
            SQLiteTypeSystem::try_from(Type::Real).unwrap(),
            SQLiteTypeSystem::Real(true)
        ));
        assert!(matches!(
            SQLiteTypeSystem::try_from(Type::Text).unwrap(),
            SQLiteTypeSystem::Text(true)
        ));
        assert!(matches!(
            SQLiteTypeSystem::try_from(Type::Blob).unwrap(),
            SQLiteTypeSystem::Blob(true)
        ));
        assert!(SQLiteTypeSystem::try_from(Type::Null).is_err());
    }

    #[test]
    fn maps_declared_types_and_affinity_fallbacks() {
        for (declared, expected) in [
            ("int4", SQLiteTypeSystem::Int4(true)),
            ("int2", SQLiteTypeSystem::Int2(true)),
            ("boolean", SQLiteTypeSystem::Bool(true)),
            ("bool", SQLiteTypeSystem::Bool(true)),
            ("date", SQLiteTypeSystem::Date(true)),
            ("time", SQLiteTypeSystem::Time(true)),
            ("datetime", SQLiteTypeSystem::Timestamp(true)),
            ("timestamp", SQLiteTypeSystem::Timestamp(true)),
            ("integer", SQLiteTypeSystem::Int8(true)),
            ("varchar", SQLiteTypeSystem::Text(true)),
            ("clob", SQLiteTypeSystem::Text(true)),
            ("text", SQLiteTypeSystem::Text(true)),
            ("real", SQLiteTypeSystem::Real(true)),
            ("float", SQLiteTypeSystem::Real(true)),
            ("double", SQLiteTypeSystem::Real(true)),
            ("blob", SQLiteTypeSystem::Blob(true)),
        ] {
            assert_eq!(
                format!(
                    "{:?}",
                    SQLiteTypeSystem::try_from((Some(declared), Type::Text)).unwrap()
                ),
                format!("{expected:?}")
            );
        }
        assert!(matches!(
            SQLiteTypeSystem::try_from((Some("other"), Type::Real)).unwrap(),
            SQLiteTypeSystem::Real(true)
        ));
        assert!(matches!(
            SQLiteTypeSystem::try_from((None, Type::Text)).unwrap(),
            SQLiteTypeSystem::Text(true)
        ));
    }
}
