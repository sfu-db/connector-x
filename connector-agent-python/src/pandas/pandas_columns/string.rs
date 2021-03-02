use super::super::pystring::PyString;
use super::{check_dtype, HasPandasColumn, PandasColumn, PandasColumnObject};
use bytes::Bytes;
use ndarray::{ArrayViewMut1, Axis};
use numpy::PyArray1;
use pyo3::{PyAny, PyResult, Python};
use std::any::TypeId;
use std::sync::{Arc, Mutex};

pub struct StringColumn<'a> {
    data: ArrayViewMut1<'a, PyString>,
    next_write: usize,
    local_buf: Vec<Option<Bytes>>,
    buf_size: usize,
    mutex: Arc<Mutex<()>>,
}

impl<'a> StringColumn<'a> {
    pub fn new(ob: &'a PyAny, mutex: Arc<Mutex<()>>) -> PyResult<Self> {
        check_dtype(ob, "string")?;
        let data = ob.getattr("_ndarray")?;
        check_dtype(data, "object")?;

        let buf_size = 40960;
        Ok(StringColumn {
            data: unsafe {
                data.downcast::<PyArray1<PyString>>()
                    .unwrap()
                    .as_array_mut()
            },
            next_write: 0,
            local_buf: Vec::with_capacity(buf_size),
            buf_size: buf_size,
            mutex,
        })
    }

    pub fn flush(&mut self) {
        let buflen = self.local_buf.len();

        if buflen > 0 {
            let py = unsafe { Python::assume_gil_acquired() };

            {
                // allocation in python is not thread safe
                let _guard = self.mutex.lock().expect("Mutex Poisoned");
                for (i, b) in self.local_buf.iter().enumerate() {
                    if let Some(b) = b {
                        self.data[self.next_write + i] = PyString::new(py, b);
                    }
                }
            }

            for (i, b) in self.local_buf.drain(..).enumerate() {
                if let Some(b) = b {
                    unsafe { self.data[self.next_write + i].write(&b) };
                }
            }

            self.next_write += buflen;
        }
    }
}

impl<'a> PandasColumnObject for StringColumn<'a> {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<Bytes>() || id == TypeId::of::<Option<Bytes>>()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn typename(&self) -> &'static str {
        std::any::type_name::<Bytes>()
    }
    fn finalize(&mut self) {
        self.flush()
    }
}

impl<'a> PandasColumn<Bytes> for StringColumn<'a> {
    fn write(&mut self, val: Bytes) {
        self.local_buf.push(Some(val));
        if self.local_buf.len() >= self.buf_size {
            self.flush();
        }
    }
}

impl<'a> PandasColumn<Option<Bytes>> for StringColumn<'a> {
    fn write(&mut self, val: Option<Bytes>) {
        self.local_buf.push(val);
        if self.local_buf.len() >= self.buf_size {
            self.flush();
        }
    }
}

impl HasPandasColumn for Bytes {
    type PandasColumn<'a> = StringColumn<'a>;
}

impl HasPandasColumn for Option<Bytes> {
    type PandasColumn<'a> = StringColumn<'a>;
}

impl<'a> StringColumn<'a> {
    pub fn partition(self, counts: &[usize]) -> Vec<StringColumn<'a>> {
        let mut partitions = vec![];
        let mut data = self.data;

        for &c in counts {
            let (splitted_data, rest) = data.split_at(Axis(0), c);
            data = rest;

            partitions.push(StringColumn {
                data: splitted_data,
                next_write: 0,
                local_buf: Vec::with_capacity(self.buf_size),
                buf_size: self.buf_size,
                mutex: self.mutex.clone(),
            });
        }

        partitions
    }
}
