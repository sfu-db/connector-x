mod boolean;
#[allow(unused)]
mod date;
mod datetime;
mod float64;
mod int64;
mod string;
#[allow(unused)]
mod uint64;
// TODO: use macro for integers

pub use boolean::{BooleanBlock, BooleanColumn};
pub use date::{DateBlock, DateColumn};
pub use datetime::{DateTimeBlock, DateTimeColumn};
use fehler::throw;
pub use float64::{Float64Block, Float64Column};
pub use int64::{Int64Block, Int64Column};
use pyo3::{exceptions::PyRuntimeError, PyAny, PyResult};
use std::any::TypeId;
pub use string::StringColumn;
pub use uint64::{UInt64Block, UInt64Column};

pub trait PandasColumnObject: Send {
    fn typecheck(&self, _: TypeId) -> bool;
    fn typename(&self) -> &'static str;
    fn len(&self) -> usize;
}

pub trait PandasColumn<V>: Sized + PandasColumnObject {
    fn write(&mut self, i: usize, val: V);
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
