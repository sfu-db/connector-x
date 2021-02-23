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
    m.add_wrapped(wrap_pyfunction!(read_sql))?;

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

#[pyfunction]
fn read_sql<'a>(
    py: Python<'a>,
    conn: &str,
    query: &str,
    schema: Vec<&str>, // TODO: remove this once inferschema is done
    return_type: &str,
) -> PyResult<&'a PyAny> {
    todo!("split the query");
    let queries = vec![];

    match return_type {
        "pandas" => Ok(crate::pandas::write_pandas(py, conn, &queries, &schema)?),
        "arrow" => todo!(),
        _ => todo!("Error handling"),
    }
}
