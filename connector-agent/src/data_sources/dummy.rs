use super::DataSource;
use crate::errors::ConnectorAgentError;
use crate::errors::Result;
use crate::types::TypeInfo;

pub struct DummySource {
    counter: u64,
}

impl DummySource {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl DataSource for DummySource {
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

impl<T> Producer for T
where
    T: TypeInfo,
{
    default fn produce(_: u64) -> Result<Self> {
        Err(ConnectorAgentError::WrongTypeForField)
    }
}

impl Producer for u64 {
    fn produce(counter: u64) -> Result<Self> {
        Ok(counter)
    }
}
