//! Destination implementation for Arrow2.

mod arrow_assoc;
mod errors;
mod funcs;
pub mod typesystem;

use super::{Consume, Destination, DestinationPartition};
use crate::constants::RECORD_BATCH_SIZE;
use crate::data_order::DataOrder;
use crate::typesystem::{Realize, TypeAssoc, TypeSystem};
use anyhow::anyhow;
use arrow2::array::{Array, MutableArray};
use arrow2::chunk::Chunk;
use arrow2::datatypes::{Field, Schema};
use arrow_assoc::ArrowAssoc;
pub use errors::{Arrow2DestinationError, Result};
use fehler::throw;
use fehler::throws;
use funcs::{FFinishBuilder, FNewBuilder, FNewField};
use polars::prelude::{DataFrame, PolarsError, Series};
use std::convert::TryFrom;
use std::sync::{Arc, Mutex};
pub use typesystem::Arrow2TypeSystem;

type Builder = Box<dyn MutableArray + 'static + Send>;
type Builders = Vec<Builder>;
type ChunkBuffer = Arc<Mutex<Vec<Chunk<Box<dyn Array>>>>>;

pub struct Arrow2Destination {
    schema: Vec<Arrow2TypeSystem>,
    names: Vec<String>,
    data: ChunkBuffer,
    arrow_schema: Arc<Schema>,
}

impl Default for Arrow2Destination {
    fn default() -> Self {
        Arrow2Destination {
            schema: vec![],
            names: vec![],
            data: Arc::new(Mutex::new(vec![])),
            arrow_schema: Arc::new(Schema::default()),
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
    type Partition<'a> = ArrowPartitionWriter;
    type Error = Arrow2DestinationError;

    fn needs_count(&self) -> bool {
        false
    }

    #[throws(Arrow2DestinationError)]
    fn allocate<S: AsRef<str>>(
        &mut self,
        _nrows: usize,
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

        // parse the metadata
        self.schema = schema.to_vec();
        self.names = names.iter().map(|n| n.as_ref().to_string()).collect();
        let fields = self
            .schema
            .iter()
            .zip(&self.names)
            .map(|(&dt, h)| Ok(Realize::<FNewField>::realize(dt)?(h.as_str())))
            .collect::<Result<Vec<_>>>()?;
        self.arrow_schema = Arc::new(Schema::from(fields));
    }

    #[throws(Arrow2DestinationError)]
    fn partition(&mut self, counts: usize) -> Vec<Self::Partition<'_>> {
        let mut partitions = vec![];
        for _ in 0..counts {
            partitions.push(ArrowPartitionWriter::new(
                self.schema.clone(),
                Arc::clone(&self.data),
            )?);
        }
        partitions
    }

    fn schema(&self) -> &[Arrow2TypeSystem] {
        self.schema.as_slice()
    }
}

impl Arrow2Destination {
    #[throws(Arrow2DestinationError)]
    pub fn arrow(self) -> (Vec<Chunk<Box<dyn Array>>>, Arc<Schema>) {
        let lock = Arc::try_unwrap(self.data).map_err(|_| anyhow!("Partitions are not freed"))?;
        (
            lock.into_inner()
                .map_err(|e| anyhow!("mutex poisoned {}", e))?,
            self.arrow_schema,
        )
    }

