pub mod arrow;
pub mod arrow2;
pub mod constants;
mod errors;
pub mod pandas;
pub mod read_sql;

use crate::constants::J4RS_BASE_PATH;
use connectorx::fed_dispatcher::run;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, PyResult};
use std::collections::HashMap;
use std::env;
use std::sync::Once;

#[macro_use]
extern crate lazy_static;

static START: Once = Once::new();

// https://github.com/PyO3/pyo3-built/issues/21
// #[allow(dead_code)]
// mod build {
//     include!(concat!(env!("OUT_DIR"), "/built.rs"));
// }

#[pymodule]
fn connectorx(_: Python, m: &PyModule) -> PyResult<()> {
    START.call_once(|| {
        let _ = env_logger::try_init();
    });

    m.add_wrapped(wrap_pyfunction!(read_sql))?;
    m.add_wrapped(wrap_pyfunction!(read_sql2))?;
    m.add_wrapped(wrap_pyfunction!(partition_sql))?;
    m.add_wrapped(wrap_pyfunction!(get_meta))?;
    m.add_class::<pandas::PandasBlockInfo>()?;
    Ok(())
}

#[pyfunction]
pub fn read_sql<'a>(
    py: Python<'a>,
    conn: &str,
    return_type: &str,
    protocol: Option<&str>,
    queries: Option<Vec<String>>,
    partition_query: Option<read_sql::PyPartitionQuery>,
) -> PyResult<&'a PyAny> {
    read_sql::read_sql(py, conn, return_type, protocol, queries, partition_query)
}

#[pyfunction]
pub fn partition_sql(
    conn: &str,
    partition_query: read_sql::PyPartitionQuery,
) -> PyResult<Vec<String>> {
    let source_conn = connectorx::source_router::parse_source(conn, None)
        .map_err(|e| crate::errors::ConnectorXPythonError::from(e))?;
    let queries = connectorx::partition::partition(&partition_query.into(), &source_conn)
        .map_err(|e| crate::errors::ConnectorXPythonError::from(e))?;
    Ok(queries.into_iter().map(|q| q.to_string()).collect())
}

#[pyfunction]
pub fn read_sql2<'a>(
    py: Python<'a>,
    sql: &str,
    db_map: HashMap<String, String>,
) -> PyResult<&'a PyAny> {
    let rbs = run(
        sql.to_string(),
        db_map,
        Some(
            env::var("J4RS_BASE_PATH")
                .unwrap_or(J4RS_BASE_PATH.to_string())
                .as_str(),
        ),
    )
    .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))?;
    let ptrs = arrow::to_ptrs(rbs);
    let obj: PyObject = ptrs.into_py(py);
    Ok(obj.into_ref(py))
}

#[pyfunction]
pub fn get_meta<'a>(
    py: Python<'a>,
    conn: &str,
    protocol: Option<&str>,
    query: String,
) -> PyResult<&'a PyAny> {
    pandas::get_meta::get_meta(py, conn, protocol.unwrap_or("binary"), query)
        .map_err(|e| From::from(e))
}
