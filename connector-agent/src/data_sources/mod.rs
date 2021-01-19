// When implementing a data source, be make sure to implement Queryable and
// Producer for all supported types in crate::types::DataType.

pub mod dummy;
pub mod postgres;

use crate::errors::Result;
use crate::types::TypeSystem;

pub trait Producer<T> {
    type TypeSystem: TypeSystem<T>;
    fn produce(&mut self) -> Result<T>;
}

pub trait DataSource: Producer<f64> + Producer<u64> {
    fn run_query(&mut self, query: &str) -> Result<()>;
}
