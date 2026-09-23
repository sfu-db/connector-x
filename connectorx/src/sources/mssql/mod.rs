//! Source implementation for SQL Server.
//!
//! Two backend implementations exist:
//! - `src_mssql_tiberius`: `tiberius` + `bb8-tiberius`, the original
//!   implementation.
//! - `src_mssql_tds`: `mssql-tds`, Microsoft's own TDS client.
//!
//! Enabling exactly one of these two Cargo features picks that backend at
//! compile time, with no runtime switch (and no cost of linking the other
//! backend in) — this is what plain `cargo build`/`cargo test` on this crate
//! do. Enabling *both* (as the Python bindings do, so `cx.mssql_driver` can
//! flip backends at runtime) links both in and dispatches through the
//! [`dual_impl`] enum wrapper instead, selecting the active one via
//! [`driver::active_driver`] / [`driver::set_active_driver`].
//!
//! All three configurations compile against the same
//! [`typesystem::MsSQLTypeSystem`] and [`errors::MsSQLSourceError`], so
//! [`crate::transports::mssql_arrow`] and everything above the source layer
//! is unaffected by which backend(s) are active. See
//! sfu-db/connector-x#942 for the migration plan.

#[cfg(all(not(feature = "src_mssql_tiberius"), not(feature = "src_mssql_tds")))]
compile_error!("MSSQL source requires either `src_mssql_tiberius` or `src_mssql_tds`");

mod driver;
mod errors;
mod typesystem;

#[cfg(feature = "src_mssql_tds")]
mod tds_impl;
#[cfg(feature = "src_mssql_tiberius")]
mod tiberius_impl;
#[cfg(all(feature = "src_mssql_tiberius", feature = "src_mssql_tds"))]
mod dual_impl;

pub use self::driver::MsSQLDriverKind;
pub use self::errors::MsSQLSourceError;
pub use self::typesystem::{FloatN, IntN, MsSQLTypeSystem};

#[cfg(all(feature = "src_mssql_tiberius", feature = "src_mssql_tds"))]
pub use self::driver::set_active_driver;

/// Returns the MSSQL driver ConnectorX currently uses. Only meaningful (and
/// only compiled in) when both `src_mssql_tiberius` and `src_mssql_tds` are
/// enabled — otherwise there is exactly one backend and no switch to query.
#[cfg(all(feature = "src_mssql_tiberius", feature = "src_mssql_tds"))]
pub fn active_driver() -> MsSQLDriverKind {
    self::driver::active_driver()
}

#[cfg(all(feature = "src_mssql_tiberius", not(feature = "src_mssql_tds")))]
pub use self::tiberius_impl::{MsSQLSource, MsSQLSourceParser, MsSQLSourcePartition};
#[cfg(feature = "src_mssql_tiberius")]
pub use self::tiberius_impl::mssql_config;

#[cfg(all(feature = "src_mssql_tds", not(feature = "src_mssql_tiberius")))]
pub use self::tds_impl::{MsSQLSource, MsSQLSourceParser, MsSQLSourcePartition};

#[cfg(all(feature = "src_mssql_tiberius", feature = "src_mssql_tds"))]
pub use self::dual_impl::{MsSQLSource, MsSQLSourceParser, MsSQLSourcePartition};

#[cfg(feature = "src_mssql_tds")]
pub(crate) use self::tds_impl::tds_get_partition_range;
