use super::super::pystring::PyString;
use super::{check_dtype, HasPandasColumn, PandasColumn, PandasColumnObject};
use ndarray::{ArrayViewMut2, Axis, Ix2};
use numpy::PyArray;
use pyo3::{FromPyObject, PyAny, PyResult, Python};
use std::any::TypeId;
use std::sync::{Arc, Mutex};

pub struct StringBlock<'a> {
    data: ArrayViewMut2<'a, PyString>,
    mutex: Arc<Mutex<()>>,
    buf_size_mb: usize,
}

impl<'a> FromPyObject<'a> for StringBlock<'a> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        check_dtype(ob, "object")?;
        let array = ob.downcast::<PyArray<PyString, Ix2>>()?;
        let data = unsafe { array.as_array_mut() };
        Ok(StringBlock {
            data,
            mutex: Arc::new(Mutex::new(())), // allocate the lock here since only StringBlock needs to aquire the GIL for now
            buf_size_mb: 16,                 // in MB
        })
    }
}

impl<'a> StringBlock<'a> {
    pub fn split(self) -> Vec<StringColumn<'a>> {
        let mut ret = vec![];
        let mut view = self.data;

        let nrows = view.ncols();
        while view.nrows() > 0 {
            let (col, rest) = view.split_at(Axis(0), 1);
            view = rest;
            ret.push(StringColumn {
                data: col
                    .into_shape(nrows)
                    .expect("reshape")
                    .into_slice()
                    .unwrap(),
                next_write: 0,
                string_lengths: vec![],
                string_buf: Vec::with_capacity(self.buf_size_mb * 2 << 20 * 11 / 10), // allocate a little bit more memory to avoid Vec growth
                buf_size: self.buf_size_mb * 2 << 20,
                mutex: self.mutex.clone(),
            })
        }
        ret
    }
}

pub struct StringColumn<'a> {
    data: &'a mut [PyString],
    next_write: usize,
    string_buf: Vec<u8>,
    string_lengths: Vec<usize>,
    buf_size: usize,
    mutex: Arc<Mutex<()>>,
}

impl<'a> PandasColumnObject for StringColumn<'a> {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<&'static [u8]>() || id == TypeId::of::<Option<&'static [u8]>>()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn typename(&self) -> &'static str {
        std::any::type_name::<&'static [u8]>()
    }
    fn finalize(&mut self) {
        self.flush()
    }
}

impl<'r, 'a> PandasColumn<&'r str> for StringColumn<'a> {
    fn write(&mut self, val: &'r str) {
        let bytes = val.as_bytes();
        self.string_lengths.push(bytes.len());
        self.string_buf.extend_from_slice(bytes);
        self.try_flush();
    }
}

impl<'r, 'a> PandasColumn<Option<&'r str>> for StringColumn<'a> {
    fn write(&mut self, val: Option<&'r str>) {
        match val {
            Some(b) => {
                let bytes = b.as_bytes();
                self.string_lengths.push(bytes.len());
                self.string_buf.extend_from_slice(bytes);
                self.try_flush();
            }
            None => {
                self.string_lengths.push(0);
            }
        }
    }
}

impl<'r> HasPandasColumn for &'r str {
    type PandasColumn<'a> = StringColumn<'a>;
}

impl<'r> HasPandasColumn for Option<&'r str> {
    type PandasColumn<'a> = StringColumn<'a>;
}

impl<'a> StringColumn<'a> {
    pub fn partition(self, counts: &[usize]) -> Vec<StringColumn<'a>> {
        let mut partitions = vec![];
        let mut data = self.data;

        for &c in counts {
            let (splitted_data, rest) = data.split_at_mut(c);
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
                        unsafe {
                            *self.data.get_unchecked_mut(self.next_write + i) =
                                PyString::new(py, &self.string_buf[start..end])
                        };
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

            self.string_buf.truncate(0);
            self.next_write += nstrings;
        }
    }

    pub fn try_flush(&mut self) {
        if self.string_buf.len() >= self.buf_size {
            self.flush();
        }
    }
}
