//! A dummy source that generates different values based on an internal counter.
//! This source is for test purpose.

mod typesystem;

pub use self::typesystem::DummyTypeSystem;
use super::{PartitionParser, Produce, Source, SourcePartition};
use crate::data_order::DataOrder;
use crate::errors::{ConnectorXError, Result};
use crate::sql::CXQuery;
use chrono::{offset, DateTime, Utc};
use fehler::{throw, throws};
use num_traits::cast::FromPrimitive;

pub struct DummySource {
    names: Vec<String>,
    schema: Vec<DummyTypeSystem>,
    queries: Vec<CXQuery<String>>,
}

impl DummySource {
    pub fn new<S: AsRef<str>>(names: &[S], schema: &[DummyTypeSystem]) -> Self {
        assert_eq!(names.len(), schema.len());
        DummySource {
            names: names.iter().map(|s| s.as_ref().to_string()).collect(),
            schema: schema.to_vec(),
            queries: vec![],
        }
    }
}

impl Source for DummySource {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type TypeSystem = DummyTypeSystem;
    type Partition = DummySourcePartition;
    type Error = ConnectorXError;

    #[throws(ConnectorXError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorXError::UnsupportedDataOrder(data_order))
        }
    }

    // query: nrows,ncols
    fn set_queries<Q: ToString>(&mut self, queries: &[CXQuery<Q>]) {
        self.queries = queries.iter().map(|q| q.map(Q::to_string)).collect();
    }

    fn set_origin_query(&mut self, _query: Option<String>) {}

    fn fetch_metadata(&mut self) -> Result<()> {
        Ok(())
    }

    fn result_rows(&mut self) -> Result<Option<usize>> {
        Ok(None)
    }

    fn names(&self) -> Vec<String> {
        self.names.clone()
    }

    fn schema(&self) -> Vec<Self::TypeSystem> {
        self.schema.clone()
    }

    fn partition(self) -> Result<Vec<Self::Partition>> {
        assert!(!self.queries.is_empty());
        let queries = self.queries;
        let schema = self.schema;

        Ok(queries
            .into_iter()
            .map(|q| DummySourcePartition::new(&schema, &q))
            .collect())
    }
}

pub struct DummySourcePartition {
    nrows: usize,
    ncols: usize,
    counter: usize,
}

impl DummySourcePartition {
    pub fn new(_schema: &[DummyTypeSystem], q: &CXQuery<String>) -> Self {
        let v: Vec<usize> = q.as_str().split(',').map(|s| s.parse().unwrap()).collect();

        DummySourcePartition {
            nrows: v[0],
            ncols: v[1],
            counter: 0,
        }
    }
}

impl SourcePartition for DummySourcePartition {
    type TypeSystem = DummyTypeSystem;
    type Parser<'a> = DummySourcePartitionParser<'a>;
    type Error = ConnectorXError;

    fn result_rows(&mut self) -> Result<()> {
        Ok(())
    }

    fn parser(&mut self) -> Result<Self::Parser<'_>> {
        Ok(DummySourcePartitionParser::new(
            &mut self.counter,
            self.nrows,
            self.ncols,
        ))
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

pub struct DummySourcePartitionParser<'a> {
    counter: &'a mut usize,
    #[allow(unused)]
    nrows: usize,
    ncols: usize,
}

impl<'a> DummySourcePartitionParser<'a> {
    fn new(counter: &'a mut usize, nrows: usize, ncols: usize) -> Self {
        DummySourcePartitionParser {
            counter,
            ncols,
            nrows,
        }
    }

    fn next_val(&mut self) -> usize {
        let ret = *self.counter / self.ncols;
        *self.counter += 1;
        ret
    }
}

impl<'a> PartitionParser<'a> for DummySourcePartitionParser<'a> {
    type TypeSystem = DummyTypeSystem;
    type Error = ConnectorXError;

    fn fetch_next(&mut self) -> Result<(usize, bool)> {
        Ok((self.nrows, true))
    }
}

macro_rules! numeric_impl {
    ($($t: ty),+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for DummySourcePartitionParser<'a> {
                type Error = ConnectorXError;

                fn produce(&mut self) -> Result<$t> {
                    let ret = self.next_val();
                    Ok(FromPrimitive::from_usize(ret).unwrap_or_default())
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for DummySourcePartitionParser<'a> {
                type Error = ConnectorXError;

                fn produce(&mut self) -> Result<Option<$t>> {
                    let ret = self.next_val();
                    Ok(Some(FromPrimitive::from_usize(ret).unwrap_or_default()))
                }
            }
        )+
    };
}

numeric_impl!(u64, i32, i64, f64);

impl<'r, 'a> Produce<'r, String> for DummySourcePartitionParser<'a> {
    type Error = ConnectorXError;

    fn produce(&mut self) -> Result<String> {
        let ret = self.next_val().to_string();
        Ok(ret)
    }
}

impl<'r, 'a> Produce<'r, Option<String>> for DummySourcePartitionParser<'a> {
    type Error = ConnectorXError;

    fn produce(&mut self) -> Result<Option<String>> {
        let ret = self.next_val().to_string();
        Ok(Some(ret))
    }
}

impl<'r, 'a> Produce<'r, bool> for DummySourcePartitionParser<'a> {
    type Error = ConnectorXError;

    fn produce(&mut self) -> Result<bool> {
        let ret = self.next_val() % 2 == 0;
        Ok(ret)
    }
}

impl<'r, 'a> Produce<'r, Option<bool>> for DummySourcePartitionParser<'a> {
    type Error = ConnectorXError;

    fn produce(&mut self) -> Result<Option<bool>> {
        let ret = match self.next_val() % 3 {
            0 => Some(true),
            1 => Some(false),
            2 => None,
            _ => unreachable!(),
        };

        Ok(ret)
    }
}

impl<'r, 'a> Produce<'r, DateTime<Utc>> for DummySourcePartitionParser<'a> {
    type Error = ConnectorXError;

    fn produce(&mut self) -> Result<DateTime<Utc>> {
        self.next_val();
        let ret = offset::Utc::now();

        Ok(ret)
    }
}

impl<'r, 'a> Produce<'r, Option<DateTime<Utc>>> for DummySourcePartitionParser<'a> {
    type Error = ConnectorXError;

    fn produce(&mut self) -> Result<Option<DateTime<Utc>>> {
        self.next_val();
        let ret = match self.next_val() % 2 {
            0 => Some(offset::Utc::now()),
            1 => None,
            _ => unreachable!(),
        };
        Ok(ret)
    }
}
