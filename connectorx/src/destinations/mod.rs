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
pub trait Destination: Sized {
    const DATA_ORDERS: &'static [DataOrder];
    type TypeSystem: TypeSystem;
    type Partition<'a>: DestinationPartition<'a, TypeSystem = Self::TypeSystem, Error = Self::Error>
    where
        Self: 'a;
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

    /// Create a bunch of partition destinations, with each write `count` number of rows.
    fn partition(&mut self, counts: usize) -> Result<Vec<Self::Partition<'_>>, Self::Error>;
    /// Return the schema of the destination.
    fn schema(&self) -> &[Self::TypeSystem];
}

/// `PartitionDestination` writes values to its own region. `PartitionDestination` is parameterized
/// on lifetime `'a`, which is the lifetime of the parent `Destination`. Usually,
/// a `PartitionDestination` can never live longer than the parent.
pub trait DestinationPartition<'a>: Send {
    type TypeSystem: TypeSystem;
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

    /// Number of rows this `PartitionDestination` controls.
    fn ncols(&self) -> usize;

    /// Final clean ups
    fn finalize(&mut self) -> Result<(), Self::Error>;

    /// Aquire n rows in final destination
    fn aquire_row(&mut self, n: usize) -> Result<usize, Self::Error>;
}

/// A type implemented `Consume<T>` means that it can consume a value `T` by adding it to it's own buffer.
pub trait Consume<T> {
    type Error: From<ConnectorXError> + Send;
    fn consume(&mut self, value: T) -> Result<(), Self::Error>;
}
