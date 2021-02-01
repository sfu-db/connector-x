use crate::errors::ConnectorAgentError;
use fehler::{throw, throws};
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DataOrder {
    RowMajor,
    ColumnMajor,
}

/// Given the supported data order from source and destination, decide the optimal data order
/// for producing and writing.
#[throws(ConnectorAgentError)]
pub fn coordinate(src: &[DataOrder], dst: &[DataOrder]) -> DataOrder {
    assert!(0 < src.len() && 0 < dst.len());

    match (src, dst) {
        ([s, ..], [d, ..]) if s == d => *s,
        ([s, ..], [_, d, ..]) if s == d => *s,
        ([_, s, ..], [d, ..]) if s == d => *s,
        _ => throw!(ConnectorAgentError::CannotResolveDataOrder(
            src.to_vec(),
            dst.to_vec()
        )),
    }
}
