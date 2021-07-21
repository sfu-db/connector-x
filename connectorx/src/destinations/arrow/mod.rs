mod arrow_assoc;
mod funcs;
pub mod types;

use super::{Consume, Destination, DestinationPartition};
use crate::data_order::DataOrder;
use crate::destinations::arrow::types::ArrowTypeSystem;
use crate::errors::{ConnectorAgentError, Result};
use crate::typesystem::{Realize, TypeAssoc, TypeSystem};
use anyhow::anyhow;
use arrow::datatypes::Schema;
use arrow::record_batch::RecordBatch;
use arrow_assoc::ArrowAssoc;
use fehler::throw;
use fehler::throws;
use funcs::{FFinishBuilder, FNewBuilder, FNewField};
use itertools::Itertools;
use polars::frame::DataFrame;
use std::any::Any;
use std::convert::TryFrom;
use std::sync::Arc;

type Builder = Box<dyn Any + Send>;
type Builders = Vec<Builder>;

pub struct ArrowDestination {
    nrows: usize,
    schema: Vec<ArrowTypeSystem>,
    names: Vec<String>,
    builders: Vec<Builders>,
}

impl Default for ArrowDestination {
    fn default() -> Self {
        ArrowDestination {
            nrows: 0,
            schema: vec![],
            builders: vec![],
            names: vec![],
        }
    }
}

impl ArrowDestination {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Destination for ArrowDestination {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::ColumnMajor, DataOrder::RowMajor];
    type TypeSystem = ArrowTypeSystem;
    type Partition<'a> = ArrowPartitionWriter<'a>;

    #[throws(ConnectorAgentError)]
    fn allocate<S: AsRef<str>>(
        &mut self,
        nrows: usize,
        names: &[S],
        schema: &[ArrowTypeSystem],
        data_order: DataOrder,
    ) {
        // todo: support colmajor
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }
        // cannot really create the builders since do not know each partition size here
        self.nrows = nrows;
        self.schema = schema.to_vec();
        self.names = names.iter().map(|n| n.as_ref().to_string()).collect();
    }

    #[throws(ConnectorAgentError)]
    fn partition(&mut self, counts: &[usize]) -> Vec<Self::Partition<'_>> {
        assert_eq!(counts.iter().sum::<usize>(), self.nrows);
        assert_eq!(self.builders.len(), 0);

        for &c in counts {
            let builders = self
                .schema
                .iter()
                .map(|&dt| Ok(Realize::<FNewBuilder>::realize(dt)?(c)))
                .collect::<Result<Vec<_>>>()?;

            self.builders.push(builders);
        }

        let schema = self.schema.clone();
        self.builders
            .iter_mut()
            .zip(counts)
            .map(|(builders, &c)| ArrowPartitionWriter::new(schema.clone(), builders, c))
            .collect()
    }

    fn schema(&self) -> &[ArrowTypeSystem] {
        self.schema.as_slice()
    }
}

impl ArrowDestination {
    #[throws(ConnectorAgentError)]
    pub fn finish(self) -> Vec<RecordBatch> {
        let fields = self
            .schema
            .iter()
            .zip_eq(self.names)
            .map(|(&dt, h)| Ok(Realize::<FNewField>::realize(dt)?(h.as_str())))
            .collect::<Result<Vec<_>>>()?;

        let arrow_schema = Arc::new(Schema::new(fields));
        let schema = self.schema.clone();
        self.builders
            .into_iter()
            .map(|pbuilder| {
                let columns = pbuilder
                    .into_iter()
                    .zip(schema.iter())
                    .map(|(builder, &dt)| Realize::<FFinishBuilder>::realize(dt)?(builder))
                    .collect::<Result<Vec<_>>>()?;
                Ok(RecordBatch::try_new(Arc::clone(&arrow_schema), columns)?)
            })
            .collect::<Result<Vec<_>>>()?
    }

    #[throws(ConnectorAgentError)]
    pub fn polars(self) -> DataFrame {
        let rbs = self.finish()?;
        DataFrame::try_from(rbs)?
    }
}

pub struct ArrowPartitionWriter<'a> {
    nrows: usize,
    schema: Vec<ArrowTypeSystem>,
    builders: &'a mut Builders,
    current_col: usize,
}

impl<'a> ArrowPartitionWriter<'a> {
    fn new(schema: Vec<ArrowTypeSystem>, builders: &'a mut Builders, nrows: usize) -> Self {
        ArrowPartitionWriter {
            nrows,
            schema,
            builders,
            current_col: 0,
        }
    }
}

impl<'a> DestinationPartition<'a> for ArrowPartitionWriter<'a> {
    type TypeSystem = ArrowTypeSystem;

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.schema.len()
    }
}

impl<'a, T> Consume<T> for ArrowPartitionWriter<'a>
where
    T: TypeAssoc<<Self as DestinationPartition<'a>>::TypeSystem> + ArrowAssoc + 'static,
{
    fn consume(&mut self, value: T) -> Result<()> {
        let col = self.current_col;
        self.current_col = (self.current_col + 1) % self.ncols();

        self.schema[col].check::<T>()?;

        <T as ArrowAssoc>::append(
            self.builders[col]
                .downcast_mut::<T::Builder>()
                .ok_or_else(|| anyhow!("cannot cast arrow builder for append"))?,
            value,
        )?;

        Ok(())
    }
}
