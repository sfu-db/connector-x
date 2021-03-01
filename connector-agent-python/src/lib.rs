#![feature(generic_associated_types)]
#![allow(incomplete_features)]

mod errors;
pub mod pandas;

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
    m.add_wrapped(wrap_pyfunction!(read_pg))?;

    Ok(())
}

#[pyfunction]
fn write_pandas<'a>(
    py: Python<'a>,
    conn: &str,
    queries: Vec<&str>,
    checked: bool,
) -> PyResult<&'a PyAny> {
    Ok(crate::pandas::write_pandas(py, conn, &queries, checked)?)
}

#[pyfunction]
fn read_pg(py: Python, conn: &str, sqls: Vec<String>, schema: &str) -> PyResult<PyObject> {
    use anyhow::Result;
    use connector_agent::pg;
    use pyo3::{
        exceptions::PyValueError,
        types::{IntoPyDict, PyTuple},
        PyResult,
    };
    use tokio::runtime;

    let ret: Result<Vec<(String, Vec<(isize, isize)>)>> = py.allow_threads(|| {
        let r = runtime::Runtime::new()?;
        let ret = r.block_on(pg::read_pg(conn, &sqls, schema))?;
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
