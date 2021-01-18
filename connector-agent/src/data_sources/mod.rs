pub mod dummy;
pub mod postgres;

use crate::errors::Result;
use crate::types::TypeInfo;

pub trait DataSource {
    fn run_query(&mut self, query: &str) -> Result<()>;
    fn produce<T: TypeInfo>(&mut self) -> Result<T>;
}
