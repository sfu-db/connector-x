use super::{check_numpy_dtype, HasPandasColumn, PandasColumn, PandasColumnObject};
use ndarray::{ArrayViewMut1, Axis};
use numpy::PyArray1;
use pyo3::{types::PyString, FromPyObject, PyAny, PyObject, PyResult, Python};
use std::any::TypeId;

// Pandas squeezes na and string object into a single array
pub struct StringColumn<'a> {
    data: ArrayViewMut1<'a, PyObject>,
}

impl<'a> FromPyObject<'a> for StringColumn<'a> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        check_numpy_dtype(ob, "string")?;
        let data = ob.getattr("_ndarray")?;
        check_numpy_dtype(data, "object")?;

        Ok(StringColumn {
            data: unsafe {
                data.downcast::<PyArray1<PyObject>>()
                    .unwrap()
                    .as_array_mut()
            },
        })
    }
}

impl<'a> PandasColumnObject for StringColumn<'a> {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<String>() || id == TypeId::of::<Option<String>>()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn typename(&self) -> &'static str {
        std::any::type_name::<String>()
    }
}

impl<'a> PandasColumn<String> for StringColumn<'a> {
    fn write(&mut self, i: usize, val: String) {
        let py = unsafe { Python::assume_gil_acquired() };
        let val = PyString::new(py, &val).into();

        self.data[i] = val;
    }
}

impl<'a> PandasColumn<Option<String>> for StringColumn<'a> {
    fn write(&mut self, i: usize, val: Option<String>) {
        match val {
            Some(s) => {
                let py = unsafe { Python::assume_gil_acquired() };
                let val = PyString::new(py, &s).into();
                self.data[i] = val;
            }
            None => {}
        }
    }
}

impl HasPandasColumn for String {
    type PandasColumn<'a> = StringColumn<'a>;
}

impl HasPandasColumn for Option<String> {
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
            });
        }

        partitions
    }
}
