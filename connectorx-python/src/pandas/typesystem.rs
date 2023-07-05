// Unfortunately, due to the orphan rule, typesystem implementation should be in this crate.
use chrono::{DateTime, Utc};
use connectorx::impl_typesystem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PandasTypeSystem {
    F64(bool),
    I64(bool),
    F64Array(bool),
    I64Array(bool),
    Bool(bool),
    BoolArray(bool),
    Char(bool),
    Str(bool),
    BoxStr(bool),
    String(bool),
    Bytes(bool),
    ByteSlice(bool),
    DateTime(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PandasBlockType {
    Boolean(bool), // bool indicates nullablity
    Int64(bool),
    Float64,
    BooleanArray,
    Int64Array,
    Float64Array,
    String,
    DateTime,
    Bytes,
}

pub enum PandasArrayType {
    NumpyArray,
    IntegerArray,
    BooleanArray,
    DatetimeArray,
}

impl From<PandasBlockType> for PandasArrayType {
    fn from(ty: PandasBlockType) -> PandasArrayType {
        match ty {
            PandasBlockType::Boolean(true) => PandasArrayType::BooleanArray,
            PandasBlockType::Int64(true) => PandasArrayType::IntegerArray,
            PandasBlockType::DateTime => PandasArrayType::DatetimeArray,
            _ => PandasArrayType::NumpyArray,
        }
    }
}

impl From<PandasTypeSystem> for PandasBlockType {
    fn from(ty: PandasTypeSystem) -> PandasBlockType {
        match ty {
            PandasTypeSystem::Bool(nullable) => PandasBlockType::Boolean(nullable),
            PandasTypeSystem::I64(nullable) => PandasBlockType::Int64(nullable),
            PandasTypeSystem::F64(_) => PandasBlockType::Float64,
            PandasTypeSystem::BoolArray(_) => PandasBlockType::BooleanArray,
            PandasTypeSystem::F64Array(_) => PandasBlockType::Float64Array,
            PandasTypeSystem::I64Array(_) => PandasBlockType::Int64Array,
            PandasTypeSystem::String(_)
            | PandasTypeSystem::BoxStr(_)
            | PandasTypeSystem::Str(_)
            | PandasTypeSystem::Char(_) => PandasBlockType::String,
            PandasTypeSystem::Bytes(_) | PandasTypeSystem::ByteSlice(_) => PandasBlockType::Bytes,
            PandasTypeSystem::DateTime(_) => PandasBlockType::DateTime,
        }
    }
}

impl_typesystem! {
    system = PandasTypeSystem,
    mappings = {
        { F64 => f64 }
        { I64 => i64 }
        { F64Array => Vec<f64> }
        { I64Array => Vec<i64> }
        { Bool => bool }
        { BoolArray => Vec<bool> }
        { Char => char }
        { Str => &'r str }
        { BoxStr => Box<str> }
        { String => String }
        { Bytes => Vec<u8> }
        { ByteSlice => &'r [u8] }
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
        matches!(
            *self,
            PandasBlockType::Boolean(true) | PandasBlockType::Int64(true)
        )
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
