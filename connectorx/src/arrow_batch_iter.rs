use crate::destinations::arrow::{ArrowDestination, ArrowPartitionWriter, ArrowTypeSystem};
// use crate::errors::{ConnectorXError, Result as CXResult};
use crate::sources::postgres::{
    BinaryProtocol as PgBinaryProtocol, PostgresBinarySourcePartitionParser, PostgresSource,
    PostgresSourcePartition, PostgresTypeSystem,
};
use crate::utils::*;
use crate::{prelude::*, sql::CXQuery};
use arrow::record_batch::RecordBatch;
use itertools::Itertools;
use log::debug;
use owning_ref::OwningHandle;
use postgres::NoTls;
use rayon::prelude::*;

/// The iterator that returns arrow result in `RecordBatch`
pub struct PostgresArrowBatchIter<'a> {
    dst: ArrowDestination,
    dst_parts: Vec<ArrowPartitionWriter>,
    src_parsers: Vec<
        OwningHandle<
            Box<PostgresSourcePartition<PgBinaryProtocol, NoTls>>,
            DummyBox<PostgresBinarySourcePartitionParser<'a>>,
        >,
    >,
    dorder: DataOrder,
    src_schema: Vec<PostgresTypeSystem>,
    dst_schema: Vec<ArrowTypeSystem>,
    batch_size: usize,
}

impl<'a> PostgresArrowBatchIter<'a> {
    pub fn new(
        src: PostgresSource<PgBinaryProtocol, NoTls>,
        mut dst: ArrowDestination,
        origin_query: Option<String>,
        queries: &[CXQuery<String>],
        batch_size: usize,
    ) -> Self {
        let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<PgBinaryProtocol, NoTls>>::new(
            src,
            &mut dst,
            queries,
            origin_query,
        );
        let (dorder, src_parts, dst_parts, src_schema, dst_schema) = dispatcher.prepare().unwrap();

        let src_parsers: Vec<_> = src_parts
            .into_par_iter()
            .map(|part| {
                OwningHandle::new_with_fn(
                    Box::new(part),
                    |part: *const PostgresSourcePartition<PgBinaryProtocol, NoTls>| unsafe {
                        DummyBox(
                            (&mut *(part as *mut PostgresSourcePartition<PgBinaryProtocol, NoTls>))
                                .parser()
                                .unwrap(),
                        )
                    },
                )
            })
            .collect();

        Self {
            dst,
            dst_parts,
            src_parsers,
            dorder,
            src_schema,
            dst_schema,
            batch_size,
        }
    }

    fn run_batch(&mut self) {
        #[cfg(all(not(feature = "branch"), not(feature = "fptr")))]
        compile_error!("branch or fptr, pick one");

        let schemas: Vec<_> = self
            .src_schema
            .iter()
            .zip_eq(&self.dst_schema)
            .map(|(&src_ty, &dst_ty)| (src_ty, dst_ty))
            .collect();

        debug!("Start writing");

        let dorder = self.dorder;
        let batch_size = self.batch_size;

        // parse and write
        self.dst_parts
            .par_iter_mut()
            .zip_eq(self.src_parsers.par_iter_mut())
            .enumerate()
            .for_each(|(i, (dst, src))| {
                let parser: &mut PostgresBinarySourcePartitionParser = &mut *src;
                let mut processed_rows = 0;

                match dorder {
                    DataOrder::RowMajor => loop {
                        let (mut n, is_last) = parser.fetch_next().unwrap();
                        n = std::cmp::min(n, batch_size - processed_rows); // only process until batch size is reached
                        processed_rows += n;
                        dst.aquire_row(n).unwrap();
                        for _ in 0..n {
                            #[allow(clippy::needless_range_loop)]
                            for col in 0..dst.ncols() {
                                {
                                    let (s1, s2) = schemas[col];
                                    PostgresArrowTransport::<PgBinaryProtocol, NoTls>::process(
                                        s1, s2, parser, dst,
                                    )
                                    .unwrap();
                                }
                            }
                        }
                        if is_last || processed_rows >= batch_size {
                            break;
                        }
                    },
                    DataOrder::ColumnMajor => loop {
                        let (mut n, is_last) = parser.fetch_next().unwrap();
                        n = std::cmp::min(n, batch_size - processed_rows);
                        processed_rows += n;
                        dst.aquire_row(n).unwrap();
                        #[allow(clippy::needless_range_loop)]
                        for col in 0..dst.ncols() {
                            for _ in 0..n {
                                {
                                    let (s1, s2) = schemas[col];
                                    PostgresArrowTransport::<PgBinaryProtocol, NoTls>::process(
                                        s1, s2, parser, dst,
                                    )
                                    .unwrap();
                                }
                            }
                        }
                        if is_last || processed_rows >= batch_size {
                            break;
                        }
                    },
                }

                debug!("Finalize partition {}", i);
                dst.finalize().unwrap();
                debug!("Partition {} finished", i);
            });
    }
}

impl Iterator for PostgresArrowBatchIter<'_> {
    type Item = RecordBatch;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.dst.record_batch().unwrap();
        if res.is_some() {
            return res;
        }
        self.run_batch();
        self.dst.record_batch().unwrap()
    }
}
