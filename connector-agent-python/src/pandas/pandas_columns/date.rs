use super::{check_dtype, HasPandasColumn, PandasColumn, PandasColumnObject};
use chrono::{Date, Utc};
use ndarray::{ArrayViewMut1, ArrayViewMut2, Axis, Ix2};
use numpy::PyArray;
use pyo3::{FromPyObject, PyAny, PyResult};
use std::any::TypeId;

// datetime64 is represented in int64 in numpy
// https://github.com/numpy/numpy/blob/master/numpy/core/include/numpy/npy_common.h#L1104
pub struct DateBlock<'a> {
    data: ArrayViewMut2<'a, i64>,
}

impl<'a> FromPyObject<'a> for DateBlock<'a> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        check_dtype(ob, "datetime64[ns]")?;
        let array = ob.downcast::<PyArray<i64, Ix2>>()?;
        let data = unsafe { array.as_array_mut() };
        Ok(DateBlock { data })
    }
}

impl<'a> DateBlock<'a> {
    pub fn split(self) -> Vec<DateColumn<'a>> {
        let mut ret = vec![];
        let mut view = self.data;

        let nrows = view.ncols();
        while view.nrows() > 0 {
            let (col, rest) = view.split_at(Axis(0), 1);
            view = rest;
            ret.push(DateColumn {
                data: col.into_shape(nrows).expect("reshape"),
            })
        }
        ret
    }
}

pub struct DateColumn<'a> {
    data: ArrayViewMut1<'a, i64>,
}

impl<'a> PandasColumnObject for DateColumn<'a> {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<Date<Utc>>() || id == TypeId::of::<Option<Date<Utc>>>()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn typename(&self) -> &'static str {
        std::any::type_name::<Date<Utc>>()
    }
}

impl<'a> PandasColumn<Date<Utc>> for DateColumn<'a> {
    fn write(&mut self, i: usize, val: Date<Utc>) {
        self.data[i] = val.and_hms(0, 0, 0).timestamp_nanos();
    }
}

impl<'a> PandasColumn<Option<Date<Utc>>> for DateColumn<'a> {
    fn write(&mut self, i: usize, val: Option<Date<Utc>>) {
        self.data[i] = val
            .map(|t| t.and_hms(0, 0, 0).timestamp_nanos())
            .unwrap_or(i64::MIN);
    }
}

impl HasPandasColumn for Date<Utc> {
    type PandasColumn<'a> = DateColumn<'a>;
}

impl HasPandasColumn for Option<Date<Utc>> {
    type PandasColumn<'a> = DateColumn<'a>;
}

impl<'a> DateColumn<'a> {
    pub fn partition(self, counts: &[usize]) -> Vec<DateColumn<'a>> {
        let mut partitions = vec![];
        let mut data = self.data;

        for &c in counts {
            let (splitted_data, rest) = data.split_at(Axis(0), c);
            data = rest;

            partitions.push(DateColumn {
                data: splitted_data,
            });
        }

        partitions
    }
}
