use super::DataSource;
use crate::errors::Result;
use crate::types::TypeInfo;
use r2d2::PooledConnection;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};

impl DataSource for PooledConnection<PostgresConnectionManager<NoTls>> {
    fn run_query(&mut self, query: &str) -> Result<()> {
        unimplemented!()
    }

    fn produce<T: TypeInfo>(&mut self) -> Result<T> {
        unimplemented!()
    }
}
