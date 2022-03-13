//! This module provides two data orders: row-wise and column-wise for tabular data,
//! as well as a function to coordinate the data order between source and destination.

use crate::errors::ConnectorXError;
use fehler::{throw, throws};
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DataOrder {
    RowMajor,
    ColumnMajor,
}

/// Given the supported data order from source and destination, decide the optimal data order
/// for producing and writing.
#[throws(ConnectorXError)]
pub fn coordinate(src: &[DataOrder], dst: &[DataOrder]) -> DataOrder {
    assert!(!src.is_empty() && !dst.is_empty());

    match (src, dst) {
        ([s, ..], [d, ..]) if s == d => *s,
        ([s, ..], [_, d, ..]) if s == d => *s,
        ([_, s, ..], [d, ..]) if s == d => *s,
        _ => throw!(ConnectorXError::CannotResolveDataOrder(
            src.to_vec(),
            dst.to_vec()
        )),
    }
}
