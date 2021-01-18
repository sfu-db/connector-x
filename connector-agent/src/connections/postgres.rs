use super::Connection;
use r2d2::PooledConnection;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};

impl Connection for PooledConnection<PostgresConnectionManager<NoTls>> {
    fn query(&self, query: &str) -> Vec<u8> {
        unimplemented!()
    }
}
