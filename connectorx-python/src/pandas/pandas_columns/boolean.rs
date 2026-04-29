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

// Boolean
pub enum BooleanBlock<'a> {
    NumPy(ArrayViewMut2<'a, bool>),
    Extention(ArrayViewMut1<'a, bool>, ArrayViewMut1<'a, bool>),
}

impl<'a> ExtractBlockFromBound<'a> for BooleanBlock<'a> {
    fn extract_block<'b: 'a>(ob: &'b pyo3::Bound<'a, PyAny>) -> PyResult<Self> {
        if let Ok(array) = ob.cast::<PyArray<bool, Ix2>>() {
            // if numpy array
            check_dtype(ob, "bool")?;
            let data = unsafe { array.as_array_mut() };
            Ok(BooleanBlock::NumPy(data))
        } else {
            // if extension array
            let tuple = ob.cast::<PyTuple>()?;
            let data = &tuple.as_slice()[0];
            let mask = &tuple.as_slice()[1];
            check_dtype(data, "bool")?;
            check_dtype(mask, "bool")?;

            Ok(BooleanBlock::Extention(
                unsafe { data.cast::<PyArray1<bool>>()?.as_array_mut() },
                unsafe { mask.cast::<PyArray1<bool>>()?.as_array_mut() },
            ))
        }
    }
}

impl<'a> BooleanBlock<'a> {
    #[throws(ConnectorXPythonError)]
    pub fn split(self) -> Vec<BooleanColumn> {
        let mut ret = vec![];
        match self {
            BooleanBlock::Extention(data, mask) => ret.push(BooleanColumn {
                data: data
                    .into_slice()
                    .ok_or_else(|| anyhow!("get None for Boolean data"))?
                    .as_mut_ptr(),
                mask: Some(
                    mask.into_slice()
                        .ok_or_else(|| anyhow!("get None for Boolean mask"))?
                        .as_mut_ptr(),
                ),
            }),
            BooleanBlock::NumPy(mut view) => {
                let nrows = view.ncols();
                while view.nrows() > 0 {
                    let (col, rest) = view.split_at(Axis(0), 1);
                    view = rest;
                    ret.push(BooleanColumn {
                        data: col
                            .into_shape_with_order(nrows)?
                            .into_slice()
                            .ok_or_else(|| anyhow!("get None for splitted Boolean data"))?
                            .as_mut_ptr(),
                        mask: None,
                    })
                }
            }
        }
        ret
    }
}

pub struct BooleanColumn {
    data: *mut bool,
    mask: Option<*mut bool>,
}

unsafe impl Send for BooleanColumn {}
unsafe impl Sync for BooleanColumn {}

impl PandasColumnObject for BooleanColumn {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<bool>() || id == TypeId::of::<Option<bool>>()
    }
    fn typename(&self) -> &'static str {
        std::any::type_name::<bool>()
    }
}

impl PandasColumn<bool> for BooleanColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: bool, row: usize) {
        unsafe { *self.data.add(row) = val };
        if let Some(mask) = self.mask.as_mut() {
            unsafe { *mask.add(row) = false };
        }
    }
}

impl PandasColumn<Option<bool>> for BooleanColumn {
    #[throws(ConnectorXPythonError)]
    fn write(&mut self, val: Option<bool>, row: usize) {
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
                    panic!("Writing null u64 to not null pandas array")
                }
            }
        }
    }
}

impl HasPandasColumn for bool {
    type PandasColumn<'a> = BooleanColumn;
}

impl HasPandasColumn for Option<bool> {
    type PandasColumn<'a> = BooleanColumn;
}

impl BooleanColumn {
    pub fn partition(self, counts: usize) -> Vec<BooleanColumn> {
        let mut partitions = vec![];

        for _ in 0..counts {
            partitions.push(BooleanColumn {
                data: self.data,
                mask: self.mask,
            });
        }

        partitions
    }
}
