use super::DataSource;
use crate::types::TypeInfo;
use crate::{
    errors::{ConnectorAgentError, Result},
    DataType,
};

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

    fn produce<T: TypeInfo>(&mut self) -> Result<T> {
        let ret = Producer::produce(self.counter)?;
        self.counter += 1;
        Ok(ret)
    }
}

trait Producer: Sized {
    fn produce(counter: u64) -> Result<Self>;
}

impl Producer for u64 {
    fn produce(counter: u64) -> Result<Self> {
        Ok(counter)
    }
}

impl<T> Producer for T
where
    T: TypeInfo,
{
    default fn produce(_: u64) -> Result<Self> {
        Err(ConnectorAgentError::UnexpectedType(DataType::U64, T::name()))
    }
}
