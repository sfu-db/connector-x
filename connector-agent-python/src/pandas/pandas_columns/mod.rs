mod boolean;
mod float64;
mod string;
mod uint64;

pub use boolean::{BooleanBlock, BooleanColumn};
use fehler::throw;
pub use float64::{Float64Block, Float64Column};
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

fn check_numpy_dtype(ob: &PyAny, expected_dtype: &str) -> PyResult<()> {
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
