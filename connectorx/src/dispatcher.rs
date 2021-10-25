///! This module provides [`dispatcher::Dispatcher`], the core struct in ConnectorX
///! that drives the data loading from a source to a destination.
use crate::{
    data_order::{coordinate, DataOrder},
    destinations::{Destination, DestinationPartition},
    errors::{ConnectorXError, Result as CXResult},
    sources::{PartitionParser, Source, SourcePartition},
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
    origin_query: Option<String>,
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
    pub fn new<Q>(src: S, dst: &'w mut D, queries: &[Q], origin_query: Option<String>) -> Self
    where
        for<'a> &'a Q: Into<CXQuery>,
    {
        Dispatcher {
            src,
            dst,
            queries: queries.iter().map(Into::into).collect(),
            origin_query,
            _phantom: PhantomData,
        }
    }

    /// Start the data loading process.
    pub fn run(mut self) -> Result<(), ET> {
        let dorder = coordinate(S::DATA_ORDERS, D::DATA_ORDERS)?;
        self.src.set_data_order(dorder)?;
        self.src.set_queries(self.queries.as_slice());
        self.src.set_origin_query(self.origin_query);

        debug!("Fetching metadata");
        self.src.fetch_metadata()?;
        let src_schema = self.src.schema();
        let dst_schema = src_schema
            .iter()
            .map(|&s| TP::convert_typesystem(s))
            .collect::<CXResult<Vec<_>>>()?;
        let names = self.src.names();

        // get number of row and partition the source
        let mut num_rows = self.src.result_rows()?;
        let mut src_partitions: Vec<S::Partition> = self.src.partition()?;
        if num_rows.is_none() {
            debug!("Manually assigned partition queries, count each and sum up");
            // run queries
            src_partitions
                .par_iter_mut()
                .try_for_each(|partition| -> Result<(), ES> { partition.prepare() })?;

            // allocate memory and create one partition for each source
            let part_rows: Vec<usize> = src_partitions
                .iter()
                .map(|partition| partition.nrows())
                .collect();
            num_rows = Some(part_rows.iter().sum());
        }
        let num_rows = num_rows.unwrap();

        debug!(
            "Allocate destination memory: {}x{}",
            num_rows,
            src_schema.len()
        );
        self.dst.allocate(num_rows, &names, &dst_schema, dorder)?;

        debug!("Create destination partition");
        let dst_partitions = self.dst.partition(self.queries.len())?;

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
            .try_for_each(|(i, (mut dst, mut src))| -> Result<(), ET> {
                #[cfg(feature = "fptr")]
                let f: Vec<_> = src_schema
                    .iter()
                    .zip_eq(&dst_schema)
                    .map(|(&src_ty, &dst_ty)| TP::processor(src_ty, dst_ty))
                    .collect::<CXResult<Vec<_>>>()?;

                let mut parser = src.parser()?;

                match dorder {
                    DataOrder::RowMajor => loop {
                        let (n, is_last) = parser.fetch_next()?;
                        if n == 0 {
                            break;
                        }
                        dst.aquire_row(n);
                        // let m = dst.aquire_row(n);
                        // debug!(
                        //     "worker {} is going to write {} rows from position {}",
                        //     i, n, m
                        // );
                        for _ in 0..n {
                            #[allow(clippy::needless_range_loop)]
                            for col in 0..dst.ncols() {
                                #[cfg(feature = "fptr")]
                                f[col](&mut parser, &mut dst)?;

                                #[cfg(feature = "branch")]
                                {
                                    let (s1, s2) = schemas[col];
                                    TP::process(s1, s2, &mut parser, &mut dst)?;
                                }
                            }
                        }
                        if is_last {
                            break;
                        }
                    },
                    DataOrder::ColumnMajor => loop {
                        let (n, is_last) = parser.fetch_next()?;
                        if n == 0 {
                            break;
                        }
                        dst.aquire_row(n);
                        #[allow(clippy::needless_range_loop)]
                        for col in 0..dst.ncols() {
                            for _ in 0..n {
                                #[cfg(feature = "fptr")]
                                f[col](&mut parser, &mut dst)?;
                                #[cfg(feature = "branch")]
                                {
                                    let (s1, s2) = schemas[col];
                                    TP::process(s1, s2, &mut parser, &mut dst)?;
                                }
                            }
                        }
                        if is_last {
                            break;
                        }
                    },
                }

                debug!("Finalize partition {}", i);
                dst.finalize()?;
                debug!("Partition {} finished", i);
                Ok(())
            })?;

        debug!("Writing finished");

        Ok(())
    }
}
