use super::super::pystring::{PyString, StringInfo};
use super::{
    check_dtype, ExtractBlockFromBound, HasPandasColumn, PandasColumn, PandasColumnObject,
    GIL_MUTEX,
};
use crate::constants::PYSTRING_BUFFER_SIZE;
use crate::errors::ConnectorXPythonError;
use anyhow::anyhow;
use fehler::throws;
use itertools::Itertools;
use ndarray::{ArrayViewMut2, Axis, Ix2};
use numpy::{PyArray, PyArrayMethods};
use pyo3::{PyAny, PyResult, Python};
use std::any::TypeId;

pub struct StringBlock<'a> {
    data: ArrayViewMut2<'a, PyString>,
    buf_size_mb: usize,
}

impl<'a> ExtractBlockFromBound<'a> for StringBlock<'a> {
    fn extract_block<'b: 'a>(ob: &'b pyo3::Bound<'a, PyAny>) -> PyResult<Self> {
        check_dtype(ob, "object")?;
        let array = ob.cast::<PyArray<PyString, Ix2>>()?;
        // SAFETY: (A)/(B) in `pandas_columns`: the array was freshly allocated by
        // `allocate_py` and this is the only view of it.
        let data = unsafe { array.as_array_mut() };
        Ok(StringBlock {
            data,
            buf_size_mb: PYSTRING_BUFFER_SIZE, // in MB
        })
    }
}

impl<'a> StringBlock<'a> {
    #[throws(ConnectorXPythonError)]
    pub fn split(self) -> Vec<StringColumn> {
        let mut ret = vec![];
        let mut view = self.data;

        let nrows = view.ncols();
        while view.nrows() > 0 {
            let (col, rest) = view.split_at(Axis(0), 1);
            view = rest;
            ret.push(StringColumn {
                data: col
                    .into_shape_with_order(nrows)?
                    .into_slice()
                    .ok_or_else(|| anyhow!("get None for splitted String data"))?
                    .as_mut_ptr(),
                string_lengths: vec![],
                row_idx: vec![],
                string_buf: Vec::with_capacity(self.buf_size_mb * (1 << 20) * 11 / 10), // allocate a little bit more memory to avoid Vec growth
                buf_size: self.buf_size_mb * (1 << 20),
            })
        }
        ret
    }
}

pub struct StringColumn {
    data: *mut PyString,
    string_buf: Vec<u8>,
    string_lengths: Vec<usize>, // usize::MAX for empty string
    row_idx: Vec<usize>,
    buf_size: usize,
}

// SAFETY: `data` targets numpy-owned memory that is not tied to a thread;
// partitions write disjoint rows (B), the array outlives them (A), and new
// Python objects are only stored while attached and holding `GIL_MUTEX` (D).
unsafe impl Send for StringColumn {}
// SAFETY: no `&self` method dereferences `data`; writes need `&mut self`.
unsafe impl Sync for StringColumn {}

impl PandasColumnObject for StringColumn {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<&'static [u8]>() || id == TypeId::of::<Option<&'static [u8]>>()
    }

    fn typename(&self) -> &'static str {
        std::any::type_name::<&'static [u8]>()
    }
    #[throws(ConnectorXPythonError)]
    fn finalize(&mut self) {
        self.flush(true)?;
    }
}

impl<'r> PandasColumn<&'r str> for StringColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: &'r str, row: usize) {
        let bytes = val.as_bytes();
        self.string_lengths.push(bytes.len());
        self.string_buf.extend_from_slice(bytes);
        self.row_idx.push(row);
        self.try_flush()?;
    }
}

impl PandasColumn<Box<str>> for StringColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: Box<str>, row: usize) {
        let bytes = val.as_bytes();
        self.string_lengths.push(bytes.len());
        self.string_buf.extend_from_slice(bytes);
        self.row_idx.push(row);
        self.try_flush()?;
    }
}

impl PandasColumn<String> for StringColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: String, row: usize) {
        let bytes = val.as_bytes();
        self.string_lengths.push(bytes.len());
        self.string_buf.extend_from_slice(bytes);
        self.row_idx.push(row);
        self.try_flush()?;
    }
}

impl PandasColumn<char> for StringColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: char, row: usize) {
        let mut buffer = [0; 4]; // a char is max to 4 bytes
        let bytes = val.encode_utf8(&mut buffer).as_bytes();
        self.string_lengths.push(bytes.len());
        self.string_buf.extend_from_slice(bytes);
        self.row_idx.push(row);
        self.try_flush()?;
    }
}

impl<'r> PandasColumn<Option<&'r str>> for StringColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: Option<&'r str>, row: usize) {
        match val {
            Some(b) => {
                let bytes = b.as_bytes();
                self.string_lengths.push(bytes.len());
                self.string_buf.extend_from_slice(bytes);
                self.row_idx.push(row);
                self.try_flush()?;
            }
            None => {
                self.string_lengths.push(usize::MAX);
                self.row_idx.push(row);
            }
        }
    }
}

