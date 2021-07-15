// Unfortunately, due to the orphan rule, typesystem implementation should be in this crate.
use chrono::{DateTime, Utc};
use connectorx::impl_typesystem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PandasTypeSystem {
    F64(bool),
    I64(bool),
    Bool(bool),
    Char(bool),
    Str(bool),
    BoxStr(bool),
    String(bool),
    Bytes(bool),
    DateTime(bool),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PandasBlockType {
    Boolean(bool),
    Int64(bool),
    Float64,
    String,
    DateTime,
    Bytes,
}

impl From<PandasTypeSystem> for PandasBlockType {
    fn from(ty: PandasTypeSystem) -> PandasBlockType {
        match ty {
            PandasTypeSystem::Bool(nullable) => PandasBlockType::Boolean(nullable),
            PandasTypeSystem::I64(nullable) => PandasBlockType::Int64(nullable),
            PandasTypeSystem::F64(_) => PandasBlockType::Float64,
            PandasTypeSystem::String(_)
            | PandasTypeSystem::BoxStr(_)
            | PandasTypeSystem::Str(_)
            | PandasTypeSystem::Char(_) => PandasBlockType::String,
            PandasTypeSystem::Bytes(_) => PandasBlockType::Bytes,
            PandasTypeSystem::DateTime(_) => PandasBlockType::DateTime,
        }
    }
}

impl_typesystem! {
    system = PandasTypeSystem,
    mappings = {
        { F64 => f64 }
        { I64 => i64 }
        { Bool => bool }
        { Char => char }
        { Str => &'r str }
        { BoxStr => Box<str> }
        { String => String }
        { Bytes => Vec<u8> }
        { DateTime => DateTime<Utc> }
    }
}

pub trait PandasDType: Sized {
    // For initialize a pandas array when creating the pandas dataframe
    fn is_masked(&self) -> bool;
    fn array_name(&self) -> &'static str;
}

impl PandasDType for PandasBlockType {
    fn is_masked(&self) -> bool {
        match *self {
            PandasBlockType::Boolean(true) | PandasBlockType::Int64(true) => true,
            _ => false,
        }
    }

    fn array_name(&self) -> &'static str {
        match *self {
            PandasBlockType::Boolean(true) => "BooleanArray",
            PandasBlockType::Int64(true) => "IntegerArray",
            PandasBlockType::DateTime => "DatetimeArray",
            _ => "",
        }
    }
}
