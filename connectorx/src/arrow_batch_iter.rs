use crate::prelude::*;
use arrow::record_batch::RecordBatch;
use itertools::Itertools;
use log::debug;
use rayon::prelude::*;
use std::marker::PhantomData;

pub fn set_global_num_thread(num: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num)
        .build_global()
        .unwrap();
}

/// The iterator that returns arrow in `RecordBatch`
pub struct ArrowBatchIter<S, TP>
where
    S: Source,
    TP: Transport<
        TSS = S::TypeSystem,
        TSD = ArrowStreamTypeSystem,
        S = S,
        D = ArrowStreamDestination,
    >,
    <S as Source>::Partition: 'static,
    <S as Source>::TypeSystem: 'static,
    <TP as Transport>::Error: 'static,
{
    dst: ArrowStreamDestination,
    dst_parts: Option<Vec<ArrowStreamPartitionWriter>>,
    src_parts: Option<Vec<S::Partition>>,
    dorder: DataOrder,
    src_schema: Vec<S::TypeSystem>,
    dst_schema: Vec<ArrowStreamTypeSystem>,
    _phantom: PhantomData<TP>,
}

impl<'a, S, TP> ArrowBatchIter<S, TP>
where
    S: Source + 'a,
    TP: Transport<
        TSS = S::TypeSystem,
        TSD = ArrowStreamTypeSystem,
        S = S,
        D = ArrowStreamDestination,
    >,
{
    pub fn new(
        src: S,
        mut dst: ArrowStreamDestination,
        origin_query: Option<String>,
        queries: &[CXQuery<String>],
    ) -> Result<Self, TP::Error> {
        let dispatcher = Dispatcher::<_, _, TP>::new(src, &mut dst, queries, origin_query);
        let (dorder, src_parts, dst_parts, src_schema, dst_schema) = dispatcher.prepare()?;

        Ok(Self {
            dst,
            dst_parts: Some(dst_parts),
            src_parts: Some(src_parts),
            dorder,
            src_schema,
            dst_schema,
            _phantom: PhantomData,
        })
    }

    fn run(&mut self) {
        let src_schema = self.src_schema.clone();
        let dst_schema = self.dst_schema.clone();
        let src_partitions = self.src_parts.take().unwrap();
        let dst_partitions = self.dst_parts.take().unwrap();
        let dorder = self.dorder;

        std::thread::spawn(move || -> Result<(), TP::Error> {
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
                .try_for_each(|(i, (mut dst, mut src))| -> Result<(), TP::Error> {
                    let mut parser = src.parser()?;

                    match dorder {
                        DataOrder::RowMajor => loop {
                            let (n, is_last) = parser.fetch_next()?;
                            dst.aquire_row(n)?;
                            for _ in 0..n {
                                #[allow(clippy::needless_range_loop)]
                                for col in 0..dst.ncols() {
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
        });
    }
}

impl<'a, S, TP> Iterator for ArrowBatchIter<S, TP>
where
    S: Source + 'a,
    TP: Transport<
        TSS = S::TypeSystem,
        TSD = ArrowStreamTypeSystem,
        S = S,
        D = ArrowStreamDestination,
    >,
{
    type Item = RecordBatch;
    /// NOTE: not thread safe
    fn next(&mut self) -> Option<Self::Item> {
        self.dst.record_batch().ok().flatten()
    }
}

pub trait RecordBatchIterator: Send {
    fn get_schema(&self) -> (RecordBatch, &[String]);
    fn prepare(&mut self);
    fn next_batch(&mut self) -> Option<RecordBatch>;
}

impl<'a, S, TP> RecordBatchIterator for ArrowBatchIter<S, TP>
where
    S: Source + 'a,
    TP: Transport<
            TSS = S::TypeSystem,
            TSD = ArrowStreamTypeSystem,
            S = S,
            D = ArrowStreamDestination,
        > + std::marker::Send,
{
    fn get_schema(&self) -> (RecordBatch, &[String]) {
        (self.dst.empty_batch(), self.dst.names())
    }

    fn prepare(&mut self) {
        self.run();
    }

    fn next_batch(&mut self) -> Option<RecordBatch> {
        self.next()
    }
}
