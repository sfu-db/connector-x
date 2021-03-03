#![feature(generic_associated_types)]
#![allow(incomplete_features)]

mod errors;
pub mod pandas;
use fehler::{throw};
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
    m.add_wrapped(wrap_pyfunction!(read_sql))?;
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

#[pyfunction]
fn read_sql<'a>(
    py: Python<'a>,
    conn: &str,
    query: &str,
    col: &str,
    min: i64,
    max: i64,
    num: i64,
    return_type: &str,
) -> PyResult<&'a PyAny> {
    let mut queries:Vec<String> = vec![];
    let partition_size= match (max - min + 1) % num == 0 {
        true => (max - min + 1) / num,
        false => (max - min + 1) / num + 1
    };

    for i in 0..num {
        let lower = min + i * partition_size;
        let upper = min + (i + 1) * partition_size;
        let partition_query = format!("{} where {} >= {} and {} < {}", query, col, lower, col, upper);
        queries.push(partition_query);
    }
    let queries: Vec<_> = queries.iter().map(|s| s.as_str()).collect();
    match return_type {
        "pandas" => Ok(crate::pandas::write_pandas(py, conn, &queries, false)?),
        "arrow" => todo!(),
        _ => throw!(errors::ConnectorAgentPythonError::UnexpectedReturnType("pandas or arrow".to_string(), return_type.to_string())),
    }
}