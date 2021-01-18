pub mod dummy;

use crate::types::DataType;

pub trait Writer {
    type PartitionWriter<'a>: PartitionWriter<'a>;

    fn allocate(nrow: usize, type_info: Vec<DataType>) -> Self;
    fn partition_writer<'a>(&'a mut self, counts: &[usize]) -> Vec<Self::PartitionWriter<'a>>;
    fn schema(&self) -> &[DataType];
}

pub trait PartitionWriter<'a> {
    unsafe fn write<T>(&mut self, row: usize, col: usize, value: T);
    fn nrows(&self) -> usize;
    fn ncols(&self) -> usize;
}
