use super::{
    check_dtype, ExtractBlockFromBound, HasPandasColumn, PandasColumn, PandasColumnObject,
};
use crate::errors::ConnectorXPythonError;
use anyhow::anyhow;
use fehler::throws;
use ndarray::{ArrayViewMut2, Axis, Ix2};
use numpy::{PyArray, PyArrayMethods};
use pyo3::{PyAny, PyResult};
use std::any::TypeId;

// Float
pub struct Float64Block<'a> {
    data: ArrayViewMut2<'a, f64>,
}

impl<'a> ExtractBlockFromBound<'a> for Float64Block<'a> {
    fn extract_block<'b: 'a>(ob: &'b pyo3::Bound<'a, PyAny>) -> PyResult<Self> {
        check_dtype(ob, "float64")?;
        let array: &pyo3::Bound<'a, PyArray<f64, Ix2>> = ob.cast()?;
        // SAFETY: (A)/(B) in `pandas_columns`: the array was freshly allocated by
        // `allocate_py` and this is the only view of it.
        let data: ArrayViewMut2<'a, f64> = unsafe { array.as_array_mut() };
        Ok(Float64Block { data })
    }
}

impl<'a> Float64Block<'a> {
    #[throws(ConnectorXPythonError)]
    pub fn split(self) -> Vec<Float64Column> {
        let mut ret = vec![];
        let mut view = self.data;

        let nrows = view.ncols();
        while view.nrows() > 0 {
            let (col, rest) = view.split_at(Axis(0), 1);
            view = rest;
            ret.push(Float64Column {
                data: col
                    .into_shape_with_order(nrows)?
                    .into_slice()
                    .ok_or_else(|| anyhow!("get None for splitted Float64 data"))?
                    .as_mut_ptr(),
            })
        }
        ret
    }
}

pub struct Float64Column {
    data: *mut f64,
}

// SAFETY: the pointer targets numpy-owned memory that is not tied to a thread;
// partitions write disjoint rows (B) and the array outlives them (A).
unsafe impl Send for Float64Column {}
// SAFETY: no `&self` method dereferences the pointer; writes need `&mut self`.
unsafe impl Sync for Float64Column {}

impl PandasColumnObject for Float64Column {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<f64>() || id == TypeId::of::<Option<f64>>()
    }

    fn typename(&self) -> &'static str {
        std::any::type_name::<f64>()
    }
}

impl PandasColumn<f64> for Float64Column {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: f64, row: usize) {
        // SAFETY: `data` spans `nrow` elements (A); `row` lies in a range this
        // partition reserved via `aquire_row` (B). `row < nrow` relies on (C),
        // which is not checked.
        unsafe { *self.data.add(row) = val };
    }
}

impl PandasColumn<Option<f64>> for Float64Column {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: Option<f64>, row: usize) {
        match val {
            // SAFETY: as in the `f64` `write` above: (A), (B), unchecked (C).
            None => unsafe { *self.data.add(row) = f64::NAN },
            // SAFETY: same as the `None` arm.
            Some(val) => unsafe { *self.data.add(row) = val },
        }
    }
}

impl HasPandasColumn for f64 {
    type PandasColumn<'a> = Float64Column;
}

impl HasPandasColumn for Option<f64> {
    type PandasColumn<'a> = Float64Column;
}

impl Float64Column {
    pub fn partition(self, counts: usize) -> Vec<Float64Column> {
        let mut partitions = vec![];
        for _ in 0..counts {
            partitions.push(Float64Column { data: self.data });
        }
        partitions
    }
}
