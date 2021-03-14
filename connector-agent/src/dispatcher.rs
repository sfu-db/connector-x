use crate::{
    data_order::{coordinate, DataOrder},
    destinations::{Destination, DestinationPartition},
    errors::Result,
    sources::{Source, SourcePartition},
    typesystem::{Transport, TypeSystem},
};
use itertools::Itertools;
use log::debug;
use rayon::prelude::*;
use std::marker::PhantomData;

/// A dispatcher owns a `SourceBuilder` `SB` and a vector of `queries`
/// `schema` is a temporary input before we implement infer schema or get schema from DB.
pub struct Dispatcher<'a, S, W, TP> {
    src: S,
    dst: &'a mut W,
    queries: Vec<String>,
    _phantom: PhantomData<TP>,
}

impl<'w, S, TSS, W, TSD, TP> Dispatcher<'w, S, W, TP>
where
    TSS: TypeSystem,
    TSD: TypeSystem,
    S: Source<TypeSystem = TSS>,
    W: Destination<TypeSystem = TSD>,
    TP: Transport<TSS = TSS, TSD = TSD, S = S, D = W>,
{
    /// Create a new dispatcher by providing a source builder, schema (temporary) and the queries
    /// to be issued to the data source.
    pub fn new<Q>(src: S, dst: &'w mut W, queries: &[Q]) -> Self
    where
        Q: ToString,
    {
        Dispatcher {
            src,
            dst,
            queries: queries.into_iter().map(ToString::to_string).collect(),
            _phantom: PhantomData,
        }
    }

    /// Run the dispatcher by specifying the src, the dispatcher will fetch, parse the data,
    /// and write the data to dst.
    pub fn run(mut self) -> Result<()> {
        let dorder = coordinate(S::DATA_ORDERS, W::DATA_ORDERS)?;
        self.src.set_data_order(dorder)?;
        self.src.set_queries(self.queries.as_slice());
        debug!("Fetching metadata");
        self.src.fetch_metadata()?;
        let src_schema = self.src.schema();
        let dst_schema = src_schema
            .iter()
            .map(|&s| TP::convert_typesystem(s))
            .collect::<Result<Vec<_>>>()?;
        let names = self.src.names();

        // generate partitions
        let mut src_partitions: Vec<S::Partition> = self.src.partition()?;
        debug!("Prepare partitions");
        // run queries
        src_partitions.par_iter_mut().for_each(|partition| {
            partition.prepare().expect("run query");
        });

        // allocate memory and create one partition for each source
        let num_rows: Vec<usize> = src_partitions
            .iter()
            .map(|partition| partition.nrows())
            .collect();

        debug!("Allocate destination memory");
        self.dst
            .allocate(num_rows.iter().sum(), &names, &dst_schema, dorder)?;

        debug!("Create destination partition");
        let dst_partitions = self.dst.partition(&num_rows)?;

        for (i, p) in dst_partitions.iter().enumerate() {
            debug!("Partition {}, {}x{}", i, p.nrows(), p.ncols());
        }

        debug!("Start writing");
        // parse and write
        dst_partitions
            .into_par_iter()
            .zip_eq(src_partitions)
            .for_each(|(mut src, mut dst)| {
                let f: Vec<_> = src_schema
                    .iter()
                    .zip_eq(&dst_schema)
                    .map(|(&src_ty, &dst_ty)| TP::process_func(src_ty, dst_ty))
                    .collect::<Result<Vec<_>>>()
                    .unwrap();

                let mut parser = dst.parser().unwrap();

                match dorder {
                    DataOrder::RowMajor => {
                        for _ in 0..src.nrows() {
                            for col in 0..src.ncols() {
                                f[col](&mut parser, &mut src).unwrap();
                            }
                        }
                    }
                    DataOrder::ColumnMajor => {
                        for col in 0..src.ncols() {
                            for _ in 0..src.nrows() {
                                f[col](&mut parser, &mut src).unwrap();
                            }
                        }
                    }
                }

                src.finalize().unwrap();
            });

        debug!("Writing finished");

        Ok(())
    }
}
