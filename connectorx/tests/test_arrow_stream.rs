use connectorx::arrow_batch_iter::ArrowBatchIter;
use connectorx::destinations::arrowstream::ArrowDestinationError;
use connectorx::impl_transport;
use connectorx::prelude::*;
use connectorx::sources::dummy::DummyTypeSystem;

#[derive(Debug, thiserror::Error)]
pub enum PanicArrowStreamTransportError {
    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),
    #[error(transparent)]
    ConnectorX(#[from] ConnectorXError),
}

pub struct PanicArrowStreamTransport;

impl_transport!(
    name = PanicArrowStreamTransport,
    error = PanicArrowStreamTransportError,
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
        val.map(|v| <Self as TypeConversion<i64, i64>>::convert(v))
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

    let first = iter.next().unwrap();
    assert_eq!(first.num_rows(), 2);

    // Two panic messages are printed during this test, both expected:
    // 1. the producer thread panics intentionally, and
    // 2. resume_unwind re-raises it on the consumer thread.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| iter.next()));
    assert!(result.is_err(), "producer panic must propagate to consumer");
}
