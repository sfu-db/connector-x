//! This module defines four traits [`Source`], [`SourcePartition`], [`PartitionParser`], and [`Produce`]  to define a source.
//! This module also contains source implementations for various databases.

#[cfg(feature = "src_bigquery")]
pub mod bigquery;
#[cfg(feature = "src_csv")]
pub mod csv;
#[cfg(feature = "src_dummy")]
pub mod dummy;
#[cfg(feature = "src_mssql")]
pub mod mssql;
#[cfg(feature = "src_mysql")]
pub mod mysql;
#[cfg(feature = "src_oracle")]
pub mod oracle;
#[cfg(feature = "src_postgres")]
pub mod postgres;
#[cfg(feature = "src_sqlite")]
pub mod sqlite;

use crate::data_order::DataOrder;
use crate::errors::ConnectorXError;
use crate::sql::CXQuery;
use crate::typesystem::{TypeAssoc, TypeSystem};
use std::fmt::Debug;

pub trait Source {
    /// Supported data orders, ordering by preference.
    const DATA_ORDERS: &'static [DataOrder];
    /// The type system this `Source` associated with.
    type TypeSystem: TypeSystem;
    // Partition needs to be send to different threads for parallel execution
    type Partition: SourcePartition<TypeSystem = Self::TypeSystem, Error = Self::Error> + Send;
    type Error: From<ConnectorXError> + Send + Debug;

    fn set_data_order(&mut self, data_order: DataOrder) -> Result<(), Self::Error>;

    fn set_queries<Q: ToString>(&mut self, queries: &[CXQuery<Q>]);

    fn set_origin_query(&mut self, query: Option<String>);

    fn fetch_metadata(&mut self) -> Result<(), Self::Error>;
    /// Get total number of rows if available
    fn result_rows(&mut self) -> Result<Option<usize>, Self::Error>;

    fn names(&self) -> Vec<String>;

    fn schema(&self) -> Vec<Self::TypeSystem>;

    fn partition(self) -> Result<Vec<Self::Partition>, Self::Error>;
}

/// In general, a `DataSource` abstracts the data source as a stream, which can produce
/// a sequence of values of variate types by repetitively calling the function `produce`.
pub trait SourcePartition {
    type TypeSystem: TypeSystem;
    type Parser<'a>: PartitionParser<'a, TypeSystem = Self::TypeSystem, Error = Self::Error>
    where
        Self: 'a;
    type Error: From<ConnectorXError> + Send + Debug;

    /// Count total number of rows in each partition.
    fn result_rows(&mut self) -> Result<(), Self::Error>;

    fn parser(&mut self) -> Result<Self::Parser<'_>, Self::Error>;

    /// Number of rows this `DataSource` got.
    /// Sometimes it is not possible for the source to know how many rows it gets before reading the whole data.
    fn nrows(&self) -> usize;

    /// Number of cols this `DataSource` got.
    fn ncols(&self) -> usize;
}

pub trait PartitionParser<'a>: Send {
    type TypeSystem: TypeSystem;
    type Error: From<ConnectorXError> + Send + Debug;

    /// Read a value `T` by calling `Produce<T>::produce`. Usually this function does not need to be
    /// implemented.
    fn parse<'r, T>(&'r mut self) -> Result<T, <Self as PartitionParser<'a>>::Error>
    where
        T: TypeAssoc<Self::TypeSystem>,
        Self: Produce<'r, T, Error = <Self as PartitionParser<'a>>::Error>,
    {
        self.produce()
    }

    /// Fetch next batch of rows from database, return actuall number of rows fetched
    fn fetch_next(&mut self) -> Result<(usize, bool), Self::Error>;
}

/// A type implemented `Produce<T>` means that it can produce a value `T` by consuming part of it's raw data buffer.
pub trait Produce<'r, T> {
    type Error: From<ConnectorXError> + Send;

    fn produce(&'r mut self) -> Result<T, Self::Error>;
}
