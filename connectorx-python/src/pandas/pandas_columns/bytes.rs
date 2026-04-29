use super::{
    check_dtype, ExtractBlockFromBound, HasPandasColumn, PandasColumn, PandasColumnObject,
    GIL_MUTEX,
};
use crate::errors::ConnectorXPythonError;
use anyhow::anyhow;
use fehler::throws;
use ndarray::{ArrayViewMut2, Axis, Ix2};
use numpy::{Element, PyArray, PyArrayDescr, PyArrayMethods};
use pyo3::{Bound, Py, PyAny, PyResult, Python};
use std::any::TypeId;

#[derive(Clone)]
#[repr(transparent)]
pub struct PyBytes(Py<pyo3::types::PyBytes>);

// In order to put it into a numpy array
unsafe impl Element for PyBytes {
    const IS_COPY: bool = false;
    fn get_dtype(py: Python<'_>) -> Bound<'_, PyArrayDescr> {
        PyArrayDescr::object(py)
    }

    fn clone_ref(&self, _py: Python<'_>) -> Self {
        Self(self.0.clone())
    }
}

pub struct BytesBlock<'a> {
    data: ArrayViewMut2<'a, PyBytes>,
    buf_size_mb: usize,
}

impl<'a> ExtractBlockFromBound<'a> for BytesBlock<'a> {
    fn extract_block<'b: 'a>(ob: &'b pyo3::Bound<'a, PyAny>) -> PyResult<Self> {
        check_dtype(ob, "object")?;
        let array = ob.cast::<PyArray<PyBytes, Ix2>>()?;
        let data = unsafe { array.as_array_mut() };
        Ok(BytesBlock {
            data,
            buf_size_mb: 16, // in MB
        })
    }
}

impl<'a> BytesBlock<'a> {
    #[throws(ConnectorXPythonError)]
    pub fn split(self) -> Vec<BytesColumn> {
        let mut ret = vec![];
        let mut view = self.data;

        let nrows = view.ncols();
        while view.nrows() > 0 {
            let (col, rest) = view.split_at(Axis(0), 1);
            view = rest;
            ret.push(BytesColumn {
                data: col
                    .into_shape_with_order(nrows)?
                    .into_slice()
                    .ok_or_else(|| anyhow!("get None for splitted String data"))?
                    .as_mut_ptr(),
                bytes_lengths: vec![],
                row_idx: vec![],
                bytes_buf: Vec::with_capacity(self.buf_size_mb * (1 << 20) * 11 / 10), // allocate a little bit more memory to avoid Vec growth
                buf_size: self.buf_size_mb * (1 << 20),
            })
        }
        ret
    }
}

pub struct BytesColumn {
    data: *mut PyBytes,
    bytes_buf: Vec<u8>,
    bytes_lengths: Vec<usize>, // usize::MAX if the string is None
    row_idx: Vec<usize>,
    buf_size: usize,
}

unsafe impl Send for BytesColumn {}
unsafe impl Sync for BytesColumn {}

impl PandasColumnObject for BytesColumn {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<&'static [u8]>() || id == TypeId::of::<Option<&'static [u8]>>()
    }
    fn typename(&self) -> &'static str {
        std::any::type_name::<&'static [u8]>()
    }
    #[throws(ConnectorXPythonError)]
    fn finalize(&mut self) {
        self.flush()?;
    }
}

impl PandasColumn<Vec<u8>> for BytesColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: Vec<u8>, row: usize) {
        self.bytes_lengths.push(val.len());
        self.bytes_buf.extend_from_slice(&val[..]);
        self.row_idx.push(row);
        self.try_flush()?;
    }
}

impl<'r> PandasColumn<&'r [u8]> for BytesColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: &'r [u8], row: usize) {
        self.bytes_lengths.push(val.len());
        self.bytes_buf.extend_from_slice(val);
        self.row_idx.push(row);
        self.try_flush()?;
    }
}

impl PandasColumn<Option<Vec<u8>>> for BytesColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: Option<Vec<u8>>, row: usize) {
        match val {
            Some(b) => {
                self.bytes_lengths.push(b.len());
                self.bytes_buf.extend_from_slice(&b[..]);
                self.row_idx.push(row);
                self.try_flush()?;
            }
            None => {
                self.bytes_lengths.push(usize::MAX);
                self.row_idx.push(row);
            }
        }
    }
}

impl<'r> PandasColumn<Option<&'r [u8]>> for BytesColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: Option<&'r [u8]>, row: usize) {
        match val {
            Some(b) => {
                self.bytes_lengths.push(b.len());
                self.bytes_buf.extend_from_slice(b);
                self.row_idx.push(row);
                self.try_flush()?;
            }
            None => {
                self.bytes_lengths.push(usize::MAX);
                self.row_idx.push(row);
            }
        }
    }
}

impl HasPandasColumn for Vec<u8> {
    type PandasColumn<'a> = BytesColumn;
}

impl HasPandasColumn for Option<Vec<u8>> {
    type PandasColumn<'a> = BytesColumn;
}

impl<'r> HasPandasColumn for &'r [u8] {
    type PandasColumn<'a> = BytesColumn;
}

impl<'r> HasPandasColumn for Option<&'r [u8]> {
    type PandasColumn<'a> = BytesColumn;
}

impl BytesColumn {
    pub fn partition(self, counts: usize) -> Vec<BytesColumn> {
        let mut partitions = vec![];

        for _ in 0..counts {
            partitions.push(BytesColumn {
                data: self.data,
                bytes_lengths: vec![],
                row_idx: vec![],
                bytes_buf: Vec::with_capacity(self.buf_size),
                buf_size: self.buf_size,
            });
        }

        partitions
    }

    #[throws(ConnectorXPythonError)]
    pub fn flush(&mut self) {
        let nstrings = self.bytes_lengths.len();

        if nstrings > 0 {
            Python::attach(|py| -> Result<(), ConnectorXPythonError> {
                // allocation in python is not thread safe
                let _guard = GIL_MUTEX
                    .lock()
                    .map_err(|e| anyhow!("mutex poisoned {}", e))?;
                let mut start = 0;
                for (i, &len) in self.bytes_lengths.iter().enumerate() {
                    if len != usize::MAX {
                        let end = start + len;
                        unsafe {
                            // allocate and write in the same time
                            let b =
                                pyo3::types::PyBytes::new(py, &self.bytes_buf[start..end]).unbind();
                            *self.data.add(self.row_idx[i]) = PyBytes(b);
                        };
                        start = end;
                    } else {
                        unsafe {
                            let b = Py::from_borrowed_ptr(py, pyo3::ffi::Py_None());
                            *self.data.add(self.row_idx[i]) = PyBytes(b);
                        }
                    }
                }
                Ok(())
            })?;

            self.bytes_buf.truncate(0);
            self.bytes_lengths.truncate(0);
            self.row_idx.truncate(0);
        }
    }

    #[throws(ConnectorXPythonError)]
    pub fn try_flush(&mut self) {
        if self.bytes_buf.len() >= self.buf_size {
            self.flush()?;
        }
    }
}
