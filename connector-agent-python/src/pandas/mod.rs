mod funcs;
mod pandas_assoc;
mod writers;

use crate::errors::{ConnectorAgentPythonError, Result};
use crate::types::FromPandasType;
use connector_agent::{AnyArrayViewMut, DataType, Dispatcher, MixedSourceBuilder, Realize};
use fehler::throws;
use funcs::FSeriesStr;
use itertools::Itertools;
use ndarray::Ix2;
use numpy::array::PyArray;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use writers::PandasWriter;

#[throws(ConnectorAgentPythonError)]
pub fn write_pandas<'a>(nrows: &[usize], schema: &[&str], py: Python<'a>) -> PyObject {
    // convert schema
    let maybe_schema: Result<Vec<DataType>> = schema
        .into_iter()
        .map(|&s| FromPandasType::from(s))
        .collect();
    let schema = maybe_schema?;

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

    py.allow_threads(|| {
        // start dispatcher
        let ncols = schema.len();
        let queries: Vec<String> = nrows.iter().map(|v| format!("{},{}", v, ncols)).collect();
        let dispatcher = Dispatcher::new(
            MixedSourceBuilder {},
            PandasWriter::new(total_rows, schema.clone(), buffers, column_buffer_index),
            schema.clone(),
            queries,
        );
        dispatcher.run_checked().expect("run dispatcher");
    });

    // return dataframe
    let df = locals.get_item("df").expect("get df!");
    df.to_object(py)
}
