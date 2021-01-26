use super::{DataSource, Parse};
use crate::errors::Result;
use crate::types::DataType;
use std::fs::File;

pub struct CSVSource {
    filename: String,
    records: Vec<csv::StringRecord>,
    counter: usize,
    pub nrows: usize,
    pub ncols: usize,
}

impl CSVSource {
    pub fn new(fname: &str) -> Self {
        Self {
            filename: String::from(fname),
            records: Vec::new(),
            counter: 0,
            nrows: 0,
            ncols: 0,
        }
    }
}

impl DataSource for CSVSource {
    type TypeSystem = DataType;
    fn run_query(&mut self, _query: &str) -> Result<()> {
        let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(File::open(self.filename.as_str()).expect("open file"));

        self.records = reader.records().map(|v| v.expect("csv record")).collect();
        self.nrows = self.records.len();
        if self.nrows > 0 {
            self.ncols = self.records[0].len();
        }
        Ok(())
    }
}

impl Parse<u64> for CSVSource {
    fn parse(&mut self) -> Result<u64> {
        let v: &str = self.records[self.counter / self.ncols][self.counter % self.ncols].as_ref();
        self.counter += 1;
        Ok(v.parse().unwrap_or_default())
    }
}

impl Parse<f64> for CSVSource {
    fn parse(&mut self) -> Result<f64> {
        let v: &str = self.records[self.counter / self.ncols][self.counter % self.ncols].as_ref();
        self.counter += 1;
        Ok(v.parse().unwrap_or_default())
    }
}

impl Parse<Option<u64>> for CSVSource {
    fn parse(&mut self) -> Result<Option<u64>> {
        let v: &str = self.records[self.counter / self.ncols][self.counter % self.ncols].as_ref();
        self.counter += 1;
        if v.is_empty() {
            return Ok(None);
        }
        Ok(Some(v.parse().unwrap_or_default()))
    }
}

impl Parse<String> for CSVSource {
    fn parse(&mut self) -> Result<String> {
        let v: &str = self.records[self.counter / self.ncols][self.counter % self.ncols].as_ref();
        self.counter += 1;
        Ok(String::from(v))
    }
}