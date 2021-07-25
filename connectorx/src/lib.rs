#![feature(generic_associated_types)]
#![allow(incomplete_features)]
#![allow(clippy::upper_case_acronyms)]

pub mod typesystem;
#[macro_use]
pub mod macros;
pub(crate) mod constants;
pub mod data_order;
pub mod destinations;
pub mod dispatcher;
#[cfg(any(feature = "src_dummy", feature = "dst_memory", feature = "src_csv"))]
pub mod dummy_typesystem;
pub mod errors;
pub mod sources;
pub mod sql;
pub mod transports;

pub use crate::data_order::DataOrder;
pub use crate::destinations::{Consume, Destination, DestinationPartition};
pub use crate::dispatcher::Dispatcher;
#[cfg(any(feature = "src_dummy", feature = "dst_memory", feature = "src_csv"))]
pub use crate::dummy_typesystem::DummyTypeSystem;
pub use crate::errors::{ConnectorXError, Result};
pub use crate::sources::{PartitionParser, Source, SourcePartition};
pub use crate::typesystem::{
    ParameterizedFunc, ParameterizedOn, Realize, Transport, TypeAssoc, TypeConversion, TypeSystem,
};
