use super::{check_dtype, HasPandasColumn, PandasColumn, PandasColumnObject};
use anyhow::anyhow;
use connectorx::ConnectorAgentError;
use fehler::throws;
use ndarray::{ArrayViewMut2, Axis, Ix2};
use numpy::{npyffi::NPY_TYPES, Element, PyArray, PyArrayDescr};
use pyo3::{FromPyObject, Py, PyAny, PyResult, Python};
use std::any::TypeId;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
#[repr(transparent)]
pub struct PyBytes(Py<pyo3::types::PyBytes>);

// In order to put it into a numpy array
impl Element for PyBytes {
    const DATA_TYPE: numpy::DataType = numpy::DataType::Object;
    fn is_same_type(dtype: &PyArrayDescr) -> bool {
        unsafe { *dtype.as_dtype_ptr() }.type_num == NPY_TYPES::NPY_OBJECT as i32
    }
}

pub struct BytesBlock<'a> {
    data: ArrayViewMut2<'a, PyBytes>,
    mutex: Arc<Mutex<()>>,
    buf_size_mb: usize,
}

impl<'a> FromPyObject<'a> for BytesBlock<'a> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        check_dtype(ob, "object")?;
        let array = ob.downcast::<PyArray<PyBytes, Ix2>>()?;
        let data = unsafe { array.as_array_mut() };
        Ok(BytesBlock {
            data,
            mutex: Arc::new(Mutex::new(())), // allocate the lock here since only BytesBlock needs to aquire the GIL for now
            buf_size_mb: 16,                 // in MB
        })
    }
}

impl<'a> BytesBlock<'a> {
    #[throws(ConnectorAgentError)]
    pub fn split(self) -> Vec<BytesColumn<'a>> {
        let mut ret = vec![];
        let mut view = self.data;

        let nrows = view.ncols();
        while view.nrows() > 0 {
            let (col, rest) = view.split_at(Axis(0), 1);
            view = rest;
            ret.push(BytesColumn {
                data: col
                    .into_shape(nrows)?
                    .into_slice()
                    .ok_or_else(|| anyhow!("get None for splitted String data"))?,
                next_write: 0,
                bytes_lengths: vec![],
                bytes_buf: Vec::with_capacity(self.buf_size_mb * 2 << 20 * 11 / 10), // allocate a little bit more memory to avoid Vec growth
                buf_size: self.buf_size_mb * 2 << 20,
                mutex: self.mutex.clone(),
            })
        }
        ret
    }
}

pub struct BytesColumn<'a> {
    data: &'a mut [PyBytes],
    next_write: usize,
    bytes_buf: Vec<u8>,
    bytes_lengths: Vec<isize>,
    buf_size: usize,
    mutex: Arc<Mutex<()>>,
}

impl<'a> PandasColumnObject for BytesColumn<'a> {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<&'static [u8]>() || id == TypeId::of::<Option<&'static [u8]>>()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn typename(&self) -> &'static str {
        std::any::type_name::<&'static [u8]>()
    }
    #[throws(ConnectorAgentError)]
    fn finalize(&mut self) {
        self.flush()?;
    }
}

impl<'a> PandasColumn<Vec<u8>> for BytesColumn<'a> {
    #[throws(ConnectorAgentError)]
    fn write(&mut self, val: Vec<u8>) {
        self.bytes_lengths.push(val.len() as isize);
        self.bytes_buf.extend_from_slice(&val[..]);
        self.try_flush()?;
    }
}

impl<'a> PandasColumn<Option<Vec<u8>>> for BytesColumn<'a> {
    #[throws(ConnectorAgentError)]
    fn write(&mut self, val: Option<Vec<u8>>) {
        match val {
            Some(b) => {
                self.bytes_lengths.push(b.len() as isize);
                self.bytes_buf.extend_from_slice(&b[..]);
                self.try_flush()?;
            }
            None => {
                self.bytes_lengths.push(-1);
            }
        }
    }
}

impl HasPandasColumn for Vec<u8> {
    type PandasColumn<'a> = BytesColumn<'a>;
}

impl HasPandasColumn for Option<Vec<u8>> {
    type PandasColumn<'a> = BytesColumn<'a>;
}

impl<'a> BytesColumn<'a> {
    pub fn partition(self, counts: &[usize]) -> Vec<BytesColumn<'a>> {
        let mut partitions = vec![];
        let mut data = self.data;

        for &c in counts {
            let (splitted_data, rest) = data.split_at_mut(c);
            data = rest;

            partitions.push(BytesColumn {
                data: splitted_data,
                next_write: 0,
                bytes_lengths: vec![],
                bytes_buf: Vec::with_capacity(self.buf_size),
                buf_size: self.buf_size,
                mutex: self.mutex.clone(),
            });
        }

        partitions
    }

    #[throws(ConnectorAgentError)]
    pub fn flush(&mut self) {
        let nstrings = self.bytes_lengths.len();

        if nstrings > 0 {
            let py = unsafe { Python::assume_gil_acquired() };

            {
                // allocation in python is not thread safe
                let _guard = self
                    .mutex
                    .lock()
                    .map_err(|e| anyhow!("mutex poisoned {}", e))?;
                let mut start = 0 as usize;
                for (i, &len) in self.bytes_lengths.iter().enumerate() {
                    if len >= 0 {
                        let end = start + (len as usize);
                        unsafe {
                            // allocate and write in the same time
                            *self.data.get_unchecked_mut(self.next_write + i) = PyBytes(
                                pyo3::types::PyBytes::new(py, &self.bytes_buf[start..end]).into(),
                            );
                        };
                        start = end;
                    } else {
                        unsafe {
                            let b: &pyo3::types::PyBytes =
                                py.from_borrowed_ptr(pyo3::ffi::Py_None());

                            *self.data.get_unchecked_mut(self.next_write + i) = PyBytes(b.into());
                        }
                    }
                }
            }

            self.bytes_buf.truncate(0);
            self.next_write += nstrings;
        }
    }

    #[throws(ConnectorAgentError)]
    pub fn try_flush(&mut self) {
        if self.bytes_buf.len() >= self.buf_size {
            self.flush()?;
        }
    }
}
