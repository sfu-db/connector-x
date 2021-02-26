mod types;

use crate::data_order::DataOrder;
use crate::data_sources::{PartitionedSource, Produce, Source};
use crate::errors::{ConnectorAgentError, Result};
use crate::types::DataType;
use chrono::{Date, DateTime, NaiveDate, Utc};
use fehler::throw;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::any::type_name;
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

impl Source for PostgresDataSourceBuilder {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = PostgresDataSource;
    type TypeSystem = DataType;

    fn set_data_order(&mut self, data_order: DataOrder) -> Result<()> {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order));
        }
        Ok(())
    }

    fn build(&mut self) -> Self::Partition {
        PostgresDataSource::new(self.pool.get().unwrap())
    }
}

pub struct PostgresDataSource {
    conn: PgConn,
    counter: usize,
    nrows: usize,
    ncols: usize,
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

impl PartitionedSource for PostgresDataSource {
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
        let v = self.next_value();
        v.parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<u64>(), v.into()))
    }
}

impl Produce<Option<u64>> for PostgresDataSource {
    fn produce(&mut self) -> Result<Option<u64>> {
        match self.next_value() {
            "" => Ok(None),
            v => Ok(Some(v.parse().map_err(|_| {
                ConnectorAgentError::CannotParse(type_name::<u64>(), v.into())
            })?)),
        }
    }
}

impl Produce<i64> for PostgresDataSource {
    fn produce(&mut self) -> Result<i64> {
        let v = self.next_value();
        v.parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<i64>(), v.into()))
    }
}

impl Produce<Option<i64>> for PostgresDataSource {
    fn produce(&mut self) -> Result<Option<i64>> {
        match self.next_value() {
            "" => Ok(None),
            v => Ok(Some(v.parse().map_err(|_| {
                ConnectorAgentError::CannotParse(type_name::<i64>(), v.into())
            })?)),
        }
    }
}

impl Produce<f64> for PostgresDataSource {
    fn produce(&mut self) -> Result<f64> {
        let v = self.next_value();
        v.parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<f64>(), v.into()))
    }
}

impl Produce<Option<f64>> for PostgresDataSource {
    fn produce(&mut self) -> Result<Option<f64>> {
        match self.next_value() {
            "" => Ok(None),
            v => Ok(Some(v.parse().map_err(|_| {
                ConnectorAgentError::CannotParse(type_name::<f64>(), v.into())
            })?)),
        }
    }
}

impl Produce<bool> for PostgresDataSource {
    fn produce(&mut self) -> Result<bool> {
        let v = self.next_value();
        let v = match v {
            "t" => true,
            "f" => false,
            _ => throw!(ConnectorAgentError::CannotParse(
                type_name::<bool>(),
                v.into()
            )),
        };
        Ok(v)
    }
}

impl Produce<Option<bool>> for PostgresDataSource {
    fn produce(&mut self) -> Result<Option<bool>> {
        let v = self.next_value();
        let v = match v {
            "t" => Some(true),
            "f" => Some(false),
            "" => None,
            _ => throw!(ConnectorAgentError::CannotParse(
                type_name::<bool>(),
                v.into()
            )),
        };
        Ok(v)
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

impl Produce<DateTime<Utc>> for PostgresDataSource {
    fn produce(&mut self) -> Result<DateTime<Utc>> {
        let v = self.next_value();
        v.parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<DateTime<Utc>>(), v.into()))
    }
}

impl Produce<Option<DateTime<Utc>>> for PostgresDataSource {
    fn produce(&mut self) -> Result<Option<DateTime<Utc>>> {
        match self.next_value() {
            "" => Ok(None),
            v => Ok(Some(v.parse().map_err(|_| {
                ConnectorAgentError::CannotParse(type_name::<DateTime<Utc>>(), v.into())
            })?)),
        }
    }
}

impl Produce<Date<Utc>> for PostgresDataSource {
    fn produce(&mut self) -> Result<Date<Utc>> {
        let v = self.next_value();
        NaiveDate::parse_from_str(v, "%Y-%m-%d")
            .map(|nd| Date::<Utc>::from_utc(nd, Utc))
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<DateTime<Utc>>(), v.into()))
    }
}

impl Produce<Option<Date<Utc>>> for PostgresDataSource {
    fn produce(&mut self) -> Result<Option<Date<Utc>>> {
        match self.next_value() {
            "" => Ok(None),
            v => Ok(Some(
                NaiveDate::parse_from_str(v, "%Y-%m-%d")
                    .map(|nd| Date::<Utc>::from_utc(nd, Utc))
                    .map_err(|_| {
                        ConnectorAgentError::CannotParse(type_name::<DateTime<Utc>>(), v.into())
                    })?,
            )),
        }
    }
}
