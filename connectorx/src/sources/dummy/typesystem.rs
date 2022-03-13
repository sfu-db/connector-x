// Each variant in DataType represents a type that connectorx currently
// supports to read from a data source and write into a destination.
// When adding a new supported type T and associate it to the native representation N, please do
// 1. Add a T variant to DataType.
// 2. Add `DataType::T => N` to the macro impl_typesystem!.
// 3. Add `DataType::T => N` to the macro impl_transmit!.
//

use chrono::{DateTime, Utc};
/// This is a dummy type system used in this library.
/// For all the sources, their output values must be one of the types defined by DummyTypeSystem.
/// For all the destinations, they must support writing any value whose type is defined by DummyTypeSystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DummyTypeSystem {
    F64(bool),
    I64(bool),
    Bool(bool),
    String(bool),
    DateTime(bool),
}

impl_typesystem! {
    system = DummyTypeSystem,
    mappings = {
        { F64 => f64 }
        { I64 => i64 }
        { Bool => bool }
        { String => String }
        { DateTime => DateTime<Utc> }
    }
}
