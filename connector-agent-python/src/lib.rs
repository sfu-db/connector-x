#![feature(generic_associated_types)]
#![allow(incomplete_features)]

mod errors;
pub mod pandas;
use anyhow::Result;
use connector_agent::partition::pg_single_col_partition_query;
use connector_agent::pg;
use dict_derive::FromPyObject;
use fehler::throw;
use pyo3::prelude::*;
use pyo3::{
    exceptions::{PyNotImplementedError, PyValueError},
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

#[derive(FromPyObject)]
pub struct PartitionQuery {
    query: String,
    column: String,
    min: i64,
    max: i64,
    num: usize,
}

#[pyfunction]
fn read_sql<'a>(
    py: Python<'a>,
    conn: &str,
    return_type: &str,
    queries: Option<Vec<String>>,
    partition_query: Option<PartitionQuery>,
) -> PyResult<&'a PyAny> {
    let queries = match (queries, partition_query) {
        (Some(queries), None) => queries,
        (
            None,
            Some(PartitionQuery {
                query,
                column: col,
                min,
                max,
                num,
            }),
        ) => {
            let mut queries = vec![];
            let num = num as i64;
            let partition_size = match (max - min + 1) % num == 0 {
                true => (max - min + 1) / num,
                false => (max - min + 1) / num + 1,
            };

            for i in 0..num {
                let lower = min + i * partition_size;
                let upper = min + (i + 1) * partition_size;
                let partition_query = pg_single_col_partition_query(&query, &col, lower, upper);
                queries.push(partition_query);
            }
            queries
        }
        (Some(_), Some(_)) => throw!(PyValueError::new_err(
            "partition_query and queries cannot be both specified",
        )),
        (None, None) => throw!(PyValueError::new_err(
            "partition_query and queries cannot be both None",
        )),
    };

    let queries: Vec<_> = queries.iter().map(|s| s.as_str()).collect();
    match return_type {
        "pandas" => Ok(crate::pandas::write_pandas(py, conn, &queries)?),
        "arrow" => Err(PyNotImplementedError::new_err(
            "arrow return type is not implemented",
        )),
        _ => Err(PyValueError::new_err(format!(
            "return type should be 'pandas' or 'arrow', got '{}'",
            return_type
        ))),
    }
}
