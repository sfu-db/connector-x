// When implementing a data source, be make sure to implement Queryable and
// Producer for all supported types in crate::types::DataType.

pub mod csv;
pub mod dummy;
pub mod postgres;

use crate::data_order::DataOrder;
use crate::errors::Result;
use crate::typesystem::{TypeAssoc, TypeSystem};

pub trait Source {
    /// Supported data orders, ordering by preference.
    const DATA_ORDERS: &'static [DataOrder];
    /// The type system this `Source` associated with.
    type TypeSystem: TypeSystem;
    // Partition needs to be send to different threads for parallel execution
    type Partition: PartitionedSource<TypeSystem = Self::TypeSystem> + Send;

    fn set_data_order(&mut self, data_order: DataOrder) -> Result<()>;
    fn build(&mut self) -> Self::Partition;
}

/// In general, a `DataSource` abstracts the data source as a stream, which can produce
/// a sequence of values of variate types by repetitively calling the function `produce`.
pub trait PartitionedSource: Sized {
    type TypeSystem: TypeSystem;

    /// Run the query and put the result into Self.
    fn prepare(&mut self, query: &str) -> Result<()>;

    /// Read a value `T` by calling `Produce<T>::produce`. Usually this function does not need to be
    /// implemented.
    fn read<T>(&mut self) -> Result<T>
    where
        T: TypeAssoc<Self::TypeSystem>,
        Self: Produce<T>,
    {
        self.produce()
    }

    /// Number of rows this `DataSource` got.
    fn nrows(&self) -> usize;

    /// Number of cols this `DataSource` got.
    fn ncols(&self) -> usize;

    /// Infer schema from return data
    fn infer_schema(&mut self) -> Result<Vec<Self::TypeSystem>> {
        unimplemented!("infer schema using self.records!");
    }
}

/// A type implemented `Produce<T>` means that it can produce a value `T` by consuming part of it's raw data buffer.
pub trait Produce<T> {
    fn produce(&mut self) -> Result<T>;
}
