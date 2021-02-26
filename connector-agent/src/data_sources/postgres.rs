use crate::data_order::DataOrder;
use crate::data_sources::{DataSource, Produce, SourceBuilder};
use crate::errors::{ConnectorAgentError, Result};
use crate::types::DataType;
use fehler::throw;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::io::Read;

type PgManager = PostgresConnectionManager<NoTls>;
type PgConn = PooledConnection<PgManager>;

pub struct PostgresDataSourceBuilder {
    pool: Pool<PgManager>,
}

impl PostgresDataSourceBuilder {
    pub fn new(conn: &str) -> Self {
        let manager = PostgresConnectionManager::new(conn.parse().unwrap(), NoTls);
        let pool = Pool::new(manager).unwrap();

        Self { pool }
    }
}

impl SourceBuilder for PostgresDataSourceBuilder {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type DataSource = PostgresDataSource;

    fn set_data_order(&mut self, data_order: DataOrder) -> Result<()> {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order));
        }
        Ok(())
    }

    fn build(&mut self) -> Self::DataSource {
        PostgresDataSource::new(self.pool.get().unwrap())
    }
}

pub struct PostgresDataSource {
    conn: PgConn,
    counter: usize,
    pub nrows: usize,
    pub ncols: usize,
    records: Vec<csv::StringRecord>,
}

impl PostgresDataSource {
    pub fn new(conn: PgConn) -> Self {
        Self {
            conn,
            counter: 0,
            nrows: 0,
            ncols: 0,
            records: Vec::new(),
        }
    }
}

impl DataSource for PostgresDataSource {
    type TypeSystem = DataType;

    fn prepare(&mut self, query: &str) -> Result<()> {
        let mut buf = vec![];
        let query = format!("COPY ({}) TO STDOUT WITH CSV", query);
        self.conn.copy_out(&*query)?.read_to_end(&mut buf)?;
        let mut buf = buf.as_slice();

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(&mut buf);

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

    fn ncols(&self) -> usize {
        self.ncols
    }
}

impl PostgresDataSource {
    fn next_value(&mut self) -> &str {
        let v = &self.records[self.counter / self.ncols][self.counter % self.ncols];
        self.counter += 1;
        v
    }
}

impl Produce<u64> for PostgresDataSource {
    fn produce(&mut self) -> Result<u64> {
        Ok(self.next_value().parse().unwrap_or_default())
    }
}

impl Produce<Option<u64>> for PostgresDataSource {
    fn produce(&mut self) -> Result<Option<u64>> {
        Ok(self.next_value().parse().ok())
    }
}

impl Produce<f64> for PostgresDataSource {
    fn produce(&mut self) -> Result<f64> {
        Ok(self.next_value().parse().unwrap_or_default())
    }
}

impl Produce<Option<f64>> for PostgresDataSource {
    fn produce(&mut self) -> Result<Option<f64>> {
        Ok(self.next_value().parse().ok())
    }
}

impl Produce<bool> for PostgresDataSource {
    fn produce(&mut self) -> Result<bool> {
        let v = self.next_value();
        let v = match v {
            "t" => true,
            "f" => false,
            _ => throw!(ConnectorAgentError::CannotParsePostgresBool(v.to_string())),
        };
        Ok(v)
    }
}

impl Produce<Option<bool>> for PostgresDataSource {
    fn produce(&mut self) -> Result<Option<bool>> {
        let v = self.next_value();
        let v = match v {
            "t" => true,
            "f" => false,
            _ => return Ok(None),
        };
        Ok(Some(v))
    }
}

impl Produce<String> for PostgresDataSource {
    fn produce(&mut self) -> Result<String> {
        Ok(String::from(self.next_value()))
    }
}

impl Produce<Option<String>> for PostgresDataSource {
    fn produce(&mut self) -> Result<Option<String>> {
        Ok(Some(String::from(self.next_value())))
    }
}
