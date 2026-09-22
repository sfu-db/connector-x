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
