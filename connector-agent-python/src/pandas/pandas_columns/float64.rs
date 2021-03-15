use super::{check_dtype, HasPandasColumn, PandasColumn, PandasColumnObject};
use ndarray::{ArrayViewMut2, Axis, Ix2};
use numpy::PyArray;
use pyo3::{FromPyObject, PyAny, PyResult};
use std::any::TypeId;

// Float
pub struct Float64Block<'a> {
    data: ArrayViewMut2<'a, f64>,
}

impl<'a> FromPyObject<'a> for Float64Block<'a> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        check_dtype(ob, "float64")?;
        let array = ob.downcast::<PyArray<f64, Ix2>>()?;
        let data = unsafe { array.as_array_mut() };
        Ok(Float64Block { data })
    }
}

impl<'a> Float64Block<'a> {
    pub fn split(self) -> Vec<Float64Column<'a>> {
        let mut ret = vec![];
        let mut view = self.data;

        let nrows = view.ncols();
        while view.nrows() > 0 {
            let (col, rest) = view.split_at(Axis(0), 1);
            view = rest;
            ret.push(Float64Column {
                data: col
                    .into_shape(nrows)
                    .expect("reshape")
                    .into_slice()
                    .expect("into_slice"),
                i: 0,
            })
        }
        ret
    }
}

pub struct Float64Column<'a> {
    data: &'a mut [f64],
    i: usize,
}

impl<'a> PandasColumnObject for Float64Column<'a> {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<f64>() || id == TypeId::of::<Option<f64>>()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn typename(&self) -> &'static str {
        std::any::type_name::<f64>()
    }
}

impl<'a> PandasColumn<f64> for Float64Column<'a> {
    fn write(&mut self, val: f64) {
        unsafe { *self.data.get_unchecked_mut(self.i) = val };
        self.i += 1;
    }
}

impl<'a> PandasColumn<Option<f64>> for Float64Column<'a> {
    fn write(&mut self, val: Option<f64>) {
        match val {
            None => unsafe { *self.data.get_unchecked_mut(self.i) = f64::NAN },
            Some(val) => unsafe { *self.data.get_unchecked_mut(self.i) = val },
        }
        self.i += 1;
    }
}

impl HasPandasColumn for f64 {
    type PandasColumn<'a> = Float64Column<'a>;
}

impl HasPandasColumn for Option<f64> {
    type PandasColumn<'a> = Float64Column<'a>;
}

impl<'a> Float64Column<'a> {
    pub fn partition(self, counts: &[usize]) -> Vec<Float64Column<'a>> {
        let mut partitions = vec![];
        let mut data = self.data;

        for &c in counts {
            let (splitted, rest) = data.split_at_mut(c);
            data = rest;
            partitions.push(Float64Column {
                data: splitted,
                i: 0,
            });
        }

        partitions
    }
}
