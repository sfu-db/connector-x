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
pub mod errors;
pub mod sources;
#[doc(hidden)]
pub mod sql;
pub mod transports;
pub mod utils;

pub mod prelude {
    pub use crate::data_order::{coordinate, DataOrder};
    pub use crate::destinations::{Consume, Destination, DestinationPartition};
    pub use crate::dispatcher::Dispatcher;
    pub use crate::errors::ConnectorXError;
    pub use crate::sources::{PartitionParser, Produce, Source, SourcePartition};
    pub use crate::typesystem::{
        ParameterizedFunc, ParameterizedOn, Realize, Transport, TypeAssoc, TypeConversion,
        TypeSystem,
    };
}
