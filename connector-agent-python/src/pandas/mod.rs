mod pandas_columns;
mod pystring;
mod writers;

use crate::errors::ConnectorAgentPythonError;
use anyhow::anyhow;
use connector_agent::{Dispatcher, PostgresSource};
use fehler::throws;
use log::debug;
use pyo3::{PyAny, Python};
pub use writers::{PandasPartitionWriter, PandasWriter};

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
