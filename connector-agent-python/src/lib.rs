#![feature(generic_associated_types)]
#![allow(incomplete_features)]

mod errors;
pub mod pandas;
mod types;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// https://github.com/PyO3/pyo3-built/issues/21
// #[allow(dead_code)]
// mod build {
//     include!(concat!(env!("OUT_DIR"), "/built.rs"));
// }

#[pymodule]
fn connector_agent_python(_: Python, m: &PyModule) -> PyResult<()> {
    // https://github.com/PyO3/pyo3-built/issues/21
    // m.add("__build__", pyo3_built!(py, build))?;
    m.add_wrapped(wrap_pyfunction!(write_pandas))?;
    Ok(())
}

#[pyfunction]
fn write_pandas<'a>(
    py: Python<'a>,
    conn: &str,
    queries: Vec<&str>,
    schema: Vec<&str>,
) -> PyResult<&'a PyAny> {
    Ok(crate::pandas::write_pandas(py, conn, &queries, &schema)?)
}
