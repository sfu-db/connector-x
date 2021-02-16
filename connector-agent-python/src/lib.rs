#![feature(generic_associated_types)]
#![allow(incomplete_features)]

mod errors;
mod pandas;
mod types;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn connector_agent(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(write_pandas))?;
    Ok(())
}

#[pyfunction]
fn write_pandas(nrows: Vec<usize>, schema: Vec<&str>, py: Python) -> PyResult<PyObject> {
    Ok(crate::pandas::write_pandas(&nrows, &schema, py)?)
}
