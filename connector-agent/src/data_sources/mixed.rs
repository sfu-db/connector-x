use super::{DataSource, Parse, SourceBuilder};
use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::types::DataType;
use fehler::{throw, throws};
use num_traits::cast::FromPrimitive;

pub struct MixedSourceBuilder {}

impl MixedSourceBuilder {
    pub fn new() -> Self {
        MixedSourceBuilder {}
    }
}

impl SourceBuilder for MixedSourceBuilder {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type DataSource = MixedSource;

    #[throws(ConnectorAgentError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }
    }

    fn build(&mut self) -> Self::DataSource {
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

impl DataSource for MixedSource {
    type TypeSystem = DataType;

    // query: nrows,ncols
    fn run_query(&mut self, query: &str) -> Result<()> {
        let v: Vec<usize> = query.split(',').map(|s| s.parse().unwrap()).collect();
        self.nrows = v[0];
        self.ncols = v[1];
        Ok(())
    }

    fn nrows(&self) -> usize {
        self.nrows
    }
}

impl Parse<u64> for MixedSource {
    fn parse(&mut self) -> Result<u64> {
        let ret = self.counter / self.ncols;
        self.counter += 1;
        Ok(FromPrimitive::from_usize(ret).unwrap_or_default())
    }
}

impl Parse<f64> for MixedSource {
    fn parse(&mut self) -> Result<f64> {
        let ret = self.counter / self.ncols;
        self.counter += 1;
        Ok(FromPrimitive::from_usize(ret).unwrap_or_default())
    }
}

impl Parse<String> for MixedSource {
    fn parse(&mut self) -> Result<String> {
        let ret = ((self.counter / self.ncols) as u64).to_string();
        self.counter += 1;
        Ok(ret)
    }
}

impl Parse<bool> for MixedSource {
    fn parse(&mut self) -> Result<bool> {
        let ret = (self.counter / self.ncols) % 2 == 0;
        self.counter += 1;
        Ok(ret)
    }
}
