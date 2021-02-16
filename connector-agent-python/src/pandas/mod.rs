mod funcs;
mod pandas_assoc;
mod writers;

use crate::errors::{ConnectorAgentPythonError, Result};
use crate::types::FromPandasType;
use connector_agent::{DataType, Dispatcher, MixedSourceBuilder, Realize};
use fehler::throws;
use funcs::FSeriesStr;
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

    let total_rows: usize = nrows.iter().sum();

    let (df, buffers, index) = create_dataframe(py, &schema, total_rows);

    let writer = PandasWriter::new(total_rows, &schema, buffers, index);
    let sb = MixedSourceBuilder {};

    // unblock python threads when copying the data
    py.allow_threads(|| -> Result<()> {
        let ncols = schema.len();
        let queries: Vec<String> = nrows.iter().map(|v| format!("{},{}", v, ncols)).collect();
        let dispatcher = Dispatcher::new(sb, writer, &schema, queries);
        dispatcher.run_checked()?;
        Ok(())
    })?;

    // return the dataframe
    df.to_object(py)
}

/// call python code to construct the dataframe and expose its buffers
fn create_dataframe<'a>(
    py: Python<'a>,
    schema: &[DataType],
    nrows: usize,
) -> (&'a PyAny, &'a PyList, &'a PyList) {
    let series: Vec<String> = schema
        .iter()
        .enumerate()
        .map(|(i, &dt)| Realize::<FSeriesStr>::realize(dt)(i, nrows))
        .collect();
    let code = format!(
        r#"import pandas as pd
df = pd.DataFrame({{{}}})
blocks = [b.values for b in df._mgr.blocks]
index = [(i, j) for i, j in zip(df._mgr.blknos, df._mgr.blklocs)]"#,
        series.join(",")
    );

    // run python code
    let locals = PyDict::new(py);
    py.run(code.as_str(), None, Some(locals)).unwrap();

    // get # of blocks in dataframe
    let buffers: &PyList = locals
        .get_item("blocks")
        .expect("cannot get `blocks` from locals")
        .downcast::<PyList>()
        .expect("cannot downcast `blocks` to PyList");

    let index = locals
        .get_item("index")
        .expect("cannot get `index` from locals")
        .downcast::<PyList>()
        .expect("cannot downcast `index` to PyList");

    let df = locals.get_item("df").expect("cannot get `df` from locals");

    (df, buffers, index)
}
