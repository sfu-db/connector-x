use super::{Consume, PartitionWriter, Writer};
use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::types::DataType;
use crate::typesystem::{Realize, TypeAssoc, TypeSystem};
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
    fn allocate<S: AsRef<str>>(
        &mut self,
        nrows: usize,
        _names: &[S],
        schema: &[DataType],
        _data_order: DataOrder,
    ) {
        // cannot really create builders since do not know each partition size here
        self.nrows = nrows;
        self.schema = schema.to_vec();
    }

    #[throws(ConnectorAgentError)]
    fn partition_writers(&mut self, counts: &[usize]) -> Vec<Self::PartitionWriter<'_>> {
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

    fn schema(&self) -> &[DataType] {
        self.schema.as_slice()
    }
}

impl ArrowWriter {
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
                    .map(|(builder, &dt)| Ok(Realize::<FFinishBuilder>::realize(dt)?(builder)))
                    .collect::<Result<Vec<_>>>()?;
                Ok(RecordBatch::try_new(Arc::clone(&arrow_schema), columns).unwrap())
            })
            .collect::<Result<Vec<_>>>()?
    }
}

pub struct ArrowPartitionWriter<'a> {
    nrows: usize,
    schema: Vec<DataType>,
    builders: &'a mut Builders,
    current_col: usize,
}

impl<'a> ArrowPartitionWriter<'a> {
    fn new(schema: Vec<DataType>, builders: &'a mut Builders, nrows: usize) -> Self {
        ArrowPartitionWriter {
            nrows,
            schema,
            builders,
            current_col: 0,
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
    unsafe fn consume(&mut self, value: T) {
        let col = self.current_col;
        self.current_col = (self.current_col + 1) % self.ncols();
        // NOTE: can use `get_mut_unchecked` instead of Mutex in the future to speed up
        <T as ArrowAssoc>::append(
            self.builders[col].downcast_mut::<T::Builder>().unwrap(),
            value,
        );
    }

    fn consume_checked(&mut self, value: T) -> Result<()> {
        let col = self.current_col;

        self.schema[col].check::<T>()?;
        unsafe { self.write(value) };
        Ok(())
    }
}
