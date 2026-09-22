use arrow::{
    array::{BooleanArray, Float64Array, Int64Array, StringArray},
    record_batch::RecordBatch,
};
use connectorx::{
    destinations::arrow::ArrowDestination,
    partition::{get_col_range, partition, PartitionQuery},
    prelude::*,
    source_router::SourceConn,
    sources::mssql::MsSQLSource,
    sql::CXQuery,
    transports::MsSQLArrowTransport,
};
use std::convert::TryFrom;
use std::sync::Arc;
use tokio::runtime::Runtime;

mod test_db;

#[cfg(feature = "src_mssql_tds")]
mod tds_pool_tests {
    use super::*;
    use connectorx::constants::DB_BUFFER_SIZE;
    use connectorx::sources::mssql::MsSQLSourcePartition;
    use std::sync::{mpsc, Barrier};
    use std::time::Duration;
    use url::Url;

    fn connection_url(label: &str) -> (String, String) {
        let appname = format!("cx_pool_{}_{}", std::process::id(), label);
        let mut url = Url::parse(&test_db::mssql_url()).unwrap();
        url.query_pairs_mut().append_pair("appname", &appname);
        (url.to_string(), appname)
    }

    fn read_ids(partition: &mut MsSQLSourcePartition) -> Vec<i32> {
        let mut parser = partition.parser().unwrap();
        let mut ids = Vec::new();
        loop {
            let (rows, finished) = parser.fetch_next().unwrap();
            for _ in 0..rows {
                ids.push(parser.parse::<i32>().unwrap());
            }
            if finished {
                return ids;
            }
        }
    }

    fn sessions(appname: &str) -> Vec<i32> {
        let mut source =
            MsSQLSource::new(Arc::new(Runtime::new().unwrap()), &test_db::mssql_url(), 1).unwrap();
        source.set_queries(&[CXQuery::naked(format!(
            "SELECT CAST(session_id AS int) FROM sys.dm_exec_sessions \
             WHERE program_name = '{appname}' ORDER BY session_id"
        ))]);
        source.fetch_metadata().unwrap();
        read_ids(&mut source.partition().unwrap()[0])
    }

    #[test]
    fn reuses_connections_after_probes_partial_reads_and_errors() {
        let (url, appname) = connection_url("reuse");
        let mut source = MsSQLSource::new(Arc::new(Runtime::new().unwrap()), &url, 1).unwrap();
        let many_rows = "SELECT TOP (100) CAST(@@SPID AS int) AS spid FROM sys.all_objects";
        let queries = [
            CXQuery::naked(many_rows),
            CXQuery::naked("SELECT CAST(@@SPID AS int) AS spid; SELECT 123 AS extra_result"),
            CXQuery::naked("SELECT missing_column FROM sys.all_objects"),
            CXQuery::naked("SELECT CAST(@@SPID AS int) AS spid"),
        ];
        source.set_queries(&queries);
        source.fetch_metadata().unwrap();
        let ids = sessions(&appname);
        assert_eq!(ids.len(), 1);

        source.set_queries(&[queries[2].clone()]);
        assert!(source.fetch_metadata().is_err());
        source.set_queries(&queries);
        source.fetch_metadata().unwrap();
        source.set_origin_query(Some(many_rows.to_string()));
        assert_eq!(source.result_rows().unwrap(), Some(100));
        assert_eq!(sessions(&appname), ids);

        // Constructing more partitions than leases must not block.
        let mut partitions = source.partition().unwrap();
        partitions[0].result_rows().unwrap();
        assert_eq!(partitions[0].nrows(), 100);
        assert_eq!(sessions(&appname), ids);
        {
            let mut parser = partitions[0].parser().unwrap();
            assert_eq!(parser.fetch_next().unwrap(), (DB_BUFFER_SIZE, false));
            assert_eq!(parser.parse::<i32>().unwrap(), ids[0]);
            // Drop with both buffered and unread wire rows remaining.
        }
        drop(partitions[1].parser().unwrap()); // Unread, multiple result sets.
        assert!(partitions[2].parser().is_err());
        assert_eq!(read_ids(&mut partitions[3]), ids);
        assert_eq!(sessions(&appname), ids);
    }

