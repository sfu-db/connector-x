use bytes::Bytes;
use numpy::{npyffi::NPY_TYPES, Element, PyArrayDescr};
use pyo3::{Py, Python};
use std::str::from_utf8_unchecked;

#[derive(Clone)]
#[repr(transparent)]
pub struct PyString(Py<pyo3::types::PyString>);

// In order to put it into a numpy array
impl Element for PyString {
    const DATA_TYPE: numpy::DataType = numpy::DataType::Object;
    fn is_same_type(dtype: &PyArrayDescr) -> bool {
        unsafe { *dtype.as_dtype_ptr() }.type_num == NPY_TYPES::NPY_OBJECT as i32
    }
}

impl PyString {
    pub fn new(py: Python, val: &Bytes) -> Self {
        PyString(pyo3::types::PyString::new(py, unsafe { from_utf8_unchecked(val) }).into())
    }
}
