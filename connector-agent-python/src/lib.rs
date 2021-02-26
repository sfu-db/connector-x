#![feature(generic_associated_types)]
#![allow(incomplete_features)]

mod errors;
mod pandas;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::sync::Once;

static START: Once = Once::new();

// https://github.com/PyO3/pyo3-built/issues/21
// #[allow(dead_code)]
// mod build {
//     include!(concat!(env!("OUT_DIR"), "/built.rs"));
// }

#[pymodule]
fn connector_agent_python(_: Python, m: &PyModule) -> PyResult<()> {
    START.call_once(|| {
        let _ = env_logger::try_init();
    });

    m.add_wrapped(wrap_pyfunction!(write_pandas))?;

    Ok(())
}

#[pyfunction]
fn write_pandas<'a>(
    py: Python<'a>,
    conn: &str,
    queries: Vec<&str>,
    schema: Vec<&str>,
    checked: bool,
) -> PyResult<&'a PyAny> {
    Ok(crate::pandas::write_pandas(
        py, conn, &queries, &schema, checked,
    )?)
}
