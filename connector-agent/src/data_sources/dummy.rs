use super::{Parser, PartitionedSource, Produce, Source};
use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::types::DataType;
use chrono::{offset, Date, DateTime, Utc};
use fehler::{throw, throws};
use num_traits::cast::FromPrimitive;

pub struct MixedSource {
    names: Vec<String>,
    schema: Vec<DataType>,
    queries: Vec<String>,
}

impl MixedSource {
    pub fn new<S: AsRef<str>>(names: &[S], schema: &[DataType]) -> Self {
        assert_eq!(names.len(), schema.len());
        MixedSource {
            names: names.into_iter().map(|s| s.as_ref().to_string()).collect(),
            schema: schema.to_vec(),
            queries: vec![],
        }
    }
}

impl Source for MixedSource {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type TypeSystem = DataType;
    type Partition = MixedSourcePartition;

    #[throws(ConnectorAgentError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }
    }

    // query: nrows,ncols
    fn set_queries<Q: AsRef<str>>(&mut self, queries: &[Q]) {
        self.queries = queries
            .into_iter()
            .map(|q| q.as_ref().to_string())
            .collect();
    }

    fn fetch_metadata(&mut self) -> Result<()> {
        Ok(())
    }

    fn names(&self) -> Vec<String> {
        self.names.clone()
    }

    fn schema(&self) -> Vec<Self::TypeSystem> {
        self.schema.clone()
    }

    fn partition(self) -> Result<Vec<Self::Partition>> {
        assert!(self.queries.len() != 0);
        let queries = self.queries;
        let schema = self.schema;

        Ok(queries
            .into_iter()
            .map(|q| MixedSourcePartition::new(&schema, &q))
            .collect())
    }
}

pub struct MixedSourcePartition {
    nrows: usize,
    ncols: usize,
    counter: usize,
}

impl MixedSourcePartition {
    pub fn new(_schema: &[DataType], q: &str) -> Self {
        let v: Vec<usize> = q.split(',').map(|s| s.parse().unwrap()).collect();

        MixedSourcePartition {
            nrows: v[0],
            ncols: v[1],
            counter: 0,
        }
    }
}

impl PartitionedSource for MixedSourcePartition {
    type TypeSystem = DataType;
    type Parser<'a> = MixedSourceParser<'a>;

    fn prepare(&mut self) -> Result<()> {
        Ok(())
    }

    fn parser(&mut self) -> Result<Self::Parser<'_>> {
        Ok(MixedSourceParser::new(
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

pub struct MixedSourceParser<'a> {
    counter: &'a mut usize,
    #[allow(unused)]
    nrows: usize,
    ncols: usize,
}

impl<'a> MixedSourceParser<'a> {
    fn new(counter: &'a mut usize, nrows: usize, ncols: usize) -> Self {
        MixedSourceParser {
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

impl<'a> Parser<'a> for MixedSourceParser<'a> {
    type TypeSystem = DataType;
}

macro_rules! numeric_impl {
    ($($t: ty),+) => {
        $(
            impl<'a> Produce<$t> for MixedSourceParser<'a> {
                fn produce(&mut self) -> Result<$t> {
                    let ret = self.next_val();
                    Ok(FromPrimitive::from_usize(ret).unwrap_or_default())
                }
            }

            impl<'a> Produce<Option<$t>> for MixedSourceParser<'a> {
                fn produce(&mut self) -> Result<Option<$t>> {
                    let ret = self.next_val();
                    Ok(Some(FromPrimitive::from_usize(ret).unwrap_or_default()))
                }
            }
        )+
    };
}

numeric_impl!(u64, i32, i64, f64);

impl<'a> Produce<String> for MixedSourceParser<'a> {
    fn produce(&mut self) -> Result<String> {
        let ret = self.next_val().to_string();
        Ok(ret)
    }
}

impl<'a> Produce<Option<String>> for MixedSourceParser<'a> {
    fn produce(&mut self) -> Result<Option<String>> {
        let ret = self.next_val().to_string();
        Ok(Some(ret))
    }
}

impl<'a> Produce<bool> for MixedSourceParser<'a> {
    fn produce(&mut self) -> Result<bool> {
        let ret = self.next_val() % 2 == 0;
        Ok(ret)
    }
}

impl<'a> Produce<Option<bool>> for MixedSourceParser<'a> {
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

impl<'a> Produce<DateTime<Utc>> for MixedSourceParser<'a> {
    fn produce(&mut self) -> Result<DateTime<Utc>> {
        self.next_val();
        let ret = offset::Utc::now();

        Ok(ret)
    }
}

impl<'a> Produce<Option<DateTime<Utc>>> for MixedSourceParser<'a> {
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

impl<'a> Produce<Date<Utc>> for MixedSourceParser<'a> {
    fn produce(&mut self) -> Result<Date<Utc>> {
        self.next_val();
        let ret = offset::Utc::now().date();
        Ok(ret)
    }
}

impl<'a> Produce<Option<Date<Utc>>> for MixedSourceParser<'a> {
    fn produce(&mut self) -> Result<Option<Date<Utc>>> {
        self.next_val();
        let ret = match self.next_val() % 2 {
            0 => Some(offset::Utc::now().date()),
            1 => None,
            _ => unreachable!(),
        };
        Ok(ret)
    }
}
