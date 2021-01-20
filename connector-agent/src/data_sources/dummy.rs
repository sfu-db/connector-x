use super::{DataSource, Parse};
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
    type TypeSystem = DataType;

    fn run_query(&mut self, query: &str) -> Result<()> {
        Ok(())
    }

    fn produce<T>(&mut self) -> Result<T>
    where
        Self::TypeSystem: TypeSystem<T>,
        Self: Parse<T>,
    {
        self.parse()
    }
}

impl Parse<u64> for U64CounterSource {
    fn parse(&mut self) -> Result<u64> {
        let ret = self.counter;
        self.counter += 1;
        Ok(FromPrimitive::from_u64(ret).unwrap_or_default())
    }
}

impl Parse<f64> for U64CounterSource {
    fn parse(&mut self) -> Result<f64> {
        let ret = self.counter;
        self.counter += 1;
        Ok(FromPrimitive::from_u64(ret).unwrap_or_default())
    }
}
