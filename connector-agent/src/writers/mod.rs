pub mod dummy;

use crate::errors::Result;
use crate::types::{DataType, DataTypeCheck};

pub trait Writer {
    type PartitionWriter<'a>: PartitionWriter<'a>;

    fn allocate(nrow: usize, schema: Vec<DataType>) -> Self;
    fn partition_writer<'a>(&'a mut self, counts: &[usize]) -> Vec<Self::PartitionWriter<'a>>;
    fn schema(&self) -> &[DataType];
}

pub trait PartitionWriter<'a> {
    unsafe fn write<T>(&mut self, row: usize, col: usize, value: T);
    fn write_safe<T>(&mut self, row: usize, col: usize, value: T) -> Result<()>
    where
        DataType: DataTypeCheck<T>;
    fn nrows(&self) -> usize;
    fn ncols(&self) -> usize;
}
