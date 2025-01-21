use crate::errors::ConnectorXPythonError;
use arrow2::{
    array::Array,
    chunk::Chunk,
    datatypes::{Field, Schema},
    ffi,
};
use connectorx::source_router::SourceConn;
use connectorx::{prelude::*, sql::CXQuery};
use fehler::throws;
use libc::uintptr_t;
use pyo3::prelude::*;
use pyo3::{PyAny, Python};
use std::sync::Arc;

#[throws(ConnectorXPythonError)]
pub fn write_arrow<'py>(
    py: Python<'py>,
    source_conn: &SourceConn,
    origin_query: Option<String>,
    queries: &[CXQuery<String>],
    pre_execution_queries: Option<&[String]>,
) -> Bound<'py, PyAny> {
    let ptrs = py.allow_threads(
        || -> Result<(Vec<String>, Vec<Vec<(uintptr_t, uintptr_t)>>), ConnectorXPythonError> {
            let destination = get_arrow2(source_conn, origin_query, queries, pre_execution_queries)?;
            let (rbs, schema) = destination.arrow()?;
            Ok(to_ptrs(rbs, schema))
        },
    )?;
    let obj: PyObject = ptrs.into_py(py);
    obj.into_bound(py)
}

fn to_ptrs(
    rbs: Vec<Chunk<Box<dyn Array>>>,
    schema: Arc<Schema>,
) -> (Vec<String>, Vec<Vec<(uintptr_t, uintptr_t)>>) {
    if rbs.is_empty() {
        return (vec![], vec![]);
    }

    let mut result = vec![];
    let names = schema.fields.iter().map(|f| f.name.clone()).collect();

    for rb in rbs.into_iter() {
        let mut cols = vec![];

        for array in rb.into_arrays() {
            let schema_ptr =
                ffi::export_field_to_c(&Field::new("", array.data_type().clone(), true));
            let array_ptr = ffi::export_array_to_c(array);
            let array_ptr = Box::into_raw(Box::new(array_ptr));
            let schema_ptr = Box::into_raw(Box::new(schema_ptr));

            cols.push((array_ptr as uintptr_t, schema_ptr as uintptr_t));
        }

        result.push(cols);
    }
    (names, result)
}
