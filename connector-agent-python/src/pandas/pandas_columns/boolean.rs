use super::{HasPandasColumn, PandasColumn, PandasColumnObject};
use ndarray::{ArrayViewMut1, ArrayViewMut2, Axis, Ix2};
use numpy::{PyArray, PyArray1};
use pyo3::{types::IntoPyDict, PyAny, PyResult, Python};
use std::any::TypeId;

// Boolean
pub enum BooleanBlock<'a> {
    NumPy(ArrayViewMut2<'a, bool>),
    Extention(ArrayViewMut1<'a, bool>, ArrayViewMut1<'a, bool>),
}

impl<'a> BooleanBlock<'a> {
    pub fn extract(py: Python<'a>, ob: &'a PyAny) -> PyResult<Self> {
        if let Ok(array) = ob.downcast::<PyArray<bool, Ix2>>() {
            let data = unsafe { array.as_array_mut() };
            Ok(BooleanBlock::NumPy(data))
        } else {
            // run python code
            let locals = [("array", ob)].into_py_dict(py);
            py.run("pair = (array._data, array._mask)", None, Some(locals))?;

            let (data, mask): (&PyAny, &PyAny) =
                locals.get_item("pair").unwrap().extract::<(_, _)>()?;

            Ok(BooleanBlock::Extention(
                unsafe { data.downcast::<PyArray1<bool>>().unwrap().as_array_mut() },
                unsafe { mask.downcast::<PyArray1<bool>>().unwrap().as_array_mut() },
            ))
        }
    }

    pub fn split(self) -> Vec<BooleanColumn<'a>> {
        let mut ret = vec![];
        match self {
            BooleanBlock::Extention(data, mask) => ret.push(BooleanColumn {
                data,
                mask: Some(mask),
            }),
            BooleanBlock::NumPy(mut view) => {
                let nrows = view.ncols();
                while view.nrows() > 0 {
                    let (col, rest) = view.split_at(Axis(0), 1);
                    view = rest;
                    ret.push(BooleanColumn {
                        data: col.into_shape(nrows).expect("reshape"),
                        mask: None,
                    })
                }
            }
        }
        ret
    }
}

pub struct BooleanColumn<'a> {
    data: ArrayViewMut1<'a, bool>,
    mask: Option<ArrayViewMut1<'a, bool>>,
}

impl<'a> PandasColumnObject for BooleanColumn<'a> {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<bool>() || id == TypeId::of::<Option<bool>>()
    }
}

impl<'a> PandasColumn<bool> for BooleanColumn<'a> {
    fn write(&mut self, i: usize, val: bool) {
        self.data[i] = val;
        if let Some(mask) = self.mask.as_mut() {
            mask[i] = false;
        }
    }
}

impl<'a> PandasColumn<Option<bool>> for BooleanColumn<'a> {
    fn write(&mut self, i: usize, val: Option<bool>) {
        match val {
            Some(val) => {
                self.data[i] = val;
                if let Some(mask) = self.mask.as_mut() {
                    mask[i] = false;
                }
            }
            None => {
                if let Some(mask) = self.mask.as_mut() {
                    mask[i] = true;
                } else {
                    panic!("Writing null u64 to not null pandas array")
                }
            }
        }
    }
}

impl HasPandasColumn for bool {
    type PandasColumn<'a> = BooleanColumn<'a>;
}

impl HasPandasColumn for Option<bool> {
    type PandasColumn<'a> = BooleanColumn<'a>;
}

impl<'a> BooleanColumn<'a> {
    pub fn partition(self, counts: &[usize]) -> Vec<BooleanColumn<'a>> {
        let mut partitions = vec![];
        let mut data = self.data;
        let mut mask = self.mask;

        for &c in counts {
            let (splitted_data, rest) = data.split_at(Axis(0), c);
            data = rest;
            let (splitted_mask, rest) = match mask {
                Some(mask) => {
                    let (a, b) = mask.split_at(Axis(0), c);
                    (Some(a), Some(b))
                }
                None => (None, None),
            };

            mask = rest;

            partitions.push(BooleanColumn {
                data: splitted_data,
                mask: splitted_mask,
            });
        }

        partitions
    }
}
