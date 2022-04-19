#![feature(generic_associated_types)]
#![feature(fmt_internals)]
#![allow(incomplete_features)]

pub mod arrow;
pub mod arrow2;
pub mod constants;
mod errors;
pub mod pandas;
pub mod read_sql;
mod source_router;

use connectorx::fed_dispatcher::run;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, PyResult};
use std::convert::TryFrom;
use std::sync::Once;
use std::collections::HashMap;

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
    partition_query: Option<read_sql::PartitionQuery>,
) -> PyResult<&'a PyAny> {
    read_sql::read_sql(py, conn, return_type, protocol, queries, partition_query)
}

#[pyfunction]
pub fn partition_sql(
    conn: &str,
    partition_query: read_sql::PartitionQuery,
) -> PyResult<Vec<String>> {
    let source_conn = source_router::SourceConn::try_from(conn)?;
    let queries = read_sql::partition(&partition_query, &source_conn)?;
    Ok(queries.into_iter().map(|q| q.to_string()).collect())
}

#[pyfunction]
pub fn read_sql2<'a>(
    py: Python<'a>,
    sql: &str,
    db_map: HashMap<String, String>,
) -> PyResult<&'a PyAny> {
    let rbs = run(sql.to_string(), db_map).unwrap();
    let ptrs = arrow::to_ptrs(rbs);
    let obj: PyObject = ptrs.into_py(py);
    Ok(obj.into_ref(py))
}
