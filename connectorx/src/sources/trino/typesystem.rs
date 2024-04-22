use super::errors::TrinoSourceError;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use fehler::{throw, throws};
use prusto::{PrestoFloat, PrestoInt, PrestoTy};
use std::convert::TryFrom;

// TODO: implement Tuple, Row, Array and Map
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TrinoTypeSystem {
    Date(bool),
    Time(bool),
    Timestamp(bool),
    Boolean(bool),
    Bigint(bool),
    Integer(bool),
    Smallint(bool),
    Tinyint(bool),
    Double(bool),
    Real(bool),
    Varchar(bool),
    Char(bool),
}

impl_typesystem! {
    system = TrinoTypeSystem,
    mappings = {
        { Date => NaiveDate }
        { Time => NaiveTime }
        { Timestamp => NaiveDateTime }
        { Boolean => bool }
        { Bigint => i64 }
        { Integer => i32 }
        { Smallint => i16 }
        { Tinyint => i8 }
        { Double => f64 }
        { Real => f32 }
        { Varchar => String }
        { Char => char }
    }
}

impl TryFrom<PrestoTy> for TrinoTypeSystem {
    type Error = TrinoSourceError;

    #[throws(TrinoSourceError)]
    fn try_from(ty: PrestoTy) -> Self {
        use TrinoTypeSystem::*;
        match ty {
            PrestoTy::Date => Date(true),
            PrestoTy::Time => Time(true),
            PrestoTy::Timestamp => Timestamp(true),
            PrestoTy::Boolean => Boolean(true),
            PrestoTy::PrestoInt(PrestoInt::I64) => Bigint(true),
            PrestoTy::PrestoInt(PrestoInt::I32) => Integer(true),
            PrestoTy::PrestoInt(PrestoInt::I16) => Smallint(true),
            PrestoTy::PrestoInt(PrestoInt::I8) => Tinyint(true),
            PrestoTy::PrestoFloat(PrestoFloat::F64) => Double(true),
            PrestoTy::PrestoFloat(PrestoFloat::F32) => Real(true),
            PrestoTy::Varchar => Varchar(true),
            PrestoTy::Char(_) => Char(true),
            PrestoTy::Tuple(_) => Varchar(true),
            PrestoTy::Row(_) => Varchar(true),
            PrestoTy::Array(_) => Varchar(true),
            PrestoTy::Map(_, _) => Varchar(true),
            PrestoTy::Decimal(_, _) => Double(true),
            PrestoTy::IpAddress => Varchar(true),
            PrestoTy::Uuid => Varchar(true),
            _ => throw!(TrinoSourceError::InferTypeFromNull),
        }
    }
}

impl TryFrom<(Option<&str>, PrestoTy)> for TrinoTypeSystem {
    type Error = TrinoSourceError;

    #[throws(TrinoSourceError)]
    fn try_from(types: (Option<&str>, PrestoTy)) -> Self {
        use TrinoTypeSystem::*;
        match types {
            (Some(decl_type), ty) => {
                let decl_type = decl_type.to_lowercase();
                match decl_type.as_str() {
                    "date" => Date(true),
                    "time" => Time(true),
                    "timestamp" => Timestamp(true),
                    "boolean" => Boolean(true),
                    "bigint" => Bigint(true),
                    "int" | "integer" => Integer(true),
                    "smallint" => Smallint(true),
                    "tinyint" => Tinyint(true),
                    "double" => Double(true),
                    "real" | "float" => Real(true),
                    "varchar" | "varbinary" | "json" => Varchar(true),
                    "char" => Char(true),
                    "tuple" => Varchar(true),
                    "row" => Varchar(true),
                    "array" => Varchar(true),
                    "map" => Varchar(true),
                    "decimal" => Double(true),
                    "ipaddress" => Varchar(true),
                    "uuid" => Varchar(true),
                    _ => TrinoTypeSystem::try_from(ty)?,
                }
            }
            // derive from value type directly if no declare type available
            (None, ty) => TrinoTypeSystem::try_from(ty)?,
        }
    }
}
