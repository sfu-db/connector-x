use crate::{
    data_order::{coordinate, DataOrder},
    data_sources::{PartitionedSource, Source},
    errors::{ConnectorAgentError, Result},
    types::{Transmit, TransmitChecked},
    typesystem::{Realize, TypeSystem},
    writers::{PartitionWriter, Writer},
};
use fehler::throw;
use itertools::Itertools;
use log::debug;
use rayon::prelude::*;

/// A dispatcher owns a `SourceBuilder` `SB` and a vector of `queries`
/// `schema` is a temporary input before we implement infer schema or get schema from DB.
pub struct Dispatcher<'a, SB, WT, TS> {
    source_builder: SB,
    writer: &'a mut WT,
    schema: Vec<TS>,
    queries: Vec<String>,
}

impl<'a, SB, WT, TS> Dispatcher<'a, SB, WT, TS>
where
    TS: TypeSystem,
    SB: Source<TypeSystem = TS>,
    WT: Writer<TypeSystem = TS>,
    TS: for<'r> Realize<Transmit<'r, SB::Partition, WT::PartitionWriter<'r>>>
        + for<'r> Realize<TransmitChecked<'r, SB::Partition, WT::PartitionWriter<'r>>>,
{
    /// Create a new dispatcher by providing a source builder, schema (temporary) and the queries
    /// to be issued to the data source.
    pub fn new<S>(source_builder: SB, writer: &'a mut WT, queries: &[S], schema: &[TS]) -> Self
    where
        S: ToString,
    {
        Dispatcher {
            source_builder,
            writer,
            schema: schema.to_vec(),
            queries: queries.into_iter().map(ToString::to_string).collect(),
        }
    }

    pub fn run_checked(self) -> Result<()> {
        self.entry(true)
    }

    pub fn run(self) -> Result<()> {
        self.entry(false)
    }

    /// Run the dispatcher by specifying the writer, the dispatcher will fetch, parse the data
    /// and return a writer with parsed result
    fn entry(mut self, checked: bool) -> Result<()> {
        let dorder = coordinate(SB::DATA_ORDERS, WT::DATA_ORDERS)?;
        self.source_builder.set_data_order(dorder)?;

        // generate sources
        let mut sources: Vec<SB::Partition> = (0..self.queries.len())
            .map(|_i| self.source_builder.build())
            .collect();

        // run queries
        sources
            .par_iter_mut()
            .zip_eq(self.queries.as_slice())
            .for_each(|(source, query)| {
                source.prepare(query.as_str()).expect("run query");
            });

        debug!("Finished data download");

        // infer schema if not given
        // self.schema = sources[0].infer_schema()?;

        // collect transmit functions for schema
        let funcs: Vec<_> = self
            .schema
            .iter()
            .map(|&ty| {
                if checked {
                    Realize::<TransmitChecked<_, _>>::realize(ty)
                } else {
                    Realize::<Transmit<_, _>>::realize(ty)
                }
            })
            .collect();

        // allocate memory and create one partition writer for each source
        let num_rows: Vec<usize> = sources.iter().map(|source| source.nrows()).collect();
        self.writer
            .allocate(num_rows.iter().sum(), &self.schema, dorder)?;
        let partition_writers = self.writer.partition_writers(num_rows.as_slice())?;

        let dims_are_same = sources
            .iter()
            .zip_eq(&partition_writers)
            .all(|(src, dst)| src.nrows() == dst.nrows() && src.ncols() == dst.ncols());
        if !dims_are_same {
            let snrows = sources.iter().map(|src| src.nrows()).sum();
            let wnrows = partition_writers.iter().map(|dst| dst.nrows()).sum();

            throw!(ConnectorAgentError::DimensionMismatch(
                snrows,
                sources[0].ncols(),
                wnrows,
                partition_writers[0].ncols()
            ))
        }

        // parse and write
        partition_writers
            .into_par_iter()
            .zip_eq(sources)
            .for_each(|(mut writer, mut source)| {
                let f = funcs.clone();

                match dorder {
                    DataOrder::RowMajor => {
                        for row in 0..writer.nrows() {
                            for col in 0..writer.ncols() {
                                f[col](&mut source, &mut writer, row, col).expect("write record");
                            }
                        }
                    }
                    DataOrder::ColumnMajor => {
                        for col in 0..writer.ncols() {
                            for row in 0..writer.nrows() {
                                f[col](&mut source, &mut writer, row, col).expect("write record");
                            }
                        }
                    }
                }
            });

        debug!("Writing finished");

        Ok(())
    }
}
