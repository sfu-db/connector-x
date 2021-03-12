use crate::{
    data_order::{coordinate, DataOrder},
    data_sources::{PartitionedSource, Source},
    errors::Result,
    typesystem::{Transport, TypeSystem},
    writers::{PartitionWriter, Writer},
};
use itertools::Itertools;
use log::debug;
use rayon::prelude::*;
use std::marker::PhantomData;

/// A dispatcher owns a `SourceBuilder` `SB` and a vector of `queries`
/// `schema` is a temporary input before we implement infer schema or get schema from DB.
pub struct Dispatcher<'a, S, W, TP> {
    source: S,
    writer: &'a mut W,
    queries: Vec<String>,
    _phantom: PhantomData<TP>,
}

impl<'w, S, TSS, W, TSW, TP> Dispatcher<'w, S, W, TP>
where
    TSS: TypeSystem,
    TSW: TypeSystem,
    S: Source<TypeSystem = TSS>,
    W: Writer<TypeSystem = TSW>,
    for<'s> TP: Transport<TS1 = TSS, TS2 = TSW, S = S, W = W>,
{
    /// Create a new dispatcher by providing a source builder, schema (temporary) and the queries
    /// to be issued to the data source.
    pub fn new<Q>(source: S, writer: &'w mut W, queries: &[Q]) -> Self
    where
        Q: ToString,
    {
        Dispatcher {
            source,
            writer,
            queries: queries.into_iter().map(ToString::to_string).collect(),
            _phantom: PhantomData,
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
        let dorder = coordinate(S::DATA_ORDERS, W::DATA_ORDERS)?;
        self.source.set_data_order(dorder)?;
        self.source.set_queries(self.queries.as_slice());
        debug!("Fetching metadata");
        self.source.fetch_metadata()?;
        let src_schema = self.source.schema();
        let dst_schema = src_schema
            .iter()
            .map(|&s| TP::convert_typesystem(s))
            .collect::<Result<Vec<_>>>()?;
        let names = self.source.names();

        // generate partitions
        let mut partitions: Vec<S::Partition> = self.source.partition()?;
        debug!("Prepare partitions");
        // run queries
        partitions.par_iter_mut().for_each(|partition| {
            partition.prepare().expect("run query");
        });

        // allocate memory and create one partition writer for each source
        let num_rows: Vec<usize> = partitions
            .iter()
            .map(|partition| partition.nrows())
            .collect();

        debug!("Allocate writer memory");
        self.writer
            .allocate(num_rows.iter().sum(), &names, &dst_schema, dorder)?;

        debug!("Create partition writers");
        let partition_writers = self.writer.partition_writers(&num_rows)?;

        for (i, p) in partition_writers.iter().enumerate() {
            debug!("Partition {}, {}x{}", i, p.nrows(), p.ncols());
        }

        let transport = if checked {
            TP::process_checked
        } else {
            TP::process
        };

        let schemas: Vec<_> = src_schema
            .iter()
            .zip_eq(&dst_schema)
            .map(|(&src_ty, &dst_ty)| (src_ty, dst_ty))
            .collect();

        debug!("Start writing");
        // parse and write
        partition_writers
            .into_par_iter()
            .zip_eq(partitions)
            .for_each(|(mut writer, mut partition)| {
                let mut parser = partition.parser().unwrap();

                match dorder {
                    DataOrder::RowMajor => {
                        for _ in 0..writer.nrows() {
                            for col in 0..writer.ncols() {
                                let (s1, s2) = schemas[col];
                                transport(s1, s2, &mut parser, &mut writer).expect("write record");
                            }
                        }
                    }
                    DataOrder::ColumnMajor => {
                        for col in 0..writer.ncols() {
                            for _ in 0..writer.nrows() {
                                let (s1, s2) = schemas[col];
                                transport(s1, s2, &mut parser, &mut writer).expect("write record");
                            }
                        }
                    }
                }

                writer.finalize().unwrap();
            });

        debug!("Writing finished");

        Ok(())
    }
}
