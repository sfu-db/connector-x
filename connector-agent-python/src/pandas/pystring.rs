use bitfield::bitfield;
use numpy::{npyffi::NPY_TYPES, Element, PyArrayDescr};
use pyo3::{ffi, Py, Python};
use std::str::from_utf8_unchecked;
// use widestring::{U16String, U32String};
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

// impl PyString {
//     pub fn new(py: Python, val: &[u8]) -> Self {
//         PyString(pyo3::types::PyString::new(py, unsafe { from_utf8_unchecked(val) }).into())
//     }
// }

#[derive(Clone, Copy)]
pub enum StringInfo {
    ASCII(usize), // len of the string, not byte length
    UCS1(usize),
    UCS2(usize),
    UCS4(usize),
}

impl StringInfo {
    pub fn inspect(s: &str) -> StringInfo {
        let mut maxchar = 0;
        let mut len = 0;

        for ch in s.chars() {
            if ch as u32 > maxchar {
                maxchar = ch as u32;
            }
            len += 1;
        }

        if maxchar <= 0x7F {
            StringInfo::ASCII(len)
        } else if maxchar <= 0xFF {
            StringInfo::UCS1(len)
        } else if maxchar <= 0xFFFF {
            StringInfo::UCS2(len)
        } else {
            StringInfo::UCS4(len)
        }
    }

    pub fn pystring(&self, py: Python) -> PyString {
        let objptr = unsafe {
            match self {
                StringInfo::ASCII(len) => ffi::PyUnicode_New(*len as ffi::Py_ssize_t, 0x7F),
                StringInfo::UCS1(len) => ffi::PyUnicode_New(*len as ffi::Py_ssize_t, 0xFF),
                StringInfo::UCS2(len) => ffi::PyUnicode_New(*len as ffi::Py_ssize_t, 0xFFFF),
                StringInfo::UCS4(len) => ffi::PyUnicode_New(*len as ffi::Py_ssize_t, 0x10FFFF),
            }
        };

        let s: &pyo3::types::PyString = unsafe { py.from_owned_ptr(objptr) };
        PyString(s.into())
    }
}

impl PyString {
    // the val should be same as the val used for new
    pub unsafe fn write(&mut self, data: &[u8], info: StringInfo) {
        match info {
            StringInfo::ASCII(len) => {
                let pyobj = PyASCIIObject::from_owned(self.0.clone());
                let buf = std::slice::from_raw_parts_mut(
                    (pyobj as *mut PyASCIIObject).offset(1) as *mut u8,
                    len as usize,
                );

                buf.copy_from_slice(data);
            }
            StringInfo::UCS1(len) => {
                let pyobj = PyCompactUnicodeObject::from_owned(self.0.clone());
                let buf = std::slice::from_raw_parts_mut(
                    (pyobj as *mut PyCompactUnicodeObject).offset(1) as *mut u8,
                    len as usize,
                );
                let data: Vec<u8> = from_utf8_unchecked(data).chars().map(|c| c as u8).collect();
                buf.copy_from_slice(&data);
            }
            StringInfo::UCS2(len) => {
                let pyobj = PyCompactUnicodeObject::from_owned(self.0.clone());
                let buf = std::slice::from_raw_parts_mut(
                    (pyobj as *mut PyCompactUnicodeObject).offset(1) as *mut u16,
                    len as usize,
                );
                let data: Vec<u16> = from_utf8_unchecked(data)
                    .chars()
                    .map(|c| c as u16)
                    .collect();
                buf.copy_from_slice(&data);

                // let ucs_string = U16String::from_str(from_utf8_unchecked(data));
                // buf.copy_from_slice(ucs_string.as_slice());
            }
            StringInfo::UCS4(len) => {
                let pyobj = PyCompactUnicodeObject::from_owned(self.0.clone());
                let buf = std::slice::from_raw_parts_mut(
                    (pyobj as *mut PyCompactUnicodeObject).offset(1) as *mut u32,
                    len as usize,
                );
                let data: Vec<u32> = from_utf8_unchecked(data)
                    .chars()
                    .map(|c| c as u32)
                    .collect();
                buf.copy_from_slice(&data);

                // let ucs_string = U32String::from_str(from_utf8_unchecked(data));
                // buf.copy_from_slice(ucs_string.as_slice());
            }
        }
    }
}

bitfield! {
    struct PyUnicodeState(u32);
    u32;
    interned, _: 1, 0;
    kind, _: 4, 2;
    compact, _: 5, 5;
    ascii, _: 6, 6;
    ready, _: 7, 7;
}

#[repr(C)]
pub struct PyASCIIObject {
    obj: ffi::PyObject,
    length: ffi::Py_ssize_t,
    hash: ffi::Py_hash_t,
    state: PyUnicodeState,
    wstr: *mut u8,
    // python string stores data right after all the fields
}

impl PyASCIIObject {
    pub unsafe fn from_owned<'a>(obj: Py<pyo3::types::PyString>) -> &'a mut Self {
        let ascii: &mut PyASCIIObject = std::mem::transmute(obj);
        ascii
    }
}

#[repr(C)]
pub struct PyCompactUnicodeObject {
    base: PyASCIIObject,
    utf8_length: ffi::Py_ssize_t,
    utf8: *mut u8,
    wstr_length: ffi::Py_ssize_t,
    // python string stores data right after all the fields
}

impl PyCompactUnicodeObject {
    pub unsafe fn from_owned<'a>(obj: Py<pyo3::types::PyString>) -> &'a mut Self {
        let unicode: &mut PyCompactUnicodeObject = std::mem::transmute(obj);
        unicode
    }
}
