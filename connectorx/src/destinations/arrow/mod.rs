use super::{Consume, Destination, DestinationPartition};
use crate::data_order::DataOrder;
use crate::dummy_typesystem::DummyTypeSystem;
use crate::errors::{ConnectorAgentError, Result};
use crate::typesystem::{Realize, TypeAssoc, TypeSystem};
use anyhow::anyhow;
use arrow::datatypes::Schema;
use arrow::record_batch::RecordBatch;
use arrow_assoc::ArrowAssoc;
use fehler::throws;
use funcs::{FFinishBuilder, FNewBuilder, FNewField};
use itertools::Itertools;
use std::any::Any;
use std::sync::Arc;

mod arrow_assoc;
mod funcs;

type Builder = Box<dyn Any + Send>;
type Builders = Vec<Builder>;

pub struct ArrowDestination {
    nrows: usize,
    schema: Vec<DummyTypeSystem>,
    builders: Vec<Builders>,
}

impl Default for ArrowDestination {
    fn default() -> Self {
        ArrowDestination {
            nrows: 0,
            schema: vec![],
            builders: vec![],
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
    type TypeSystem = DummyTypeSystem;
    type Partition<'a> = ArrowPartitionWriter<'a>;

    #[throws(ConnectorAgentError)]
    fn allocate<S: AsRef<str>>(
        &mut self,
        nrows: usize,
        _names: &[S],
        schema: &[DummyTypeSystem],
        _data_order: DataOrder,
    ) {
        // cannot really create builders since do not know each partition size here
        self.nrows = nrows;
        self.schema = schema.to_vec();
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

    fn schema(&self) -> &[DummyTypeSystem] {
        self.schema.as_slice()
    }
}

impl ArrowDestination {
    #[throws(ConnectorAgentError)]
    pub fn finish(self, headers: Vec<String>) -> Vec<RecordBatch> {
        let fields = self
            .schema
            .iter()
            .zip_eq(headers)
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
}

pub struct ArrowPartitionWriter<'a> {
    nrows: usize,
    schema: Vec<DummyTypeSystem>,
    builders: &'a mut Builders,
    current_col: usize,
}

impl<'a> ArrowPartitionWriter<'a> {
    fn new(schema: Vec<DummyTypeSystem>, builders: &'a mut Builders, nrows: usize) -> Self {
        ArrowPartitionWriter {
            nrows,
            schema,
            builders,
            current_col: 0,
        }
    }
}

impl<'a> DestinationPartition<'a> for ArrowPartitionWriter<'a> {
    type TypeSystem = DummyTypeSystem;

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
