use super::{
    check_dtype, ExtractBlockFromBound, HasPandasColumn, PandasColumn, PandasColumnObject,
};
use crate::errors::ConnectorXPythonError;
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use fehler::throws;
use ndarray::{ArrayViewMut2, Axis, Ix2};
use numpy::{PyArray, PyArrayMethods};
use pyo3::{PyAny, PyResult};
use std::any::TypeId;

// datetime64 is represented in int64 in numpy
// https://github.com/numpy/numpy/blob/master/numpy/core/include/numpy/npy_common.h#L1104
pub struct DateTimeBlock<'a> {
    data: ArrayViewMut2<'a, i64>,
}

impl<'a> ExtractBlockFromBound<'a> for DateTimeBlock<'a> {
    fn extract_block<'b: 'a>(ob: &'b pyo3::Bound<'a, PyAny>) -> PyResult<Self> {
        check_dtype(ob, "int64")?;
        let array = ob.cast::<PyArray<i64, Ix2>>()?;
        let data = unsafe { array.as_array_mut() };
        Ok(DateTimeBlock { data })
    }
}

impl<'a> DateTimeBlock<'a> {
    #[throws(ConnectorXPythonError)]
    pub fn split(self) -> Vec<DateTimeColumn> {
        let mut ret = vec![];
        let mut view = self.data;

        let nrows = view.ncols();
        while view.nrows() > 0 {
            let (col, rest) = view.split_at(Axis(0), 1);
            view = rest;
            ret.push(DateTimeColumn {
                data: col
                    .into_shape_with_order(nrows)?
                    .into_slice()
                    .ok_or_else(|| anyhow!("get None for splitted DateTime data"))?
                    .as_mut_ptr(),
            })
        }
        ret
    }
}

pub struct DateTimeColumn {
    data: *mut i64,
}

unsafe impl Send for DateTimeColumn {}
unsafe impl Sync for DateTimeColumn {}

impl PandasColumnObject for DateTimeColumn {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<DateTime<Utc>>() || id == TypeId::of::<Option<DateTime<Utc>>>()
    }

    fn typename(&self) -> &'static str {
        std::any::type_name::<DateTime<Utc>>()
    }
}

impl PandasColumn<DateTime<Utc>> for DateTimeColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: DateTime<Utc>, row: usize) {
        unsafe {
            *self.data.add(row) = val
                .timestamp_nanos_opt()
                .unwrap_or_else(|| panic!("out of range DateTime"))
        };
    }
}

impl PandasColumn<Option<DateTime<Utc>>> for DateTimeColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: Option<DateTime<Utc>>, row: usize) {
        // numpy use i64::MIN as NaT
        unsafe {
            *self.data.add(row) = val
                .map(|t| {
                    t.timestamp_nanos_opt()
                        .unwrap_or_else(|| panic!("out of range DateTime"))
                })
                .unwrap_or(i64::MIN);
        };
    }
}

impl HasPandasColumn for DateTime<Utc> {
    type PandasColumn<'a> = DateTimeColumn;
}

impl HasPandasColumn for Option<DateTime<Utc>> {
    type PandasColumn<'a> = DateTimeColumn;
}

impl DateTimeColumn {
    pub fn partition(self, counts: usize) -> Vec<DateTimeColumn> {
        let mut partitions = vec![];

        for _ in 0..counts {
            partitions.push(DateTimeColumn { data: self.data });
        }

        partitions
    }
}
