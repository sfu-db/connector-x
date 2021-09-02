//! Destination implementation for Arrow2.

use std::sync::Arc;

mod arrow_assoc;
mod errors;
mod funcs;
pub mod typesystem;

use anyhow::anyhow;
use fehler::throw;
use fehler::throws;

use arrow2::array::MutableArray;
use arrow2::datatypes::Schema;
use arrow2::record_batch::RecordBatch;

use crate::data_order::DataOrder;
use crate::typesystem::{Realize, TypeAssoc, TypeSystem};

use super::{Consume, Destination, DestinationPartition};

use arrow_assoc::ArrowAssoc;
pub use errors::{ArrowDestinationError, Result};
use funcs::{FFinishBuilder, FNewBuilder, FNewField};
pub use typesystem::ArrowTypeSystem;

type Builder = Box<dyn MutableArray + 'static + Send>;
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
    type Error = ArrowDestinationError;

    #[throws(ArrowDestinationError)]
    fn allocate<S: AsRef<str>>(
        &mut self,
        nrows: usize,
        names: &[S],
        schema: &[ArrowTypeSystem],
        data_order: DataOrder,
    ) {
        // todo: support colmajor
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(crate::errors::ConnectorXError::UnsupportedDataOrder(
                data_order
            ))
        }
        // cannot really create the builders since do not know each partition size here
        self.nrows = nrows;
        self.schema = schema.to_vec();
        self.names = names.iter().map(|n| n.as_ref().to_string()).collect();
    }

    #[throws(ArrowDestinationError)]
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
    #[throws(ArrowDestinationError)]
    pub fn arrow(self) -> Vec<RecordBatch> {
        assert_eq!(self.schema.len(), self.names.len());
        let fields = self
            .schema
            .iter()
            .zip(self.names)
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
                    .collect::<std::result::Result<Vec<_>, crate::errors::ConnectorXError>>()?;
                Ok(RecordBatch::try_new(Arc::clone(&arrow_schema), columns)?)
            })
            .collect::<Result<Vec<_>>>()?
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
    type Error = ArrowDestinationError;

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
    type Error = ArrowDestinationError;

    #[throws(ArrowDestinationError)]
    fn consume(&mut self, value: T) {
        let col = self.current_col;
        self.current_col = (self.current_col + 1) % self.ncols();

        self.schema[col].check::<T>()?;

        <T as ArrowAssoc>::push(
            self.builders[col]
                .as_mut_any()
                .downcast_mut::<T::Builder>()
                .ok_or_else(|| anyhow!("cannot cast arrow builder for append"))?,
            value,
        );
    }
}
