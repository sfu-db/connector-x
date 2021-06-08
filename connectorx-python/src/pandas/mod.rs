mod destination;
mod mysql_pandas;
mod pandas_columns;
mod pystring;
mod transport;
mod types;

pub use self::destination::{PandasDestination, PandasPartitionDestination};
pub use self::transport::PostgresPandasTransport;
pub use self::types::{PandasDType, PandasTypeSystem};
use crate::errors::ConnectorAgentPythonError;
use anyhow::anyhow;
use connectorx::{
    sources::postgres::{Binary, PostgresSource, CSV},
    Dispatcher,
};
use fehler::throws;
use log::debug;
use pyo3::{PyAny, Python};

#[throws(ConnectorAgentPythonError)]
pub fn write_pandas<'a>(py: Python<'a>, conn: &str, queries: &[&str], protocol: &str) -> &'a PyAny {
    let mut destination = PandasDestination::new(py);

    // TODO: unlock gil if possible
    debug!("Protocol: {}", protocol);
    if conn.starts_with("mysql") {
        todo!(something);
    } else if conn.starts_with("postgres") {
        match protocol {
            "csv" => {
                let sb = PostgresSource::<CSV>::new(conn, queries.len())?;
                let dispatcher = Dispatcher::<_, _, PostgresPandasTransport<CSV>>::new(
                    sb,
                    &mut destination,
                    queries,
                );

                debug!("Running dispatcher");
                dispatcher.run()?;
            }
            "binary" => {
                let sb = PostgresSource::<Binary>::new(conn, queries.len())?;
                let dispatcher = Dispatcher::<_, _, PostgresPandasTransport<Binary>>::new(
                    sb,
                    &mut destination,
                    queries,
                );

                debug!("Running dispatcher");
                dispatcher.run()?;
            }
            _ => unimplemented!("{} protocol not supported", protocol),
        }
    }
    destination
        .result()
        .ok_or_else(|| anyhow!("destination not run"))?
}
