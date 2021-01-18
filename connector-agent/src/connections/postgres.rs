use super::Connection;
use async_trait::async_trait;
use bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::tls::NoTls;

impl<'a> Connection for PooledConnection<'a, PostgresConnectionManager<NoTls>> {
    fn query(&self, query: &str) -> Vec<u8> {
        unimplemented!()
    }
}
