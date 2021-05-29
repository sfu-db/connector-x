// When implementing a data source, be make sure to implement Queryable and
// Producer for all supported types in crate::types::DataType.

pub mod csv;
pub mod dummy;
pub mod postgres;
pub mod mysql;

use crate::data_order::DataOrder;
use crate::errors::Result;
use crate::typesystem::{TypeAssoc, TypeSystem};

pub trait Source {
    /// Supported data orders, ordering by preference.
    const DATA_ORDERS: &'static [DataOrder];
    /// The type system this `Source` associated with.
    type TypeSystem: TypeSystem;
    // Partition needs to be send to different threads for parallel execution
    type Partition: SourcePartition<TypeSystem = Self::TypeSystem> + Send;

    fn set_data_order(&mut self, data_order: DataOrder) -> Result<()>;

    fn set_queries<Q: AsRef<str>>(&mut self, queries: &[Q]);

    fn fetch_metadata(&mut self) -> Result<()>;

    fn names(&self) -> Vec<String>;

    fn schema(&self) -> Vec<Self::TypeSystem>;

    fn partition(self) -> Result<Vec<Self::Partition>>;
}

/// In general, a `DataSource` abstracts the data source as a stream, which can produce
/// a sequence of values of variate types by repetitively calling the function `produce`.
pub trait SourcePartition: Sized {
    type TypeSystem: TypeSystem;
    type Parser<'a>: PartitionParser<'a, TypeSystem = Self::TypeSystem>;
    /// Run the query and put the result into Self.
    fn prepare(&mut self) -> Result<()>;

    fn parser(&mut self) -> Result<Self::Parser<'_>>;

    /// Number of rows this `DataSource` got.
    /// Sometimes it is not possible for the source to know how many rows it gets before reading the whole data.
    fn nrows(&self) -> usize;

    /// Number of cols this `DataSource` got.
    fn ncols(&self) -> usize;
}

pub trait PartitionParser<'a> {
    type TypeSystem: TypeSystem;
    /// Read a value `T` by calling `Produce<T>::produce`. Usually this function does not need to be
    /// implemented.
    fn parse<'r, T>(&'r mut self) -> Result<T>
    where
        T: TypeAssoc<Self::TypeSystem>,
        Self: Produce<'r, T>,
    {
        self.produce()
    }
}

/// A type implemented `Produce<T>` means that it can produce a value `T` by consuming part of it's raw data buffer.
pub trait Produce<'r, T> {
    fn produce(&'r mut self) -> Result<T>;
}
