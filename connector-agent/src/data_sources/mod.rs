// When implementing a data source, be make sure to implement Queryable and
// Producer for all supported types in crate::types::DataType.

pub mod csv;
pub mod dummy;
pub mod mixed;
pub mod postgres;

use crate::data_order::DataOrder;
use crate::errors::Result;
use crate::typesystem::{TypeAssoc, TypeSystem};

pub trait SourceBuilder {
    /// Supported data orders, ordering by preference.
    const DATA_ORDERS: &'static [DataOrder];
    type DataSource: DataSource;

    fn set_data_order(&mut self, data_order: DataOrder) -> Result<()>;
    fn build(&mut self) -> Self::DataSource;
}

/// In general, a `DataSource` abstracts the data source as a stream, which can produce
/// a sequence of values of variate types by repetitively calling the function `produce`.
pub trait DataSource: Sized {
    /// The type system this `DataSource` associated with.
    type TypeSystem: TypeSystem;

    /// Run the query and put the result into Self.
    fn run_query(&mut self, query: &str) -> Result<()>;

    /// Produce a value `T` by calling `Parse<T>::parse`. Usually this function does not need to be
    /// implemented.
    fn produce<T>(&mut self) -> Result<T>
    where
        T: TypeAssoc<Self::TypeSystem>,
        Self: Parse<T>,
    {
        self.parse()
    }

    /// Number of rows this `DataSource` get.
    fn nrows(&self) -> usize;
}

/// A type implemented `Parse<T>` means that it can produce a value `T` by consuming part of it's raw data buffer.
pub trait Parse<T> {
    fn parse(&mut self) -> Result<T>;
}
