mod pandas_columns;
mod pystring;
mod writers;

use crate::errors::{ConnectorAgentPythonError, Result};
use anyhow::anyhow;
use connector_agent::{DataType, Dispatcher, PostgresSource};
use fehler::{throw, throws};
use log::debug;
use pyo3::{PyAny, Python};
use writers::PandasWriter;

#[throws(ConnectorAgentPythonError)]
pub fn write_pandas<'a>(py: Python<'a>, conn: &str, queries: &[&str], checked: bool) -> &'a PyAny {
    let mut writer = PandasWriter::new(py);
    let sb = PostgresSource::new(conn, queries.len());

    // TODO: unlock gil for these two line
    let dispatcher = Dispatcher::new(sb, &mut writer, queries);

    debug!("Running dispatcher");
    if checked {
        dispatcher.run_checked()?;
    } else {
        dispatcher.run()?;
    }

    debug!("Start writing string");
    writer.write_string_columns()?;
    debug!("String writing finished");

    writer.result().ok_or(anyhow!("writer not run"))?
}

pub trait PandasDType: Sized {
    fn dtype(&self) -> &'static str;
    // For initialize a numpy array when creating the pandas dataframe
    fn npdtype(&self) -> &'static str;
    fn parse(ty: &str) -> Result<Self>;
}

impl PandasDType for DataType {
    fn dtype(&self) -> &'static str {
        match *self {
            DataType::I64(false) => "int64",
            DataType::I64(true) => "Int64",
            DataType::F64(_) => "float64",
            DataType::Bool(false) => "bool",
            DataType::Bool(true) => "boolean",
            DataType::String(_) => "string",
            DataType::DateTime(_) => "datetime64[ns]",
        }
    }

    fn npdtype(&self) -> &'static str {
        match *self {
            DataType::I64(_) => "i8",
            DataType::F64(_) => "f8",
            DataType::Bool(_) => "b1",
            DataType::String(_) => "O",
            DataType::DateTime(_) => "M8[ns]",
        }
    }

    #[throws(ConnectorAgentPythonError)]
    fn parse(ty: &str) -> DataType {
        match ty {
            "int64" => DataType::I64(false),
            "Int64" => DataType::I64(true),
            "float64" => DataType::F64(true),
            "bool" => DataType::Bool(false),
            "boolean" => DataType::Bool(true),
            "string" => DataType::String(true),
            "datetime" => DataType::DateTime(true),
            // "date" => DataType::Date(true),
            ty => throw!(ConnectorAgentPythonError::UnknownPandasType(ty.to_string())),
        }
    }
}
