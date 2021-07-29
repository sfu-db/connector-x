use crate::{
    data_order::{coordinate, DataOrder},
    destinations::{Destination, DestinationPartition},
    errors::{ConnectorXError, Result as CXResult},
    sources::{Source, SourcePartition},
    sql::CXQuery,
    typesystem::{Transport, TypeSystem},
};
use itertools::Itertools;
use log::debug;
use rayon::prelude::*;
use std::marker::PhantomData;

/// A dispatcher takes a `S: Source`, a `D: Destination`, a `TP: Transport` and a vector of `queries` as input to
/// load data from `S` to `D` using the queries.
pub struct Dispatcher<'a, S, D, TP> {
    src: S,
    dst: &'a mut D,
    queries: Vec<CXQuery<String>>,
    _phantom: PhantomData<TP>,
}

impl<'w, S, TSS, D, TSD, TP, ES, ED, ET> Dispatcher<'w, S, D, TP>
where
    TSS: TypeSystem,
    S: Source<TypeSystem = TSS, Error = ES>,
    ES: From<ConnectorXError> + Send,

    TSD: TypeSystem,
    D: Destination<TypeSystem = TSD, Error = ED>,
    ED: From<ConnectorXError> + Send,

    TP: Transport<TSS = TSS, TSD = TSD, S = S, D = D, Error = ET>,
    ET: From<ConnectorXError> + From<ES> + From<ED> + Send,
{
    /// Create a new dispatcher by providing a source, a destination and the queries.
    pub fn new<Q: ToString>(src: S, dst: &'w mut D, queries: &[CXQuery<Q>]) -> Self {
        Dispatcher {
            src,
            dst,
            queries: queries.iter().map(|q| q.map(Q::to_string)).collect(),
            _phantom: PhantomData,
        }
    }

    /// Start the data loading process.
    pub fn run(mut self) -> Result<(), ET> {
        let dorder = coordinate(S::DATA_ORDERS, D::DATA_ORDERS)?;
        self.src.set_data_order(dorder)?;
        self.src.set_queries(self.queries.as_slice());
        debug!("Fetching metadata");
        self.src.fetch_metadata()?;
        let src_schema = self.src.schema();
        let dst_schema = src_schema
            .iter()
            .map(|&s| TP::convert_typesystem(s))
            .collect::<CXResult<Vec<_>>>()?;
        let names = self.src.names();

        // generate partitions
        let mut src_partitions: Vec<S::Partition> = self.src.partition()?;
        debug!("Prepare partitions");
        // run queries
        src_partitions
            .par_iter_mut()
            .try_for_each(|partition| -> Result<(), ES> { partition.prepare() })?;

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

        #[cfg(all(not(feature = "branch"), not(feature = "fptr")))]
        compile_error!("branch or fptr, pick one");

        #[cfg(feature = "branch")]
        let schemas: Vec<_> = src_schema
            .iter()
            .zip_eq(&dst_schema)
            .map(|(&src_ty, &dst_ty)| (src_ty, dst_ty))
            .collect();

        debug!("Start writing");
        // parse and write
        dst_partitions
            .into_par_iter()
            .zip_eq(src_partitions)
            .enumerate()
            .try_for_each(|(i, (mut src, mut dst))| -> Result<(), ET> {
                #[cfg(feature = "fptr")]
                let f: Vec<_> = src_schema
                    .iter()
                    .zip_eq(&dst_schema)
                    .map(|(&src_ty, &dst_ty)| TP::processor(src_ty, dst_ty))
                    .collect::<CXResult<Vec<_>>>()?;

                let mut parser = dst.parser()?;

                match dorder {
                    DataOrder::RowMajor => {
                        for _ in 0..src.nrows() {
                            #[allow(clippy::needless_range_loop)]
                            for col in 0..src.ncols() {
                                #[cfg(feature = "fptr")]
                                f[col](&mut parser, &mut src)?;

                                #[cfg(feature = "branch")]
                                {
                                    let (s1, s2) = schemas[col];
                                    TP::process(s1, s2, &mut parser, &mut src)?;
                                }
                            }
                        }
                    }
                    DataOrder::ColumnMajor =>
                    {
                        #[allow(clippy::needless_range_loop)]
                        for col in 0..src.ncols() {
                            for _ in 0..src.nrows() {
                                #[cfg(feature = "fptr")]
                                f[col](&mut parser, &mut src)?;
                                #[cfg(feature = "branch")]
                                {
                                    let (s1, s2) = schemas[col];
                                    TP::process(s1, s2, &mut parser, &mut src)?;
                                }
                            }
                        }
                    }
                }

                debug!("Finalize partition {}", i);
                src.finalize()?;
                debug!("Partition {} finished", i);
                Ok(())
            })?;

        debug!("Writing finished");

        Ok(())
    }
}
