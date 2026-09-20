//! Source implementation for SQL Server.
//!
//! Two backend implementations exist, selected at compile time by mutually
//! exclusive Cargo features:
//! - `src_mssql_tiberius` (default): `tiberius` + `bb8-tiberius`.
//! - `src_mssql_tds`: `mssql-tds`.
//!
//! Both compile against the same [`typesystem::MsSQLTypeSystem`] and
//! [`errors::MsSQLSourceError`], so [`crate::transports::mssql_arrow`] and
//! everything above the source layer is unaffected by which backend is
//! active. See sfu-db/connector-x#942 for the migration plan.

#[cfg(all(feature = "src_mssql_tiberius", feature = "src_mssql_tds"))]
compile_error!(
    "features `src_mssql_tiberius` and `src_mssql_tds` are mutually exclusive; pick one MSSQL backend"
);

#[cfg(all(not(feature = "src_mssql_tiberius"), not(feature = "src_mssql_tds")))]
compile_error!("feature `src_mssql` requires either `src_mssql_tiberius` or `src_mssql_tds`");

mod driver;
mod errors;
mod typesystem;

#[cfg(feature = "src_mssql_tds")]
mod tds_impl;
#[cfg(feature = "src_mssql_tiberius")]
mod tiberius_impl;

pub use self::driver::MsSQLDriverKind;
pub use self::errors::MsSQLSourceError;
pub use self::typesystem::{FloatN, IntN, MsSQLTypeSystem};

#[cfg(feature = "src_mssql_tiberius")]
pub use self::tiberius_impl::{mssql_config, MsSQLSource, MsSQLSourceParser, MsSQLSourcePartition};

#[cfg(feature = "src_mssql_tds")]
pub use self::tds_impl::{MsSQLSource, MsSQLSourceParser, MsSQLSourcePartition};
