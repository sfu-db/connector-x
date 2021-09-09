use super::{check_dtype, HasPandasColumn, PandasColumn, PandasColumnObject};
use crate::errors::ConnectorXPythonError;
use anyhow::anyhow;
use fehler::throws;
use itertools::Itertools;
use ndarray::{ArrayViewMut2, Axis, Ix2};
use numpy::{npyffi::NPY_TYPES, Element, PyArray, PyArrayDescr};
use pyo3::{FromPyObject, Py, PyAny, PyResult, Python};
use std::any::TypeId;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
#[repr(transparent)]
pub struct PyList(Py<pyo3::types::PyList>);

// In order to put it into a numpy array
impl Element for PyList {
    const DATA_TYPE: numpy::DataType = numpy::DataType::Object;
    fn is_same_type(dtype: &PyArrayDescr) -> bool {
        unsafe { *dtype.as_dtype_ptr() }.type_num == NPY_TYPES::NPY_OBJECT as i32
    }
}

pub struct FloatArrayBlock<'a> {
    data: ArrayViewMut2<'a, PyList>,
    mutex: Arc<Mutex<()>>,
    buf_size_mb: usize,
}

impl<'a> FromPyObject<'a> for FloatArrayBlock<'a> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        check_dtype(ob, "object")?;
        let array = ob.downcast::<PyArray<PyList, Ix2>>()?;
        let data = unsafe { array.as_array_mut() };
        Ok(FloatArrayBlock {
            data,
            mutex: Arc::new(Mutex::new(())), // allocate the lock here since only a few blocks needs to aquire the GIL for now
            buf_size_mb: 16,                 // in MB
        })
    }
}

impl<'a> FloatArrayBlock<'a> {
    #[throws(ConnectorXPythonError)]
    pub fn split(self) -> Vec<FloatArrayColumn<'a>> {
        let mut ret = vec![];
        let mut view = self.data;

        let nrows = view.ncols();
        while view.nrows() > 0 {
            let (col, rest) = view.split_at(Axis(0), 1);
            view = rest;
            ret.push(FloatArrayColumn {
                data: col
                    .into_shape(nrows)?
                    .into_slice()
                    .ok_or_else(|| anyhow!("get None for splitted FloatArray data"))?,
                next_write: 0,
                lengths: vec![],
                buffer: Vec::with_capacity(self.buf_size_mb * (1 << 17) * 11 / 10), // allocate a little bit more memory to avoid Vec growth
                buf_size: self.buf_size_mb * (1 << 17),
                mutex: self.mutex.clone(),
            })
        }
        ret
    }
}

pub struct FloatArrayColumn<'a> {
    data: &'a mut [PyList],
    next_write: usize,
    buffer: Vec<f64>,
    lengths: Vec<usize>, // usize::MAX if the string is None
    buf_size: usize,
    mutex: Arc<Mutex<()>>,
}

impl<'a> PandasColumnObject for FloatArrayColumn<'a> {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<PyList>() || id == TypeId::of::<Option<PyList>>()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn typename(&self) -> &'static str {
        std::any::type_name::<PyList>()
    }
}

impl<'a> PandasColumn<Vec<f64>> for FloatArrayColumn<'a> {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: Vec<f64>) {
        self.lengths.push(val.len());
        self.buffer.extend_from_slice(&val[..]);
        self.try_flush()?;
    }
}

impl<'a> PandasColumn<Option<Vec<f64>>> for FloatArrayColumn<'a> {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: Option<Vec<f64>>) {
        match val {
            Some(v) => {
                self.lengths.push(v.len());
                self.buffer.extend_from_slice(&v[..]);
                self.try_flush()?;
            }
            None => {
                self.lengths.push(usize::MAX);
            }
        }
    }
}

impl HasPandasColumn for Vec<f64> {
    type PandasColumn<'a> = FloatArrayColumn<'a>;
}

impl HasPandasColumn for Option<Vec<f64>> {
    type PandasColumn<'a> = FloatArrayColumn<'a>;
}

impl<'a> FloatArrayColumn<'a> {
    pub fn partition(self, counts: &[usize]) -> Vec<FloatArrayColumn<'a>> {
        let mut partitions = vec![];
        let mut data = self.data;

        for &c in counts {
            let (splitted, rest) = data.split_at_mut(c);
            data = rest;
            partitions.push(FloatArrayColumn {
                data: splitted,
                next_write: 0,
                lengths: vec![],
                buffer: Vec::with_capacity(self.buf_size),
                buf_size: self.buf_size,
                mutex: self.mutex.clone(),
            });
        }
        partitions
    }

    #[throws(ConnectorXPythonError)]
    pub fn flush(&mut self) {
        let nvecs = self.lengths.len();

        if nvecs > 0 {
            let py = unsafe { Python::assume_gil_acquired() };

            {
                // allocation in python is not thread safe
                let _guard = self
                    .mutex
                    .lock()
                    .map_err(|e| anyhow!("mutex poisoned {}", e))?;
                let mut start = 0;
                for (i, &len) in self.lengths.iter().enumerate() {
                    if len != usize::MAX {
                        let end = start + len;
                        unsafe {
                            // allocate and write in the same time
                            *self.data.get_unchecked_mut(self.next_write + i) = PyList(
                                pyo3::types::PyList::new(py, &self.buffer[start..end]).into(),
                            );
                        };
                        start = end;
                    } else {
                        unsafe {
                            let n: &pyo3::types::PyList =
                                py.from_borrowed_ptr(pyo3::ffi::Py_None());
                            *self.data.get_unchecked_mut(self.next_write + i) = PyList(n.into());
                        }
                    }
                }
            }

            self.buffer.truncate(0);
            self.lengths.truncate(0);
            self.next_write += nvecs;
        }
    }

    #[throws(ConnectorXPythonError)]
    pub fn try_flush(&mut self) {
        if self.buffer.len() >= self.buf_size {
            self.flush()?;
        }
    }
}
