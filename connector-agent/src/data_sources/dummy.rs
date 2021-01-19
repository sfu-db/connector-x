use super::{DataSource, Producer};
use crate::errors::Result;
use crate::types::{DataType, TypeSystem};
use num_traits::cast::FromPrimitive;

pub struct U64CounterSource {
    counter: u64,
}

impl U64CounterSource {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl DataSource for U64CounterSource {
    fn run_query(&mut self, query: &str) -> Result<()> {
        Ok(())
    }
}

impl<T> Producer<T> for U64CounterSource
where
    T: FromPrimitive + Default,
    DataType: TypeSystem<T>,
{
    type TypeSystem = DataType;

    fn produce(&mut self) -> Result<T> {
        let ret = self.counter;
        self.counter += 1;
        Ok(FromPrimitive::from_u64(ret).unwrap_or_default())
    }
}
