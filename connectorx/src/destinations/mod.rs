#[cfg(feature = "dst_arrow")]
pub mod arrow;
#[cfg(feature = "dst_memory")]
pub mod memory;

use crate::data_order::DataOrder;
use crate::errors::Result;
use crate::typesystem::{TypeAssoc, TypeSystem};

/// A `Destination` is associated with a `TypeSystem` and a `PartitionDestination`.
/// `PartitionDestination` allows multiple threads write data into the buffer owned by `Destination`.
pub trait Destination: Sized {
    const DATA_ORDERS: &'static [DataOrder];
    type TypeSystem: TypeSystem;
    type Partition<'a>: DestinationPartition<'a, TypeSystem = Self::TypeSystem>;

    /// Construct the `Destination`.
    /// This allocates the memory based on the types of each columns
    /// and the number of rows.
    fn allocate<S: AsRef<str>>(
        &mut self,
        nrow: usize,
        names: &[S],
        schema: &[Self::TypeSystem],
        data_order: DataOrder,
    ) -> Result<()>;

    /// Create a bunch of partition destinations, with each write `count` number of rows.
    fn partition(&mut self, counts: &[usize]) -> Result<Vec<Self::Partition<'_>>>;
    /// Return the schema of the destination.
    fn schema(&self) -> &[Self::TypeSystem];
}

/// `PartitionDestination` writes values to its own region. `PartitionDestination` is parameterized
/// on lifetime `'a`, which is the lifetime of the parent `Destination`. This indicates
/// the `PartitionDestination` can never live longer than the parent.
pub trait DestinationPartition<'a>: Send {
    type TypeSystem: TypeSystem;

    /// Write a value of type T to the location (row, col). If T mismatch with the
    /// schema, `ConnectorAgentError::TypeCheckFailed` will return.
    fn write<T>(&mut self, value: T) -> Result<()>
    where
        T: TypeAssoc<Self::TypeSystem>,
        Self: Consume<T>,
    {
        self.consume(value)
    }

    /// Number of rows this `PartitionDestination` controls.
    fn nrows(&self) -> usize;

    /// Number of rows this `PartitionDestination` controls.
    fn ncols(&self) -> usize;

    /// Final clean ups
    fn finalize(&mut self) -> Result<()> {
        Ok(())
    }
}

/// A type implemented `Consume<T>` means that it can consume a value `T` by adding it to it's own buffer.
pub trait Consume<T> {
    fn consume(&mut self, value: T) -> Result<()>;
}
