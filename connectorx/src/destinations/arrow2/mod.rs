//! Destination implementation for Arrow2.

use std::sync::Arc;

mod arrow_assoc;
mod errors;
mod funcs;
pub mod typesystem;

use super::{Consume, Destination, DestinationPartition};
use crate::data_order::DataOrder;
use crate::typesystem::{Realize, TypeAssoc, TypeSystem};
use anyhow::anyhow;
use arrow2::array::MutableArray;
use arrow2::datatypes::Schema;
use arrow2::record_batch::RecordBatch;
use arrow_assoc::ArrowAssoc;
pub use errors::{Arrow2DestinationError, Result};
use fehler::throw;
use fehler::throws;
use funcs::{FFinishBuilder, FNewBuilder, FNewField};
use polars::frame::DataFrame;
use std::convert::TryFrom;
pub use typesystem::Arrow2TypeSystem;

type Builder = Box<dyn MutableArray + 'static + Send>;
type Builders = Vec<Builder>;

pub struct Arrow2Destination {
    nrows: usize,
    schema: Vec<Arrow2TypeSystem>,
    names: Vec<String>,
    builders: Vec<Builders>,
}

impl Default for Arrow2Destination {
    fn default() -> Self {
        Arrow2Destination {
            nrows: 0,
            schema: vec![],
            builders: vec![],
            names: vec![],
        }
    }
}

impl Arrow2Destination {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Destination for Arrow2Destination {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::ColumnMajor, DataOrder::RowMajor];
    type TypeSystem = Arrow2TypeSystem;
    type Partition<'a> = ArrowPartitionWriter<'a>;
    type Error = Arrow2DestinationError;

    #[throws(Arrow2DestinationError)]
    fn allocate<S: AsRef<str>>(
        &mut self,
        nrows: usize,
        names: &[S],
        schema: &[Arrow2TypeSystem],
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

    #[throws(Arrow2DestinationError)]
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

    fn schema(&self) -> &[Arrow2TypeSystem] {
        self.schema.as_slice()
    }
}

impl Arrow2Destination {
    #[throws(Arrow2DestinationError)]
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

    #[throws(Arrow2DestinationError)]
    pub fn polars(self) -> DataFrame {
        let rbs = self.arrow()?;
        DataFrame::try_from(rbs)?
    }
}

pub struct ArrowPartitionWriter<'a> {
    nrows: usize,
    schema: Vec<Arrow2TypeSystem>,
    builders: &'a mut Builders,
    current_col: usize,
}

impl<'a> ArrowPartitionWriter<'a> {
    fn new(schema: Vec<Arrow2TypeSystem>, builders: &'a mut Builders, nrows: usize) -> Self {
        ArrowPartitionWriter {
            nrows,
            schema,
            builders,
            current_col: 0,
        }
    }
}

impl<'a> DestinationPartition<'a> for ArrowPartitionWriter<'a> {
    type TypeSystem = Arrow2TypeSystem;
    type Error = Arrow2DestinationError;

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
    type Error = Arrow2DestinationError;

    #[throws(Arrow2DestinationError)]
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
