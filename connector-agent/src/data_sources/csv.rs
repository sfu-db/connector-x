use super::{Parser, PartitionedSource, Produce, Source};
use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::types::DataType;
use chrono::{DateTime, Utc};
use fehler::{throw, throws};
use regex::{Regex, RegexBuilder};
use std::any::type_name;
use std::collections::HashSet;
use std::fs::File;

pub struct CSVSource {
    schema: Vec<DataType>,
    files: Vec<String>,
    names: Vec<String>,
}

impl CSVSource {
    pub fn new(schema: Option<&[DataType]>) -> Self {
        match schema {
            None => CSVSource {
                schema: vec![],
                files: vec![],
                names: vec![],
            },
            Some(schema) => CSVSource {
                schema: schema.to_vec(),
                files: vec![],
                names: vec![],
            },
        }
    }

    pub fn infer_schema(&mut self) -> Result<Vec<DataType>> {
        // regular expressions for infer DataType from string
        let decimal_re: Regex = Regex::new(r"^-?(\d+\.\d+)$").unwrap();
        let integer_re: Regex = Regex::new(r"^-?(\d+)$").unwrap();
        let boolean_re: Regex = RegexBuilder::new(r"^(true)$|^(false)$")
            .case_insensitive(true)
            .build()
            .unwrap();
        let datetime_re: Regex = Regex::new(r"^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d$").unwrap();

        // read max_records rows to infer possible DataTypes for each field
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(File::open(&self.files[0]).expect("open file"));

        let max_records_to_read = 50;
        let num_cols = self.names.len();

        let mut column_types: Vec<HashSet<DataType>> = vec![HashSet::new(); num_cols];
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
                        let dt: DataType;

                        if string.starts_with('"') {
                            dt = DataType::String(false);
                        } else if boolean_re.is_match(string) {
                            dt = DataType::Bool(false);
                        } else if decimal_re.is_match(string) {
                            dt = DataType::F64(false);
                        } else if integer_re.is_match(string) {
                            dt = DataType::I64(false);
                        } else if datetime_re.is_match(string) {
                            dt = DataType::DateTime(false);
                        } else {
                            dt = DataType::String(false);
                        }
                        column_types[field_counter].insert(dt);
                    }
                }
            }
        }

        // determine DataType based on possible candidates
        let mut schema = vec![];

        for field_counter in 0..num_cols {
            let possibilities = &column_types[field_counter];
            let has_nulls = nulls[field_counter];

            match possibilities.len() {
                1 => {
                    for dt in possibilities.iter() {
                        match dt.clone() {
                            DataType::I64(false) => {
                                schema.push(DataType::I64(has_nulls));
                            }
                            DataType::F64(false) => {
                                schema.push(DataType::F64(has_nulls));
                            }
                            DataType::Bool(false) => {
                                schema.push(DataType::Bool(has_nulls));
                            }
                            DataType::String(false) => {
                                schema.push(DataType::String(has_nulls));
                            }
                            DataType::DateTime(false) => {
                                schema.push(DataType::DateTime(has_nulls));
                            }
                            _ => {}
                        }
                    }
                }
                2 => {
                    if possibilities.contains(&DataType::I64(false))
                        && possibilities.contains(&DataType::F64(false))
                    {
                        // Integer && Float -> Float
                        schema.push(DataType::F64(has_nulls));
                    } else {
                        // Conflicting DataTypes -> String
                        schema.push(DataType::String(has_nulls));
                    }
                }
                _ => {
                    // Conflicting DataTypes -> String
                    schema.push(DataType::String(has_nulls));
                }
            }
        }
        Ok(schema)
    }
}

impl Source for CSVSource {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = CSVSourcePartition;
    type TypeSystem = DataType;

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

impl PartitionedSource for CSVSourcePartition {
    type TypeSystem = DataType;
    type Parser<'a> = CSVSourceParser<'a>;

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
        Ok(CSVSourceParser {
            records: &mut self.records,
            counter: &mut self.counter,
            ncols: self.ncols,
        })
    }
}

pub struct CSVSourceParser<'a> {
    records: &'a mut [csv::StringRecord],
    counter: &'a mut usize,
    ncols: usize,
}

impl<'a> CSVSourceParser<'a> {
    fn next_val(&mut self) -> &str {
        let v: &str = self.records[*self.counter / self.ncols][*self.counter % self.ncols].as_ref();
        *self.counter += 1;

        v
    }
}

impl<'a> Parser<'a> for CSVSourceParser<'a> {
    type TypeSystem = DataType;
}

impl<'a> Produce<i64> for CSVSourceParser<'a> {
    fn produce(&mut self) -> Result<i64> {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<i64>(), v.into()))
    }
}

impl<'a> Produce<Option<i64>> for CSVSourceParser<'a> {
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

impl<'a> Produce<f64> for CSVSourceParser<'a> {
    fn produce(&mut self) -> Result<f64> {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<f64>(), v.into()))
    }
}

impl<'a> Produce<Option<f64>> for CSVSourceParser<'a> {
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

impl<'a> Produce<bool> for CSVSourceParser<'a> {
    fn produce(&mut self) -> Result<bool> {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<bool>(), v.into()))
    }
}

impl<'a> Produce<Option<bool>> for CSVSourceParser<'a> {
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

impl<'a> Produce<String> for CSVSourceParser<'a> {
    fn produce(&mut self) -> Result<String> {
        let v = self.next_val();
        Ok(String::from(v))
    }
}

impl<'a> Produce<Option<String>> for CSVSourceParser<'a> {
    fn produce(&mut self) -> Result<Option<String>> {
        let v = self.next_val();
        Ok(Some(String::from(v)))
    }
}

impl<'a> Produce<DateTime<Utc>> for CSVSourceParser<'a> {
    fn produce(&mut self) -> Result<DateTime<Utc>> {
        let v = self.next_val();
        v.parse()
            .map_err(|_| ConnectorAgentError::CannotParse(type_name::<DateTime<Utc>>(), v.into()))
    }
}

impl<'a> Produce<Option<DateTime<Utc>>> for CSVSourceParser<'a> {
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
