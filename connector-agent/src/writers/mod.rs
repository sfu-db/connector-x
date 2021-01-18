pub mod dummy;

use crate::errors::Result;
use crate::types::{DataType, TypeInfo};

pub trait Writer: Sized {
    type PartitionWriter<'a>: PartitionWriter<'a>;

    fn allocate(nrow: usize, schema: Vec<DataType>) -> Result<Self>;
    fn partition_writer<'a>(&'a mut self, counts: &[usize]) -> Vec<Self::PartitionWriter<'a>>;
    fn schema(&self) -> &[DataType];
}

pub trait PartitionWriter<'a> {
    unsafe fn write<T>(&mut self, row: usize, col: usize, value: T);
    fn write_checked<T>(&mut self, row: usize, col: usize, value: T) -> Result<()>
    where
        T: TypeInfo;
    fn nrows(&self) -> usize;
    fn ncols(&self) -> usize;
}
