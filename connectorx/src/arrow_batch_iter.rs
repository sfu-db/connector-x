use crate::{
    data_order::DataOrder,
    destinations::{
        arrow::{ArrowDestination, ArrowPartitionWriter, ArrowTypeSystem},
        DestinationPartition,
    },
    dispatcher::Dispatcher,
    sources::{PartitionParser, Source, SourcePartition},
    sql::CXQuery,
    typesystem::Transport,
    utils::*,
};
use arrow::record_batch::RecordBatch;
use itertools::Itertools;
use log::debug;
use owning_ref::OwningHandle;
use rayon::prelude::*;
use std::marker::PhantomData;

type SourceParserHandle<'a, S> = OwningHandle<
    Box<<S as Source>::Partition>,
    DummyBox<<<S as Source>::Partition as SourcePartition>::Parser<'a>>,
>;

/// The iterator that returns arrow in `RecordBatch`
pub struct ArrowBatchIter<'a, S: 'a + Source, TP> {
    dst: ArrowDestination,
    dst_parts: Vec<ArrowPartitionWriter>,
    src_parsers: Vec<SourceParserHandle<'a, S>>,
    dorder: DataOrder,
    src_schema: Vec<S::TypeSystem>,
    dst_schema: Vec<ArrowTypeSystem>,
    batch_size: usize,
    _phantom: PhantomData<TP>,
}

impl<'a, S, TP> ArrowBatchIter<'a, S, TP>
where
    S: Source + 'a,
    TP: Transport<TSS = S::TypeSystem, TSD = ArrowTypeSystem, S = S, D = ArrowDestination>,
{
    pub fn new(
        src: S,
        mut dst: ArrowDestination,
        origin_query: Option<String>,
        queries: &[CXQuery<String>],
        batch_size: usize,
    ) -> Result<Self, TP::Error> {
        let dispatcher = Dispatcher::<_, _, TP>::new(src, &mut dst, queries, origin_query);
        let (dorder, src_parts, dst_parts, src_schema, dst_schema) = dispatcher.prepare()?;

        let src_parsers: Vec<_> = src_parts
            .into_iter()
            .map(|part| {
                OwningHandle::new_with_fn(Box::new(part), |part: *const S::Partition| unsafe {
                    DummyBox((&mut *(part as *mut S::Partition)).parser().unwrap())
                })
            })
            .collect();

        Ok(Self {
            dst,
            dst_parts,
            src_parsers,
            dorder,
            src_schema,
            dst_schema,
            batch_size,
            _phantom: PhantomData,
        })
    }

    fn run_batch(&mut self) -> Result<(), TP::Error> {
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
            .try_for_each(|(i, (dst, src))| -> Result<(), TP::Error> {
                let parser: &mut <S::Partition as crate::sources::SourcePartition>::Parser<'_> =
                    &mut *src;
                let mut processed_rows = 0;

                match dorder {
                    DataOrder::RowMajor => loop {
                        let (mut n, is_last) = parser.fetch_next()?;
                        n = std::cmp::min(n, batch_size - processed_rows); // only process until batch size is reached
                        processed_rows += n;
                        dst.aquire_row(n)?;
                        for _ in 0..n {
                            #[allow(clippy::needless_range_loop)]
                            for col in 0..dst.ncols() {
                                {
                                    let (s1, s2) = schemas[col];
                                    TP::process(s1, s2, parser, dst)?;
                                }
                            }
                        }
                        if is_last || processed_rows >= batch_size {
                            break;
                        }
                    },
                    DataOrder::ColumnMajor => loop {
                        let (mut n, is_last) = parser.fetch_next()?;
                        n = std::cmp::min(n, batch_size - processed_rows);
                        processed_rows += n;
                        dst.aquire_row(n)?;
                        #[allow(clippy::needless_range_loop)]
                        for col in 0..dst.ncols() {
                            for _ in 0..n {
                                {
                                    let (s1, s2) = schemas[col];
                                    TP::process(s1, s2, parser, dst)?;
                                }
                            }
                        }
                        if is_last || processed_rows >= batch_size {
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
    }
}

// impl<'a, S, TP> Iterator for ArrowBatchIter<'a, S, TP>
// where
//     S: Source + 'a,
//     TP: Transport<TSS = S::TypeSystem, TSD = ArrowTypeSystem, S = S, D = ArrowDestination>,
// {
//     type Item = RecordBatch;
//     fn next(&mut self) -> Option<Self::Item> {
//         let res = self.dst.record_batch().unwrap();
//         if res.is_some() {
//             return res;
//         }
//         self.run_batch().unwrap();
//         self.dst.record_batch().unwrap()
//     }
// }

impl<'a, S, TP> Iterator for ArrowBatchIter<'a, S, TP>
where
    S: Source + 'a,
    TP: Transport<TSS = S::TypeSystem, TSD = ArrowTypeSystem, S = S, D = ArrowDestination>,
{
    type Item = Result<RecordBatch, TP::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.dst.record_batch() {
            Ok(Some(res)) => return Some(Ok(res)),
            Ok(None) => {}
            Err(e) => return Some(Err(e.into())),
        }

        match self.run_batch() {
            Err(e) => return Some(Err(e)),
            Ok(()) => {}
        }

        match self.dst.record_batch() {
            Err(e) => Some(Err(e.into())),
            Ok(Some(res)) => Some(Ok(res)),
            Ok(None) => None,
        }
    }
}
