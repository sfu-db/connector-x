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

unsafe impl Send for Float64Column {}
unsafe impl Sync for Float64Column {}

impl<'a> PandasColumnObject for Float64Column {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<f64>() || id == TypeId::of::<Option<f64>>()
    }

    fn typename(&self) -> &'static str {
        std::any::type_name::<f64>()
    }
}

impl<'a> PandasColumn<f64> for Float64Column {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: f64, row: usize) {
        unsafe { *self.data.add(row) = val };
    }
}

impl<'a> PandasColumn<Option<f64>> for Float64Column {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: Option<f64>, row: usize) {
        match val {
            None => unsafe { *self.data.add(row) = f64::NAN },
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
