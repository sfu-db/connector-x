use super::DataSource;
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
    fn query(&mut self, query: &str) -> Result<()> {
        Ok(())
    }

    fn produce<T>(&mut self) -> Result<T> {
        let ret = Producer::produce(self.counter);
        self.counter += 1;
        Ok(ret)
    }
}

trait Producer {
    fn produce(counter: u64) -> Self;
}

impl<T> Producer for T {
    default fn produce(counter: u64) -> Self {
        unimplemented!()
    }
}

impl Producer for u64 {
    fn produce(counter: u64) -> Self {
        counter
    }
}

impl Producer for f64 {
    fn produce(counter: u64) -> Self {
        counter as f64
    }
}
