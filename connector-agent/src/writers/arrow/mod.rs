use super::{Consume, PartitionWriter, Writer};
use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::types::DataType;
use crate::typesystem::{Realize, TypeAssoc, TypeSystem};
use arrow::datatypes::{Field, Schema};
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

pub struct ArrowWriter {
    nrows: usize,
    schema: Vec<DataType>,
    builders: Vec<Builders>,
}

impl ArrowWriter {
    pub fn new() -> Self {
        ArrowWriter {
            nrows: 0,
            schema: vec![],
            builders: vec![],
        }
    }
}

impl Writer for ArrowWriter {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::ColumnMajor, DataOrder::RowMajor];
    type TypeSystem = DataType;
    type PartitionWriter<'a> = ArrowPartitionWriter<'a>;

    #[throws(ConnectorAgentError)]
    fn allocate(&mut self, nrows: usize, schema: Vec<DataType>, _data_order: DataOrder) {
        // cannot really create builders since do not know each partition size here
        self.nrows = nrows;
        self.schema = schema;
    }

    fn partition_writers(&mut self, counts: &[usize]) -> Vec<Self::PartitionWriter<'_>> {
        assert_eq!(counts.iter().sum::<usize>(), self.nrows);
        assert_eq!(self.builders.len(), 0);

        for &c in counts {
            let builders: Vec<_> = self
                .schema
                .iter()
                .map(|&dt| Realize::<FNewBuilder>::realize(dt)(c))
                .collect();

            self.builders.push(builders);
        }

        let schema = self.schema.clone();
        self.builders
            .iter_mut()
            .zip(counts)
            .map(|(builders, &c)| ArrowPartitionWriter::new(schema.clone(), builders, c))
            .collect()
    }

    fn schema(&self) -> &[DataType] {
        self.schema.as_slice()
    }
}

impl ArrowWriter {
    pub fn finish(self, headers: Vec<String>) -> Vec<RecordBatch> {
        let fields: Vec<Field> = self
            .schema
            .iter()
            .zip_eq(headers)
            .map(|(&dt, h)| Realize::<FNewField>::realize(dt)(h.as_str()))
            .collect();

        let arrow_schema = Arc::new(Schema::new(fields));
        let schema = self.schema.clone();
        self.builders
            .into_iter()
            .map(|pbuilder| {
                let columns = pbuilder
                    .into_iter()
                    .zip(schema.iter())
                    .map(|(builder, &dt)| Realize::<FFinishBuilder>::realize(dt)(builder))
                    .collect();
                RecordBatch::try_new(Arc::clone(&arrow_schema), columns).unwrap()
            })
            .collect()
    }
}

pub struct ArrowPartitionWriter<'a> {
    nrows: usize,
    schema: Vec<DataType>,
    builders: &'a mut Builders,
}

impl<'a> ArrowPartitionWriter<'a> {
    fn new(schema: Vec<DataType>, builders: &'a mut Builders, nrows: usize) -> Self {
        ArrowPartitionWriter {
            nrows,
            schema,
            builders,
        }
    }
}

impl<'a> PartitionWriter<'a> for ArrowPartitionWriter<'a> {
    type TypeSystem = DataType;

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.schema.len()
    }
}

impl<'a, T> Consume<T> for ArrowPartitionWriter<'a>
where
    T: TypeAssoc<<Self as PartitionWriter<'a>>::TypeSystem> + ArrowAssoc + 'static,
{
    unsafe fn consume(&mut self, _row: usize, col: usize, value: T) {
        // NOTE: can use `get_mut_unchecked` instead of Mutex in the future to speed up
        <T as ArrowAssoc>::append(
            self.builders[col].downcast_mut::<T::Builder>().unwrap(),
            value,
        );
    }

    fn consume_checked(&mut self, row: usize, col: usize, value: T) -> Result<()> {
        self.schema[col].check::<T>()?;
        unsafe { self.write(row, col, value) };
        Ok(())
    }
}
