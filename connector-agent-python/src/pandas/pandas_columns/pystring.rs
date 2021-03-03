use numpy::{npyffi::NPY_TYPES, Element, PyArrayDescr};
use pyo3::{ffi, Py, Python};

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
    pub fn new(py: Python, val: &str) -> Self {
        let objptr = unsafe {
            ffi::PyUnicode_New(val.len() as ffi::Py_ssize_t, /*0x10FFFF*/ 127)
        };
        for (i, ch) in val.chars().enumerate() {
            let retc = unsafe { ffi::PyUnicode_WriteChar(objptr, i as ffi::Py_ssize_t, ch as u32) };
            assert_eq!(retc, 0);
        }

        let s: &pyo3::types::PyString = unsafe { py.from_owned_ptr(objptr) };
        PyString(s.into())
    }
}
