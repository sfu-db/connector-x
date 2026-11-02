use anyhow::anyhow;
use connectorx::arrow_batch_iter::ArrowBatchIter;
use connectorx::impl_transport;
use connectorx::prelude::*;
use connectorx::sources::dummy::DummyTypeSystem;
use connectorx::sources::{PartitionParser, Produce, Source, SourcePartition};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use std::time::Duration;

pub struct PanicArrowStreamTransport;

impl_transport!(
    name = PanicArrowStreamTransport,
    error = ConnectorXOutError,
    systems = DummyTypeSystem => ArrowStreamTypeSystem,
    route = DummySource => ArrowStreamDestination,
    mappings = {
        { I64[i64] => Int64[i64] | conversion none }
    }
);

impl TypeConversion<i64, i64> for PanicArrowStreamTransport {
    fn convert(val: i64) -> i64 {
        if val == 3 {
            panic!("intentional panic in stream producer");
        }
        val
    }
}

impl TypeConversion<Option<i64>, Option<i64>> for PanicArrowStreamTransport {
    fn convert(val: Option<i64>) -> Option<i64> {
        val.map(<Self as TypeConversion<i64, i64>>::convert)
    }
}

#[test]
fn producer_panic_propagates() {
    let schema = [DummyTypeSystem::I64(false)];
    let queries = [CXQuery::naked("10,1")];
    let dst = ArrowStreamDestination::new_with_batch_size(2);
    let mut iter = ArrowBatchIter::<_, PanicArrowStreamTransport>::new(
        DummySource::new(&["a"], &schema),
        dst,
        None,
        &queries,
    )
    .unwrap();
    iter.prepare();

    let first = iter.next().unwrap().unwrap();
    assert_eq!(first.num_rows(), 2);

    // Two panic messages are printed during this test, both expected:
    // 1. the producer thread panics intentionally, and
    // 2. resume_unwind re-raises it on the consumer thread.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| iter.next()));
    assert!(result.is_err(), "producer panic must propagate to consumer");
}

/// A single-column source of `i64` values `0, 1, 2, ...` per partition. A
/// partition's parser fails when asked for value `fail_at`, if set.
///
/// A parser given a `release` receiver also blocks in `Drop` until the test
/// releases it, which mimics a driver that drains the rest of the result set
/// on teardown (MySQL does this) and lets the test check that the error
/// reaches the consumer *before* that teardown starts.
pub struct FailingSource {
    partitions: Vec<FailingSourcePartition>,
}

impl FailingSource {
    fn new(nrows: usize, fail_at: usize, release: Receiver<()>, dropped: Arc<AtomicBool>) -> Self {
        Self {
            partitions: vec![FailingSourcePartition::new(
                nrows,
                Some(fail_at),
                Some(release),
                dropped,
            )],
        }
    }

    fn with_partitions(partitions: Vec<FailingSourcePartition>) -> Self {
        Self { partitions }
    }
}

impl Source for FailingSource {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type TypeSystem = DummyTypeSystem;
    type Partition = FailingSourcePartition;
    type Error = ConnectorXError;

    fn set_data_order(&mut self, _data_order: DataOrder) -> Result<(), ConnectorXError> {
        Ok(())
    }

    fn set_queries<Q: ToString>(&mut self, _queries: &[CXQuery<Q>]) {}

    fn set_origin_query(&mut self, _query: Option<String>) {}

    fn fetch_metadata(&mut self) -> Result<(), ConnectorXError> {
        Ok(())
    }

    fn result_rows(&mut self) -> Result<Option<usize>, ConnectorXError> {
        Ok(None)
    }

    fn names(&self) -> Vec<String> {
        vec!["a".to_string()]
    }

    fn schema(&self) -> Vec<DummyTypeSystem> {
        vec![DummyTypeSystem::I64(false)]
    }

    fn partition(self) -> Result<Vec<FailingSourcePartition>, ConnectorXError> {
        Ok(self.partitions)
    }
}

pub struct FailingSourcePartition {
    nrows: usize,
    fail_at: Option<usize>,
    counter: usize,
    release: Option<Receiver<()>>,
    dropped: Arc<AtomicBool>,
}

impl FailingSourcePartition {
    fn new(
        nrows: usize,
        fail_at: Option<usize>,
        release: Option<Receiver<()>>,
        dropped: Arc<AtomicBool>,
    ) -> Self {
        Self {
            nrows,
            fail_at,
            counter: 0,
            release,
            dropped,
        }
    }
}

impl SourcePartition for FailingSourcePartition {
    type TypeSystem = DummyTypeSystem;
    type Parser<'a> = FailingSourceParser<'a>;
    type Error = ConnectorXError;

    fn result_rows(&mut self) -> Result<(), ConnectorXError> {
        Ok(())
    }

