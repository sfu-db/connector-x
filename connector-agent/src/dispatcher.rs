use crate::{
    data_order::{coordinate, DataOrder},
    data_sources::{PartitionedSource, Source},
    errors::Result,
    transmit::{Transmit, TransmitChecked},
    typesystem::{Realize, TypeSystem},
    writers::{PartitionWriter, Writer},
};
use log::debug;
use rayon::prelude::*;

/// A dispatcher owns a `SourceBuilder` `SB` and a vector of `queries`
/// `schema` is a temporary input before we implement infer schema or get schema from DB.
pub struct Dispatcher<'a, S, W, TS>
where
    TS: TypeSystem,
    S: Source<TypeSystem = TS>,
    W: Writer<TypeSystem = TS>,
{
    source: S,
    writer: &'a mut W,
    queries: Vec<String>,
}

impl<'w, S, W, TS> Dispatcher<'w, S, W, TS>
where
    TS: TypeSystem,
    S: Source<TypeSystem = TS>,
    W: Writer<TypeSystem = TS>,
    TS: for<'s, 'r> Realize<
        Transmit<<S::Partition as PartitionedSource>::Parser<'s>, W::PartitionWriter<'r>>,
    >,
    TS: for<'r, 's> Realize<
        TransmitChecked<<S::Partition as PartitionedSource>::Parser<'s>, W::PartitionWriter<'r>>,
    >,
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
        self.source.fetch_metadata()?;
        let schema = self.source.schema();
        let names = self.source.names();

        // generate partitions
        let mut partitions: Vec<S::Partition> = self.source.partition()?;

        // run queries
        partitions.par_iter_mut().for_each(|partition| {
            partition.prepare().expect("run query");
        });

        debug!("Finished data download");

        // allocate memory and create one partition writer for each source
        let num_rows: Vec<usize> = partitions
            .iter()
            .map(|partition| partition.nrows())
            .collect();

        self.writer
            .allocate(num_rows.iter().sum(), &names, &schema, dorder)?;

        let partition_writers = self.writer.partition_writers(&num_rows)?;

        for (i, p) in partition_writers.iter().enumerate() {
            debug!("Partition {}, {}x{}", i, p.nrows(), p.ncols());
        }

        // parse and write
        partition_writers
            .into_par_iter()
            .zip_eq(partitions)
            .for_each(|(mut writer, mut partition)| {
                let f: Vec<_> = schema
                    .iter()
                    .map(|&ty| {
                        if checked {
                            Realize::<TransmitChecked<_, _>>::realize(ty)
                        } else {
                            Realize::<Transmit<_, _>>::realize(ty)
                        }
                    })
                    .collect();

                let mut parser = partition.parser().unwrap();

                match dorder {
                    DataOrder::RowMajor => {
                        for row in 0..writer.nrows() {
                            for col in 0..writer.ncols() {
                                f[col](&mut parser, &mut writer, row, col).expect("write record");
                            }
                        }
                    }
                    DataOrder::ColumnMajor => {
                        for col in 0..writer.ncols() {
                            for row in 0..writer.nrows() {
                                f[col](&mut parser, &mut writer, row, col).expect("write record");
                            }
                        }
                    }
                }
            });

        debug!("Writing finished");

        Ok(())
    }
}
