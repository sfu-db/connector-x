#![feature(generic_associated_types)]
#![allow(incomplete_features)]

pub mod arrow;
mod errors;
pub mod pandas;
pub mod read_sql;
mod source_router;

use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, PyResult};
use std::sync::Once;

static START: Once = Once::new();

// https://github.com/PyO3/pyo3-built/issues/21
// #[allow(dead_code)]
// mod build {
//     include!(concat!(env!("OUT_DIR"), "/built.rs"));
// }

#[pymodule]
fn connectorx_python(_: Python, m: &PyModule) -> PyResult<()> {
    START.call_once(|| {
        let _ = env_logger::try_init();
    });

    m.add_wrapped(wrap_pyfunction!(read_sql))?;
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
