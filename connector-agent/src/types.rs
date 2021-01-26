// Each variant in DataType represents a type that connector-agent currently
// supports to read from a data source and write into a writer.
// When adding a new supported type T and associate it to the native representation N, please do
// 1. Add a T variant to DataType.
// 2. Add `DataType::T => N` to the macro impl_typesystem!.
// 3. Add `DataType::T => N` to the macro impl_transmit!.
//

use crate::{errors::Result, typesystem::TypeSystem, writers::PartitionWriter};

/// This is our intermediate type system used in this library.
/// For all the sources, their output values must be one of the types defined by DataType.
/// For all the writers, they must support writing any value whose type is defined by DataType.
#[derive(Debug, Clone, Copy)]
pub enum DataType {
    F64,
    U64,
    Bool,
    String
}

impl_typesystem!(DataType, DataType::F64 => f64, DataType::U64 => u64, DataType::Bool => bool, DataType::String => String);
impl_transmit!(DataType, DataType::F64 => f64, DataType::U64 => u64, DataType::Bool => bool, DataType::String => String);
