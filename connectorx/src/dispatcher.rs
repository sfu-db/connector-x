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

    pub fn prepare(
        mut self,
    ) -> Result<
        (
            DataOrder,
            Vec<S::Partition>,
            Vec<D::Partition<'w>>,
            Vec<TSS>,
            Vec<TSD>,
        ),
        ET,
    > {
        debug!("Prepare");
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

        let mut total_rows = if self.dst.needs_count() {
            // return None if cannot derive total count
            debug!("Try get row rounts for entire result");
            self.src.result_rows()?
        } else {
            debug!("Do not need counts in advance");
            Some(0)
        };
        let mut src_partitions: Vec<S::Partition> = self.src.partition()?;
        if self.dst.needs_count() && total_rows.is_none() {
            debug!("Manually count rows of each partitioned query and sum up");
            // run queries
            src_partitions
                .par_iter_mut()
                .try_for_each(|partition| -> Result<(), ES> { partition.result_rows() })?;

            // get number of row of each partition from the source
            let part_rows: Vec<usize> = src_partitions
                .iter()
                .map(|partition| partition.nrows())
                .collect();
            total_rows = Some(part_rows.iter().sum());
        }
        let total_rows = total_rows.ok_or_else(ConnectorXError::CountError)?;

        debug!(
            "Allocate destination memory: {}x{}",
            total_rows,
            src_schema.len()
        );
        self.dst.allocate(total_rows, &names, &dst_schema, dorder)?;

        debug!("Create destination partition");
        let dst_partitions = self.dst.partition(self.queries.len())?;

        Ok((
            dorder,
            src_partitions,
            dst_partitions,
            src_schema,
            dst_schema,
        ))
    }

    /// Start the data loading process.
    pub fn run(self) -> Result<(), ET> {
        debug!("Run dispatcher");
        let (dorder, src_partitions, dst_partitions, src_schema, dst_schema) = self.prepare()?;

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
                        dst.aquire_row(n)?;
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
                        dst.aquire_row(n)?;
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

    /// Only fetch the metadata (header) of the destination.
    pub fn get_meta(&mut self) -> Result<(), ET> {
        let dorder = coordinate(S::DATA_ORDERS, D::DATA_ORDERS)?;
        self.src.set_data_order(dorder)?;
        self.src.set_queries(self.queries.as_slice());
        self.src.set_origin_query(self.origin_query.clone());
        self.src.fetch_metadata()?;
        let src_schema = self.src.schema();
        let dst_schema = src_schema
            .iter()
            .map(|&s| TP::convert_typesystem(s))
            .collect::<CXResult<Vec<_>>>()?;
        let names = self.src.names();
        self.dst.allocate(0, &names, &dst_schema, dorder)?;
        Ok(())
    }
}
