use super::{
    check_dtype, ExtractBlockFromBound, HasPandasColumn, PandasColumn, PandasColumnObject,
};
use crate::errors::ConnectorXPythonError;
use anyhow::anyhow;
use fehler::throws;
use ndarray::{ArrayViewMut1, ArrayViewMut2, Axis, Ix2};
use numpy::{PyArray, PyArray1, PyArrayMethods};
use pyo3::{
    types::{PyTuple, PyTupleMethods},
    PyAny, PyResult,
};
use std::any::TypeId;

pub enum Int64Block<'a> {
    NumPy(ArrayViewMut2<'a, i64>),
    Extention(ArrayViewMut1<'a, i64>, ArrayViewMut1<'a, bool>),
}

impl<'a> ExtractBlockFromBound<'a> for Int64Block<'a> {
    fn extract_block<'b: 'a>(ob: &'b pyo3::Bound<'a, PyAny>) -> PyResult<Self> {
        if let Ok(array) = ob.cast::<PyArray<i64, Ix2>>() {
            check_dtype(ob, "int64")?;
            let data = unsafe { array.as_array_mut() };
            Ok(Int64Block::NumPy(data))
        } else {
            let tuple = ob.cast::<PyTuple>()?;
            // let data = tuple.get_borrowed_item(0)?;
            let data = &tuple.as_slice()[0];
            let mask = &tuple.as_slice()[1];
            check_dtype(data, "int64")?;
            check_dtype(mask, "bool")?;
            Ok(Int64Block::Extention(
                unsafe { data.cast::<PyArray1<i64>>()?.as_array_mut() },
                unsafe { mask.cast::<PyArray1<bool>>()?.as_array_mut() },
            ))
        }
    }
}

impl<'a> Int64Block<'a> {
    #[throws(ConnectorXPythonError)]
    pub fn split(self) -> Vec<Int64Column> {
        let mut ret = vec![];
        match self {
            Int64Block::Extention(data, mask) => ret.push(Int64Column {
                data: data
                    .into_slice()
                    .ok_or_else(|| anyhow!("get None for Int64 data"))?
                    .as_mut_ptr(),
                mask: Some(
                    mask.into_slice()
                        .ok_or_else(|| anyhow!("get None for Int64 mask"))?
                        .as_mut_ptr(),
                ),
            }),
            Int64Block::NumPy(mut view) => {
                let nrows = view.ncols();
                while view.nrows() > 0 {
                    let (col, rest) = view.split_at(Axis(0), 1);
                    view = rest;
                    ret.push(Int64Column {
                        data: col
                            .into_shape_with_order(nrows)?
                            .into_slice()
                            .ok_or_else(|| anyhow!("get None for splitted Int64 data"))?
                            .as_mut_ptr(),
                        mask: None,
                    })
                }
            }
        }
        ret
    }
}

// for uint64 and Int64
pub struct Int64Column {
    data: *mut i64,
    mask: Option<*mut bool>,
}

unsafe impl Send for Int64Column {}
unsafe impl Sync for Int64Column {}

impl PandasColumnObject for Int64Column {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<i64>() || id == TypeId::of::<Option<i64>>()
    }

    fn typename(&self) -> &'static str {
        std::any::type_name::<i64>()
    }
}

impl PandasColumn<i64> for Int64Column {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: i64, row: usize) {
        unsafe { *self.data.add(row) = val };
        if let Some(mask) = self.mask.as_mut() {
            unsafe { *mask.add(row) = false };
        }
    }
}

impl PandasColumn<Option<i64>> for Int64Column {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: Option<i64>, row: usize) {
        match val {
            Some(val) => {
                unsafe { *self.data.add(row) = val };
                if let Some(mask) = self.mask.as_mut() {
                    unsafe { *mask.add(row) = false };
                }
            }
            None => {
                if let Some(mask) = self.mask.as_mut() {
                    unsafe { *mask.add(row) = true };
                } else {
                    panic!("Writing null i64 to not null pandas array")
                }
            }
        }
    }
}

impl HasPandasColumn for i64 {
    type PandasColumn<'a> = Int64Column;
}

impl HasPandasColumn for Option<i64> {
    type PandasColumn<'a> = Int64Column;
}

impl Int64Column {
    pub fn partition(self, counts: usize) -> Vec<Int64Column> {
        let mut partitions = vec![];

        for _ in 0..counts {
            partitions.push(Int64Column {
                data: self.data,
                mask: self.mask,
            });
        }

        partitions
    }
}
