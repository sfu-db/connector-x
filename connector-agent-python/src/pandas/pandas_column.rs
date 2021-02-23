use crate::errors::{ConnectorAgentPythonError, Result};
use ndarray::{ArrayViewMut1, ArrayViewMut2, Axis, Ix, Ix2};
use numpy::{PyArray, PyArray1};
use pyo3::{types::IntoPyDict, FromPyObject, PyAny, PyResult, Python};
use std::any::TypeId;
use std::mem::transmute;

pub trait PandasColumnObject: Send {
    fn elem_type_id(&self) -> TypeId;
}

pub trait PandasColumn<V>: Sized + PandasColumnObject {
    fn write(&mut self, i: usize, val: V);
}

// Indicates a type has an associated pandas column
pub trait HasPandasColumn: Sized {
    type PandasColumn<'a>: PandasColumn<Self>;
}

// uint64
pub enum UInt64Block<'a> {
    NumPy(ArrayViewMut2<'a, u64>),
    Extention(ArrayViewMut1<'a, u64>, ArrayViewMut1<'a, bool>),
}

impl<'a> UInt64Block<'a> {
    pub fn extract(py: Python<'a>, ob: &'a PyAny) -> PyResult<Self> {
        if let Ok(array) = ob.downcast::<PyArray<u64, Ix2>>() {
            let data = unsafe { array.as_array_mut() };
            Ok(UInt64Block::NumPy(data))
        } else {
            // run python code
            let locals = [("array", ob)].into_py_dict(py);
            py.run("pair = (array._data, array._mask)", None, Some(locals))?;

            let (data, mask): (&PyAny, &PyAny) =
                locals.get_item("pair").unwrap().extract::<(_, _)>()?;

            Ok(UInt64Block::Extention(
                unsafe { data.downcast::<PyArray1<u64>>().unwrap().as_array_mut() },
                unsafe { mask.downcast::<PyArray1<bool>>().unwrap().as_array_mut() },
            ))
        }
    }

    pub fn split(self) -> Vec<UInt64Column<'a>> {
        let mut ret = vec![];
        match self {
            UInt64Block::Extention(data, mask) => ret.push(UInt64Column {
                data,
                mask: Some(mask),
            }),
            UInt64Block::NumPy(mut view) => {
                let nrows = view.ncols();
                while view.nrows() > 1 {
                    let (col, rest) = view.split_at(Axis(0), 1);
                    view = rest;
                    ret.push(UInt64Column {
                        data: col.into_shape(nrows).expect("reshape"),
                        mask: None,
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
}

impl<'a> PandasColumnObject for UInt64Column<'a> {
    fn elem_type_id(&self) -> TypeId {
        TypeId::of::<u64>()
    }
}

impl<'a> PandasColumn<u64> for UInt64Column<'a> {
    fn write(&mut self, i: usize, val: u64) {
        self.data[i] = val;
        if let Some(mask) = self.mask.as_mut() {
            mask[i] = false;
        }
    }
}

impl<'a> PandasColumn<Option<u64>> for UInt64Column<'a> {
    fn write(&mut self, i: usize, val: Option<u64>) {
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
            });
        }

        partitions
    }
}

// Float
pub struct Float64Block<'a> {
    data: ArrayViewMut2<'a, f64>,
}

impl<'a> Float64Block<'a> {
    pub fn extract(py: Python<'a>, ob: &'a PyAny) -> PyResult<Self> {
        let array = ob.downcast::<PyArray<f64, Ix2>>()?;
        let data = unsafe { array.as_array_mut() };
        Ok(Float64Block { data })
    }
    pub fn split(self) -> Vec<Float64Column<'a>> {
        let mut ret = vec![];
        let mut view = self.data;

        let nrows = view.ncols();
        while view.nrows() > 1 {
            let (col, rest) = view.split_at(Axis(0), 1);
            view = rest;
            ret.push(Float64Column {
                data: col.into_shape(nrows).expect("reshape"),
            })
        }
        ret
    }
}

pub struct Float64Column<'a> {
    data: ArrayViewMut1<'a, f64>,
}

impl<'a> PandasColumnObject for Float64Column<'a> {
    fn elem_type_id(&self) -> TypeId {
        TypeId::of::<f64>()
    }
}

impl<'a> PandasColumn<f64> for Float64Column<'a> {
    fn write(&mut self, i: usize, val: f64) {
        self.data[i] = val;
    }
}

impl<'a> PandasColumn<Option<f64>> for Float64Column<'a> {
    fn write(&mut self, i: usize, val: Option<f64>) {
        match val {
            None => self.data[i] = f64::NAN,
            Some(val) => self.data[i] = val,
        }
    }
}

impl HasPandasColumn for f64 {
    type PandasColumn<'a> = Float64Column<'a>;
}

impl HasPandasColumn for Option<f64> {
    type PandasColumn<'a> = Float64Column<'a>;
}

impl<'a> Float64Column<'a> {
    pub fn partition(self, counts: &[usize]) -> Vec<Float64Column<'a>> {
        let mut partitions = vec![];
        let mut data = self.data;

        for &c in counts {
            let (splitted, rest) = data.split_at(Axis(0), c);
            data = rest;
            partitions.push(Float64Column {
                data: splitted.into_shape(c).expect("reshape"),
            });
        }

        partitions
    }
}
// String
pub struct StringColumn<'a> {
    data: ArrayViewMut1<'a, String>, // todo: fix me
    mask: ArrayViewMut1<'a, bool>,
}

impl<'a> PandasColumnObject for StringColumn<'a> {
    fn elem_type_id(&self) -> TypeId {
        TypeId::of::<String>()
    }
}

impl<'a> PandasColumn<String> for StringColumn<'a> {
    fn write(&mut self, i: usize, val: String) {
        todo!()
    }
}

impl<'a> PandasColumn<Option<String>> for StringColumn<'a> {
    fn write(&mut self, i: usize, val: Option<String>) {
        todo!()
    }
}

impl HasPandasColumn for String {
    type PandasColumn<'a> = StringColumn<'a>;
}

impl HasPandasColumn for Option<String> {
    type PandasColumn<'a> = StringColumn<'a>;
}

// Boolean
pub struct BooleanColumn<'a> {
    data: ArrayViewMut1<'a, bool>,
    mask: ArrayViewMut1<'a, bool>,
}

impl<'a> PandasColumnObject for BooleanColumn<'a> {
    fn elem_type_id(&self) -> TypeId {
        TypeId::of::<bool>()
    }
}

impl<'a> PandasColumn<bool> for BooleanColumn<'a> {
    fn write(&mut self, i: usize, val: bool) {
        todo!()
    }
}

impl<'a> PandasColumn<Option<bool>> for BooleanColumn<'a> {
    fn write(&mut self, i: usize, val: Option<bool>) {
        todo!()
    }
}

impl HasPandasColumn for bool {
    type PandasColumn<'a> = BooleanColumn<'a>;
}

impl HasPandasColumn for Option<bool> {
    type PandasColumn<'a> = BooleanColumn<'a>;
}
