mod pandas_columns;
mod pystring;
mod writers;

use crate::errors::{ConnectorAgentPythonError, Result};
use anyhow::anyhow;
use connector_agent::{DataType, Dispatcher, PostgresDataSourceBuilder};
use fehler::{throw, throws};
use pyo3::{PyAny, Python};
use writers::PandasWriter;

#[throws(ConnectorAgentPythonError)]
pub fn write_pandas<'a>(
    py: Python<'a>,
    conn: &str,
    queries: &[&str],
    schema: &[&str],
) -> &'a PyAny {
    // convert schema
    let maybe_schema: Result<Vec<DataType>> =
        schema.into_iter().map(|&s| PandasDType::parse(s)).collect();
    let schema = maybe_schema?;

    let mut writer = PandasWriter::new(py);
    let sb = PostgresDataSourceBuilder::new(conn);

    // TODO: unlock gil for these two line
    let dispatcher = Dispatcher::new(sb, &mut writer, queries, &schema);
    dispatcher.run_checked()?;

    writer.write_string_columns()?;

    writer.result().ok_or(anyhow!("writer not run"))?
}

pub trait PandasDType: Sized {
    fn dtype(&self) -> &'static str;
    fn parse(ty: &str) -> Result<Self>;
}

impl PandasDType for DataType {
    fn dtype(&self) -> &'static str {
        match *self {
            DataType::U64(false) => "uint64",
            DataType::U64(true) => "UInt64",
            DataType::F64(_) => "float64",
            DataType::Bool(false) => "bool",
            DataType::Bool(true) => "boolean",
            DataType::String(_) => "string",
        }
    }

    #[throws(ConnectorAgentPythonError)]
    fn parse(ty: &str) -> DataType {
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
