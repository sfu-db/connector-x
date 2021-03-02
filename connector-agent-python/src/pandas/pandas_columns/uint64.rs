use super::{check_dtype, HasPandasColumn, PandasColumn, PandasColumnObject};
use ndarray::{ArrayViewMut1, ArrayViewMut2, Axis, Ix2};
use numpy::{PyArray, PyArray1};
use pyo3::{FromPyObject, PyAny, PyResult};
use std::any::TypeId;

pub enum UInt64Block<'a> {
    NumPy(ArrayViewMut2<'a, u64>),
    Extention(ArrayViewMut1<'a, u64>, ArrayViewMut1<'a, bool>),
}
impl<'a> FromPyObject<'a> for UInt64Block<'a> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        if let Ok(array) = ob.downcast::<PyArray<u64, Ix2>>() {
            check_dtype(ob, "uint64")?;
            let data = unsafe { array.as_array_mut() };
            Ok(UInt64Block::NumPy(data))
        } else {
            let data = ob.getattr("_data")?;
            let mask = ob.getattr("_mask")?;

            Ok(UInt64Block::Extention(
                unsafe { data.downcast::<PyArray1<u64>>().unwrap().as_array_mut() },
                unsafe { mask.downcast::<PyArray1<bool>>().unwrap().as_array_mut() },
            ))
        }
    }
}

impl<'a> UInt64Block<'a> {
    pub fn split(self) -> Vec<UInt64Column<'a>> {
        let mut ret = vec![];
        match self {
            UInt64Block::Extention(data, mask) => ret.push(UInt64Column {
                data,
                mask: Some(mask),
                i: 0,
            }),
            UInt64Block::NumPy(mut view) => {
                let nrows = view.ncols();
                while view.nrows() > 0 {
                    let (col, rest) = view.split_at(Axis(0), 1);
                    view = rest;
                    ret.push(UInt64Column {
                        data: col.into_shape(nrows).expect("reshape"),
                        mask: None,
                        i: 0,
                    })
                }
            }
        }
        ret
    }
}

// for uint64 and UInt64
pub struct UInt64Column<'a> {
    data: ArrayViewMut1<'a, u64>,
    mask: Option<ArrayViewMut1<'a, bool>>,
    i: usize,
}

impl<'a> PandasColumnObject for UInt64Column<'a> {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<u64>() || id == TypeId::of::<Option<u64>>()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn typename(&self) -> &'static str {
        std::any::type_name::<u64>()
    }
}

impl<'a> PandasColumn<u64> for UInt64Column<'a> {
    fn write(&mut self, val: u64) {
        self.data[self.i] = val;
        if let Some(mask) = self.mask.as_mut() {
            mask[self.i] = false;
        }
        self.i += 1;
    }
}

impl<'a> PandasColumn<Option<u64>> for UInt64Column<'a> {
    fn write(&mut self, val: Option<u64>) {
        match val {
            Some(val) => {
                self.data[self.i] = val;
                if let Some(mask) = self.mask.as_mut() {
                    mask[self.i] = false;
                }
            }
            None => {
                if let Some(mask) = self.mask.as_mut() {
                    mask[self.i] = true;
                } else {
                    panic!("Writing null u64 to not null pandas array")
                }
            }
        }
        self.i += 1;
    }
}

impl HasPandasColumn for u64 {
    type PandasColumn<'a> = UInt64Column<'a>;
}

impl HasPandasColumn for Option<u64> {
    type PandasColumn<'a> = UInt64Column<'a>;
}

impl<'a> UInt64Column<'a> {
    pub fn partition(self, counts: &[usize]) -> Vec<UInt64Column<'a>> {
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

            partitions.push(UInt64Column {
                data: splitted_data,
                mask: splitted_mask,
                i: 0,
            });
        }

        partitions
    }
}
