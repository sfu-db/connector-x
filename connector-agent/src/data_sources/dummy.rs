use super::{PartitionedSource, Produce, Source};
use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::types::DataType;
use chrono::{offset, Date, DateTime, Utc};
use fehler::{throw, throws};
use num_traits::cast::FromPrimitive;

pub struct MixedSourceBuilder {}

impl MixedSourceBuilder {
    pub fn new() -> Self {
        MixedSourceBuilder {}
    }
}

impl Source for MixedSourceBuilder {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = MixedSource;
    type TypeSystem = DataType;

    #[throws(ConnectorAgentError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }
    }

    fn build(&mut self) -> Self::Partition {
        MixedSource::new()
    }
}

pub struct MixedSource {
    nrows: usize,
    ncols: usize,
    counter: usize,
}

impl MixedSource {
    pub fn new() -> Self {
        MixedSource {
            nrows: 0,
            ncols: 0,
            counter: 0,
        }
    }
}

impl PartitionedSource for MixedSource {
    type TypeSystem = DataType;

    // query: nrows,ncols
    fn prepare(&mut self, query: &str) -> Result<()> {
        let v: Vec<usize> = query.split(',').map(|s| s.parse().unwrap()).collect();
        self.nrows = v[0];
        self.ncols = v[1];
        Ok(())
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

impl Produce<u64> for MixedSource {
    fn produce(&mut self) -> Result<u64> {
        let ret = self.counter / self.ncols;
        self.counter += 1;
        Ok(FromPrimitive::from_usize(ret).unwrap_or_default())
    }
}

impl Produce<Option<u64>> for MixedSource {
    fn produce(&mut self) -> Result<Option<u64>> {
        let ret = self.counter / self.ncols;
        self.counter += 1;
        Ok(Some(FromPrimitive::from_usize(ret).unwrap_or_default()))
    }
}

impl Produce<i64> for MixedSource {
    fn produce(&mut self) -> Result<i64> {
        let ret = self.counter / self.ncols;
        self.counter += 1;
        Ok(FromPrimitive::from_usize(ret).unwrap_or_default())
    }
}

impl Produce<Option<i64>> for MixedSource {
    fn produce(&mut self) -> Result<Option<i64>> {
        let ret = self.counter / self.ncols;
        self.counter += 1;
        Ok(Some(FromPrimitive::from_usize(ret).unwrap_or_default()))
    }
}

impl Produce<f64> for MixedSource {
    fn produce(&mut self) -> Result<f64> {
        let ret = self.counter / self.ncols;
        self.counter += 1;
        Ok(FromPrimitive::from_usize(ret).unwrap_or_default())
    }
}

impl Produce<Option<f64>> for MixedSource {
    fn produce(&mut self) -> Result<Option<f64>> {
        let ret = self.counter / self.ncols;
        self.counter += 1;
        Ok(FromPrimitive::from_usize(ret))
    }
}

impl Produce<String> for MixedSource {
    fn produce(&mut self) -> Result<String> {
        let ret = ((self.counter / self.ncols) as u64).to_string();
        self.counter += 1;
        Ok(ret)
    }
}

impl Produce<Option<String>> for MixedSource {
    fn produce(&mut self) -> Result<Option<String>> {
        let ret = ((self.counter / self.ncols) as u64).to_string();
        self.counter += 1;
        Ok(Some(ret))
    }
}

impl Produce<bool> for MixedSource {
    fn produce(&mut self) -> Result<bool> {
        let ret = (self.counter / self.ncols) % 2 == 0;
        self.counter += 1;
        Ok(ret)
    }
}

impl Produce<Option<bool>> for MixedSource {
    fn produce(&mut self) -> Result<Option<bool>> {
        let ret = match (self.counter / self.ncols) % 3 {
            0 => Some(true),
            1 => Some(false),
            2 => None,
            _ => unreachable!(),
        };

        self.counter += 1;
        Ok(ret)
    }
}

impl Produce<DateTime<Utc>> for MixedSource {
    fn produce(&mut self) -> Result<DateTime<Utc>> {
        let ret = offset::Utc::now();
        self.counter += 1;
        Ok(ret)
    }
}

impl Produce<Option<DateTime<Utc>>> for MixedSource {
    fn produce(&mut self) -> Result<Option<DateTime<Utc>>> {
        let ret = match (self.counter / self.ncols) % 2 {
            0 => Some(offset::Utc::now()),
            1 => None,
            _ => unreachable!(),
        };
        self.counter += 1;
        Ok(ret)
    }
}

impl Produce<Date<Utc>> for MixedSource {
    fn produce(&mut self) -> Result<Date<Utc>> {
        let ret = offset::Utc::now().date();
        self.counter += 1;
        Ok(ret)
    }
}

impl Produce<Option<Date<Utc>>> for MixedSource {
    fn produce(&mut self) -> Result<Option<Date<Utc>>> {
        let ret = match (self.counter / self.ncols) % 2 {
            0 => Some(offset::Utc::now().date()),
            1 => None,
            _ => unreachable!(),
        };
        self.counter += 1;
        Ok(ret)
    }
}
