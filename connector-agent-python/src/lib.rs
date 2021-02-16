#![feature(generic_associated_types)]
#![allow(incomplete_features)]

use crate::writers::pandas::{
    // funcs::{FArrayViewMut2, FSeriesStr},
    funcs::FSeriesStr,
    PandasWriter,
};
use connector_agent::{pg, s3, AnyArrayViewMut, DataType, Dispatcher, MixedSourceBuilder, Realize};
use failure::Fallible;
use itertools::Itertools;
use ndarray::Ix2;
use numpy::array::PyArray;
use phf::phf_map;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList, PyTuple};
use pyo3::wrap_pyfunction;
use tokio::runtime;

mod writers;

#[pymodule]
fn connector_agent(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_s3))?;
    m.add_wrapped(wrap_pyfunction!(read_pg))?;
    m.add_wrapped(wrap_pyfunction!(write_pandas))?;
    Ok(())
}

// mapping pandas datatype to connector agent datatype
static TYPE_MAPPING: phf::Map<&'static str, DataType> = phf_map! {
    "uint64" => DataType::U64,
    "float64" => DataType::F64,
    "bool" => DataType::Bool,
    "object" => DataType::String,
    "UInt64" => DataType::OptU64,
};

#[pyfunction]
fn write_pandas(nrows: Vec<usize>, schema: Vec<String>, py: Python) -> PyResult<PyObject> {
    // convert schema
    let schema: Vec<DataType> = schema
        .into_iter()
        .map(|s| TYPE_MAPPING[s.as_str()])
        .collect();

    // prepare python code to construct dataframe
    let total_rows: usize = nrows.iter().sum();
    let series: Vec<String> = schema
        .iter()
        .enumerate()
        .map(|(i, &dt)| Realize::<FSeriesStr>::realize(dt)(i, total_rows))
        .collect();
    let code = format!(
        r#"import pandas as pd
df = pd.DataFrame({{{}}})
blocks = [b.values for b in df._mgr.blocks]
index = [(i, j) for i, j in zip(df._mgr.blknos, df._mgr.blklocs)]"#,
        series.join(",")
    );
    // println!("# python code:\n{}", code);

    // run python code
    let locals = PyDict::new(py);
    py.run(code.as_str(), None, Some(locals)).unwrap();

    // get # of blocks in dataframe
    let buffers: &PyList = locals
        .get_item("blocks")
        .expect("get blocks!")
        .downcast::<PyList>()
        .unwrap();
    let nbuffers = buffers.len();

    // get index for each column: (index of block, index of column within the block)
    let column_buffer_index: Vec<(usize, usize)> = locals
        .get_item("index")
        .expect("get index!")
        .downcast::<PyList>()
        .unwrap()
        .iter()
        .map(|tuple| tuple.extract().unwrap())
        .collect();

    // get schema for each block, init by U64
    let mut block_schema_index = vec![DataType::U64; nbuffers];
    column_buffer_index
        .iter()
        .zip_eq(&schema)
        .for_each(|((b, _), s)| block_schema_index[*b] = *s);

    // get array view of each block so we can write data into using rust
    // TODO: cannot support multi-type using FArrayViewMut2 since PyArray does not support String and Option type
    let buffers = buffers
        .iter()
        .enumerate()
        // .map(|(i, array)| Realize::<FArrayViewMut2>::realize(block_schema_index[i])(array))
        .map(|(_i, array)| {
            let pyarray = array.downcast::<PyArray<u64, Ix2>>().unwrap();
            let mut_view = unsafe { pyarray.as_array_mut() };
            AnyArrayViewMut::<Ix2>::new(mut_view)
        })
        .collect();

    // start dispatcher
    let ncols = schema.len();
    let queries: Vec<String> = nrows.iter().map(|v| format!("{},{}", v, ncols)).collect();
    let dispatcher = Dispatcher::new(
        MixedSourceBuilder {},
        PandasWriter::new(total_rows, schema.clone(), buffers, column_buffer_index),
        schema.clone(),
        queries,
    );
    let _dw = dispatcher.run_checked().expect("run dispatcher");

    // return dataframe
    let df = locals.get_item("df").expect("get df!");
    PyResult::Ok(df.to_object(py))
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
