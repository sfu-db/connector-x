#![feature(generic_associated_types)]
#![allow(incomplete_features)]

mod errors;
pub mod pandas;
pub mod read_sql;

use anyhow::Result;
use connector_agent::pg;
use pyo3::prelude::*;
use pyo3::{
    exceptions::PyValueError,
    types::{IntoPyDict, PyTuple},
    wrap_pyfunction, PyResult,
};
use std::sync::Once;
use tokio::runtime;

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

    m.add_wrapped(wrap_pyfunction!(read_pg))?;
    m.add_wrapped(wrap_pyfunction!(read_sql))?;
    Ok(())
}

#[pyfunction]
fn read_pg(py: Python, conn: &str, sqls: Vec<String>, schema: &str) -> PyResult<PyObject> {
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

#[pyfunction]
pub fn read_sql<'a>(
    py: Python<'a>,
    conn: &str,
    return_type: &str,
    protocol: Option<&str>,
    queries: Option<Vec<String>>,
    partition_query: Option<read_sql::PartitionQuery>,
) -> PyResult<&'a PyAny> {
    read_sql::read_sql(py, conn, return_type, protocol, queries, partition_query)
}
