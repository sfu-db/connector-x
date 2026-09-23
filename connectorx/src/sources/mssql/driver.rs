//! Internal MSSQL driver seam.
//!
//! ConnectorX can talk to SQL Server through either `tiberius` + `bb8-tiberius`
//! (the original, default implementation) or `mssql-tds` (Microsoft's own TDS
//! client), following the phased migration plan tracked against
//! sfu-db/connector-x#942.
//!
//! Phase 2 (this change): add the `mssql-tds`-backed implementation behind
//! the opt-in `src_mssql_tds` Cargo feature, mutually exclusive with
//! `src_mssql_tiberius`. [`active_driver`] reflects whichever backend was
//! compiled in; there is no runtime switch yet (Phase 3 adds that, plus the
//! Python-facing module property).

/// Which MSSQL wire-protocol driver ConnectorX uses for a given source.
///
/// Exactly one of `Tiberius` or `MssqlTds` is compiled in today, selected by
/// the `src_mssql_tiberius` / `src_mssql_tds` Cargo features (a
/// `compile_error!` in this crate enforces that both cannot be enabled at
/// once). Phase 3 will turn this into a real runtime choice.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MsSQLDriverKind {
    /// `tiberius` + `bb8-tiberius`.
    Tiberius,
    /// `mssql-tds`.
    MssqlTds,
}

/// Returns the MSSQL driver ConnectorX was compiled with.
#[cfg(feature = "src_mssql_tiberius")]
pub(crate) fn active_driver() -> MsSQLDriverKind {
    MsSQLDriverKind::Tiberius
}

/// Returns the MSSQL driver ConnectorX was compiled with.
#[cfg(all(feature = "src_mssql_tds", not(feature = "src_mssql_tiberius")))]
pub(crate) fn active_driver() -> MsSQLDriverKind {
    MsSQLDriverKind::MssqlTds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "src_mssql_tiberius")]
    fn active_driver_is_tiberius() {
        assert_eq!(active_driver(), MsSQLDriverKind::Tiberius);
    }

    #[test]
    #[cfg(all(feature = "src_mssql_tds", not(feature = "src_mssql_tiberius")))]
    fn active_driver_is_mssql_tds() {
        assert_eq!(active_driver(), MsSQLDriverKind::MssqlTds);
    }
}
