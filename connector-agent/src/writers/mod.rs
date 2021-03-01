pub mod arrow;
pub mod memory;
pub mod pandas;

use crate::data_order::DataOrder;
use crate::errors::Result;
use crate::typesystem::{TypeAssoc, TypeSystem};

/// A `Writer` is associated with a `TypeSystem` and a `PartitionWriter`.
/// `PartitionWriter` allows multiple threads write data into the buffer owned by `Writer`.
pub trait Writer: Sized {
    const DATA_ORDERS: &'static [DataOrder];
    type TypeSystem: TypeSystem;
    type PartitionWriter<'a>: PartitionWriter<'a, TypeSystem = Self::TypeSystem>;

    /// Construct the `Writer`.
    /// This allocates the memory based on the types of each columns
    /// and the number of rows.
    fn allocate<S: AsRef<str>>(
        &mut self,
        nrow: usize,
        names: &[S],
        schema: &[Self::TypeSystem],
        data_order: DataOrder,
    ) -> Result<()>;

    /// Create a bunch of partition writers, with each write `count` number of rows.
    fn partition_writers(&mut self, counts: &[usize]) -> Result<Vec<Self::PartitionWriter<'_>>>;
    /// Return the schema of the writer.
    fn schema(&self) -> &[Self::TypeSystem];
}

/// `PartitionWriter` writes values to its own region. `PartitionWriter` is parameterized
/// on lifetime `'a`, which is the lifetime of the parent `Writer`. This indicates
/// the `PartitionWriter` can never live longer than the parent.
pub trait PartitionWriter<'a>: Send {
    type TypeSystem: TypeSystem;

    /// Write a value of type T to the location (row, col). The value is unchecked against the schema.
    /// This function is unsafe due to unchecked.
    unsafe fn write<T>(&mut self, value: T)
    where
        T: TypeAssoc<Self::TypeSystem>,
        Self: Consume<T>,
    {
        self.consume(value)
    }
    /// Write a value of type T to the location (row, col), checked version. If T mismatch with the
    /// schema, `ConnectorAgentError::UnexpectedType` will return.
    fn write_checked<T>(&mut self, value: T) -> Result<()>
    where
        T: TypeAssoc<Self::TypeSystem>,
        Self: Consume<T>,
    {
        self.consume_checked(value)
    }
    /// Number of rows this `PartitionWriter` controls.
    fn nrows(&self) -> usize;
    /// Number of rows this `PartitionWriter` controls.
    fn ncols(&self) -> usize;
}

/// A type implemented `Consume<T>` means that it can consume a value `T` by adding it to it's own buffer.
pub trait Consume<T> {
    unsafe fn consume(&mut self, value: T);
    fn consume_checked(&mut self, value: T) -> Result<()>;
}
