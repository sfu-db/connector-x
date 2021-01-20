#[doc(hidden)]
pub mod pg;
#[doc(hidden)]
pub mod s3;
#[macro_use]
mod typesystem;
pub mod data_sources;
mod errors;
mod types;
mod worker;
pub mod writers;

pub use crate::data_sources::dummy::U64CounterSource;
pub use crate::errors::ConnectorAgentError;
pub use crate::types::DataType;
pub use crate::typesystem::{Transmit, TypeSystem};
pub use crate::worker::Worker;
pub use crate::writers::dummy::{U64PartitionWriter, U64Writer};
pub use crate::writers::{PartitionWriter, Writer};

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
