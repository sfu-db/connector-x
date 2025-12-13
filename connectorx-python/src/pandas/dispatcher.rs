use super::{destination::PandasDestination, typesystem::PandasTypeSystem};
use crate::errors::ConnectorXPythonError;
use connectorx::errors::Result as CXResult;
use connectorx::prelude::*;
use itertools::Itertools;
use log::debug;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::marker::PhantomData;

pub struct PandasDispatcher<'py, S, TP> {
    src: S,
    dst: PandasDestination<'py>,
    queries: Vec<CXQuery<String>>,
    origin_query: Option<String>,
    _phantom: PhantomData<TP>,
}

impl<'py, S, TP> PandasDispatcher<'py, S, TP>
where
    S: Source,
    TP: Transport<TSS = S::TypeSystem, TSD = PandasTypeSystem, S = S, D = PandasDestination<'py>>,
    <TP as connectorx::typesystem::Transport>::Error: From<ConnectorXPythonError>,
{
    /// Create a new dispatcher by providing a source, a destination and the queries.
    pub fn new<Q>(
        src: S,
        dst: PandasDestination<'py>,
        queries: &[Q],
        origin_query: Option<String>,
    ) -> Self
    where
        for<'a> &'a Q: Into<CXQuery>,
    {
        Self {
            src,
            dst,
            queries: queries.iter().map(Into::into).collect(),
            origin_query,
            _phantom: PhantomData,
        }
    }

    pub fn set_pre_execution_queries(&mut self, pre_execution_queries: Option<&[String]>) {
        self.src.set_pre_execution_queries(pre_execution_queries);
    }

    /// Start the data loading process.
    pub fn run(mut self, py: Python<'py>) -> Result<Bound<'py, PyAny>, TP::Error> {
        debug!("Run dispatcher");

        debug!("Prepare");
        let dorder = coordinate(S::DATA_ORDERS, PandasDestination::DATA_ORDERS)?;
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
                .try_for_each(|partition| -> Result<(), S::Error> { partition.result_rows() })?;

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
        self.dst
            .allocate_py(py, total_rows, &names, &dst_schema, dorder)?;

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

        // release GIL
        py.detach(move || -> Result<(), TP::Error> {
            // parse and write
            dst_partitions
                .into_par_iter()
                .zip_eq(src_partitions)
                .enumerate()
                .try_for_each(|(i, (mut dst, mut src))| -> Result<(), TP::Error> {
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
            Ok(())
        })?;
        debug!("Writing finished");

        Ok(self.dst.result(py).unwrap())
    }

    /// Only fetch the metadata (header) of the destination.
    pub fn get_meta(mut self, py: Python<'py>) -> Result<Bound<'py, PyAny>, TP::Error> {
        let dorder = coordinate(S::DATA_ORDERS, PandasDestination::DATA_ORDERS)?;
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
        self.dst.allocate_py(py, 0, &names, &dst_schema, dorder)?;
        Ok(self.dst.result(py).unwrap())
    }
}
