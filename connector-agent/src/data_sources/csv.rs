use csv::StringRecord;

use super::{Producer, Queryable};
use crate::errors::Result;
use crate::types::TypeInfo;
use std::fs::File;
use std::str::FromStr;

pub struct CSVSource {
    reader: csv::Reader<File>,
    counter: usize,
    records: Vec<csv::StringRecord>,
    nrows: usize,
    ncols: usize,
}

impl CSVSource {
    pub fn new(fname: &str) -> Self {
        Self {
            reader: csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(File::open(fname).unwrap()),
            counter: 0,
            records: Vec::new(),
            nrows: 0,
            ncols: 0,
        }
    }
}

impl Queryable for CSVSource {
    fn run_query(&mut self, query: &str) -> Result<()> {
        // TODO: filter by query
        self.records = self.reader.records().map(|v| v.expect("csv record")).collect();
        self.nrows = self.records.len();
        self.ncols = self.records[0].len();
        Ok(())
    }
}

impl<T> Producer<T> for CSVSource
where
    T: TypeInfo + FromStr + Default
{
    fn produce(&mut self) -> Result<T> {
        let v: &str = self.records[self.counter / self.ncols][self.counter % self.ncols].as_ref();
        self.counter += 1;
        Ok(v.parse().unwrap_or_default())
    }
}