use super::DataSource;
use crate::errors::Result;
use crate::types::TypeInfo;
use r2d2::PooledConnection;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::thread;

impl DataSource for PooledConnection<PostgresConnectionManager<NoTls>> {
    fn run_query(&mut self, query: &str) -> Result<()> {
        let mut buf = vec![];
        let query = format!("COPY ({}) TO STDOUT WITH CSV", query);

        let manager = PostgresConnectionManager::new(
            "host=localhost user=postgres dbname=tpch port=6666 password=postgres".parse().unwrap(),
            NoTls,
        );

        let pool = r2d2::Pool::new(manager).unwrap();

        for i in 0..10i32 {
            let pool = pool.clone();
            thread::spawn(move || {
                let mut client = pool.get().unwrap();
                client.copy_out(&*query)?.read_to_end(&mut buf)?;
            });
        }
    }

    fn produce<T: TypeInfo>(&mut self) -> Result<T> {
        unimplemented!()
    }
}
