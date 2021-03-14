use super::{check_dtype, HasPandasColumn, PandasColumn, PandasColumnObject};
use chrono::{DateTime, Utc};
use ndarray::{ArrayViewMut2, Axis, Ix2};
use numpy::PyArray;
use pyo3::{FromPyObject, PyAny, PyResult};
use std::any::TypeId;

// datetime64 is represented in int64 in numpy
// https://github.com/numpy/numpy/blob/master/numpy/core/include/numpy/npy_common.h#L1104
pub struct DateTimeBlock<'a> {
    data: ArrayViewMut2<'a, i64>,
}

impl<'a> FromPyObject<'a> for DateTimeBlock<'a> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        check_dtype(ob, "datetime64[ns]")?;
        let array = ob.downcast::<PyArray<i64, Ix2>>()?;
        let data = unsafe { array.as_array_mut() };
        Ok(DateTimeBlock { data })
    }
}

impl<'a> DateTimeBlock<'a> {
    pub fn split(self) -> Vec<DateTimeColumn<'a>> {
        let mut ret = vec![];
        let mut view = self.data;

        let nrows = view.ncols();
        while view.nrows() > 0 {
            let (col, rest) = view.split_at(Axis(0), 1);
            view = rest;
            ret.push(DateTimeColumn {
                data: col
                    .into_shape(nrows)
                    .expect("reshape")
                    .into_slice()
                    .unwrap(),
                i: 0,
            })
        }
        ret
    }
}

pub struct DateTimeColumn<'a> {
    data: &'a mut [i64],
    i: usize,
}

impl<'a> PandasColumnObject for DateTimeColumn<'a> {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<DateTime<Utc>>() || id == TypeId::of::<Option<DateTime<Utc>>>()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn typename(&self) -> &'static str {
        std::any::type_name::<DateTime<Utc>>()
    }
}

impl<'a> PandasColumn<DateTime<Utc>> for DateTimeColumn<'a> {
    fn write(&mut self, val: DateTime<Utc>) {
        unsafe { *self.data.get_unchecked_mut(self.i) = val.timestamp_nanos() };
        self.i += 1;
    }
}

impl<'a> PandasColumn<Option<DateTime<Utc>>> for DateTimeColumn<'a> {
    fn write(&mut self, val: Option<DateTime<Utc>>) {
        // numpy use i64::MIN as NaT
        unsafe {
            *self.data.get_unchecked_mut(self.i) =
                val.map(|t| t.timestamp_nanos()).unwrap_or(i64::MIN);
        };
        self.i += 1;
    }
}

impl HasPandasColumn for DateTime<Utc> {
    type PandasColumn<'a> = DateTimeColumn<'a>;
}

impl HasPandasColumn for Option<DateTime<Utc>> {
    type PandasColumn<'a> = DateTimeColumn<'a>;
}

impl<'a> DateTimeColumn<'a> {
    pub fn partition(self, counts: &[usize]) -> Vec<DateTimeColumn<'a>> {
        let mut partitions = vec![];
        let mut data = self.data;

        for &c in counts {
            let (splitted_data, rest) = data.split_at_mut(c);
            data = rest;

            partitions.push(DateTimeColumn {
                data: splitted_data,
                i: 0,
            });
        }

        partitions
    }
}
