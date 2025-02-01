mod array;
mod boolean;
mod bytes;
mod datetime;
mod float64;
mod int64;
mod string;
// TODO: use macro for integers

use crate::errors::Result;
pub use crate::pandas::pandas_columns::array::{ArrayBlock, PyList};
pub use crate::pandas::pandas_columns::bytes::{BytesBlock, PyBytes};
pub use boolean::BooleanBlock;
pub use datetime::DateTimeBlock;
use fehler::throw;
pub use float64::Float64Block;
pub use int64::Int64Block;
use pyo3::{exceptions::PyRuntimeError, intern, types::PyAnyMethods, PyAny, PyResult};
use std::any::TypeId;
use std::sync::Mutex;
pub use string::StringBlock;

// A global GIL lock for Python object allocations like string, bytes and list
lazy_static! {
    static ref GIL_MUTEX: Mutex<()> = Mutex::new(());
}

pub trait PandasColumnObject: Send {
    #[allow(dead_code)]
    fn typecheck(&self, _: TypeId) -> bool;
    #[allow(dead_code)]
    fn typename(&self) -> &'static str;
    fn finalize(&mut self) -> Result<()> {
        Ok(())
    }
}

pub trait PandasColumn<V>: Sized + PandasColumnObject {
    fn write(&mut self, val: V, row: usize) -> Result<()>;
}

// Indicates a type has an associated pandas column
pub trait HasPandasColumn: Sized {
    type PandasColumn<'a>: PandasColumn<Self>;
}

pub trait ExtractBlockFromBound<'a>: Sized {
    fn extract_block<'b: 'a>(ob: &'b pyo3::Bound<'a, PyAny>) -> PyResult<Self>;
}

pub fn check_dtype<'py>(ob: &pyo3::Bound<'py, PyAny>, expected_dtype: &str) -> PyResult<()> {
    let dtype = ob.getattr(intern!(ob.py(), "dtype"))?.str()?;
    // https://pyo3.rs/main/doc/pyo3/types/struct.pystring#equality
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