    #[test]
    fn bounds_concurrent_leases_and_unblocks_waiting_partitions() {
        for nconn in [1, 2] {
            let (url, appname) = connection_url(&format!("bound_{nconn}"));
            let mut source =
                MsSQLSource::new(Arc::new(Runtime::new().unwrap()), &url, nconn).unwrap();
            let query = CXQuery::naked("SELECT CAST(@@SPID AS int) AS spid");
            source.set_queries(&vec![query; nconn + 1]);
            source.fetch_metadata().unwrap();
            let mut partitions = source.partition().unwrap();
            let mut waiting_partition = partitions.pop().unwrap();
            let held: Vec<_> = partitions
                .iter_mut()
                .map(|partition| partition.parser().unwrap())
                .collect();
            let ids = sessions(&appname);
            assert_eq!(ids.len(), nconn);

            let (tx, rx) = mpsc::channel();
            let ready = Arc::new(Barrier::new(2));
            let worker_ready = ready.clone();
            let worker = std::thread::spawn(move || {
                worker_ready.wait();
                tx.send(read_ids(&mut waiting_partition)).unwrap();
            });
            ready.wait();
            assert!(matches!(
                rx.recv_timeout(Duration::from_millis(200)),
                Err(mpsc::RecvTimeoutError::Timeout)
            ));
            assert_eq!(sessions(&appname), ids);
            drop(held);
            let reused = rx.recv_timeout(Duration::from_secs(10)).unwrap();
            assert_eq!(reused.len(), 1);
            assert!(ids.contains(&reused[0]));
            worker.join().unwrap();
        }
    }

    #[test]
    fn trailing_sql_errors_are_reported_and_next_query_succeeds() {
        let (url, _) = connection_url("trailing_error");
        let mut source = MsSQLSource::new(Arc::new(Runtime::new().unwrap()), &url, 1).unwrap();
        let error_query = CXQuery::naked(
            "SELECT CAST(@@SPID AS int) AS spid; RAISERROR ('pool_cleanup_error', 16, 1)",
        );
        source.set_queries(&[error_query.clone()]);
        assert!(source
            .fetch_metadata()
            .unwrap_err()
            .to_string()
            .contains("pool_cleanup_error"));
        source.set_queries(&[CXQuery::naked("SELECT CAST(@@SPID AS int) AS spid")]);
        source.fetch_metadata().unwrap();
        source.set_queries(&[
            error_query,
            CXQuery::naked("SELECT CAST(@@SPID AS int) AS spid"),
        ]);
        let mut partitions = source.partition().unwrap();
        {
            let mut parser = partitions[0].parser().unwrap();
            assert!(parser.fetch_next().is_err());
        }
        // Cleanup errors on an abandoned parser must also discard the lease.
        drop(partitions[0].parser().unwrap());
        assert_eq!(read_ids(&mut partitions[1]).len(), 1);
    }

    #[test]
    fn standalone_partition_reuses_and_resets_its_connection() {
        let (url, _) = connection_url("standalone");
        let rt = Arc::new(Runtime::new().unwrap());
        let mut source = MsSQLSource::new(rt.clone(), &url, 1).unwrap();
        source.set_queries(&[CXQuery::naked("SELECT CAST(@@DATEFIRST AS int)")]);
        source.fetch_metadata().unwrap();
        let schema = source.schema();
        let default = read_ids(&mut source.partition().unwrap()[0])[0];
        let changed = if default == 1 { 2 } else { 1 };
        let query = CXQuery::naked(format!(
            "SELECT CAST(@@DATEFIRST AS int); SET DATEFIRST {changed}"
        ));
        let mut partition =
            MsSQLSourcePartition::new(rt, Url::parse(&url).unwrap(), &query, &schema);
        assert_eq!(read_ids(&mut partition), [default]);
        assert_eq!(read_ids(&mut partition), [default]);
    }
}

#[test]
fn test_mssql_partition_ranges() {
    let dburl = test_db::mssql_url();
    let conn = SourceConn::try_from(dburl.as_str()).unwrap();
    for (ty, min, max) in [
        ("tinyint", "1", "255"),
        ("smallint", "-32768", "32767"),
        ("int", "-2147483648", "2147483647"),
        ("bigint", "-9007199254740993", "9007199254740993"),
        ("real", "-12.75", "23.5"),
        ("float", "-12.75", "23.5"),
    ] {
        let query = format!(
            "SELECT CAST(v AS {ty}) AS partition_key FROM (VALUES ({min}), ({max}), (NULL)) AS data(v)"
        );
        let expected = if ty == "real" || ty == "float" {
            (-12, 23)
        } else {
            (min.parse().unwrap(), max.parse().unwrap())
        };
        assert_eq!(
            get_col_range(&conn, &query, "partition_key").unwrap(),
            expected,
            "{ty}"
        );
        assert_eq!(
            get_col_range(&conn, &format!("{query} WHERE 1 = 0"), "partition_key").unwrap(),
            (0, 0),
            "{ty}: empty"
        );
        assert_eq!(
            get_col_range(
                &conn,
                &format!("SELECT CAST(NULL AS {ty}) AS partition_key"),
                "partition_key"
            )
            .unwrap(),
            (0, 0),
            "{ty}: NULL"
        );
    }
    for ty in ["varchar(10)", "decimal(10, 2)"] {
        for value in ["NULL", "1"] {
            let query = format!("SELECT CAST({value} AS {ty}) AS partition_key");
            let err = get_col_range(&conn, &query, "partition_key").unwrap_err();
            assert!(
                err.to_string()
                    .contains("Partition can only be done on int or float columns"),
                "{}",
                err
            );
        }
    }
    assert!(get_col_range(&conn, "SELECT 1 AS partition_key", "missing_column").is_err());
}

