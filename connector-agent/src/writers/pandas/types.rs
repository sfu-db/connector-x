// Unfortunately, due to the orphan rule, typesystem implementation should be in this crate.
use crate::errors::{ConnectorAgentError, Result};
use crate::TypeSystem;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use fehler::throws;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PandasTypes {
    F64(bool),
    I64(bool),
    Bool(bool),
    String(bool),
    DateTime(bool),
}

impl TypeSystem for PandasTypes {}

pub trait PandasDType: Sized {
    fn dtype(&self) -> &'static str;
    // For initialize a numpy array when creating the pandas dataframe
    fn npdtype(&self) -> &'static str;
    fn parse(ty: &str) -> Result<Self>;
}

impl PandasDType for PandasTypes {
    fn dtype(&self) -> &'static str {
        match *self {
            PandasTypes::I64(false) => "int64",
            PandasTypes::I64(true) => "Int64",
            PandasTypes::F64(_) => "float64",
            PandasTypes::Bool(false) => "bool",
            PandasTypes::Bool(true) => "boolean",
            PandasTypes::String(_) => "object",
            // PandasTypes::String(_) => "string",
            PandasTypes::DateTime(_) => "datetime64[ns]",
        }
    }

    fn npdtype(&self) -> &'static str {
        match *self {
            PandasTypes::I64(_) => "i8",
            PandasTypes::F64(_) => "f8",
            PandasTypes::Bool(_) => "b1",
            PandasTypes::String(_) => "O",
            PandasTypes::DateTime(_) => "M8[ns]",
        }
    }

    #[throws(ConnectorAgentError)]
    fn parse(ty: &str) -> Self {
        match ty {
            "int64" => PandasTypes::I64(false),
            "Int64" => PandasTypes::I64(true),
            "float64" => PandasTypes::F64(true),
            "bool" => PandasTypes::Bool(false),
            "boolean" => PandasTypes::Bool(true),
            "string" => PandasTypes::String(true),
            "datetime" => PandasTypes::DateTime(true),
            ty => unimplemented!("{}", ty),
        }
    }
}

associate_typesystem! {
    PandasTypes,
    [PandasTypes::F64] => f64,
    [PandasTypes::I64] => i64,
    [PandasTypes::Bool] => bool,
    [PandasTypes::String] => Bytes,
    [PandasTypes::DateTime] => DateTime<Utc>,
}
