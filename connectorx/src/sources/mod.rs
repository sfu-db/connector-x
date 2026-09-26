//! This module defines four traits [`Source`], [`SourcePartition`], [`PartitionParser`], and [`Produce`]  to define a source.
//! This module also contains source implementations for various databases.

#[cfg(feature = "src_bigquery")]
pub mod bigquery;
#[cfg(feature = "src_clickhouse")]
pub mod clickhouse;
#[cfg(feature = "src_csv")]
pub mod csv;
#[cfg(feature = "src_dummy")]
pub mod dummy;
#[cfg(feature = "src_mssql_common")]
pub mod mssql;
#[cfg(feature = "src_mysql")]
pub mod mysql;
#[cfg(feature = "src_oracle")]
pub mod oracle;
#[cfg(feature = "src_postgres")]
pub mod postgres;
#[cfg(feature = "src_sqlite")]
pub mod sqlite;
#[cfg(feature = "src_trino")]
pub mod trino;

use crate::data_order::DataOrder;
use crate::errors::ConnectorXError;
use crate::sql::CXQuery;
use crate::typesystem::{TypeAssoc, TypeSystem};
use std::fmt::Debug;

/// A `Source` connects to a database, fetches the metadata of a query and splits the work into
/// [`SourcePartition`]s, one per partitioned query.
///
/// [`Dispatcher`](crate::dispatcher::Dispatcher) drives a `Source` in this order:
/// [`set_data_order`](Source::set_data_order), [`set_queries`](Source::set_queries),
/// [`set_origin_query`](Source::set_origin_query), [`fetch_metadata`](Source::fetch_metadata),
/// then [`schema`](Source::schema) and [`names`](Source::names), optionally
/// [`result_rows`](Source::result_rows), and finally [`partition`](Source::partition).
pub trait Source {
    /// Supported data orders, ordering by preference.
    const DATA_ORDERS: &'static [DataOrder];
    /// The type system this `Source` associated with.
    type TypeSystem: TypeSystem;
    /// The partition type produced by [`partition`](Source::partition). It is sent to a worker
    /// thread, so it must be `Send`.
    type Partition: SourcePartition<TypeSystem = Self::TypeSystem, Error = Self::Error> + Send;
    /// The error type returned by this source and its partitions.
    type Error: From<ConnectorXError> + Send + Debug;

    /// Set the data order negotiated between this source and the destination, one of
    /// [`DATA_ORDERS`](Source::DATA_ORDERS). Return an error if the order is not supported.
    fn set_data_order(&mut self, data_order: DataOrder) -> Result<(), Self::Error>;

    /// Set the partitioned queries. Each query becomes one [`SourcePartition`].
    fn set_queries<Q: ToString>(&mut self, queries: &[CXQuery<Q>]);

    /// Set the original, unpartitioned query, if there is one. Sources can use it to
    /// count the rows of the whole result in [`result_rows`](Source::result_rows).
    fn set_origin_query(&mut self, query: Option<String>);

    /// Set queries to run on each connection before the partitioned queries.
    /// The default implementation panics; sources that support it override it.
    fn set_pre_execution_queries(&mut self, _pre_execution_queries: Option<&[String]>) {
        unimplemented!("pre_execution_queries is not implemented in this source type");
    }

    /// Fetch the result schema (column names and types) of the queries, e.g. by preparing the
    /// statement or running a `LIMIT 0` query. Called before [`names`](Source::names) and
    /// [`schema`](Source::schema).
    fn fetch_metadata(&mut self) -> Result<(), Self::Error>;
    /// Get total number of rows if available.
    ///
    /// Return `Ok(None)` when the total cannot be derived; the dispatcher then counts each
    /// partition with [`SourcePartition::result_rows`] instead.
    fn result_rows(&mut self) -> Result<Option<usize>, Self::Error>;

    /// The column names of the result, available after [`fetch_metadata`](Source::fetch_metadata).
    fn names(&self) -> Vec<String>;

    /// The column types of the result, available after [`fetch_metadata`](Source::fetch_metadata).
    fn schema(&self) -> Vec<Self::TypeSystem>;

    /// Consume the source and create one partition per query given to
    /// [`set_queries`](Source::set_queries), in the same order.
    fn partition(self) -> Result<Vec<Self::Partition>, Self::Error>;
}

/// In general, a `DataSource` abstracts the data source as a stream, which can produce
/// a sequence of values of variate types by repetitively calling the function `produce`.
pub trait SourcePartition {
    /// The type system of the parent [`Source`].
    type TypeSystem: TypeSystem;
    /// The parser that reads the rows of this partition, borrowing from it.
    type Parser<'a>: PartitionParser<'a, TypeSystem = Self::TypeSystem, Error = Self::Error>
    where
        Self: 'a;
    /// The error type of the parent [`Source`].
    type Error: From<ConnectorXError> + Send + Debug;

    /// Count total number of rows in each partition.
    ///
    /// The count is stored in the partition and reported by [`nrows`](SourcePartition::nrows).
    /// It is only called when the destination needs a count and
    /// [`Source::result_rows`] returned `None`.
    fn result_rows(&mut self) -> Result<(), Self::Error>;

    /// Run the partition's query and return a parser over its result.
    fn parser(&mut self) -> Result<Self::Parser<'_>, Self::Error>;

    /// Number of rows this `DataSource` got.
    /// Sometimes it is not possible for the source to know how many rows it gets before reading the whole data.
    fn nrows(&self) -> usize;

    /// Number of cols this `DataSource` got.
    fn ncols(&self) -> usize;
}

/// A `PartitionParser` reads the values of one partition's result, one cell at a time.
///
/// The dispatcher calls [`fetch_next`](PartitionParser::fetch_next) to buffer a batch of rows,
/// then [`parse`](PartitionParser::parse) once per cell in the negotiated
/// [`DataOrder`]. Each cell's type comes from the source schema.
pub trait PartitionParser<'a>: Send {
    /// The type system of the parent [`Source`].
    type TypeSystem: TypeSystem;
    /// The error type of the parent [`Source`].
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

    /// Fetch next batch of rows from database, return (number of rows fetched to local, whether all rows are fechted from database).
    /// There might be rows that are not consumed yet when calling the next fetch_next.
    /// The function might be called even after the last batch is fetched.
    fn fetch_next(&mut self) -> Result<(usize, bool), Self::Error>;
}

/// A type implemented `Produce<T>` means that it can produce a value `T` by consuming part of it's raw data buffer.
pub trait Produce<'r, T> {
    /// The error returned when the next value cannot be produced as `T`.
    type Error: From<ConnectorXError> + Send;

    /// Produce the next value from the buffer as `T` and advance past it.
    fn produce(&'r mut self) -> Result<T, Self::Error>;
}
