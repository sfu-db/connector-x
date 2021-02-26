use super::{DataSource, Produce, SourceBuilder};
use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::types::DataType;
use chrono::{DateTime, Utc};
use fehler::{throw, throws};
use std::any::type_name;
use std::fs::File;

pub struct CSVSourceBuilder {}

impl CSVSourceBuilder {
    pub fn new() -> Self {
        CSVSourceBuilder {}
    }
}

impl SourceBuilder for CSVSourceBuilder {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type DataSource = CSVSource;

    #[throws(ConnectorAgentError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }
    }

    fn build(&mut self) -> Self::DataSource {
        CSVSource::new()
    }
}

pub struct CSVSource {
    records: Vec<csv::StringRecord>,
    counter: usize,
    pub nrows: usize,
    pub ncols: usize,
}

impl CSVSource {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            counter: 0,
            nrows: 0,
            ncols: 0,
        }
    }
}

impl DataSource for CSVSource {
    type TypeSystem = DataType;

    /// The parameter `query` is the path of the csv file
    fn prepare(&mut self, query: &str) -> Result<()> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(File::open(query).expect("open file"));

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

    fn infer_schema(&mut self) -> Result<Vec<DataType>> {
        unimplemented!("infer schema using self.records!");
    }
}

impl Produce<u64> for CSVSource {
    fn produce(&mut self) -> Result<u64> {
        let v: &str = self.records[self.counter / self.ncols][self.counter % self.ncols].as_ref();
        self.counter += 1;
        Ok(v.parse().unwrap_or_default())
    }
}

impl Produce<Option<u64>> for CSVSource {
    fn produce(&mut self) -> Result<Option<u64>> {
        let v: &str = self.records[self.counter / self.ncols][self.counter % self.ncols].as_ref();
        self.counter += 1;
        if v.is_empty() {
            return Ok(None);
        }
        Ok(Some(v.parse().unwrap_or_default()))
    }
}

impl Produce<f64> for CSVSource {
    fn produce(&mut self) -> Result<f64> {
        let v: &str = self.records[self.counter / self.ncols][self.counter % self.ncols].as_ref();
        self.counter += 1;
        Ok(v.parse().unwrap_or_default())
    }
}

impl Produce<Option<f64>> for CSVSource {
    fn produce(&mut self) -> Result<Option<f64>> {
        let v: &str = self.records[self.counter / self.ncols][self.counter % self.ncols].as_ref();
        self.counter += 1;
        Ok(v.parse().ok())
    }
}

impl Produce<bool> for CSVSource {
    fn produce(&mut self) -> Result<bool> {
        let v: &str = self.records[self.counter / self.ncols][self.counter % self.ncols].as_ref();
        self.counter += 1;
        Ok(v.parse().unwrap_or_default())
    }
}

impl Produce<Option<bool>> for CSVSource {
    fn produce(&mut self) -> Result<Option<bool>> {
        let v: &str = self.records[self.counter / self.ncols][self.counter % self.ncols].as_ref();
        self.counter += 1;
        Ok(v.parse().ok())
    }
}

impl Produce<String> for CSVSource {
    fn produce(&mut self) -> Result<String> {
        let v: &str = self.records[self.counter / self.ncols][self.counter % self.ncols].as_ref();
        self.counter += 1;
        Ok(String::from(v))
    }
}

impl Produce<Option<String>> for CSVSource {
    fn produce(&mut self) -> Result<Option<String>> {
        let v: &str = &self.records[self.counter / self.ncols][self.counter % self.ncols];
        self.counter += 1;
        Ok(Some(String::from(v)))
    }
}

impl Produce<DateTime<Utc>> for CSVSource {
    fn produce(&mut self) -> Result<DateTime<Utc>> {
        let v = &self.records[self.counter / self.ncols][self.counter % self.ncols];
        v.parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<DateTime<Utc>>(), v.into()))
    }
}

impl Produce<Option<DateTime<Utc>>> for CSVSource {
    fn produce(&mut self) -> Result<Option<DateTime<Utc>>> {
        match &self.records[self.counter / self.ncols][self.counter % self.ncols] {
            "" => Ok(None),
            v => Ok(Some(v.parse().map_err(|_| {
                ConnectorAgentError::CannotParse(type_name::<DateTime<Utc>>(), v.into())
            })?)),
        }
    }
}
