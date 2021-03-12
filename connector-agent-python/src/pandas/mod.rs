mod pandas_columns;
mod pystring;
mod transport;
mod types;
mod writers;

use crate::errors::ConnectorAgentPythonError;
use anyhow::anyhow;
use connector_agent::{Dispatcher, PostgresSource};
use fehler::throws;
use log::debug;
use pyo3::{PyAny, Python};
pub use transport::PostgresPandasTransport;
pub use types::{PandasDType, PandasTypes};
pub use writers::{PandasPartitionWriter, PandasWriter};

#[throws(ConnectorAgentPythonError)]
pub fn write_pandas<'a>(py: Python<'a>, conn: &str, queries: &[&str]) -> &'a PyAny {
    let mut writer = PandasWriter::new(py);
    let sb = PostgresSource::new(conn, queries.len());

    // TODO: unlock gil for these two line
    let dispatcher = Dispatcher::<PostgresSource, PandasWriter, PostgresPandasTransport>::new(
        sb,
        &mut writer,
        queries,
    );

    debug!("Running dispatcher");
    dispatcher.run()?;

    writer.result().ok_or(anyhow!("writer not run"))?
}
