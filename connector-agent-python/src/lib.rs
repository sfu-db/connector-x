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
    m.add_wrapped(wrap_pyfunction!(test))?;
    Ok(())
}

#[pyfunction]
fn test() -> PyResult<()> {
    use arrow::csv;
    use arrow::datatypes::{DataType, DateUnit, Field, Schema};
    use std::fs::File;
    use std::sync::Arc;

    let schema = Schema::new(vec![
        Field::new("l_orderkey", DataType::UInt64, false),
        Field::new("l_partkey", DataType::UInt64, false),
        Field::new("l_suppkey", DataType::UInt64, false),
        Field::new("l_linenumber", DataType::UInt64, false),
        Field::new("l_quantity", DataType::Float64, false),
        Field::new("l_extendedprice", DataType::Float64, false),
        Field::new("l_discount", DataType::Float64, false),
        Field::new("l_tax", DataType::Float64, false),
        Field::new("l_returnflag", DataType::Utf8, false),
        Field::new("l_linestatus", DataType::Utf8, false),
        Field::new("l_shipdate", DataType::Date32(DateUnit::Day), false),
        Field::new("l_commitdate", DataType::Date32(DateUnit::Day), false),
        Field::new("l_receiptdate", DataType::Date32(DateUnit::Day), false),
        Field::new("l_shipinstruct", DataType::Utf8, false),
        Field::new("l_shipmode", DataType::Utf8, false),
        Field::new("l_comment", DataType::Utf8, false),
    ]);
    // use std::fs::File;
    // use arrow::csv;
    // let file = File::open("tmp.csv")?;
    // let mut csv = csv::Reader::new(file, schema.clone(), false, None, 1024, None, None);
    // let batch = csv.next().unwrap().unwrap();
    // println!("{} {}", batch.num_columns(), batch.num_rows());

    let file = File::open("tmp.csv").unwrap();
    let mut csv = csv::Reader::new(file, Arc::new(schema), true, Some(b','), 1024, None, None);
    let batch = csv.next().unwrap().unwrap();
    println!("{} {}", batch.num_columns(), batch.num_rows());
    Ok(())
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
