// Each variant in DataType represents a type that connector-agent currently
// supports to read from a data source and write into a writer.
// When adding a new supported type T and associate it to the native representation N, please do
// 1. Add a T variant to DataType.
// 2. Add `DataType::T => N` to the macro impl_typesystem!.
// 3. Add `DataType::T => N` to the macro impl_transmit!.
//

use crate::typesystem::TypeSystem;
use chrono::{DateTime, Utc};
/// This is our intermediate type system used in this library.
/// For all the sources, their output values must be one of the types defined by DataType.
/// For all the writers, they must support writing any value whose type is defined by DataType.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DataType {
    F64(bool), // nullable
    I64(bool),
    Bool(bool),
    String(bool),
    DateTime(bool),
}

define_typesystem! {
    DataType,
    [DataType::F64] => [f64],
    [DataType::I64] => [i64],
    [DataType::Bool] => [bool],
    [DataType::String] => [String],
    [DataType::DateTime] => [DateTime<Utc>],
}
