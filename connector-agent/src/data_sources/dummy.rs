use super::{DataSource, Parse};
use crate::errors::Result;
use crate::types::DataType;
use anyhow::anyhow;
use fehler::throw;
use num_traits::cast::FromPrimitive;

/// This `DataSource` only produces T which can be derived from u64.
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

    fn run_query(&mut self, _: &str) -> Result<()> {
        Ok(())
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

impl Parse<String> for U64CounterSource {
    fn parse(&mut self) -> Result<String> {
        let ret = self.counter.to_string();
        self.counter += 1;
        Ok(ret)
    }
}

impl Parse<bool> for U64CounterSource {
    fn parse(&mut self) -> Result<bool> {
        let ret = self.counter % 2 == 0;
        self.counter += 1;
        Ok(ret)
    }
}

pub struct StringSource {
    rand_string: String,
}

impl StringSource {
    pub fn new() -> Self {
        Self {
            rand_string: "0".to_string(),
        }
    }
}

impl DataSource for StringSource {
    type TypeSystem = DataType;

    fn run_query(&mut self, _: &str) -> Result<()> {
        Ok(())
    }
}

impl Parse<String> for StringSource {
    fn parse(&mut self) -> Result<String> {
        let ret = self.rand_string.clone();
        let new_val = ret.clone().parse::<u64>().unwrap() + 1;
        self.rand_string = new_val.to_string();

        Ok(ret)
    }
}

impl Parse<u64> for StringSource {
    fn parse(&mut self) -> Result<u64> {
        let ret = self.rand_string.clone().parse::<u64>().unwrap();
        let new_string = ret.clone() + 1;
        self.rand_string = new_string.to_string();

        Ok(FromPrimitive::from_u64(ret).unwrap_or_default())
    }
}

impl Parse<f64> for StringSource {
    fn parse(&mut self) -> Result<f64> {
        let ret = self.rand_string.clone().parse::<u64>().unwrap();
        let new_string = ret.clone() + 1;
        self.rand_string = new_string.to_string();

        Ok(FromPrimitive::from_u64(ret).unwrap_or_default())
    }
}

impl Parse<bool> for StringSource {
    fn parse(&mut self) -> Result<bool> {
        throw!(anyhow!("StringSource only support string!"))
    }
}

/// This `DataSource` only produces T which can be derived from bool.
pub struct BoolCounterSource {
    counter: bool,
}

impl BoolCounterSource {
    pub fn new() -> Self {
        Self { counter: false }
    }
}

impl DataSource for BoolCounterSource {
    type TypeSystem = DataType;
    fn run_query(&mut self, _: &str) -> Result<()> {
        Ok(())
    }
}

impl Parse<u64> for BoolCounterSource {
    fn parse(&mut self) -> Result<u64> {
        let ret = 1;
        self.counter = !self.counter;
        Ok(ret)
    }
}
impl Parse<f64> for BoolCounterSource {
    fn parse(&mut self) -> Result<f64> {
        let ret = 1.0;
        self.counter = !self.counter;
        Ok(ret)
    }
}
impl Parse<bool> for BoolCounterSource {
    fn parse(&mut self) -> Result<bool> {
        let ret = self.counter;
        self.counter = !self.counter;
        Ok(ret)
    }
}

impl Parse<String> for BoolCounterSource {
    fn parse(&mut self) -> Result<String> {
        throw!(anyhow!("StringSource only support string!"))
    }
}
