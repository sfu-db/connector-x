// #![feature(custom_test_frameworks)]
// #![test_runner(criterion::runner)]

// use anyhow::Error;
// use criterion::{black_box, Criterion};
// use criterion_macro::criterion;
// use ndarray::{ArrayViewMut1, Axis};
// use numpy::PyArray1;
// use pyo3::{
//     types::{IntoPyDict, PyString},
//     PyObject, PyResult, Python, ToPyObject,
// };
// use rayon::prelude::*;
// use std::fs::read_to_string;

// #[criterion(Criterion::default())]
// fn benchmark(c: &mut Criterion) {
//     let data = read_to_string("data.txt").unwrap();
//     let mut strings: Vec<_> = data.split('\n').collect();
//     strings.extend(strings.clone());
//     strings.extend(strings.clone());
//     strings.extend(strings.clone());
//     strings.extend(strings.clone());

//     rayon::ThreadPoolBuilder::new()
//         .num_threads(1)
//         .build_global()
//         .unwrap();

//     c.bench_function("single_thread_batch_alloc", |b| {
//         b.iter(|| {
//             Python::with_gil(|py| single_thread_batch_alloc(py, black_box(&strings)).unwrap())
//         })
//     });

//     c.bench_function("single_thread", |b| {
//         b.iter(|| Python::with_gil(|py| single_thread(py, black_box(&strings)).unwrap()))
//     });
// }

// fn single_thread(py: Python, strings: &[&str]) -> Result<(), Error> {
//     let mut view = create_numpy_object_array(py, strings.len()).unwrap();

//     for (i, s) in strings.iter().enumerate() {
//         view[i] = PyString::new(py, s).into();
//     }
//     Ok(())
// }

// fn single_thread_batch_alloc(py: Python, strings: &[&str]) -> Result<(), Error> {
//     let chunksize = 30000;
//     let mut view = create_numpy_object_array(py, strings.len()).unwrap();

//     let mut views = vec![];

//     while view.len() != 0 {
//         let size = view.len().min(chunksize);
//         let (a, b) = view.split_at(Axis(0), size);
//         views.push(a);
//         view = b;
//     }

//     py.allow_threads(|| {
//         strings
//             .par_chunks(chunksize)
//             .zip_eq(views)
//             .for_each(|(s, mut v)| {
//                 Python::with_gil(|py| {
//                     for i in 0..s.len() {
//                         v[i] = pystring::new(py, s[i].len()).to_object(py);
//                     }
//                 });

//                 let py = unsafe { Python::assume_gil_acquired() };
//                 unsafe {
//                     for i in 0..s.len() {
//                         pystring::write(v[i].as_ref(py).downcast::<PyString>().unwrap(), &s[i])
//                     }
//                 }
//             });
//     });

//     Ok(())
// }

// fn create_numpy_object_array<'a>(py: Python<'a>, num: usize) -> PyResult<ArrayViewMut1<PyObject>> {
//     let locals = [("pd", py.import("pandas")?), ("np", py.import("numpy")?)].into_py_dict(py);
//     let code = format!("np.full({}, pd.NA)", num);
//     let array = py.eval(&code, None, Some(locals))?;
//     let pyarray = array.downcast::<PyArray1<PyObject>>()?;
//     Ok(unsafe { pyarray.as_array_mut() })
// }

// mod pystring {
//     use pyo3::{ffi, types::PyString, Py, Python};

//     pub fn new(py: Python, len: usize) -> Py<PyString> {
//         let objptr = unsafe {
//             ffi::PyUnicode_New(len as ffi::Py_ssize_t, /*0x10FFFF*/ 127)
//         };

//         let s: &PyString = unsafe { py.from_owned_ptr(objptr) };
//         s.into()
//     }

//     pub unsafe fn write(pystring: &PyString, val: &str) {
//         let ascii = PyASCIIObject::from_ref(pystring);
//         let buf = std::slice::from_raw_parts_mut(
//             (ascii as *mut PyASCIIObject).offset(1) as *mut u8,
//             ascii.length as usize,
//         );
//         buf.copy_from_slice(val.as_bytes());
//     }

//     #[repr(C)]
//     pub struct PyASCIIObject {
//         obj: ffi::PyObject,
//         length: ffi::Py_ssize_t,
//         hash: ffi::Py_hash_t,
//         opaque: u32,
//         wstr: *mut u8,
//     }

//     impl PyASCIIObject {
//         // pub unsafe fn from_owned<'a>(obj: Py<PyString>) -> &'a mut Self {
//         //     let ascii: &mut PyASCIIObject = std::mem::transmute(obj);
//         //     ascii
//         // }

//         pub unsafe fn from_ref(obj: &PyString) -> &mut Self {
//             #[allow(mutable_transmutes)]
//             let ascii: &mut PyASCIIObject = std::mem::transmute(obj);
//             ascii
//         }
//     }
// }
