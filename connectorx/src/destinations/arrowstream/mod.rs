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
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
};

type Builder = Box<dyn Any + Send>;
type Builders = Vec<Builder>;

pub struct ArrowDestination {
    schema: Vec<ArrowTypeSystem>,
    names: Vec<String>,
    arrow_schema: Arc<Schema>,
    batch_size: usize,
    sender: Option<Sender<RecordBatch>>,
    receiver: Receiver<RecordBatch>,
}

impl Default for ArrowDestination {
    fn default() -> Self {
        let (tx, rx) = channel();
        ArrowDestination {
            schema: vec![],
            names: vec![],
            arrow_schema: Arc::new(Schema::empty()),
            batch_size: RECORD_BATCH_SIZE,
            sender: Some(tx),
            receiver: rx,
        }
    }
}

impl ArrowDestination {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_batch_size(batch_size: usize) -> Self {
        let (tx, rx) = channel();
        ArrowDestination {
            schema: vec![],
            names: vec![],
            arrow_schema: Arc::new(Schema::empty()),
            batch_size,
            sender: Some(tx),
            receiver: rx,
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
        let sender = self.sender.take().unwrap();
        for _ in 0..counts {
            partitions.push(ArrowPartitionWriter::new(
                self.schema.clone(),
                Arc::clone(&self.arrow_schema),
                self.batch_size,
                sender.clone(),
            )?);
        }
        partitions
        // self.sender should be freed
    }

    fn schema(&self) -> &[ArrowTypeSystem] {
        self.schema.as_slice()
    }
}

impl ArrowDestination {
    #[throws(ArrowDestinationError)]
    pub fn arrow(self) -> Vec<RecordBatch> {
        if self.sender.is_some() {
            // should not happen since it is dropped after partition
            // but need to make sure here otherwise recv will be blocked forever
            std::mem::drop(self.sender);
        }
        let mut data = vec![];
        loop {
            match self.receiver.recv() {
                Ok(rb) => data.push(rb),
                Err(_) => break,
            }
        }
        data
    }

    #[throws(ArrowDestinationError)]
    pub fn record_batch(&mut self) -> Option<RecordBatch> {
        match self.receiver.recv() {
            Ok(rb) => Some(rb),
            Err(_) => None,
        }
    }

    pub fn empty_batch(&self) -> RecordBatch {
        RecordBatch::new_empty(self.arrow_schema.clone())
    }

    pub fn arrow_schema(&self) -> Arc<Schema> {
        self.arrow_schema.clone()
    }

    pub fn names(&self) -> &[String] {
        self.names.as_slice()
    }
}

pub struct ArrowPartitionWriter {
    schema: Vec<ArrowTypeSystem>,
    builders: Option<Builders>,
    current_row: usize,
    current_col: usize,
    arrow_schema: Arc<Schema>,
    batch_size: usize,
    sender: Option<Sender<RecordBatch>>,
}

// unsafe impl Sync for ArrowPartitionWriter {}

impl ArrowPartitionWriter {
    #[throws(ArrowDestinationError)]
    fn new(
        schema: Vec<ArrowTypeSystem>,
        arrow_schema: Arc<Schema>,
        batch_size: usize,
        sender: Sender<RecordBatch>,
    ) -> Self {
        let mut pw = ArrowPartitionWriter {
            schema,
            builders: None,
            current_row: 0,
            current_col: 0,
            arrow_schema,
            batch_size,
            sender: Some(sender),
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
        self.sender.as_ref().and_then(|s| s.send(rb).ok());

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
        // need to release the sender so receiver knows when the stream is exhasted
        std::mem::drop(self.sender.take());
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
