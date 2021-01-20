// Each variant in DataType represents a type that connector-agent currently
// supports to read from a data source and write into a writer.
// When adding a new supported type T and associate it to the native representation N, please do
// 1. Add a T variant to DataType.
// 2. Add `DataType::T => N` to the macro impl_typesystem!.
// 3. Add `DataType::T => N` to the macro impl_transmit!.
//

use crate::{errors::Result, typesystem::TypeSystem, writers::PartitionWriter};

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    F64,
    U64,
}

impl_typesystem!(DataType, DataType::F64 => f64, DataType::U64 => u64);
impl_transmit!(DataType, DataType::F64 => f64, DataType::U64 => u64);
