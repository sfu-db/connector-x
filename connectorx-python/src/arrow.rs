use crate::errors::ConnectorXPythonError;
use anyhow::anyhow;
use arrow::record_batch::RecordBatch;
use connectorx::source_router::SourceConn;
use connectorx::{prelude::*, sql::CXQuery};
use culpa::throws;
use libc::uintptr_t;
use pyo3::pyclass;
use pyo3::{prelude::*, IntoPyObjectExt};
use pyo3::{PyAny, Python};
use std::convert::TryFrom;
use std::sync::Arc;

/// Python-exposed RecordBatch wrapper
#[pyclass]
pub struct PyRecordBatch(Option<RecordBatch>);

/// Python-exposed iterator over RecordBatches
#[pyclass(module = "connectorx")]
pub struct PyRecordBatchIterator(Box<dyn RecordBatchIterator + Send + Sync>);

#[pymethods]
impl PyRecordBatch {
    pub fn num_rows(&self) -> usize {
        self.0.as_ref().map_or(0, |rb| rb.num_rows())
    }

    pub fn num_columns(&self) -> usize {
        self.0.as_ref().map_or(0, |rb| rb.num_columns())
    }

    #[throws(ConnectorXPythonError)]
    pub fn to_ptrs<'py>(&mut self, py: Python<'py>) -> Bound<'py, PyAny> {
        // Convert the RecordBatch to a vector of pointers, once the RecordBatch is taken, it cannot be reached again.
        let rb = self
            .0
            .take()
            .ok_or_else(|| anyhow!("RecordBatch is None, cannot convert to pointers"))?;
        let ptrs = py.detach(
            || -> Result<Vec<(uintptr_t, uintptr_t)>, ConnectorXPythonError> { Ok(to_ptrs_rb(rb)) },
        )?;
        let obj: Py<PyAny> = ptrs.into_py_any(py)?;
        obj.into_bound(py)
    }
}

#[pymethods]
impl PyRecordBatchIterator {
    #[throws(ConnectorXPythonError)]
    fn schema_ptr<'py>(&self, py: Python<'py>) -> Bound<'py, PyAny> {
        let (rb, _) = self.0.get_schema();
        let ptrs = py.detach(
            || -> Result<(Vec<String>, Vec<Vec<(uintptr_t, uintptr_t)>>), ConnectorXPythonError> {
                let rbs = vec![rb];
                Ok(to_ptrs(rbs))
            },
        )?;
        let obj: Py<PyAny> = ptrs.into_py_any(py)?;
        obj.into_bound(py)
    }
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__<'py>(
        mut slf: PyRefMut<'py, Self>,
        py: Python<'py>,
    ) -> PyResult<Option<Py<PyRecordBatch>>> {
        match slf.0.next_batch() {
            Some(rb) => {
                let wrapped = PyRecordBatch(Some(rb));
                let py_obj = Py::new(py, wrapped)?;
                Ok(Some(py_obj))
            }

            None => Ok(None),
        }
    }
}

#[throws(ConnectorXPythonError)]
pub fn write_arrow<'py>(
    py: Python<'py>,
    source_conn: &SourceConn,
    origin_query: Option<String>,
    queries: &[CXQuery<String>],
    pre_execution_queries: Option<&[String]>,
) -> Bound<'py, PyAny> {
    let ptrs = py.detach(
        || -> Result<(Vec<String>, Vec<Vec<(uintptr_t, uintptr_t)>>), ConnectorXPythonError> {
            let destination = get_arrow(source_conn, origin_query, queries, pre_execution_queries)?;
            let rbs = destination.arrow()?;
            Ok(to_ptrs(rbs))
        },
    )?;
    let obj: Py<PyAny> = ptrs.into_py_any(py)?;
    obj.into_bound(py)
}

#[throws(ConnectorXPythonError)]
pub fn get_arrow_rb_iter<'py>(
    py: Python<'py>,
    source_conn: &SourceConn,
    origin_query: Option<String>,
    queries: &[CXQuery<String>],
    pre_execution_queries: Option<&[String]>,
    batch_size: usize,
) -> Bound<'py, PyAny> {
    let mut arrow_iter: Box<dyn RecordBatchIterator> = new_record_batch_iter(
        source_conn,
        origin_query,
        queries,
        batch_size,
        pre_execution_queries,
    );

    arrow_iter.prepare();
    let py_rb_iter = PyRecordBatchIterator(unsafe {
        std::mem::transmute::<
            Box<dyn RecordBatchIterator>,
            Box<dyn RecordBatchIterator + Send + Sync>,
        >(arrow_iter)
    });

    let obj: Py<PyAny> = py_rb_iter.into_py_any(py)?;
    obj.into_bound(py)
}

pub fn to_ptrs_rb(rb: RecordBatch) -> Vec<(uintptr_t, uintptr_t)> {
    let mut cols = vec![];

    for array in rb.columns().into_iter() {
        let data = array.to_data();
        let array_ptr = Arc::new(arrow::ffi::FFI_ArrowArray::new(&data));
        let schema_ptr = Arc::new(
            arrow::ffi::FFI_ArrowSchema::try_from(data.data_type()).expect("export schema c"),
        );
        cols.push((
            Arc::into_raw(array_ptr) as uintptr_t,
            Arc::into_raw(schema_ptr) as uintptr_t,
        ));
    }

    cols
}

pub fn to_ptrs(rbs: Vec<RecordBatch>) -> (Vec<String>, Vec<Vec<(uintptr_t, uintptr_t)>>) {
    if rbs.is_empty() {
        return (vec![], vec![]);
    }

    let mut result = vec![];
    let names = rbs[0]
        .schema()
        .fields()
        .iter()
        .map(|f| f.name().clone())
        .collect();

    for rb in rbs.into_iter() {
        result.push(to_ptrs_rb(rb));
    }
    (names, result)
}
