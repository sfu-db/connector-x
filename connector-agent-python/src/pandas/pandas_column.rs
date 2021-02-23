use crate::errors::{ConnectorAgentPythonError, Result};
use ndarray::{ArrayViewMut1, ArrayViewMut2};
use pyo3::{FromPyObject, PyAny, PyResult};

pub trait PandasColumn: Sized {
    fn write() {}
}

// for uint64 and UInt64
pub struct UInt64Column<'a> {
    data: ArrayViewMut1<'a, f64>,
    mask: Option<ArrayViewMut1<'a, bool>>,
}

pub struct Float64Block<'a> {
    data: ArrayViewMut2<'a, f64>,
}

impl<'a> FromPyObject<'a> for Float64Block<'a> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        todo!()
    }
}

pub struct Float64Column<'a> {
    data: ArrayViewMut1<'a, f64>,
}

pub struct StringColumn<'a> {
    data: ArrayViewMut1<'a, f64>,
    mask: ArrayViewMut1<'a, bool>,
}
