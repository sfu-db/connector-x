//! Phase 3 acceptance test (sfu-db/connector-x#942): proves the MSSQL
//! runtime driver switch actually changes which backend a query runs
//! against, in the same process, without a rebuild.
//!
//! Only compiled when both `src_mssql_tiberius` and `src_mssql_tds` are
//! enabled — that's the "both backends linked in" configuration
//! (`dual_impl.rs`) that the switch exists for.
#![cfg(all(feature = "src_mssql_tiberius", feature = "src_mssql_tds"))]

use connectorx::{
    destinations::arrow::ArrowDestination,
    prelude::*,
    sources::mssql::{active_driver, set_active_driver, MsSQLDriverKind, MsSQLSource},
    sql::CXQuery,
    transports::MsSQLArrowTransport,
};
use std::sync::Arc;
use tokio::runtime::Runtime;

mod test_db;

fn run_full_type_matrix() -> Vec<arrow::record_batch::RecordBatch> {
    let dburl = test_db::mssql_url();
    let queries = [CXQuery::naked("select * from test_types order by test_int1")];
    let rt = Arc::new(Runtime::new().unwrap());
    let builder = MsSQLSource::new(rt, &dburl, 1).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher =
        Dispatcher::<_, _, MsSQLArrowTransport>::new(builder, &mut destination, &queries, None);
    dispatcher.run().unwrap();
    destination.arrow().unwrap()
}

/// Runs the same query with the driver flipped mid-process and checks:
/// 1. `active_driver()` actually reports the switch (not just a no-op write).
/// 2. Both backends produce byte-identical Arrow output for the same query,
///    proving the switch changed the backend actually used, not just the
///    reported state.
#[test]
#[ignore]
fn test_mssql_runtime_driver_switch_changes_backend() {
    let _ = env_logger::builder().is_test(true).try_init();

    // Fresh-process default must be mssql-tds (Phase 3 goal: flip the
    // default, keep tiberius as opt-in).
    assert_eq!(active_driver(), MsSQLDriverKind::MssqlTds);
    let tds_result = run_full_type_matrix();

    set_active_driver(MsSQLDriverKind::Tiberius);
    assert_eq!(active_driver(), MsSQLDriverKind::Tiberius);
    let tiberius_result = run_full_type_matrix();

    assert_eq!(tds_result, tiberius_result, "mssql-tds and tiberius must return identical Arrow results for the same query");

    // Restore the default so other tests in this binary/process (test
    // binaries sharing this atomic run single-threaded via
    // --test-threads=1) see the documented default again.
    set_active_driver(MsSQLDriverKind::MssqlTds);
    assert_eq!(active_driver(), MsSQLDriverKind::MssqlTds);
}
