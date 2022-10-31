//! Destination implementation for Arrow and Polars.

mod arrow_assoc;
mod errors;
mod funcs;
pub mod typesystem;

pub use self::errors::{ArrowDestinationError, Result};
pub use self::typesystem::ArrowTypeSystem;
use super::{Consume, Destination, DestinationPartition};
use crate::constants::RECORD_BATCH_SIZE;
use crate::data_order::DataOrder;
use crate::typesystem::{Realize, TypeAssoc, TypeSystem};
use anyhow::anyhow;
use arrow::{datatypes::Schema, record_batch::RecordBatch};
use arrow_assoc::ArrowAssoc;
use fehler::{throw, throws};
use funcs::{FFinishBuilder, FNewBuilder, FNewField};
use itertools::Itertools;
use std::{
    any::Any,
    sync::{Arc, Mutex},
};

type Builder = Box<dyn Any + Send>;
type Builders = Vec<Builder>;

pub struct ArrowDestination {
    schema: Vec<ArrowTypeSystem>,
    names: Vec<String>,
    data: Arc<Mutex<Vec<RecordBatch>>>,
    arrow_schema: Arc<Schema>,
    batch_size: usize,
}

impl Default for ArrowDestination {
    fn default() -> Self {
        ArrowDestination {
            schema: vec![],
            names: vec![],
            data: Arc::new(Mutex::new(vec![])),
            arrow_schema: Arc::new(Schema::empty()),
            batch_size: RECORD_BATCH_SIZE,
        }
    }
}

impl ArrowDestination {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_batch_size(batch_size: usize) -> Self {
        ArrowDestination {
            schema: vec![],
            names: vec![],
            data: Arc::new(Mutex::new(vec![])),
            arrow_schema: Arc::new(Schema::empty()),
            batch_size,
        }
    }
}

impl Destination for ArrowDestination {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::ColumnMajor, DataOrder::RowMajor];
    type TypeSystem = ArrowTypeSystem;
    type Partition<'a> = ArrowPartitionWriter;
    type Error = ArrowDestinationError;

    fn needs_count(&self) -> bool {
        false
    }

    #[throws(ArrowDestinationError)]
    fn allocate<S: AsRef<str>>(
        &mut self,
        _nrow: usize,
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

        // parse the metadata
        self.schema = schema.to_vec();
        self.names = names.iter().map(|n| n.as_ref().to_string()).collect();
        let fields = self
            .schema
            .iter()
            .zip_eq(&self.names)
            .map(|(&dt, h)| Ok(Realize::<FNewField>::realize(dt)?(h.as_str())))
            .collect::<Result<Vec<_>>>()?;
        self.arrow_schema = Arc::new(Schema::new(fields));
    }

    #[throws(ArrowDestinationError)]
    fn partition(&mut self, counts: usize) -> Vec<Self::Partition<'_>> {
        let mut partitions = vec![];
        for _ in 0..counts {
            partitions.push(ArrowPartitionWriter::new(
                self.schema.clone(),
                Arc::clone(&self.data),
                Arc::clone(&self.arrow_schema),
                self.batch_size,
            )?);
        }
        partitions
    }

    fn schema(&self) -> &[ArrowTypeSystem] {
        self.schema.as_slice()
    }
}

impl ArrowDestination {
    #[throws(ArrowDestinationError)]
    pub fn arrow(self) -> Vec<RecordBatch> {
        let lock = Arc::try_unwrap(self.data).map_err(|_| anyhow!("Partitions are not freed"))?;
        lock.into_inner()
            .map_err(|e| anyhow!("mutex poisoned {}", e))?
    }

    #[throws(ArrowDestinationError)]
    pub fn record_batch(&mut self) -> Option<RecordBatch> {
        let mut guard = self
            .data
            .lock()
            .map_err(|e| anyhow!("mutex poisoned {}", e))?;
        (*guard).pop()
    }

    pub fn arrow_schema(&self) -> Arc<Schema> {
        self.arrow_schema.clone()
    }
}

pub struct ArrowPartitionWriter {
    schema: Vec<ArrowTypeSystem>,
    builders: Option<Builders>,
    current_row: usize,
    current_col: usize,
    data: Arc<Mutex<Vec<RecordBatch>>>,
    arrow_schema: Arc<Schema>,
    batch_size: usize,
}

// unsafe impl Sync for ArrowPartitionWriter {}

impl ArrowPartitionWriter {
    #[throws(ArrowDestinationError)]
    fn new(
        schema: Vec<ArrowTypeSystem>,
        data: Arc<Mutex<Vec<RecordBatch>>>,
        arrow_schema: Arc<Schema>,
        batch_size: usize,
    ) -> Self {
        let mut pw = ArrowPartitionWriter {
            schema,
            builders: None,
            current_row: 0,
            current_col: 0,
            data,
            arrow_schema,
            batch_size,
        };
        pw.allocate()?;
        pw
    }

    #[throws(ArrowDestinationError)]
    fn allocate(&mut self) {
        let builders = self
            .schema
            .iter()
            .map(|dt| Ok(Realize::<FNewBuilder>::realize(*dt)?(self.batch_size)))
            .collect::<Result<Vec<_>>>()?;
        self.builders.replace(builders);
    }

    #[throws(ArrowDestinationError)]
    fn flush(&mut self) {
        let builders = self
            .builders
            .take()
            .unwrap_or_else(|| panic!("arrow builder is none when flush!"));
        let columns = builders
            .into_iter()
            .zip(self.schema.iter())
            .map(|(builder, &dt)| Realize::<FFinishBuilder>::realize(dt)?(builder))
            .collect::<std::result::Result<Vec<_>, crate::errors::ConnectorXError>>()?;
        let rb = RecordBatch::try_new(Arc::clone(&self.arrow_schema), columns)?;
        {
            let mut guard = self
                .data
                .lock()
                .map_err(|e| anyhow!("mutex poisoned {}", e))?;
            let inner_data = &mut *guard;
            inner_data.push(rb);
        }

        self.current_row = 0;
        self.current_col = 0;
    }
}

impl<'a> DestinationPartition<'a> for ArrowPartitionWriter {
    type TypeSystem = ArrowTypeSystem;
    type Error = ArrowDestinationError;

    #[throws(ArrowDestinationError)]
    fn finalize(&mut self) {
        if self.builders.is_some() {
            self.flush()?;
        }
    }

    #[throws(ArrowDestinationError)]
    fn aquire_row(&mut self, _n: usize) -> usize {
        self.current_row
    }

    fn ncols(&self) -> usize {
        self.schema.len()
    }
}

impl<'a, T> Consume<T> for ArrowPartitionWriter
where
    T: TypeAssoc<<Self as DestinationPartition<'a>>::TypeSystem> + ArrowAssoc + 'static,
{
    type Error = ArrowDestinationError;

    #[throws(ArrowDestinationError)]
    fn consume(&mut self, value: T) {
        let col = self.current_col;
        self.current_col = (self.current_col + 1) % self.ncols();
        self.schema[col].check::<T>()?;

        loop {
            match &mut self.builders {
                Some(builders) => {
                    <T as ArrowAssoc>::append(
                        builders[col]
                            .downcast_mut::<T::Builder>()
                            .ok_or_else(|| anyhow!("cannot cast arrow builder for append"))?,
                        value,
                    )?;
                    break;
                }
                None => self.allocate()?, // allocate if builders are not initialized
            }
        }

        // flush if exceed batch_size
        if self.current_col == 0 {
            self.current_row += 1;
            if self.current_row >= self.batch_size {
                self.flush()?;
                self.allocate()?;
            }
        }
    }
}
