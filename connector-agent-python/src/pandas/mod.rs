mod destination;
mod pandas_columns;
mod pystring;
mod transport;
mod types;

use crate::errors::ConnectorAgentPythonError;
use anyhow::anyhow;
use connector_agent::{sources::postgres::PostgresSource, Dispatcher};
pub use destination::{PandasDestination, PandasPartitionDestination};
use fehler::throws;
use log::debug;
use pyo3::{PyAny, Python};
pub use transport::PostgresPandasTransport;
pub use types::{PandasDType, PandasTypeSystem};

#[throws(ConnectorAgentPythonError)]
pub fn write_pandas<'a>(py: Python<'a>, conn: &str, queries: &[&str]) -> &'a PyAny {
    let mut destination = PandasDestination::new(py);
    let sb = PostgresSource::new(conn, queries.len());

    // TODO: unlock gil if possible
    let dispatcher = Dispatcher::<PostgresSource, PandasDestination, PostgresPandasTransport>::new(
        sb,
        &mut destination,
        queries,
    );

    debug!("Running dispatcher");
    dispatcher.run()?;

    destination.result().ok_or(anyhow!("destination not run"))?
}
