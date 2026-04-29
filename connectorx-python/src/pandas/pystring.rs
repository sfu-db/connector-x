use bitfield::bitfield;
use numpy::{Element, PyArrayDescr};
use pyo3::{ffi, Bound, Py, Python};
use std::str::from_utf8_unchecked;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct PyString(Py<pyo3::types::PyString>);

// In order to put it into a numpy array
unsafe impl Element for PyString {
    const IS_COPY: bool = false;
    fn get_dtype(py: Python<'_>) -> Bound<'_, PyArrayDescr> {
        PyArrayDescr::object(py)
    }

    fn clone_ref(&self, _py: Python<'_>) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum StringInfo {
    ASCII(usize), // len of the string, not byte length
    UCS1(usize),
    UCS2(usize),
    UCS4(usize),
}

impl StringInfo {
    pub unsafe fn detect(s: &[u8]) -> StringInfo {
        let s = from_utf8_unchecked(s);
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

        let s: Py<pyo3::types::PyString> = unsafe { Py::from_owned_ptr(py, objptr) };

        PyString(s)
    }
}

impl PyString {
    // get none string converted from none object, otherwise default strings are zeros
    pub fn none(py: Python) -> PyString {
        // this is very unsafe because Py_None is not a PyString from Rust's perspective. But it is fine because
        // later these stuff will all be converted to a python object
        let s = unsafe { Py::from_borrowed_ptr(py, ffi::Py_None()) };
        PyString(s)
    }

    // the val should be same as the val used for new
    pub unsafe fn write(&mut self, data: &[u8], info: StringInfo) {
        match info {
            StringInfo::ASCII(len) => {
                let pyobj = PyASCIIObject::from_mut_ref(&mut self.0);
                let buf = std::slice::from_raw_parts_mut(
                    (pyobj as *mut PyASCIIObject).offset(1) as *mut u8,
                    len as usize,
                );

                buf.copy_from_slice(data);
            }
            StringInfo::UCS1(len) => {
                let pyobj = PyCompactUnicodeObject::from_mut_ref(&mut self.0);
                let buf = std::slice::from_raw_parts_mut(
                    (pyobj as *mut PyCompactUnicodeObject).offset(1) as *mut u8,
                    len as usize,
                );
                let data: Vec<u8> = from_utf8_unchecked(data).chars().map(|c| c as u8).collect();
                buf.copy_from_slice(&data);
            }
            StringInfo::UCS2(len) => {
                let pyobj = PyCompactUnicodeObject::from_mut_ref(&mut self.0);
                let buf = std::slice::from_raw_parts_mut(
                    (pyobj as *mut PyCompactUnicodeObject).offset(1) as *mut u16,
                    len as usize,
                );
                let data: Vec<u16> = from_utf8_unchecked(data)
                    .chars()
                    .map(|c| c as u16)
                    .collect();
                buf.copy_from_slice(&data);
            }
            StringInfo::UCS4(len) => {
                let pyobj = PyCompactUnicodeObject::from_mut_ref(&mut self.0);
                let buf = std::slice::from_raw_parts_mut(
                    (pyobj as *mut PyCompactUnicodeObject).offset(1) as *mut u32,
                    len as usize,
                );
                let data: Vec<u32> = from_utf8_unchecked(data)
                    .chars()
                    .map(|c| c as u32)
                    .collect();
                buf.copy_from_slice(&data);
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
    #[cfg(not(Py_3_12))]
    ready, _: 7, 7;
    #[cfg(Py_3_12)]
    statically_allocated, _: 7, 7;
}

#[repr(C)]
pub struct PyASCIIObject {
    obj: ffi::PyObject,
    length: ffi::Py_ssize_t,
    hash: ffi::Py_hash_t,
    state: PyUnicodeState,
    #[cfg(not(Py_3_12))]
    wstr: *mut u8,
    // python string stores data right after all the fields
}

impl PyASCIIObject {
    pub unsafe fn from_mut_ref<'a>(obj: &'a mut Py<pyo3::types::PyString>) -> &'a mut Self {
        let ascii: &mut &mut PyASCIIObject = std::mem::transmute(obj);
        *ascii
    }
}

#[repr(C)]
pub struct PyCompactUnicodeObject {
    base: PyASCIIObject,
    utf8_length: ffi::Py_ssize_t,
    utf8: *mut u8,
    #[cfg(not(Py_3_12))]
    wstr_length: ffi::Py_ssize_t,
    // python string stores data right after all the fields
}

impl PyCompactUnicodeObject {
    pub unsafe fn from_mut_ref<'a>(obj: &'a mut Py<pyo3::types::PyString>) -> &'a mut Self {
        let unicode: &mut &mut PyCompactUnicodeObject = std::mem::transmute(obj);
        *unicode
    }
}
