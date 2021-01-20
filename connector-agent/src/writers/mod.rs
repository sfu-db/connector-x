pub mod dummy;

use crate::errors::Result;
use crate::typesystem::TypeSystem;

/// A `Writer` is associated with a `TypeSystem` and a `PartitionWriter`.
/// `PartitionWriter` allows multiple threads write data into the buffer owned by `Writer`.
pub trait Writer<'a>: Sized {
    type TypeSystem;
    type PartitionWriter: PartitionWriter<'a, TypeSystem = Self::TypeSystem>;

    /// Construct the `Writer`. This allocates the memory based on the types of each columns
    /// and the number of rows.
    fn allocate(nrow: usize, schema: Vec<Self::TypeSystem>) -> Result<Self>;
    /// Create a bunch of partition writers, with each write `count` number of rows.
    fn partition_writers(&'a mut self, counts: &[usize]) -> Vec<Self::PartitionWriter>;
    /// Return the schema of the writer.
    fn schema(&self) -> &[Self::TypeSystem];
}

/// `PartitionWriter` writes values to its own region. `PartitionWriter` is parameterized
/// on lifetime `'a`, which is the lifetime of the parent `Writer`. This indicates
/// the `PartitionWriter` can never live longer than the parent.
pub trait PartitionWriter<'a> {
    type TypeSystem;

    /// Write a value of type T to the location (row, col). The value is unchecked against the schema.
    /// This function is unsafe due to unchecked.
    unsafe fn write<T>(&mut self, row: usize, col: usize, value: T);
    /// Write a value of type T to the location (row, col), checked version. If T mismatch with the
    /// schema, `ConnectorAgentError::UnexpectedType` will return.
    fn write_checked<T>(&mut self, row: usize, col: usize, value: T) -> Result<()>
    where
        Self::TypeSystem: TypeSystem<T>;
    /// Number of rows this `PartitionWriter` controls.
    fn nrows(&self) -> usize;
    /// Number of rows this `PartitionWriter` controls.
    fn ncols(&self) -> usize;
}
