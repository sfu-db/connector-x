pub mod dummy;

use crate::errors::Result;
use crate::types::{DataType, TypeSystem};

pub trait Writer<'a>: Sized {
    type PartitionWriter: PartitionWriter<'a, TypeSystem = Self::TypeSystem>;
    type TypeSystem;

    fn allocate(nrow: usize, schema: Vec<DataType>) -> Result<Self>;
    fn partition_writer(&'a mut self, counts: &[usize]) -> Vec<Self::PartitionWriter>;
    fn schema(&self) -> &[DataType];
}

pub trait PartitionWriter<'a> {
    type TypeSystem;

    unsafe fn write<T>(&mut self, row: usize, col: usize, value: T);
    fn write_checked<T>(&mut self, row: usize, col: usize, value: T) -> Result<()>
    where
        Self::TypeSystem: TypeSystem<T>;
    fn nrows(&self) -> usize;
    fn ncols(&self) -> usize;
}
