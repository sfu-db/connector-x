// When implementing a data source, be make sure to implement Queryable and
// Producer for all supported types in crate::types::DataType.

pub mod dummy;
pub mod postgres;

use crate::errors::Result;
use crate::types::TypeInfo;

pub trait Queryable {
    fn run_query(&mut self, query: &str) -> Result<()>;
}

pub trait Producer<T: TypeInfo> {
    fn produce(&mut self) -> Result<T>;
}

pub trait DataSource: Queryable + Producer<f64> + Producer<u64> {}

impl<T> DataSource for T where T: Queryable + Producer<f64> + Producer<u64> {}