#[test]
fn test_mssql_automatic_partitioning() {
    let dburl = test_db::mssql_url();
    let conn = SourceConn::try_from(dburl.as_str()).unwrap();
    let query = "SELECT test_int FROM test_table";
    let queries = partition(
        &PartitionQuery::new(query, "test_int", None, None, 3),
        &conn,
    )
    .unwrap();
    assert_eq!(queries.len(), 3);
    let builder = MsSQLSource::new(Arc::new(Runtime::new().unwrap()), &dburl, 3).unwrap();
    let mut destination = ArrowDestination::new();
    Dispatcher::<_, _, MsSQLArrowTransport>::new(
        builder,
        &mut destination,
        &queries,
        Some(query.to_string()),
    )
    .run()
    .unwrap();
    let mut values = Vec::new();
    for batch in destination.arrow().unwrap() {
        let column = batch
            .column(0)
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();
        values.extend(column.iter().map(|v| v.unwrap()));
    }
    values.sort_unstable();
    assert_eq!(values, vec![0, 1, 2, 3, 4, 1314]);
}

#[test]
fn test_mssql() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = test_db::mssql_url();

    let queries = [
        CXQuery::naked("select * from test_table where test_int < 2"),
        CXQuery::naked("select * from test_table where test_int >= 2"),
    ];
    let rt = Arc::new(Runtime::new().unwrap());

    let builder = MsSQLSource::new(rt, &dburl, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher =
        Dispatcher::<_, _, MsSQLArrowTransport>::new(builder, &mut destination, &queries, None);
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();
    verify_arrow_results(result);
}

#[test]
fn test_mssql_agg() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = test_db::mssql_url();

    let queries = [CXQuery::naked(
        "SELECT test_bool, SUM(test_float) AS SUM FROM test_table GROUP BY test_bool",
    )];
    let rt = Arc::new(Runtime::new().unwrap());

    let builder = MsSQLSource::new(rt, &dburl, 1).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, MsSQLArrowTransport>::new(
        builder,
        &mut destination,
        &queries,
        Some(String::from(
            "SELECT test_bool, SUM(test_float) AS SUM FROM test_table GROUP BY test_bool",
        )),
    );
    dispatcher.run().unwrap();

    let mut result = destination.arrow().unwrap();
    assert!(result.len() == 1);
    let rb = result.pop().unwrap();
    assert!(rb.columns().len() == 2);

    assert!(rb
        .column(0)
        .as_any()
        .downcast_ref::<BooleanArray>()
        .unwrap()
        .eq(&BooleanArray::from(vec![None, Some(false), Some(true)])));

    assert!(rb
        .column(1)
        .as_any()
        .downcast_ref::<Float64Array>()
        .unwrap()
        .eq(&Float64Array::from(vec![
            Some(10.9),
            Some(5.2),
            Some(-10.0),
        ])));
}

pub fn verify_arrow_results(result: Vec<RecordBatch>) {
    assert!(result.len() == 2);

    for rb in result {
        assert!(rb.columns().len() == 5);
        match rb.num_rows() {
            2 => {
                assert!(rb
                    .column(0)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![1, 0])));

                assert!(rb
                    .column(1)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![Some(3), Some(5)])));

                assert!(rb
                    .column(2)
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .eq(&StringArray::from(vec![Some("str1"), Some("a"),])));

                assert!(rb
                    .column(3)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![None, Some(3.1_f64)])));

                assert!(rb
                    .column(4)
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .unwrap()
                    .eq(&BooleanArray::from(vec![Some(true), None])));
            }
            4 => {
                assert!(rb
                    .column(0)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![2, 3, 4, 1314])));

                assert!(rb
                    .column(1)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![None, Some(7), Some(9), Some(2)])));

                assert!(rb
                    .column(2)
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .eq(&StringArray::from(vec![
                        Some("str2"),
                        Some("b"),
                        Some("c"),
                        None,
                    ])));

                assert!(rb
                    .column(3)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![
                        Some(2.2_f64),
                        Some(3_f64),
                        Some(7.8_f64),
                        Some(-10_f64),
                    ])));

                assert!(rb
                    .column(4)
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .unwrap()
                    .eq(&BooleanArray::from(vec![
                        Some(false),
                        Some(false),
                        None,
                        Some(true),
                    ])));
            }
            _ => unreachable!(),
        }
    }
}
