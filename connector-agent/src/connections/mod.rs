mod postgres;

pub trait Connection {
    fn query(&self, query: &str) -> Vec<u8>;
}

pub struct DummyConnection;

impl Connection for DummyConnection {
    fn query(&self, query: &str) -> Vec<u8> {
        vec![]
    }
}
