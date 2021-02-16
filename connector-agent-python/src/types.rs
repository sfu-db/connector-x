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
            "uint64" => DataType::U64,
            "float64" => DataType::F64,
            "bool" => DataType::Bool,
            "object" => DataType::String,
            "UInt64" => DataType::OptU64,
            ty => throw!(ConnectorAgentPythonError::UnknownPandasType(ty.to_string())),
        }
    }
}
