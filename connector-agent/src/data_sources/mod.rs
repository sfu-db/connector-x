// When implementing a data source, be make sure to implement Queryable and
// Producer for all supported types in crate::types::DataType.

pub mod dummy;
pub mod postgres;

use crate::errors::Result;
use crate::types::TypeSystem;

pub trait DataSource: Parse<u64> + Parse<f64> {
    type TypeSystem: TypeSystem<u64> + TypeSystem<f64>;

    fn run_query(&mut self, query: &str) -> Result<()>;

    fn produce<T>(&mut self) -> Result<T>
    where
        Self::TypeSystem: TypeSystem<T>,
        Self: Parse<T>,
    {
        self.parse()
    }
}

pub trait Parse<T> {
    fn parse(&mut self) -> Result<T>;
}
