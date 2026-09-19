//! SQL Server source backed by `mssql-tds`.

mod errors;
mod tds_impl;
mod typesystem;

pub use self::errors::MsSQLSourceError;
pub(crate) use self::tds_impl::mssql_get_partition_range;
pub use self::tds_impl::{MsSQLSource, MsSQLSourceParser, MsSQLSourcePartition};
pub use self::typesystem::{FloatN, IntN, MsSQLTypeSystem};
