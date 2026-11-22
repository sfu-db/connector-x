//! Zero-copy column writers for the pandas destination.
//!
//! # Safety
//!
//! The `// SAFETY:` comments in the submodules refer to these invariants:
//!
//! - **(A) Allocation.** `PandasDestination::allocate_py` creates every block
//!   with `PyArray{1,2}::zeros(.., false)` (C-contiguous) and `nrow` equal to
//!   the dispatcher's total row count, so each column is one contiguous run of
//!   `nrow` elements. The arrays are owned by `PandasDestination::block_datas`
//!   and reach Python only through `result()`, after all writes. Python holds
//!   no reference to them meanwhile, so writing after the dispatcher has
//!   released the GIL (`py.detach`) races with no Python code. The partitions
//!   holding the raw column pointers borrow the destination mutably, so the
//!   arrays outlive them.
//! - **(B) Exclusive access.** `extract_block` (and its `as_array_mut`) runs
//!   once per block in `PandasDestination::partition`, on arrays nothing else
//!   references, so no other view exists. Each column's `partition` hands the
//!   *same* base pointer to every partition; rows are not split up front.
//!   Partitions stay disjoint because `PandasPartitionDestination::aquire_row`
//!   reserves a unique `[cur_row, cur_row + n)` range per batch with
//!   `fetch_add` on a shared counter, so no two partitions write the same row.
//! - **(C) Bounds.** `row < nrow` holds only if the rows fetched by all
//!   partitions add up to at most the count used for allocation. This is not
//!   checked: `write` does no bounds check and `aquire_row` never compares
//!   against `nrow`. A source that returns more rows than it counted (e.g. the
//!   table grew between the count query and the fetch) writes out of bounds.
//! - **(D) Object columns** (string, bytes, list) have dtype `object`, so each
//!   element is a `PyObject*`. `zeros` fills them with owned references to the
//!   int `0`, which is what the first assignment drops. New objects are created
//!   and stored while attached to the interpreter (`Python::attach`) and
//!   holding `GIL_MUTEX`. The Rust element types (`Py<PyString>` etc.) are not
//!   type-checked: slots may also hold `0` or `None`, so only assignment and
//!   drop may be relied on for slots this code did not just fill.

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
