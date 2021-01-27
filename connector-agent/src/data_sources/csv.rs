use super::{DataSource, Parse, SourceBuilder};
use crate::errors::Result;
use crate::types::DataType;
use std::fs::File;
use std::str::FromStr;

pub struct CSVSourceBuilder {}

impl CSVSourceBuilder {
    pub fn new() -> Self {
        CSVSourceBuilder {}
    }
}

impl SourceBuilder for CSVSourceBuilder {
    type DataSource = CSVSource;

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
    fn run_query(&mut self, query: &str) -> Result<()> {
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
}

impl<T> Parse<T> for CSVSource
where
    T: FromStr + Default,
{
    fn parse(&mut self) -> Result<T> {
        let v: &str = self.records[self.counter / self.ncols][self.counter % self.ncols].as_ref();
        self.counter += 1;
        Ok(v.parse().unwrap_or_default())
    }
}
