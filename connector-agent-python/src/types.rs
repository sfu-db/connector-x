use crate::errors::{ConnectorAgentPythonError, Result};
use connector_agent::DataType;
use fehler::{throw, throws};

pub trait FromPandasType: Sized {
    fn from(ty: &str) -> Result<Self>;
}

impl FromPandasType for DataType {
    #[throws(ConnectorAgentPythonError)]
    fn from(ty: &str) -> DataType {
        match ty {
            "uint64" => DataType::U64(false),
            "UInt64" => DataType::U64(true),
            "float64" => DataType::F64(true),
            "bool" => DataType::Bool(false),
            "boolean" => DataType::Bool(true),
            "string" => DataType::String(true),
            ty => throw!(ConnectorAgentPythonError::UnknownPandasType(ty.to_string())),
        }
    }
}
