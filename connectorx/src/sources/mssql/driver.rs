//! Internal MSSQL driver seam.
//!
//! ConnectorX currently talks to SQL Server exclusively through `tiberius` +
//! `bb8-tiberius`. This module introduces the seam that a future `mssql-tds`
//! backend will plug into (see the phased migration plan tracked against
//! sfu-db/connector-x#942), without changing any observable behavior today.
//!
//! Phase 0 (this change): define [`MsSQLDriverKind`] and wire an
//! [`active_driver`] accessor into the existing Tiberius path. Only
//! [`MsSQLDriverKind::Tiberius`] exists, so behavior is unchanged.
//!
//! Phase 2 will add an `mssql-tds`-backed implementation behind an opt-in
//! Cargo feature and extend this module with the actual connection/query
//! trait boundary once the second implementation's real shape is known.
//! Designing that trait now, against a single implementation with the
//! existing self-referential `OwningHandle`-based row iterator, would risk
//! locking in the wrong abstraction before `mssql-tds`'s row/result-set API
//! is exercised for real.

/// Which MSSQL wire-protocol driver ConnectorX uses for a given source.
///
/// Only `Tiberius` is implemented today. `MssqlTds` is reserved for Phase 2
/// and intentionally not constructible yet.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum MsSQLDriverKind {
    /// The current, sole implementation: `tiberius` + `bb8-tiberius`.
    Tiberius,
}

/// Returns the MSSQL driver ConnectorX will use.
///
/// Always returns [`MsSQLDriverKind::Tiberius`] until Phase 2 lands a second
/// implementation and Phase 3 makes the choice configurable.
pub(crate) fn active_driver() -> MsSQLDriverKind {
    MsSQLDriverKind::Tiberius
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_driver_defaults_to_tiberius() {
        assert_eq!(active_driver(), MsSQLDriverKind::Tiberius);
    }
}
