#![feature(generic_associated_types)]
#![allow(incomplete_features)]

#[doc(hidden)]
pub mod pg;
#[doc(hidden)]
pub mod s3;
#[macro_use]
pub mod typesystem;
pub mod data_order;
pub mod data_sources;
pub mod dispatcher;
pub mod dummy_typesystem;
pub mod errors;
pub mod partition;
pub mod transport;
pub mod writers;

pub use crate::data_order::DataOrder;
pub use crate::data_sources::{PartitionedSource, Source};
pub use crate::dispatcher::Dispatcher;
pub use crate::dummy_typesystem::DummyTypeSystem;
pub use crate::errors::{ConnectorAgentError, Result};
pub use crate::typesystem::{
    ParameterizedFunc, ParameterizedOn, Realize, TypeAssoc, TypeConversion, TypeSystem,
};
pub use crate::writers::{Consume, PartitionWriter, Writer};
