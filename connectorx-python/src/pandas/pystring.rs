use bitfield::bitfield;
use numpy::{Element, PyArrayDescr};
use pyo3::{ffi, Bound, Py, Python};
use std::str::from_utf8_unchecked;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct PyString(Py<pyo3::types::PyString>);

// In order to put it into a numpy array
// SAFETY: `PyString` is `repr(transparent)` over `Py<_>`, i.e. one non-null
// `PyObject*`, which is what an `object` dtype array stores; `IS_COPY` is false
// and `clone_ref` only increfs. numpy does not check the Python type, so a slot
// may hold the `0` from `zeros` or `None` rather than a `str`.
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
    /// Counts the code points of `s` and finds the narrowest CPython kind for it.
    ///
    /// # Safety
    ///
    /// `s` must be valid UTF-8; it is read with `from_utf8_unchecked`.
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
        // SAFETY: `py` proves the thread is attached. `len` came from a Rust slice
        // length, so it fits in `Py_ssize_t`, and each `maxchar` is a valid bound.
        let objptr = unsafe {
            match self {
                StringInfo::ASCII(len) => ffi::PyUnicode_New(*len as ffi::Py_ssize_t, 0x7F),
                StringInfo::UCS1(len) => ffi::PyUnicode_New(*len as ffi::Py_ssize_t, 0xFF),
                StringInfo::UCS2(len) => ffi::PyUnicode_New(*len as ffi::Py_ssize_t, 0xFFFF),
                StringInfo::UCS4(len) => ffi::PyUnicode_New(*len as ffi::Py_ssize_t, 0x10FFFF),
            }
        };

        // SAFETY: `PyUnicode_New` returns a new (owned) reference to a `str`, so the
        // unchecked cast is correct; on failure it returns NULL, on which
        // `from_owned_ptr` panics rather than producing a dangling object.
        let s: Py<pyo3::types::PyString> = unsafe {
            Bound::from_owned_ptr(py, objptr)
                .cast_into_unchecked()
                .unbind()
        };

        PyString(s)
    }
}

impl PyString {
    // get none string converted from none object, otherwise default strings are zeros
    pub fn none(py: Python) -> PyString {
        // SAFETY: `Py_None` is a valid object and `from_borrowed_ptr` takes a new
        // reference. The cast to `PyString` is a lie that is only sound because the
        // result is just stored in an `object` array and dropped; callers must never
        // use it as a `str` (`StringColumn::flush` skips `None` slots in `write`).
        let s = unsafe {
            Bound::from_borrowed_ptr(py, ffi::Py_None())
                .cast_into_unchecked::<pyo3::types::PyString>()
                .unbind()
        };
        PyString(s)
    }

    /// Fills the character buffer of a string created by [`StringInfo::pystring`].
    ///
    /// # Safety
    ///
    /// - `self` must have been created by `info.pystring(..)` with
    ///   `info == StringInfo::detect(data)`, and `data` must be valid UTF-8, so the
    ///   object's kind and length match what is written here.
    /// - No other code (Python or Rust) may access the object concurrently; the
    ///   buffer is written without the GIL.
    /// - Relies on `PyASCIIObject` / `PyCompactUnicodeObject` below matching the
    ///   CPython layout of the interpreter built against; nothing checks this.
    pub unsafe fn write(&mut self, data: &[u8], info: StringInfo) {
        match info {
            StringInfo::ASCII(len) => {
                let pyobj = PyASCIIObject::from_mut_ref(&mut self.0);
                let buf = std::slice::from_raw_parts_mut(
                    (pyobj as *mut PyASCIIObject).offset(1) as *mut u8,
                    len,
                );

                buf.copy_from_slice(data);
            }
            StringInfo::UCS1(len) => {
                let pyobj = PyCompactUnicodeObject::from_mut_ref(&mut self.0);
                let buf = std::slice::from_raw_parts_mut(
                    (pyobj as *mut PyCompactUnicodeObject).offset(1) as *mut u8,
                    len,
                );
                let data: Vec<u8> = from_utf8_unchecked(data).chars().map(|c| c as u8).collect();
                buf.copy_from_slice(&data);
            }
            StringInfo::UCS2(len) => {
                let pyobj = PyCompactUnicodeObject::from_mut_ref(&mut self.0);
                let buf = std::slice::from_raw_parts_mut(
                    (pyobj as *mut PyCompactUnicodeObject).offset(1) as *mut u16,
                    len,
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
                    len,
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
    /// Reinterprets a `str` handle as the CPython struct it points to.
    ///
    /// # Safety
    ///
    /// `obj` must point to a compact ASCII `str` (as made by `PyUnicode_New` with
    /// `maxchar <= 0x7F`) and nothing else may access it while the result lives.
    /// The transmute relies on `Py<T>` being `repr(transparent)` over a non-null
    /// `PyObject*`.
    pub unsafe fn from_mut_ref(obj: &mut Py<pyo3::types::PyString>) -> &mut Self {
        let ascii: &mut &mut PyASCIIObject = std::mem::transmute(obj);
        ascii
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
    /// Reinterprets a `str` handle as the CPython struct it points to.
    ///
    /// # Safety
    ///
    /// `obj` must point to a compact non-ASCII `str` (as made by `PyUnicode_New`
    /// with `maxchar > 0x7F`) and nothing else may access it while the result
    /// lives. Same transmute assumption as [`PyASCIIObject::from_mut_ref`].
    pub unsafe fn from_mut_ref(obj: &mut Py<pyo3::types::PyString>) -> &mut Self {
        let unicode: &mut &mut PyCompactUnicodeObject = std::mem::transmute(obj);
        unicode
    }
}
