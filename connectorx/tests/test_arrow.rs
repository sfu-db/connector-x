use arrow::array::{BooleanArray, Float64Array, Int32Array, Int64Array, LargeStringArray};
use arrow::record_batch::RecordBatch;
use connectorx::{
    destinations::arrow::ArrowDestination,
    prelude::*,
    sources::{
        dummy::DummySource,
        postgres::{connection, BinaryProtocol, PostgresSource},
    },
    sql::CXQuery,
    transports::{DummyArrowTransport, PostgresArrowTransport},
};
use postgres::NoTls;
use std::env;

#[test]
fn test_arrow() {
    let schema = [
        DummyTypeSystem::I64(true),
        DummyTypeSystem::F64(true),
        DummyTypeSystem::Bool(false),
        DummyTypeSystem::String(true),
        DummyTypeSystem::F64(false),
    ];
    let nrows = vec![4, 7];
    let ncols = schema.len();
    let queries: Vec<CXQuery> = nrows
        .iter()
        .map(|v| CXQuery::naked(format!("{},{}", v, ncols)))
        .collect();
    let mut destination = ArrowDestination::new();

    let dispatcher = Dispatcher::<_, _, DummyArrowTransport>::new(
        DummySource::new(&["a", "b", "c", "d", "e"], &schema),
        &mut destination,
        &queries,
    );
    dispatcher.run().expect("run dispatcher");

    let records: Vec<RecordBatch> = destination.finish().unwrap();
    assert_eq!(2, records.len());

    for col in 0..ncols {
        match col {
            0 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![0, 1, 2, 3])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![0, 1, 2, 3, 4, 5, 6])));
            }
            1 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![0.0, 1.0, 2.0, 3.0])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0])));
            }
            2 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .unwrap()
                    .eq(&BooleanArray::from(vec![true, false, true, false])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .unwrap()
                    .eq(&BooleanArray::from(vec![
                        true, false, true, false, true, false, true
                    ])));
            }
            3 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<LargeStringArray>()
                    .unwrap()
                    .eq(&LargeStringArray::from(vec!["0", "1", "2", "3"])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<LargeStringArray>()
                    .unwrap()
                    .eq(&LargeStringArray::from(vec![
                        "0", "1", "2", "3", "4", "5", "6"
                    ])));
            }
            4 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![0.0, 1.0, 2.0, 3.0])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0])));
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_postgres_arrow() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [
        CXQuery::naked("select * from test_table where test_int < 2"),
        CXQuery::naked("select * from test_table where test_int >= 2"),
    ];
    let (config, _tls) = connection::rewrite_tls_args(&dburl).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
    );

    dispatcher.run().expect("run dispatcher");

    let ncols = destination.schema().len();

    let records: Vec<RecordBatch> = destination.finish().unwrap();
    assert_eq!(2, records.len());

    for col in 0..ncols {
        match col {
            0 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Int32Array>()
                    .unwrap()
                    .eq(&Int32Array::from(vec![1, 0])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Int32Array>()
                    .unwrap()
                    .eq(&Int32Array::from(vec![2, 3, 4, 1314])));
            }
            1 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Int32Array>()
                    .unwrap()
                    .eq(&Int32Array::from(vec![3, 5])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Int32Array>()
                    .unwrap()
                    .eq(&Int32Array::from(vec![None, Some(7), Some(9), Some(2)])));
            }
            2 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<LargeStringArray>()
                    .unwrap()
                    .eq(&LargeStringArray::from(vec!["str1", "a"])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<LargeStringArray>()
                    .unwrap()
                    .eq(&LargeStringArray::from(vec![
                        Some("str2"),
                        Some("b"),
                        Some("c"),
                        None
                    ])));
            }
            3 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![None, Some(3.1)])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![2.2, 3., 7.8, -10.])));
            }
            4 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .unwrap()
                    .eq(&BooleanArray::from(vec![Some(true), None])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .unwrap()
                    .eq(&BooleanArray::from(vec![
                        Some(false),
                        Some(false),
                        None,
                        Some(true)
                    ])));
            }
            _ => unreachable!(),
        }
    }
    // assert_eq!(
    //     array![Some(1), Some(0), Some(2), Some(3), Some(4), Some(1314)],
    //     destination.column_view::<Option<i64>>(0).unwrap()
    // );
    // assert_eq!(
    //     array![Some(3), Some(5), None, Some(7), Some(9), Some(2)],
    //     destination.column_view::<Option<i64>>(1).unwrap()
    // );
    // assert_eq!(
    //     array![
    //         Some("str1".to_string()),
    //         Some("a".to_string()),
    //         Some("str2".to_string()),
    //         Some("b".to_string()),
    //         Some("c".to_string()),
    //         None
    //     ],
    //     destination.column_view::<Option<String>>(2).unwrap()
    // );

    // assert_eq!(
    //     array![
    //         None,
    //         Some(3.1 as f64),
    //         Some(2.2 as f64),
    //         Some(3 as f64),
    //         Some(7.8 as f64),
    //         Some(-10 as f64)
    //     ],
    //     destination.column_view::<Option<f64>>(3).unwrap()
    // );

    // assert_eq!(
    //     array![Some(true), None, Some(false), Some(false), None, Some(true)],
    //     destination.column_view::<Option<bool>>(4).unwrap()
    // );
}
