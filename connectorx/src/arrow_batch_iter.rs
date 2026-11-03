use crate::errors::OutResult;
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
    handle: Option<std::thread::JoinHandle<()>>,
    _phantom: PhantomData<TP>,
}

impl<S, TP> ArrowBatchIter<S, TP>
where
    S: Source,
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
            handle: None,
            _phantom: PhantomData,
        })
    }

    fn run(&mut self)
    where
        ConnectorXOutError: From<TP::Error>,
    {
        let src_schema = self.src_schema.clone();
        let dst_schema = self.dst_schema.clone();
        let src_partitions = self.src_parts.take().unwrap();
        let dst_partitions = self.dst_parts.take().unwrap();
        let dorder = self.dorder;

        self.handle = Some(std::thread::spawn(move || {
            let schemas: Vec<_> = src_schema
                .iter()
                .zip_eq(&dst_schema)
                .map(|(&src_ty, &dst_ty)| (src_ty, dst_ty))
                .collect();

            debug!("Start writing");
            // parse and write
            //
            // Each partition reports its own error through the record batch
            // channel (see `ArrowPartitionWriter::send_error`), so the consumer
            // learns about it without joining this thread. `Err(())` only tells
            // rayon to stop scheduling the remaining partitions.
            let _ = dst_partitions
                .into_par_iter()
                .zip_eq(src_partitions)
                .enumerate()
                .try_for_each(|(i, (mut dst, mut src))| -> Result<(), ()> {
                    let mut parser = match src.parser() {
                        Ok(parser) => parser,
                        Err(e) => {
                            dst.send_error(TP::Error::from(e).into());
                            return Err(());
                        }
                    };

                    let result = (|| -> Result<(), TP::Error> {
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
                    })();

                    match result {
                        Ok(()) => Ok(()),
                        Err(e) => {
                            debug!("Partition {} failed: {:?}", i, e);
                            // `parser` is still alive here on purpose; see
                            // `ArrowPartitionWriter::send_error`.
                            dst.send_error(e.into());
                            Err(())
                        }
                    }
                });

            debug!("Writing finished");
        }));
    }
}

impl<S, TP> Iterator for ArrowBatchIter<S, TP>
where
    S: Source,
    TP: Transport<
        TSS = S::TypeSystem,
        TSD = ArrowStreamTypeSystem,
        S = S,
        D = ArrowStreamDestination,
    >,
    ConnectorXOutError: From<TP::Error>,
{
    type Item = OutResult<RecordBatch>;

    /// Yields the next batch, or the first error raised by a partition.
    ///
    /// The error is returned as soon as the failing partition reports it,
    /// without waiting for the producer to shut down. Calling `next` again
    /// after an error joins the producer and returns `None`. That call blocks
    /// until the other partitions have stopped at their next batch boundary
    /// and every source has been torn down, which for MySQL includes draining
    /// the rest of the result set.
    ///
    /// NOTE: not thread safe
    fn next(&mut self) -> Option<Self::Item> {
        match self.dst.record_batch() {
            Some(msg) => Some(msg),
            None => {
                // On stream end we must join the producer;
                // a detached handle would swallow producer panics
                if let Some(handle) = self.handle.take() {
                    if let Err(payload) = handle.join() {
                        std::panic::resume_unwind(payload);
                    }
                }
                None
            }
        }
    }
}

pub trait RecordBatchIterator: Send {
    fn get_schema(&self) -> (RecordBatch, &[String]);
    fn prepare(&mut self);
    /// See [`ArrowBatchIter::next`] for the error semantics.
    fn next_batch(&mut self) -> Option<OutResult<RecordBatch>>;
}

impl<S, TP> RecordBatchIterator for ArrowBatchIter<S, TP>
where
    S: Source,
    TP: Transport<
            TSS = S::TypeSystem,
            TSD = ArrowStreamTypeSystem,
            S = S,
            D = ArrowStreamDestination,
        > + std::marker::Send,
    ConnectorXOutError: From<TP::Error>,
{
    fn get_schema(&self) -> (RecordBatch, &[String]) {
        (self.dst.empty_batch(), self.dst.names())
    }

    fn prepare(&mut self) {
        self.run();
    }

    fn next_batch(&mut self) -> Option<OutResult<RecordBatch>> {
        self.next()
    }
}
