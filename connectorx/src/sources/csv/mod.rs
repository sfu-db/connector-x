//! Source implementation for CSV files.

mod errors;
mod typesystem;

pub use self::errors::CSVSourceError;
pub use self::typesystem::CSVTypeSystem;
use super::{PartitionParser, Produce, Source, SourcePartition};
use crate::{data_order::DataOrder, errors::ConnectorXError, sql::CXQuery};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use fehler::{throw, throws};
#[cfg(feature = "src_csv")]
use regex::{Regex, RegexBuilder};
use std::collections::HashSet;
use std::fs::File;

pub struct CSVSource {
    schema: Vec<CSVTypeSystem>,
    files: Vec<CXQuery<String>>,
    names: Vec<String>,
}

impl CSVSource {
    pub fn new(schema: &[CSVTypeSystem]) -> Self {
        CSVSource {
            schema: schema.to_vec(),
            files: vec![],
            names: vec![],
        }
    }

    #[throws(CSVSourceError)]
    pub fn infer_schema(&mut self) -> Vec<CSVTypeSystem> {
        // regular expressions for infer CSVTypeSystem from string
        let decimal_re: Regex = Regex::new(r"^-?(\d+\.\d+)$")?;
        let integer_re: Regex = Regex::new(r"^-?(\d+)$")?;
        let boolean_re: Regex = RegexBuilder::new(r"^(true)$|^(false)$")
            .case_insensitive(true)
            .build()?;
        let datetime_re: Regex = Regex::new(r"^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d$")?;

        // read max_records rows to infer possible CSVTypeSystems for each field
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(File::open(self.files[0].as_str())?);

        let max_records_to_read = 50;
        let num_cols = self.names.len();

        let mut column_types: Vec<HashSet<CSVTypeSystem>> = vec![HashSet::new(); num_cols];
        let mut nulls: Vec<bool> = vec![false; num_cols];

        let mut record = csv::StringRecord::new();

        for _record_counter in 0..max_records_to_read {
            if !reader.read_record(&mut record)? {
                break;
            }
            for field_counter in 0..num_cols {
                if let Some(string) = record.get(field_counter) {
                    if string.is_empty() {
                        nulls[field_counter] = true;
                    } else {
                        let dt: CSVTypeSystem;

                        if string.starts_with('"') {
                            dt = CSVTypeSystem::String(false);
                        } else if boolean_re.is_match(string) {
                            dt = CSVTypeSystem::Bool(false);
                        } else if decimal_re.is_match(string) {
                            dt = CSVTypeSystem::F64(false);
                        } else if integer_re.is_match(string) {
                            dt = CSVTypeSystem::I64(false);
                        } else if datetime_re.is_match(string) {
                            dt = CSVTypeSystem::DateTime(false);
                        } else {
                            dt = CSVTypeSystem::String(false);
                        }
                        column_types[field_counter].insert(dt);
                    }
                }
            }
        }

        // determine CSVTypeSystem based on possible candidates
        let mut schema = vec![];

        for field_counter in 0..num_cols {
            let possibilities = &column_types[field_counter];
            let has_nulls = nulls[field_counter];

            match possibilities.len() {
                1 => {
                    for dt in possibilities.iter() {
                        match *dt {
                            CSVTypeSystem::I64(false) => {
                                schema.push(CSVTypeSystem::I64(has_nulls));
                            }
                            CSVTypeSystem::F64(false) => {
                                schema.push(CSVTypeSystem::F64(has_nulls));
                            }
                            CSVTypeSystem::Bool(false) => {
                                schema.push(CSVTypeSystem::Bool(has_nulls));
                            }
                            CSVTypeSystem::String(false) => {
                                schema.push(CSVTypeSystem::String(has_nulls));
                            }
                            CSVTypeSystem::DateTime(false) => {
                                schema.push(CSVTypeSystem::DateTime(has_nulls));
                            }
                            _ => {}
                        }
                    }
                }
                2 => {
                    if possibilities.contains(&CSVTypeSystem::I64(false))
                        && possibilities.contains(&CSVTypeSystem::F64(false))
                    {
                        // Integer && Float -> Float
                        schema.push(CSVTypeSystem::F64(has_nulls));
                    } else {
                        // Conflicting CSVTypeSystems -> String
                        schema.push(CSVTypeSystem::String(has_nulls));
                    }
                }
                _ => {
                    // Conflicting CSVTypeSystems -> String
                    schema.push(CSVTypeSystem::String(has_nulls));
                }
            }
        }
        schema
    }
}

impl Source for CSVSource {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = CSVSourcePartition;
    type TypeSystem = CSVTypeSystem;
    type Error = CSVSourceError;

    #[throws(CSVSourceError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorXError::UnsupportedDataOrder(data_order))
        }
    }

    fn set_queries<Q: ToString>(&mut self, queries: &[CXQuery<Q>]) {
        self.files = queries.iter().map(|q| q.map(Q::to_string)).collect();
    }

    fn set_origin_query(&mut self, _query: Option<String>) {}

    #[throws(CSVSourceError)]
    fn fetch_metadata(&mut self) {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(File::open(self.files[0].as_str())?);
        let header = reader.headers()?;

        self.names = header.iter().map(|s| s.to_string()).collect();

        if self.schema.is_empty() {
            self.schema = self.infer_schema()?;
        }

        assert_eq!(header.len(), self.schema.len());
    }

    #[throws(CSVSourceError)]
    fn result_rows(&mut self) -> Option<usize> {
        None
    }

    fn names(&self) -> Vec<String> {
        self.names.clone()
    }

    fn schema(&self) -> Vec<Self::TypeSystem> {
        self.schema.clone()
    }

    #[throws(CSVSourceError)]
    fn partition(self) -> Vec<Self::Partition> {
        let mut partitions = vec![];
        for file in self.files {
            partitions.push(CSVSourcePartition::new(file)?);
        }
        partitions
    }
}