impl PandasColumn<Option<Box<str>>> for StringColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: Option<Box<str>>, row: usize) {
        match val {
            Some(b) => {
                let bytes = b.as_bytes();
                self.string_lengths.push(bytes.len());
                self.string_buf.extend_from_slice(bytes);
                self.row_idx.push(row);
                self.try_flush()?;
            }
            None => {
                self.string_lengths.push(usize::MAX);
                self.row_idx.push(row);
            }
        }
    }
}
impl PandasColumn<Option<String>> for StringColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: Option<String>, row: usize) {
        match val {
            Some(b) => {
                let bytes = b.as_bytes();
                self.string_lengths.push(bytes.len());
                self.string_buf.extend_from_slice(bytes);
                self.row_idx.push(row);
                self.try_flush()?;
            }
            None => {
                self.string_lengths.push(usize::MAX);
                self.row_idx.push(row);
            }
        }
    }
}

impl PandasColumn<Option<char>> for StringColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: Option<char>, row: usize) {
        match val {
            Some(b) => {
                let mut buffer = [0; 4]; // a char is max to 4 bytes
                let bytes = b.encode_utf8(&mut buffer).as_bytes();
                self.string_lengths.push(bytes.len());
                self.string_buf.extend_from_slice(bytes);
                self.row_idx.push(row);
                self.try_flush()?;
            }
            None => {
                self.string_lengths.push(usize::MAX);
                self.row_idx.push(row);
            }
        }
    }
}

impl HasPandasColumn for &str {
    type PandasColumn<'a> = StringColumn;
}

impl HasPandasColumn for Option<&str> {
    type PandasColumn<'a> = StringColumn;
}

impl HasPandasColumn for String {
    type PandasColumn<'a> = StringColumn;
}

impl HasPandasColumn for Option<String> {
    type PandasColumn<'a> = StringColumn;
}

impl HasPandasColumn for char {
    type PandasColumn<'a> = StringColumn;
}

impl HasPandasColumn for Option<char> {
    type PandasColumn<'a> = StringColumn;
}

impl HasPandasColumn for Box<str> {
    type PandasColumn<'a> = StringColumn;
}

impl HasPandasColumn for Option<Box<str>> {
    type PandasColumn<'a> = StringColumn;
}

impl StringColumn {
    pub fn partition(self, counts: usize) -> Vec<StringColumn> {
        let mut partitions = vec![];

        for _ in 0..counts {
            partitions.push(StringColumn {
                data: self.data,
                string_lengths: vec![],
                row_idx: vec![],
                string_buf: Vec::with_capacity(self.buf_size),
                buf_size: self.buf_size,
            });
        }

        partitions
    }

    #[throws(ConnectorXPythonError)]
    pub fn flush(&mut self, force: bool) {
        let nstrings = self.string_lengths.len();
        if nstrings == 0 {
            return;
        }

        let guard = if force {
            GIL_MUTEX
                .lock()
                .map_err(|e| anyhow!("mutex poisoned {}", e))?
        } else {
            match GIL_MUTEX.try_lock() {
                Ok(guard) => guard,
                Err(_) => return,
            }
        };

        // NOTE: from Python 3.12, we have to allocate the string with a real Python<'py> token
        // previous `let py = unsafe { Python::assume_gil_acquired() }` approach will lead to segment fault when partition is enabled
        let mut string_infos = Vec::with_capacity(self.string_lengths.len());
        Python::attach(|py| {
            let mut start = 0;
            for (i, &len) in self.string_lengths.iter().enumerate() {
                if len != usize::MAX {
                    let end = start + len;

                    // SAFETY: `string_buf[start..end]` is exactly one value pushed by a
                    // `write` impl, all of which take `str`/`char` input, so it is valid
                    // UTF-8 for `detect`. The slot store follows (A)-(D): `row_idx[i]` was
                    // reserved by this partition, bounds rely on unchecked (C), and the
                    // old value dropped is the `0` from `zeros` while attached.
                    unsafe {
                        let string_info = StringInfo::detect(&self.string_buf[start..end]);
                        *self.data.add(self.row_idx[i]) = string_info.pystring(py);
                        string_infos.push(Some(string_info));
                    };

                    start = end;
                } else {
                    string_infos.push(None);

                    // SAFETY: same slot store as above, (A)-(D).
                    unsafe { *self.data.add(self.row_idx[i]) = PyString::none(py) };
                }
            }
        });

        // unlock GIL
        std::mem::drop(guard);

        if !string_infos.is_empty() {
            let mut start = 0;
            for (i, (len, info)) in self
                .string_lengths
                .drain(..)
                .zip_eq(string_infos)
                .enumerate()
            {
                if len != usize::MAX {
                    let end = start + len;
                    // SAFETY: the slot holds the string created in the loop above from
                    // `info`, which was detected on these same bytes. Python has no
                    // reference to it yet, so filling it without the GIL is unobserved
                    // (for `len == 0` it is CPython's shared empty string, but zero
                    // bytes are copied). `None` slots are skipped, so never `Py_None`.
                    unsafe {
                        (*self.data.add(self.row_idx[i]))
                            .write(&self.string_buf[start..end], info.unwrap())
                    };

                    start = end;
                }
            }

            self.string_buf.clear();
            self.row_idx.clear();
        }
    }

    #[throws(ConnectorXPythonError)]
    pub fn try_flush(&mut self) {
        if self.string_buf.len() >= self.buf_size {
            self.flush(true)?;
            return;
        }
        #[cfg(feature = "nbstr")]
        if self.string_buf.len() >= self.buf_size / 2 {
            self.flush(false)?;
        }
    }
}
