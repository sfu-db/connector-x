pub mod dummy;
pub mod postgres;

use crate::errors::Result;
use crate::types::TypeInfo;

pub trait DataSource {
    fn query(&mut self, query: &str) -> Result<()>;
    fn produce<T>(&mut self) -> Result<T>;
}
