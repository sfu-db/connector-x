use super::{HasPandasColumn, PandasColumn, PandasColumnObject};
use ndarray::{ArrayViewMut1, ArrayViewMut2, Axis, Ix2};
use numpy::{PyArray, PyArray1};
use pyo3::{types::IntoPyDict, PyAny, PyResult, Python};
use std::any::TypeId;

// String
pub struct StringColumn<'a> {
    data: ArrayViewMut1<'a, String>, // todo: fix me
    mask: ArrayViewMut1<'a, bool>,
}

impl<'a> PandasColumnObject for StringColumn<'a> {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<String>() || id == TypeId::of::<Option<String>>()
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
