mod boolean;
mod bytes;
mod datetime;
mod float64;
mod int64;
mod string;
// TODO: use macro for integers

pub use crate::pandas::pandas_columns::bytes::{BytesBlock, BytesColumn};
pub use boolean::{BooleanBlock, BooleanColumn};
use connectorx::Result;
pub use datetime::{DateTimeBlock, DateTimeColumn};
use fehler::throw;
pub use float64::{Float64Block, Float64Column};
pub use int64::{Int64Block, Int64Column};
use pyo3::{exceptions::PyRuntimeError, PyAny, PyResult};
use std::any::TypeId;
pub use string::{StringBlock, StringColumn};

pub trait PandasColumnObject: Send {
    fn typecheck(&self, _: TypeId) -> bool;
    fn typename(&self) -> &'static str;
    fn len(&self) -> usize;
    fn finalize(&mut self) -> Result<()> {
        Ok(())
    }
}

pub trait PandasColumn<V>: Sized + PandasColumnObject {
    fn write(&mut self, val: V) -> Result<()>;
}

// Indicates a type has an associated pandas column
pub trait HasPandasColumn: Sized {
    type PandasColumn<'a>: PandasColumn<Self>;
}

pub fn check_dtype(ob: &PyAny, expected_dtype: &str) -> PyResult<()> {
    let dtype = ob.getattr("dtype")?.str()?;
    let dtype = dtype.to_str()?;
    if dtype != expected_dtype {
        throw!(PyRuntimeError::new_err(format!(
            "expecting ndarray to be '{}' found '{}' at {}:{}",
            expected_dtype,
            dtype,
            file!(),
            line!()
        )));
    }
    Ok(())
}
