//! Internal MSSQL driver seam.
//!
//! ConnectorX can talk to SQL Server through either `tiberius` + `bb8-tiberius`
//! (the original implementation) or `mssql-tds` (Microsoft's own TDS
//! client), following the phased migration plan tracked against
//! sfu-db/connector-x#942.
//!
//! - When only one of `src_mssql_tiberius` / `src_mssql_tds` is enabled,
//!   [`active_driver`] is a compile-time constant: that's the only backend
//!   linked in, and there is no runtime switch.
//! - When both are enabled (this is what the Python bindings build with, as
//!   of Phase 3), both backends are linked into the same binary and
//!   [`active_driver`] reads a process-wide atomic that [`set_active_driver`]
//!   can flip at runtime. `mssql-tds` is the default; switching to
//!   `tiberius` is the opt-in.
//!
//! Switching the driver only affects [`super::MsSQLSource`]s constructed
//! *after* the switch — an in-flight source/partition/parser keeps using
//! whichever backend it was built with.

/// Which MSSQL wire-protocol driver ConnectorX uses for a given source.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MsSQLDriverKind {
    /// `tiberius` + `bb8-tiberius`.
    Tiberius,
    /// `mssql-tds`.
    MssqlTds,
}

impl MsSQLDriverKind {
    /// The name used on the Rust/Python opt-in surfaces (`"tiberius"` /
    /// `"mssql-tds"`).
    pub fn as_str(self) -> &'static str {
        match self {
            MsSQLDriverKind::Tiberius => "tiberius",
            MsSQLDriverKind::MssqlTds => "mssql-tds",
        }
    }

    /// Parses the opt-in name back into a [`MsSQLDriverKind`]. Returns
    /// `None` for anything other than `"tiberius"` / `"mssql-tds"`.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "tiberius" => Some(MsSQLDriverKind::Tiberius),
            "mssql-tds" => Some(MsSQLDriverKind::MssqlTds),
            _ => None,
        }
    }
}

/// Returns the MSSQL driver ConnectorX was compiled with.
#[cfg(all(feature = "src_mssql_tiberius", not(feature = "src_mssql_tds")))]
pub(crate) fn active_driver() -> MsSQLDriverKind {
    MsSQLDriverKind::Tiberius
}

/// Returns the MSSQL driver ConnectorX was compiled with.
#[cfg(all(feature = "src_mssql_tds", not(feature = "src_mssql_tiberius")))]
pub(crate) fn active_driver() -> MsSQLDriverKind {
    MsSQLDriverKind::MssqlTds
}

// Both backends linked in: a real runtime switch, defaulting to mssql-tds
// (Phase 3 of sfu-db/connector-x#942).
#[cfg(all(feature = "src_mssql_tiberius", feature = "src_mssql_tds"))]
mod runtime_switch {
    use super::MsSQLDriverKind;
    use std::sync::atomic::{AtomicU8, Ordering};

    const TIBERIUS: u8 = 0;
    const MSSQL_TDS: u8 = 1;

    static ACTIVE_DRIVER: AtomicU8 = AtomicU8::new(MSSQL_TDS);

    pub fn active_driver() -> MsSQLDriverKind {
        match ACTIVE_DRIVER.load(Ordering::SeqCst) {
            TIBERIUS => MsSQLDriverKind::Tiberius,
            _ => MsSQLDriverKind::MssqlTds,
        }
    }

    /// Switches the MSSQL driver used by `MsSQLSource`s constructed from now
    /// on. Only available when both `src_mssql_tiberius` and `src_mssql_tds`
    /// are compiled in.
    pub fn set_active_driver(kind: MsSQLDriverKind) {
        let v = match kind {
            MsSQLDriverKind::Tiberius => TIBERIUS,
            MsSQLDriverKind::MssqlTds => MSSQL_TDS,
        };
        ACTIVE_DRIVER.store(v, Ordering::SeqCst);
    }
}

#[cfg(all(feature = "src_mssql_tiberius", feature = "src_mssql_tds"))]
pub(crate) use runtime_switch::active_driver;
#[cfg(all(feature = "src_mssql_tiberius", feature = "src_mssql_tds"))]
pub use runtime_switch::set_active_driver;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(all(feature = "src_mssql_tiberius", not(feature = "src_mssql_tds")))]
    fn active_driver_is_tiberius() {
        assert_eq!(active_driver(), MsSQLDriverKind::Tiberius);
    }

    #[test]
    #[cfg(all(feature = "src_mssql_tds", not(feature = "src_mssql_tiberius")))]
    fn active_driver_is_mssql_tds() {
        assert_eq!(active_driver(), MsSQLDriverKind::MssqlTds);
    }

    #[test]
    #[cfg(all(feature = "src_mssql_tiberius", feature = "src_mssql_tds"))]
    fn runtime_switch_defaults_to_mssql_tds_and_round_trips() {
        assert_eq!(active_driver(), MsSQLDriverKind::MssqlTds);
        set_active_driver(MsSQLDriverKind::Tiberius);
        assert_eq!(active_driver(), MsSQLDriverKind::Tiberius);
        // restore the default so other tests in this process observe it
        set_active_driver(MsSQLDriverKind::MssqlTds);
        assert_eq!(active_driver(), MsSQLDriverKind::MssqlTds);
    }

    #[test]
    #[cfg(all(feature = "src_mssql_tiberius", feature = "src_mssql_tds"))]
    fn driver_kind_name_round_trips() {
        assert_eq!(
            MsSQLDriverKind::from_name("tiberius"),
            Some(MsSQLDriverKind::Tiberius)
        );
        assert_eq!(
            MsSQLDriverKind::from_name("mssql-tds"),
            Some(MsSQLDriverKind::MssqlTds)
        );
        assert_eq!(MsSQLDriverKind::from_name("nonsense"), None);
        assert_eq!(MsSQLDriverKind::Tiberius.as_str(), "tiberius");
        assert_eq!(MsSQLDriverKind::MssqlTds.as_str(), "mssql-tds");
    }
}
