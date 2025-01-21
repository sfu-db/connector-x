pub mod arrow;
pub mod arrow2;
pub mod constants;
pub mod cx_read_sql;
mod errors;
pub mod pandas;

use crate::constants::J4RS_BASE_PATH;
use ::connectorx::{fed_dispatcher::run, partition::partition, source_router::parse_source};
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
fn connectorx(_: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
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
pub fn read_sql<'py>(
    py: Python<'py>,
    conn: &str,
    return_type: &str,
    protocol: Option<&str>,
    queries: Option<Vec<String>>,
    partition_query: Option<cx_read_sql::PyPartitionQuery>,
    pre_execution_queries: Option<Vec<String>>,
) -> PyResult<Bound<'py, PyAny>> {
    cx_read_sql::read_sql(py, conn, return_type, protocol, queries, partition_query, pre_execution_queries)
}

#[pyfunction]
pub fn partition_sql(
    conn: &str,
    partition_query: cx_read_sql::PyPartitionQuery,
) -> PyResult<Vec<String>> {
    let source_conn =
        parse_source(conn, None).map_err(|e| crate::errors::ConnectorXPythonError::from(e))?;
    let queries = partition(&partition_query.into(), &source_conn)
        .map_err(|e| crate::errors::ConnectorXPythonError::from(e))?;
    Ok(queries.into_iter().map(|q| q.to_string()).collect())
}

#[pyfunction]
pub fn read_sql2<'py>(
    py: Python<'py>,
    sql: &str,
    db_map: HashMap<String, String>,
    strategy: Option<&str>,
) -> PyResult<Bound<'py, PyAny>> {
    let rbs = run(
        sql.to_string(),
        db_map,
        Some(
            env::var("J4RS_BASE_PATH")
                .unwrap_or(J4RS_BASE_PATH.to_string())
                .as_str(),
        ),
        strategy.unwrap_or("pushdown"),
    )
    .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))?;
    let ptrs = arrow::to_ptrs(rbs);
    let obj: PyObject = ptrs.into_py(py);
    Ok(obj.into_bound(py))
}

#[pyfunction]
pub fn get_meta<'py>(
    py: Python<'py>,
    conn: &str,
    query: String,
    protocol: Option<&str>,
) -> PyResult<Bound<'py, PyAny>> {
    pandas::get_meta::get_meta(py, conn, protocol.unwrap_or("binary"), query)
        .map_err(|e| From::from(e))
}
