//! This module defines three traits [`Destination`], [`DestinationPartition`], and [`Consume`] to define a destination.
//! This module also contains destination implementations for various dataframes.

#[cfg(feature = "dst_arrow")]
pub mod arrow;
#[cfg(feature = "dst_arrow")]
pub mod arrowstream;

use crate::data_order::DataOrder;
use crate::errors::ConnectorXError;
use crate::typesystem::{TypeAssoc, TypeSystem};

/// A `Destination` is associated with a `TypeSystem` and a `PartitionDestination`.
/// `PartitionDestination` allows multiple threads write data into the buffer owned by `Destination`.
///
/// [`Dispatcher`](crate::dispatcher::Dispatcher) calls [`needs_count`](Destination::needs_count),
/// then [`allocate`](Destination::allocate), then [`partition`](Destination::partition), and
/// hands each partition to a worker thread.
pub trait Destination: Sized {
    /// Supported data orders, ordering by preference.
    const DATA_ORDERS: &'static [DataOrder];
    /// The type system this `Destination` is associated with.
    type TypeSystem: TypeSystem;
    /// The partition type created by [`partition`](Destination::partition). It borrows the
    /// destination's buffer.
    type Partition<'a>: DestinationPartition<'a, TypeSystem = Self::TypeSystem, Error = Self::Error>
    where
        Self: 'a;
    /// The error type returned by this destination and its partitions.
    type Error: From<ConnectorXError> + Send;

    /// Specify whether the destination needs total rows in advance
    /// in order to pre-allocate the buffer.
    fn needs_count(&self) -> bool;

    /// Construct the `Destination`.
    /// This allocates the memory based on the types of each columns
    /// and the number of rows.
    fn allocate<S: AsRef<str>>(
        &mut self,
        nrow: usize,
        names: &[S],
        schema: &[Self::TypeSystem],
        data_order: DataOrder,
    ) -> Result<(), Self::Error>;

    /// Create `counts` partition destinations, one per source partition, in the same order.
    fn partition(&mut self, counts: usize) -> Result<Vec<Self::Partition<'_>>, Self::Error>;
    /// Return the schema of the destination.
    fn schema(&self) -> &[Self::TypeSystem];
}

/// `PartitionDestination` writes values to its own region. `PartitionDestination` is parameterized
/// on lifetime `'a`, which is the lifetime of the parent `Destination`. Usually,
/// a `PartitionDestination` can never live longer than the parent.
pub trait DestinationPartition<'a>: Send {
    /// The type system of the parent [`Destination`].
    type TypeSystem: TypeSystem;
    /// The error type of the parent [`Destination`].
    type Error: From<ConnectorXError> + Send;

    /// Write a value of type T to the location (row, col). If T mismatch with the
    /// schema, `ConnectorXError::TypeCheckFailed` will return.
    fn write<T>(&mut self, value: T) -> Result<(), <Self as DestinationPartition<'a>>::Error>
    where
        T: TypeAssoc<Self::TypeSystem>,
        Self: Consume<T, Error = <Self as DestinationPartition<'a>>::Error>,
    {
        self.consume(value)
    }

    /// Number of columns this `PartitionDestination` writes.
    fn ncols(&self) -> usize;

    /// Final clean ups, called once after the last value of the partition is written.
    fn finalize(&mut self) -> Result<(), Self::Error>;

    /// Aquire n rows in final destination.
    ///
    /// Called before each batch of `n` rows is written. Destinations that write into a shared,
    /// pre-allocated buffer (see [`needs_count`](Destination::needs_count)) use this to claim the
    /// next `n` rows for this partition. Returns the starting row index of the claimed range; the
    /// dispatcher currently ignores it.
    fn aquire_row(&mut self, n: usize) -> Result<usize, Self::Error>;
}

/// A type implemented `Consume<T>` means that it can consume a value `T` by adding it to it's own buffer.
pub trait Consume<T> {
    /// The error returned when `T` cannot be written.
    type Error: From<ConnectorXError> + Send;
    /// Write `value` to the next position of the buffer.
    fn consume(&mut self, value: T) -> Result<(), Self::Error>;
}
