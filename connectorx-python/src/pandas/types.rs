// Unfortunately, due to the orphan rule, typesystem implementation should be in this crate.
use chrono::{DateTime, Utc};
use connectorx::errors::{ConnectorAgentError, Result};
use connectorx::impl_typesystem;
use fehler::throws;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PandasTypeSystem {
    F64(bool),
    I64(bool),
    Bool(bool),
    String(bool),
    DateTime(bool),
}

impl_typesystem! {
    system = PandasTypeSystem,
    mappings = {
        { F64 => f64 }
        { I64 => i64 }
        { Bool => bool }
        { String => &'r str }
        { DateTime => DateTime<Utc> }
    }
}

pub trait PandasDType: Sized {
    fn dtype(&self) -> &'static str;
    // For initialize a numpy array when creating the pandas dataframe
    fn npdtype(&self) -> &'static str;
    fn parse(ty: &str) -> Result<Self>;
    fn is_extension(&self) -> bool;
    fn block_name(&self) -> &'static str;
}

impl PandasDType for PandasTypeSystem {
    fn dtype(&self) -> &'static str {
        match *self {
            PandasTypeSystem::I64(false) => "int64",
            PandasTypeSystem::I64(true) => "Int64",
            PandasTypeSystem::F64(_) => "float64",
            PandasTypeSystem::Bool(false) => "bool",
            PandasTypeSystem::Bool(true) => "boolean",
            PandasTypeSystem::String(_) => "object",
            PandasTypeSystem::DateTime(_) => "datetime64[ns]",
        }
    }

    fn npdtype(&self) -> &'static str {
        match *self {
            PandasTypeSystem::I64(_) => "i8",
            PandasTypeSystem::F64(_) => "f8",
            PandasTypeSystem::Bool(_) => "b1",
            PandasTypeSystem::String(_) => "O",
            PandasTypeSystem::DateTime(_) => "M8[ns]",
        }
    }

    #[throws(ConnectorAgentError)]
    fn parse(ty: &str) -> Self {
        match ty {
            "int64" => PandasTypeSystem::I64(false),
            "Int64" => PandasTypeSystem::I64(true),
            "float64" => PandasTypeSystem::F64(true),
            "bool" => PandasTypeSystem::Bool(false),
            "boolean" => PandasTypeSystem::Bool(true),
            "object" => PandasTypeSystem::String(true),
            "datetime" => PandasTypeSystem::DateTime(true),
            ty => unimplemented!("{}", ty),
        }
    }

    fn is_extension(&self) -> bool {
        match *self {
            PandasTypeSystem::I64(false) => false,
            PandasTypeSystem::I64(true) => true,
            PandasTypeSystem::F64(_) => false,
            PandasTypeSystem::Bool(false) => false,
            PandasTypeSystem::Bool(true) => true,
            PandasTypeSystem::String(_) => false, // we use object instead of string (Extension) for now
            PandasTypeSystem::DateTime(_) => false,
        }
    }

    fn block_name(&self) -> &'static str {
        match *self {
            PandasTypeSystem::I64(false) => "IntBlock",
            PandasTypeSystem::I64(true) => "ExtensionBlock",
            PandasTypeSystem::F64(_) => "FloatBlock",
            PandasTypeSystem::Bool(false) => "BoolBlock",
            PandasTypeSystem::Bool(true) => "ExtensionBlock",
            PandasTypeSystem::String(_) => "ObjectBlock", // we use object instead of string (Extension) for now
            PandasTypeSystem::DateTime(_) => "DatetimeBlock",
        }
    }
}
