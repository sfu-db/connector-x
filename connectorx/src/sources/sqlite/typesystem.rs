use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rusqlite::types::Type;

#[derive(Copy, Clone, Debug)]
pub enum SqliteTypeSystem {
    Bool(bool),
    Integer(bool),
    Real(bool),
    Text(bool),
    Date(bool),
    Time(bool),
    Timestamp(bool),
}

impl_typesystem! {
    system = SqliteTypeSystem,
    mappings = {
        { Bool => bool }
        { Integer => i64 }
        { Real => f64 }
        { Text => Box<str> }
        { Date => NaiveDate}
        { Time => NaiveTime}
        { Timestamp => NaiveDateTime}
    }
}

impl From<(Option<&str>, Type)> for SqliteTypeSystem {
    fn from(types: (Option<&str>, Type)) -> SqliteTypeSystem {
        use SqliteTypeSystem::*;
        match types {
            (Some(decl_type), _) => match decl_type.to_lowercase().as_str() {
                "boolean" => Bool(true),
                "integer" => Integer(true),
                "real" => Real(true),
                "text" => Text(true),
                "date" => Date(true),
                "time" => Time(true),
                "datetime" => Timestamp(true),
                _ => unimplemented!("{}", decl_type),
            },
            (None, ty) => match ty {
                Type::Integer => Integer(true),
                Type::Real => Real(true),
                Type::Text => Text(true),
                _ => unimplemented!("{}", ty),
            },
        }
    }
}
