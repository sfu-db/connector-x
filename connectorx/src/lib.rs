#![feature(generic_associated_types)]
#![allow(incomplete_features)]
#![allow(clippy::upper_case_acronyms)]

pub mod typesystem;
#[macro_use]
mod macros;
mod constants;
pub mod data_order;
pub mod destinations;
mod dispatcher;
#[cfg(any(feature = "src_dummy", feature = "dst_memory", feature = "src_csv"))]
mod dummy_typesystem;
pub mod errors;
pub mod sources;
#[doc(hidden)]
pub mod sql;
pub mod transports;

pub mod prelude {
    pub use crate::data_order::{coordinate, DataOrder};
    pub use crate::destinations::{Consume, Destination, DestinationPartition};
    pub use crate::dispatcher::Dispatcher;
    #[cfg(any(feature = "src_dummy", feature = "dst_memory", feature = "src_csv"))]
    pub use crate::dummy_typesystem::DummyTypeSystem;
    pub use crate::errors::ConnectorXError;
    pub use crate::sources::{PartitionParser, Produce, Source, SourcePartition};
    pub use crate::typesystem::{
        ParameterizedFunc, ParameterizedOn, Realize, Transport, TypeAssoc, TypeConversion,
        TypeSystem,
    };
}
