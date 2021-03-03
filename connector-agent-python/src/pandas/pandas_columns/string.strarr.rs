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
    string_buf: Vec<u8>,
    string_lengths: Vec<usize>,
    buf_size: usize,
    mutex: Arc<Mutex<()>>,
}

impl<'a> StringColumn<'a> {
    pub fn new(ob: &'a PyAny, mutex: Arc<Mutex<()>>) -> PyResult<Self> {
        check_dtype(ob, "string")?;
        let data = ob.getattr("_ndarray")?;
        check_dtype(data, "object")?;

        let buf_size_mb = 16; // in MB
        Ok(StringColumn {
            data: unsafe {
                data.downcast::<PyArray1<PyString>>()
                    .unwrap()
                    .as_array_mut()
            },
            next_write: 0,
            string_buf: Vec::with_capacity(buf_size_mb * 2 << 20),
            string_lengths: vec![],
            buf_size: buf_size_mb * 2 << 20,
            mutex,
        })
    }

    pub fn flush(&mut self) {
        let nstrings = self.string_lengths.len();

        if nstrings > 0 {
            let py = unsafe { Python::assume_gil_acquired() };

            {
                // allocation in python is not thread safe
                let _guard = self.mutex.lock().expect("Mutex Poisoned");
                let mut start = 0;
                for (i, &len) in self.string_lengths.iter().enumerate() {
                    let end = start + len;
                    if len != 0 {
                        self.data[self.next_write + i] =
                            PyString::new(py, &self.string_buf[start..end]);
                    }
                    start = end;
                }
            }

            let mut start = 0;
            for (i, len) in self.string_lengths.drain(..).enumerate() {
                let end = start + len;
                if len != 0 {
                    unsafe { self.data[self.next_write + i].write(&self.string_buf[start..end]) };
                }
                start = end;
            }

            self.string_buf.drain(..);
            self.next_write += nstrings;
        }
    }

    pub fn try_flush(&mut self) {
        if self.string_buf.len() >= self.buf_size {
            self.flush();
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
        self.string_lengths.push(val.len());
        self.string_buf.extend(val);
        self.try_flush();
    }
}

impl<'a> PandasColumn<Option<Bytes>> for StringColumn<'a> {
    fn write(&mut self, val: Option<Bytes>) {
        match val {
            Some(b) => {
                self.string_lengths.push(b.len());
                self.string_buf.extend(b);
                self.try_flush();
            }
            None => {
                self.string_lengths.push(0);
            }
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
                string_lengths: vec![],
                string_buf: Vec::with_capacity(self.buf_size),
                buf_size: self.buf_size,
                mutex: self.mutex.clone(),
            });
        }

        partitions
    }
}
