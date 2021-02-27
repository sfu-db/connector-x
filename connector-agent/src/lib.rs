#![feature(generic_associated_types)]
#![allow(incomplete_features)]

#[doc(hidden)]
pub mod pg;
#[doc(hidden)]
pub mod s3;
#[macro_use]
mod typesystem;
mod any_array;
mod data_order;
pub mod data_sources;
mod dispatcher;
mod errors;
mod transmit;
mod types;
pub mod writers;

pub use crate::any_array::{AnyArray, AnyArrayView, AnyArrayViewMut};
pub use crate::data_order::DataOrder;
pub use crate::data_sources::{
    csv::{CSVSource, CSVSourceParser, CSVSourcePartition},
    dummy::{MixedSource, MixedSourceParser, MixedSourcePartition},
    postgres::{PostgresSource, PostgresSourceParser, PostgresSourcePartition},
    {PartitionedSource, Source},
};
pub use crate::dispatcher::Dispatcher;
pub use crate::errors::{ConnectorAgentError, Result};
pub use crate::types::DataType;
pub use crate::typesystem::{ParameterizedFunc, ParameterizedOn, Realize, TypeAssoc, TypeSystem};
pub use crate::writers::{Consume, PartitionWriter, Writer};

// pub struct Partition {
//     col: String,
//     min: i64,
//     max: i64,
//     num: u64,
// }

// #[throws(ConnectorAgentError)]
// pub fn read_sql(sql: &str, conn: &str, partition: Partition) {
//     // Start the BB8 connection pool
// }
