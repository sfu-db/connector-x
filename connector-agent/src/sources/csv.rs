use super::{PartitionParser, Produce, Source, SourcePartition};
use crate::data_order::DataOrder;
use crate::dummy_typesystem::DummyTypeSystem;
use crate::errors::{ConnectorAgentError, Result};
use chrono::{DateTime, Utc};
use fehler::{throw, throws};
use std::any::type_name;
use std::fs::File;

pub struct CSVSource {
    schema: Vec<DummyTypeSystem>,
    files: Vec<String>,
    names: Vec<String>,
}

impl CSVSource {
    pub fn new(schema: &[DummyTypeSystem]) -> Self {
        CSVSource {
            schema: schema.to_vec(),
            files: vec![],
            names: vec![],
        }
    }
}

impl Source for CSVSource {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = CSVSourcePartition;
    type TypeSystem = DummyTypeSystem;

    #[throws(ConnectorAgentError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }
    }

    fn set_queries<Q: AsRef<str>>(&mut self, queries: &[Q]) {
        self.files = queries
            .into_iter()
            .map(|fname| fname.as_ref().to_string())
            .collect();
    }

    fn fetch_metadata(&mut self) -> Result<()> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(File::open(&self.files[0]).expect("open file"));
        let header = reader.headers()?;
        assert_eq!(header.len(), self.schema.len());
        self.names = header.iter().map(|s| s.to_string()).collect();

        Ok(())
    }

    fn names(&self) -> Vec<String> {
        self.names.clone()
    }

    fn schema(&self) -> Vec<Self::TypeSystem> {
        self.schema.clone()
    }

    fn partition(self) -> Result<Vec<Self::Partition>> {
        Ok(self
            .files
            .into_iter()
            .map(|f| CSVSourcePartition::new(&f))
            .collect())
    }
}

pub struct CSVSourcePartition {
    fname: String,
    records: Vec<csv::StringRecord>,
    counter: usize,
    nrows: usize,
    ncols: usize,
}

impl CSVSourcePartition {
    pub fn new(fname: &str) -> Self {
        Self {
            fname: fname.into(),
            records: Vec::new(),
            counter: 0,
            nrows: 0,
            ncols: 0,
        }
    }
}

impl SourcePartition for CSVSourcePartition {
    type TypeSystem = DummyTypeSystem;
    type Parser<'a> = CSVSourcePartitionParser<'a>;

    /// The parameter `query` is the path of the csv file
    fn prepare(&mut self) -> Result<()> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(File::open(&self.fname).expect("open file"));

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

    fn parser(&mut self) -> Result<Self::Parser<'_>> {
        Ok(CSVSourcePartitionParser {
            records: &mut self.records,
            counter: &mut self.counter,
            ncols: self.ncols,
        })
    }
}

pub struct CSVSourcePartitionParser<'a> {
    records: &'a mut [csv::StringRecord],
    counter: &'a mut usize,
    ncols: usize,
}

impl<'a> CSVSourcePartitionParser<'a> {
    fn next_val(&mut self) -> &str {
        let v: &str = self.records[*self.counter / self.ncols][*self.counter % self.ncols].as_ref();
        *self.counter += 1;

        v
    }
}

impl<'a> PartitionParser<'a> for CSVSourcePartitionParser<'a> {
    type TypeSystem = DummyTypeSystem;
}

impl<'a> Produce<i64> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<i64> {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<i64>(), v.into()))
    }
}

impl<'a> Produce<Option<i64>> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<Option<i64>> {
        let v = self.next_val();
        if v.is_empty() {
            return Ok(None);
        }
        let v = v
            .parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<Option<i64>>(), v.into()))?;

        Ok(Some(v))
    }
}

impl<'a> Produce<f64> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<f64> {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<f64>(), v.into()))
    }
}

impl<'a> Produce<Option<f64>> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<Option<f64>> {
        let v = self.next_val();
        if v.is_empty() {
            return Ok(None);
        }
        let v = v
            .parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<Option<f64>>(), v.into()))?;

        Ok(Some(v))
    }
}

impl<'a> Produce<bool> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<bool> {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<bool>(), v.into()))
    }
}

impl<'a> Produce<Option<bool>> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<Option<bool>> {
        let v = self.next_val();
        if v.is_empty() {
            return Ok(None);
        }
        let v = v
            .parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<Option<bool>>(), v.into()))?;

        Ok(Some(v))
    }
}

impl<'a> Produce<String> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<String> {
        let v = self.next_val();
        Ok(String::from(v))
    }
}

impl<'a> Produce<Option<String>> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<Option<String>> {
        let v = self.next_val();
        Ok(Some(String::from(v)))
    }
}

impl<'a> Produce<DateTime<Utc>> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<DateTime<Utc>> {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<DateTime<Utc>>(), v.into()))
    }
}

impl<'a> Produce<Option<DateTime<Utc>>> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<Option<DateTime<Utc>>> {
        let v = self.next_val();
        if v.is_empty() {
            return Ok(None);
        }
        let v = v.parse().map_err(|_| {
            ConnectorAgentError::CannotParse(type_name::<DateTime<Utc>>(), v.into())
        })?;
        Ok(Some(v))
    }
}
