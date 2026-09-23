//! Phase 3 acceptance test (sfu-db/connector-x#942): proves the MSSQL
//! runtime driver switch actually changes which backend a query runs
//! against, in the same process, without a rebuild.
//!
//! Only compiled when both `src_mssql_tiberius` and `src_mssql_tds` are
//! enabled — that's the "both backends linked in" configuration
//! (`dual_impl.rs`) that the switch exists for.
#![cfg(all(
    feature = "src_mssql_tiberius",
    feature = "src_mssql_tds",
    feature = "dst_arrow"
))]

use connectorx::{
    destinations::arrow::ArrowDestination,
    partition::get_col_range,
    prelude::*,
    source_router::SourceConn,
    sources::mssql::{active_driver, set_active_driver, MsSQLDriverKind, MsSQLSource},
    sql::CXQuery,
    transports::MsSQLArrowTransport,
};
use std::convert::TryFrom;
use std::sync::Arc;
use tokio::runtime::Runtime;

mod test_db;

fn run_full_type_matrix(builder: MsSQLSource) -> Vec<arrow::record_batch::RecordBatch> {
    let queries = [CXQuery::naked(
        "select * from test_types order by test_int1",
    )];
    let mut destination = ArrowDestination::new();
    let dispatcher =
        Dispatcher::<_, _, MsSQLArrowTransport>::new(builder, &mut destination, &queries, None);
    dispatcher.run().unwrap();
    destination.arrow().unwrap()
}

/// Existing sources keep their backend after a switch, and both backends
/// produce identical Arrow output.
#[test]
fn test_mssql_runtime_driver_switch_changes_backend() {
    let _ = env_logger::builder().is_test(true).try_init();
    let dburl = test_db::mssql_url();
    let rt = Arc::new(Runtime::new().unwrap());

    // Fresh-process default must be mssql-tds (Phase 3 goal: flip the
    // default, keep tiberius as opt-in).
    assert_eq!(active_driver(), MsSQLDriverKind::MssqlTds);
    let tds_source = MsSQLSource::new(rt.clone(), &dburl, 1).unwrap();
    assert!(matches!(&tds_source, MsSQLSource::MssqlTds(_)));

    // TDS rejects custom CA validation before connecting. This distinguishes
    // its partition probe from Tiberius even when both drivers are compiled.
    let mut ca_url = url::Url::parse(&dburl).unwrap();
    ca_url.query_pairs_mut().append_pair(
        "trust_server_certificate_ca",
        "nonexistent-runtime-switch-ca.pem",
    );
    let ca_conn = SourceConn::try_from(ca_url.as_str()).unwrap();
    let err = get_col_range(&ca_conn, "SELECT 1 AS id", "id").unwrap_err();
    assert!(err.to_string().contains("not CA validation"), "{}", err);
    let conn = SourceConn::try_from(dburl.as_str()).unwrap();
    let range_query = "SELECT 1 AS id UNION ALL SELECT 3 AS id";
    assert_eq!(get_col_range(&conn, range_query, "id").unwrap(), (1, 3));

    set_active_driver(MsSQLDriverKind::Tiberius);
    assert_eq!(active_driver(), MsSQLDriverKind::Tiberius);
    let tiberius_source = MsSQLSource::new(rt, &dburl, 1).unwrap();
    assert!(matches!(&tiberius_source, MsSQLSource::Tiberius(_)));
    assert_eq!(get_col_range(&conn, range_query, "id").unwrap(), (1, 3));
    let tds_result = run_full_type_matrix(tds_source);
    set_active_driver(MsSQLDriverKind::MssqlTds);
    let tiberius_result = run_full_type_matrix(tiberius_source);

    assert_eq!(
        tds_result, tiberius_result,
        "mssql-tds and tiberius must return identical Arrow results for the same query"
    );

    // Restore the default so other tests in this binary/process (test
    // binaries sharing this atomic run single-threaded via
    // --test-threads=1) see the documented default again.
    set_active_driver(MsSQLDriverKind::MssqlTds);
    assert_eq!(active_driver(), MsSQLDriverKind::MssqlTds);
}
