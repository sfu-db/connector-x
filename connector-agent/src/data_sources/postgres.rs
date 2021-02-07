use crate::data_order::DataOrder;
use crate::data_sources::{DataSource, Produce, SourceBuilder};
use crate::errors::{ConnectorAgentError, Result};
use crate::types::DataType;
use fehler::throw;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::io::Read;
use std::str::FromStr;

type PgManager = PostgresConnectionManager<NoTls>;
type PgConn = PooledConnection<PgManager>;

pub struct PostgresDataSourceBuilder {
    pool: Pool<PgManager>,
    data_order: Option<DataOrder>,
}

impl PostgresDataSourceBuilder {
    pub fn new(conn: &str) -> Self {
        // "host=localhost user=postgres dbname=tpch port=6666 password=postgres"
        let manager = PostgresConnectionManager::new(conn.parse().unwrap(), NoTls);
        let pool = Pool::new(manager).unwrap();
        Self {
            pool,
            data_order: None,
        }
    }
}

impl SourceBuilder for PostgresDataSourceBuilder {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type DataSource = PostgresDataSource;

    fn set_data_order(&mut self, data_order: DataOrder) -> Result<()> {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order));
        }
        self.data_order = Some(data_order);
        Ok(())
    }
    fn build(&mut self) -> Self::DataSource {
        PostgresDataSource::new(self.pool.get().unwrap())
    }
}

pub struct PostgresDataSource {
    conn: PgConn,
    buf: Vec<u8>,
    counter: usize,
    pub nrows: usize,
    pub ncols: usize,
}

impl PostgresDataSource {
    pub fn new(conn: PgConn) -> Self {
        Self {
            conn,
            buf: vec![],
            counter: 0,
            nrows: 0,
            ncols: 0,
        }
    }
}

impl DataSource for PostgresDataSource {
    type TypeSystem = DataType;

    fn run_query(&mut self, query: &str) -> Result<()> {
        if self.buf.len() != 0 {
            unimplemented!()
        }
        let csv = "year,make,model,description
        1948,Porsche,356,Luxury sports car
        1967,Ford,Mustang fastback 1967,American car";
        let a = csv.as_bytes()
        let query = format!("COPY ({}) TO STDOUT WITH CSV", query);
        self.conn.copy_out(&*query)?.read_to_end(&mut self.buf)?;

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader( &self.buf);
        self.records = reader.records().map(|v| v.expect("csv record")).collect();
        self.nrows = self.records.len();
        if self.nrows > 0 {
            self.ncols = self.records[0].len();
        }
        Ok(())
    }
    fn nrows(&self) -> usize {
        self.nrows
    }
}

// impl Produce<u64> for PostgresDataSource {
//     fn produce(&mut self) -> Result<u64> {
//         unimplemented!()
//     }
// }
//
// impl Produce<f64> for PostgresDataSource {
//     fn produce(&mut self) -> Result<f64> {
//         unimplemented!()
//     }
// }
//
// impl Produce<String> for PostgresDataSource {
//     fn produce(&mut self) -> Result<String> {
//         unimplemented!()
//     }
// }
//
// impl Produce<bool> for PostgresDataSource {
//     fn produce(&mut self) -> Result<bool> {
//         unimplemented!()
//     }
// }

impl<T> Produce<T> for PostgresDataSource
where
    T: FromStr + Default,
{
    fn produce(&mut self) -> Result<T> {
        let v: &str = self.records[self.counter / self.ncols][self.counter % self.ncols].as_ref();
        self.counter += 1;
        Ok(v.parse().unwrap_or_default())
    }
}

