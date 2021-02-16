#![feature(generic_associated_types)]
#![allow(incomplete_features)]

mod errors;
mod pandas;
mod types;

use connector_agent::{pg, s3};
use failure::Fallible;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyTuple};
use pyo3::wrap_pyfunction;
use tokio::runtime;

#[pymodule]
fn connector_agent(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_s3))?;
    m.add_wrapped(wrap_pyfunction!(read_pg))?;
    m.add_wrapped(wrap_pyfunction!(write_pandas))?;
    Ok(())
}

#[pyfunction]
fn write_pandas(nrows: Vec<usize>, schema: Vec<&str>, py: Python) -> PyResult<PyObject> {
    Ok(crate::pandas::write_pandas(&nrows, &schema, py)?)
}

#[pyfunction]
fn read_s3(
    bucket: &str,
    objects: Vec<String>,
    schema: &str,
    json_format: &str,
    py: Python,
) -> PyResult<PyObject> {
    let ret: Fallible<Vec<(String, Vec<(isize, isize)>)>> = py.allow_threads(|| {
        let r = runtime::Runtime::new()?;

        let ret = r.block_on(s3::read_s3(bucket, &objects, schema, json_format.parse()?))?;
        Ok(ret
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    v.into_iter()
                        .map(|(a, b)| (a as isize, b as isize))
                        .collect(),
                )
            })
            .collect())
    });

    let ret: Vec<_> = ret
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?
        .into_iter()
        .map(|(k, v)| (k, PyTuple::new(py, v)))
        .collect();
    PyResult::Ok(ret.into_py_dict(py).to_object(py))
}

#[pyfunction]
fn read_pg(sqls: Vec<String>, schema: &str, py: Python) -> PyResult<PyObject> {
    let ret: Fallible<Vec<(String, Vec<(isize, isize)>)>> = py.allow_threads(|| {
        let r = runtime::Runtime::new()?;
        let ret = r.block_on(pg::read_pg(&sqls, schema))?;
        Ok(ret
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    v.into_iter()
                        .map(|(a, b)| (a as isize, b as isize))
                        .collect(),
                )
            })
            .collect())
    });

    let ret: Vec<_> = ret
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?
        .into_iter()
        .map(|(k, v)| (k, PyTuple::new(py, v)))
        .collect();
    PyResult::Ok(ret.into_py_dict(py).to_object(py))
}
