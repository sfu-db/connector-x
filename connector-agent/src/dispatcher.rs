use crate::{
    data_order::{coordinate, DataOrder},
    data_sources::{PartitionedSource, Source},
    errors::Result,
    transmit::{Transmit, TransmitChecked},
    typesystem::{Realize, TransmitHack, TypeSystem, TypeSystemConversion},
    writers::{PartitionWriter, Writer},
};
use itertools::Itertools;
use log::debug;
use rayon::prelude::*;

/// A dispatcher owns a `SourceBuilder` `SB` and a vector of `queries`
/// `schema` is a temporary input before we implement infer schema or get schema from DB.
pub struct Dispatcher<'a, S, TSS, W, TSW>
where
    TSS: TypeSystem,
    TSW: TypeSystem,
    S: Source<TypeSystem = TSS>,
    W: Writer<TypeSystem = TSW>,
{
    source: S,
    writer: &'a mut W,
    queries: Vec<String>,
}

impl<'w, S, TSS, W, TSW> Dispatcher<'w, S, TSS, W, TSW>
where
    TSS: TypeSystem,
    TSW: TypeSystem + TypeSystemConversion<TSS>,
    S: Source<TypeSystem = TSS>,
    W: Writer<TypeSystem = TSW>,
    (TSS, TSW): for<'r, 's> Realize<
        Transmit<<S::Partition as PartitionedSource>::Parser<'s>, W::PartitionWriter<'r>>,
    >,
    (TSS, TSW): for<'r, 's> Realize<
        TransmitChecked<<S::Partition as PartitionedSource>::Parser<'s>, W::PartitionWriter<'r>>,
    >,
    (TSS, TSW): for<'r, 's> TransmitHack<
        <S::Partition as PartitionedSource>::Parser<'s>,
        W::PartitionWriter<'r>,
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
        debug!("Fetching metadata");
        self.source.fetch_metadata()?;
        let src_schema = self.source.schema();
        let dst_schema = src_schema
            .iter()
            .map(|&s| TSW::from(s))
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

        debug!("Start writing");
        // parse and write
        partition_writers
            .into_par_iter()
            .zip_eq(partitions)
            .for_each(|(mut writer, mut partition)| {
                let f: Vec<_> = src_schema
                    .iter()
                    .zip_eq(&dst_schema)
                    .map(|(&src_ty, &dst_ty)| {
                        if checked {
                            Realize::<TransmitChecked<_, _>>::realize((src_ty, dst_ty))
                        } else {
                            Realize::<Transmit<_, _>>::realize((src_ty, dst_ty))
                        }
                    })
                    .collect::<Result<Vec<_>>>()
                    .unwrap();

                let mut parser = partition.parser().unwrap();

                match dorder {
                    DataOrder::RowMajor => {
                        // for row in 0..writer.nrows() {
                        //     for col in 0..writer.ncols() {
                        //         f[col](&mut parser, &mut writer, row, col).expect("write record");
                        //     }
                        // }
                        for row in 0..writer.nrows() {
                            for col in 0..writer.ncols() {
                                (src_schema[col], dst_schema[col])
                                    .transmit(&mut parser, &mut writer, row, col)
                                    .expect("write record");
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
