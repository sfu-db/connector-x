#![feature(specialization, generic_associated_types)]

pub mod data_sources;
mod errors;
pub mod pg;
pub mod s3;
mod types;
mod worker;
pub mod writers;

pub use errors::ConnectorAgentError;
use fehler::throws;
pub use types::DataType;
pub use worker::Worker;

pub struct Partition {
    col: String,
    min: i64,
    max: i64,
    num: u64,
}

#[throws(ConnectorAgentError)]
pub fn read_sql(sql: &str, conn: &str, partition: Partition) {
    // Start the BB8 connection pool
}
