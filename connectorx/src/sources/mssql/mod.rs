//! SQL Server source backed by `mssql-tds`.

mod errors;
mod tds_impl;
mod typesystem;

pub use self::errors::MsSQLSourceError;
pub(crate) use self::tds_impl::tds_get_partition_range;
pub use self::tds_impl::{
    set_user_agent_info, MsSQLSource, MsSQLSourceParser, MsSQLSourcePartition,
};
pub use self::typesystem::{FloatN, IntN, MsSQLTypeSystem};