    #[throws(Arrow2DestinationError)]
    pub fn polars(self) -> DataFrame {
        let (rbs, schema): (Vec<Chunk<Box<dyn Array>>>, Arc<Schema>) = self.arrow()?;
        //let fields = schema.fields.as_slice();
        let fields: &[Field] = schema.fields.as_slice();

        // This should be in polars but their version needs updating.
        // Whave placed this here contained in an inner function until the fix is merged upstream
        fn try_from(
            chunks: (Vec<Chunk<Box<dyn Array>>>, &[Field]),
        ) -> std::result::Result<DataFrame, PolarsError> {
            use polars::prelude::NamedFrom;

            let mut series: Vec<Series> = vec![];

            for chunk in chunks.0.into_iter() {
                let columns_results: std::result::Result<Vec<Series>, PolarsError> = chunk
                    .into_arrays()
                    .into_iter()
                    .zip(chunks.1)
                    .map(|(arr, field)| {
                        let a = Series::try_from((field.name.as_str(), arr)).map_err(|_| {
                            PolarsError::ComputeError("Couldn't build Series from box".into())
                        });
                        a
                    })
                    .collect();

                let columns = columns_results?;

                if series.is_empty() {
                    for col in columns.iter() {
                        let name = col.name().to_string();
                        series.push(Series::new(&name, col));
                    }
                    continue;
                }

                for (i, col) in columns.into_iter().enumerate() {
                    series[i].append(&col)?;
                }
            }

            DataFrame::new(series)
        }

        try_from((rbs, fields)).unwrap()
    }
}

pub struct ArrowPartitionWriter {
    schema: Vec<Arrow2TypeSystem>,
    builders: Option<Builders>,
    current_row: usize,
    current_col: usize,
    data: ChunkBuffer,
}

impl ArrowPartitionWriter {
    #[throws(Arrow2DestinationError)]
    fn new(schema: Vec<Arrow2TypeSystem>, data: ChunkBuffer) -> Self {
        let mut pw = ArrowPartitionWriter {
            schema,
            builders: None,
            current_row: 0,
            current_col: 0,
            data,
        };
        pw.allocate()?;
        pw
    }

    #[throws(Arrow2DestinationError)]
    fn allocate(&mut self) {
        let builders = self
            .schema
            .iter()
            .map(|&dt| Ok(Realize::<FNewBuilder>::realize(dt)?(RECORD_BATCH_SIZE)))
            .collect::<Result<Vec<_>>>()?;
        self.builders.replace(builders);
    }

    #[throws(Arrow2DestinationError)]
    fn flush(&mut self) {
        let builders = self
            .builders
            .take()
            .unwrap_or_else(|| panic!("arrow builder is none when flush!"));

        let columns = builders
            .into_iter()
            .zip(self.schema.iter())
            .map(|(builder, &dt)| Realize::<FFinishBuilder>::realize(dt)?(builder))
            .collect::<std::result::Result<Vec<Box<dyn Array>>, crate::errors::ConnectorXError>>(
            )?;

        let rb = Chunk::try_new(columns)?;
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
    type TypeSystem = Arrow2TypeSystem;
    type Error = Arrow2DestinationError;

    fn ncols(&self) -> usize {
        self.schema.len()
    }

    #[throws(Arrow2DestinationError)]
    fn finalize(&mut self) {
        if self.builders.is_some() {
            self.flush()?;
        }
    }

    #[throws(Arrow2DestinationError)]
    fn aquire_row(&mut self, _n: usize) -> usize {
        self.current_row
    }
}

impl<'a, T> Consume<T> for ArrowPartitionWriter
where
    T: TypeAssoc<<Self as DestinationPartition<'a>>::TypeSystem> + ArrowAssoc + 'static,
{
    type Error = Arrow2DestinationError;

    #[throws(Arrow2DestinationError)]
    fn consume(&mut self, value: T) {
        let col = self.current_col;
        self.current_col = (self.current_col + 1) % self.ncols();
        self.schema[col].check::<T>()?;

        match &mut self.builders {
            Some(builders) => {
                <T as ArrowAssoc>::push(
                    builders[col]
                        .as_mut_any()
                        .downcast_mut::<T::Builder>()
                        .ok_or_else(|| anyhow!("cannot cast arrow builder for append"))?,
                    value,
                );
            }
            None => throw!(anyhow!("arrow arrays are empty!")),
        }

        // flush if exceed batch_size
        if self.current_col == 0 {
            self.current_row += 1;
            if self.current_row >= RECORD_BATCH_SIZE {
                self.flush()?;
                self.allocate()?;
            }
        }
    }
}
