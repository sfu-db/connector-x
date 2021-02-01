// When implementing a data source, be make sure to implement Queryable and
// Producer for all supported types in crate::types::DataType.

pub mod csv;
pub mod dummy;
pub mod mixed;
pub mod postgres;

use crate::errors::Result;
use crate::typesystem::{TypeAssoc, TypeSystem};

pub trait SourceBuilder {
    type DataSource: DataSource;

    fn build(&mut self) -> Self::DataSource;
}

/// A DataSource should be able to `run_query` and store the query result in its own buffer.
/// A DataSource should also be able to produce any type T, which is defined by its associated TypeSystem,
/// by calling the function `DataSource::produce`.
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
