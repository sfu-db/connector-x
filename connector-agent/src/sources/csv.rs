use super::{PartitionParser, Produce, Source, SourcePartition};
use crate::data_order::DataOrder;
use crate::dummy_typesystem::DummyTypeSystem;
use crate::errors::{ConnectorAgentError, Result};
use chrono::{DateTime, Utc};
use fehler::{throw, throws};
use regex::{Regex, RegexBuilder};
use std::collections::HashSet;
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

    pub fn infer_schema(&mut self) -> Result<Vec<DummyTypeSystem>> {
        // regular expressions for infer DummyTypeSystem from string
        let decimal_re: Regex = Regex::new(r"^-?(\d+\.\d+)$").unwrap();
        let integer_re: Regex = Regex::new(r"^-?(\d+)$").unwrap();
        let boolean_re: Regex = RegexBuilder::new(r"^(true)$|^(false)$")
            .case_insensitive(true)
            .build()
            .unwrap();
        let datetime_re: Regex = Regex::new(r"^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d$").unwrap();

        // read max_records rows to infer possible DummyTypeSystems for each field
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(File::open(&self.files[0]).expect("open file"));

        let max_records_to_read = 50;
        let num_cols = self.names.len();

        let mut column_types: Vec<HashSet<DummyTypeSystem>> = vec![HashSet::new(); num_cols];
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
                        let dt: DummyTypeSystem;

                        if string.starts_with('"') {
                            dt = DummyTypeSystem::String(false);
                        } else if boolean_re.is_match(string) {
                            dt = DummyTypeSystem::Bool(false);
                        } else if decimal_re.is_match(string) {
                            dt = DummyTypeSystem::F64(false);
                        } else if integer_re.is_match(string) {
                            dt = DummyTypeSystem::I64(false);
                        } else if datetime_re.is_match(string) {
                            dt = DummyTypeSystem::DateTime(false);
                        } else {
                            dt = DummyTypeSystem::String(false);
                        }
                        column_types[field_counter].insert(dt);
                    }
                }
            }
        }

        // determine DummyTypeSystem based on possible candidates
        let mut schema = vec![];

        for field_counter in 0..num_cols {
            let possibilities = &column_types[field_counter];
            let has_nulls = nulls[field_counter];

            match possibilities.len() {
                1 => {
                    for dt in possibilities.iter() {
                        match dt.clone() {
                            DummyTypeSystem::I64(false) => {
                                schema.push(DummyTypeSystem::I64(has_nulls));
                            }
                            DummyTypeSystem::F64(false) => {
                                schema.push(DummyTypeSystem::F64(has_nulls));
                            }
                            DummyTypeSystem::Bool(false) => {
                                schema.push(DummyTypeSystem::Bool(has_nulls));
                            }
                            DummyTypeSystem::String(false) => {
                                schema.push(DummyTypeSystem::String(has_nulls));
                            }
                            DummyTypeSystem::DateTime(false) => {
                                schema.push(DummyTypeSystem::DateTime(has_nulls));
                            }
                            _ => {}
                        }
                    }
                }
                2 => {
                    if possibilities.contains(&DummyTypeSystem::I64(false))
                        && possibilities.contains(&DummyTypeSystem::F64(false))
                    {
                        // Integer && Float -> Float
                        schema.push(DummyTypeSystem::F64(has_nulls));
                    } else {
                        // Conflicting DummyTypeSystems -> String
                        schema.push(DummyTypeSystem::String(has_nulls));
                    }
                }
                _ => {
                    // Conflicting DummyTypeSystems -> String
                    schema.push(DummyTypeSystem::String(has_nulls));
                }
            }
        }
        Ok(schema)
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

        self.names = header.iter().map(|s| s.to_string()).collect();

        if self.schema.len() == 0 {
            self.schema = self.infer_schema().unwrap_or_default();
        }

        assert_eq!(header.len(), self.schema.len());

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

impl<'r, 'a> Produce<'r, i64> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<i64> {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorAgentError::cannot_produce::<i64>(Some(v.into())))
    }
}

impl<'r, 'a> Produce<'r, Option<i64>> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<Option<i64>> {
        let v = self.next_val();
        if v.is_empty() {
            return Ok(None);
        }
        let v = v
            .parse()
            .map_err(|_| ConnectorAgentError::cannot_produce::<Option<i64>>(Some(v.into())))?;

        Ok(Some(v))
    }
}

impl<'r, 'a> Produce<'r, f64> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<f64> {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorAgentError::cannot_produce::<f64>(Some(v.into())))
    }
}

impl<'r, 'a> Produce<'r, Option<f64>> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<Option<f64>> {
        let v = self.next_val();
        if v.is_empty() {
            return Ok(None);
        }
        let v = v
            .parse()
            .map_err(|_| ConnectorAgentError::cannot_produce::<Option<f64>>(Some(v.into())))?;

        Ok(Some(v))
    }
}

impl<'r, 'a> Produce<'r, bool> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<bool> {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorAgentError::cannot_produce::<bool>(Some(v.into())))
    }
}

impl<'r, 'a> Produce<'r, Option<bool>> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<Option<bool>> {
        let v = self.next_val();
        if v.is_empty() {
            return Ok(None);
        }
        let v = v
            .parse()
            .map_err(|_| ConnectorAgentError::cannot_produce::<Option<bool>>(Some(v.into())))?;

        Ok(Some(v))
    }
}

impl<'r, 'a> Produce<'r, String> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<String> {
        let v = self.next_val();
        Ok(String::from(v))
    }
}

impl<'a, 'r> Produce<'r, Option<String>> for CSVSourcePartitionParser<'a> {
    fn produce(&'r mut self) -> Result<Option<String>> {
        let v = self.next_val();
        Ok(Some(String::from(v)))
    }
}

impl<'r, 'a> Produce<'r, DateTime<Utc>> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<DateTime<Utc>> {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorAgentError::cannot_produce::<DateTime<Utc>>(Some(v.into())))
    }
}

impl<'r, 'a> Produce<'r, Option<DateTime<Utc>>> for CSVSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<Option<DateTime<Utc>>> {
        let v = self.next_val();
        if v.is_empty() {
            return Ok(None);
        }
        let v = v
            .parse()
            .map_err(|_| ConnectorAgentError::cannot_produce::<DateTime<Utc>>(Some(v.into())))?;
        Ok(Some(v))
    }
}
