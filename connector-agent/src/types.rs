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

impl TypeSystem for DataType {}

associate_typesystem! {
    DataType,
    DataType::F64(false) => f64,
    DataType::F64(true) => Option<f64>,
    DataType::I64(false) => i64,
    DataType::I64(true) => Option<i64>,
    DataType::Bool(false) => bool,
    DataType::Bool(true) => Option<bool>,
    DataType::String(false) => String,
    DataType::String(true) => Option<String>,
    DataType::DateTime(false) => DateTime<Utc>,
    DataType::DateTime(true) => Option<DateTime<Utc>>
}