    fn parser(&mut self) -> Result<FailingSourceParser<'_>, ConnectorXError> {
        Ok(FailingSourceParser { part: self })
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        1
    }
}

pub struct FailingSourceParser<'a> {
    part: &'a mut FailingSourcePartition,
}

impl Drop for FailingSourceParser<'_> {
    fn drop(&mut self) {
        if let Some(release) = self.part.release.take() {
            // Give up eventually so a regression fails the test instead of hanging it.
            let _ = release.recv_timeout(Duration::from_secs(5));
        }
        self.part.dropped.store(true, Ordering::SeqCst);
    }
}

impl<'a> PartitionParser<'a> for FailingSourceParser<'a> {
    type TypeSystem = DummyTypeSystem;
    type Error = ConnectorXError;

    fn fetch_next(&mut self) -> Result<(usize, bool), ConnectorXError> {
        Ok((self.part.nrows, true))
    }
}

impl<'r, 'a> Produce<'r, i64> for FailingSourceParser<'a> {
    type Error = ConnectorXError;

    fn produce(&'r mut self) -> Result<i64, ConnectorXError> {
        let val = self.part.counter;
        if Some(val) == self.part.fail_at {
            return Err(anyhow!("intentional error at value {}", val).into());
        }
        self.part.counter += 1;
        Ok(val as i64)
    }
}

impl<'r, 'a> Produce<'r, Option<i64>> for FailingSourceParser<'a> {
    type Error = ConnectorXError;

    fn produce(&'r mut self) -> Result<Option<i64>, ConnectorXError> {
        <Self as Produce<'r, i64>>::produce(self).map(Some)
    }
}

pub struct FailingArrowStreamTransport;

impl_transport!(
    name = FailingArrowStreamTransport,
    error = ConnectorXOutError,
    systems = DummyTypeSystem => ArrowStreamTypeSystem,
    route = FailingSource => ArrowStreamDestination,
    mappings = {
        { I64[i64] => Int64[i64] | conversion auto }
    }
);

#[test]
fn producer_error_is_reported_before_teardown() {
    let (release, release_rx) = channel();
    let dropped = Arc::new(AtomicBool::new(false));
    let src = FailingSource::new(10, 5, release_rx, Arc::clone(&dropped));
    let dst = ArrowStreamDestination::new_with_batch_size(2);
    let mut iter = ArrowBatchIter::<_, FailingArrowStreamTransport>::new(
        src,
        dst,
        None,
        &[CXQuery::naked("")],
    )
    .unwrap();
    iter.prepare();

    // Values 0..4 make two full batches before value 5 fails.
    assert_eq!(iter.next().unwrap().unwrap().num_rows(), 2);
    assert_eq!(iter.next().unwrap().unwrap().num_rows(), 2);

    let err = iter.next().unwrap().unwrap_err();
    assert!(
        err.to_string().contains("intentional error at value 5"),
        "unexpected error: {}",
        err
    );
    assert!(
        !dropped.load(Ordering::SeqCst),
        "the error must reach the consumer before the source parser is dropped"
    );

    release.send(()).unwrap();

    // Once an error has been reported the stream is exhausted.
    assert!(iter.next().is_none());
    assert!(iter.next().is_none());
}

#[test]
fn other_partitions_stop_after_an_error() {
    // Partition 0 fails at its second value. Partition 1 has far more rows
    // than it could produce during the test, so it only finishes if it stops
    // once the consumer has the error.
    let src = FailingSource::with_partitions(vec![
        FailingSourcePartition::new(10, Some(1), None, Arc::new(AtomicBool::new(false))),
        FailingSourcePartition::new(1 << 40, None, None, Arc::new(AtomicBool::new(false))),
    ]);
    let dst = ArrowStreamDestination::new_with_batch_size(2);
    let mut iter = ArrowBatchIter::<_, FailingArrowStreamTransport>::new(
        src,
        dst,
        None,
        &[CXQuery::naked(""), CXQuery::naked("")],
    )
    .unwrap();
    iter.prepare();

    // Partition 1 may deliver batches before partition 0's error.
    let err = loop {
        match iter.next().expect("stream ended without the error") {
            Ok(_) => continue,
            Err(e) => break e,
        }
    };
    assert!(
        err.to_string().contains("intentional error at value 1"),
        "unexpected error: {}",
        err
    );

    // The next call joins the producer, which returns only after partition 1
    // has stopped. Bound the wait so a regression fails instead of hanging.
    let (done, done_rx) = channel();
    std::thread::spawn(move || {
        let _ = done.send(iter.next().is_none());
    });
    let ended = done_rx
        .recv_timeout(Duration::from_secs(10))
        .expect("partition 1 kept running after the error");
    assert!(ended, "the stream must end after an error");
}
