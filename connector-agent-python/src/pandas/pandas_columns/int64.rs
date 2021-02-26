use super::{check_dtype, HasPandasColumn, PandasColumn, PandasColumnObject};
use ndarray::{ArrayViewMut1, ArrayViewMut2, Axis, Ix2};
use numpy::{PyArray, PyArray1};
use pyo3::{FromPyObject, PyAny, PyResult};
use std::any::TypeId;

pub enum Int64Block<'a> {
    NumPy(ArrayViewMut2<'a, i64>),
    Extention(ArrayViewMut1<'a, i64>, ArrayViewMut1<'a, bool>),
}
impl<'a> FromPyObject<'a> for Int64Block<'a> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        if let Ok(array) = ob.downcast::<PyArray<i64, Ix2>>() {
            check_dtype(ob, "int64")?;
            let data = unsafe { array.as_array_mut() };
            Ok(Int64Block::NumPy(data))
        } else {
            let data = ob.getattr("_data")?;
            let mask = ob.getattr("_mask")?;

            Ok(Int64Block::Extention(
                unsafe { data.downcast::<PyArray1<i64>>().unwrap().as_array_mut() },
                unsafe { mask.downcast::<PyArray1<bool>>().unwrap().as_array_mut() },
            ))
        }
    }
}

impl<'a> Int64Block<'a> {
    pub fn split(self) -> Vec<Int64Column<'a>> {
        let mut ret = vec![];
        match self {
            Int64Block::Extention(data, mask) => ret.push(Int64Column {
                data,
                mask: Some(mask),
            }),
            Int64Block::NumPy(mut view) => {
                let nrows = view.ncols();
                while view.nrows() > 0 {
                    let (col, rest) = view.split_at(Axis(0), 1);
                    view = rest;
                    ret.push(Int64Column {
                        data: col.into_shape(nrows).expect("reshape"),
                        mask: None,
                    })
                }
            }
        }
        ret
    }
}

// for uint64 and Int64
pub struct Int64Column<'a> {
    data: ArrayViewMut1<'a, i64>,
    mask: Option<ArrayViewMut1<'a, bool>>,
}

impl<'a> PandasColumnObject for Int64Column<'a> {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<i64>() || id == TypeId::of::<Option<i64>>()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn typename(&self) -> &'static str {
        std::any::type_name::<i64>()
    }
}

impl<'a> PandasColumn<i64> for Int64Column<'a> {
    fn write(&mut self, i: usize, val: i64) {
        self.data[i] = val;
        if let Some(mask) = self.mask.as_mut() {
            mask[i] = false;
        }
    }
}

impl<'a> PandasColumn<Option<i64>> for Int64Column<'a> {
    fn write(&mut self, i: usize, val: Option<i64>) {
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
                    panic!("Writing null i64 to not null pandas array")
                }
            }
        }
    }
}

impl HasPandasColumn for i64 {
    type PandasColumn<'a> = Int64Column<'a>;
}

impl HasPandasColumn for Option<i64> {
    type PandasColumn<'a> = Int64Column<'a>;
}

impl<'a> Int64Column<'a> {
    pub fn partition(self, counts: &[usize]) -> Vec<Int64Column<'a>> {
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

            partitions.push(Int64Column {
                data: splitted_data,
                mask: splitted_mask,
            });
        }

        partitions
    }
}