pub struct CSVSourcePartition {
    records: Vec<csv::StringRecord>,
    counter: usize,
    nrows: usize,
    ncols: usize,
}

impl CSVSourcePartition {
    #[throws(CSVSourceError)]
    pub fn new(fname: CXQuery<String>) -> Self {
        let reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(File::open(fname.as_str())?);
        let mut records = vec![];
        reader
            .into_records()
            .try_for_each(|v| -> Result<(), CSVSourceError> {
                records.push(v.map_err(|e| anyhow!(e))?);
                Ok(())
            })?;

        let nrows = records.len();
        let ncols = if nrows > 0 { records[0].len() } else { 0 };

        Self {
            records,
            counter: 0,
            nrows,
            ncols,
        }
    }
}

impl SourcePartition for CSVSourcePartition {
    type TypeSystem = CSVTypeSystem;
    type Parser<'a> = CSVSourcePartitionParser<'a>;
    type Error = CSVSourceError;

    /// The parameter `query` is the path of the csv file
    #[throws(CSVSourceError)]
    fn result_rows(&mut self) {}

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }

    #[throws(CSVSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        CSVSourcePartitionParser {
            records: &mut self.records,
            counter: &mut self.counter,
            ncols: self.ncols,
        }
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
    type TypeSystem = CSVTypeSystem;
    type Error = CSVSourceError;

    #[throws(CSVSourceError)]
    fn fetch_next(&mut self) -> (usize, bool) {
        (self.records.len(), true)
    }
}

impl<'r, 'a> Produce<'r, i64> for CSVSourcePartitionParser<'a> {
    type Error = CSVSourceError;

    #[throws(CSVSourceError)]
    fn produce(&mut self) -> i64 {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorXError::cannot_produce::<i64>(Some(v.into())))?
    }
}

impl<'r, 'a> Produce<'r, Option<i64>> for CSVSourcePartitionParser<'a> {
    type Error = CSVSourceError;

    #[throws(CSVSourceError)]
    fn produce(&mut self) -> Option<i64> {
        let v = self.next_val();
        if v.is_empty() {
            return None;
        }
        let v = v
            .parse()
            .map_err(|_| ConnectorXError::cannot_produce::<Option<i64>>(Some(v.into())))?;

        Some(v)
    }
}

impl<'r, 'a> Produce<'r, f64> for CSVSourcePartitionParser<'a> {
    type Error = CSVSourceError;

    #[throws(CSVSourceError)]
    fn produce(&mut self) -> f64 {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorXError::cannot_produce::<f64>(Some(v.into())))?
    }
}

impl<'r, 'a> Produce<'r, Option<f64>> for CSVSourcePartitionParser<'a> {
    type Error = CSVSourceError;

    #[throws(CSVSourceError)]
    fn produce(&mut self) -> Option<f64> {
        let v = self.next_val();
        if v.is_empty() {
            return None;
        }
        let v = v
            .parse()
            .map_err(|_| ConnectorXError::cannot_produce::<Option<f64>>(Some(v.into())))?;

        Some(v)
    }
}

impl<'r, 'a> Produce<'r, bool> for CSVSourcePartitionParser<'a> {
    type Error = CSVSourceError;

    #[throws(CSVSourceError)]
    fn produce(&mut self) -> bool {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorXError::cannot_produce::<bool>(Some(v.into())))?
    }
}

impl<'r, 'a> Produce<'r, Option<bool>> for CSVSourcePartitionParser<'a> {
    type Error = CSVSourceError;

    #[throws(CSVSourceError)]
    fn produce(&mut self) -> Option<bool> {
        let v = self.next_val();
        if v.is_empty() {
            return None;
        }
        let v = v
            .parse()
            .map_err(|_| ConnectorXError::cannot_produce::<Option<bool>>(Some(v.into())))?;

        Some(v)
    }
}

impl<'r, 'a> Produce<'r, String> for CSVSourcePartitionParser<'a> {
    type Error = CSVSourceError;

    #[throws(CSVSourceError)]
    fn produce(&mut self) -> String {
        let v = self.next_val();
        String::from(v)
    }
}

impl<'a, 'r> Produce<'r, Option<String>> for CSVSourcePartitionParser<'a> {
    type Error = CSVSourceError;

    #[throws(CSVSourceError)]
    fn produce(&'r mut self) -> Option<String> {
        let v = self.next_val();

        Some(String::from(v))
    }
}

impl<'r, 'a> Produce<'r, DateTime<Utc>> for CSVSourcePartitionParser<'a> {
    type Error = CSVSourceError;

    #[throws(CSVSourceError)]
    fn produce(&mut self) -> DateTime<Utc> {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorXError::cannot_produce::<DateTime<Utc>>(Some(v.into())))?
    }
}

impl<'r, 'a> Produce<'r, Option<DateTime<Utc>>> for CSVSourcePartitionParser<'a> {
    type Error = CSVSourceError;

    #[throws(CSVSourceError)]
    fn produce(&mut self) -> Option<DateTime<Utc>> {
        let v = self.next_val();
        if v.is_empty() {
            return None;
        }
        let v = v
            .parse()
            .map_err(|_| ConnectorXError::cannot_produce::<DateTime<Utc>>(Some(v.into())))?;
        Some(v)
    }
}
