#![feature(generic_associated_types)]
#![feature(log_syntax)]
#![allow(incomplete_features)]

#[doc(hidden)]
pub mod pg;
#[doc(hidden)]
pub mod s3;
pub mod typesystem;
#[macro_use]
pub mod macros;
pub mod data_order;
pub mod destinations;
pub mod dispatcher;
pub mod dummy_typesystem;
pub mod errors;
pub mod partition;
pub mod sources;
pub mod transports;

pub use crate::data_order::DataOrder;
pub use crate::destinations::{Consume, Destination, DestinationPartition};
pub use crate::dispatcher::Dispatcher;
pub use crate::dummy_typesystem::DummyTypeSystem;
pub use crate::errors::{ConnectorAgentError, Result};
pub use crate::sources::{PartitionParser, Source, SourcePartition};
pub use crate::typesystem::{
    ParameterizedFunc, ParameterizedOn, Realize, Transport, TypeAssoc, TypeConversion, TypeSystem,
};
