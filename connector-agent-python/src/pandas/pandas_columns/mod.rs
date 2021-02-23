mod boolean;
mod float64;
mod string;
mod uint64;

pub use boolean::{BooleanBlock, BooleanColumn};
pub use float64::{Float64Block, Float64Column};
use std::any::TypeId;
pub use string::StringColumn;
pub use uint64::{UInt64Block, UInt64Column};

pub trait PandasColumnObject: Send {
    fn typecheck(&self, _: TypeId) -> bool;
}

pub trait PandasColumn<V>: Sized + PandasColumnObject {
    fn write(&mut self, i: usize, val: V);
}

// Indicates a type has an associated pandas column
pub trait HasPandasColumn: Sized {
    type PandasColumn<'a>: PandasColumn<Self>;
}
