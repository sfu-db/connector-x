use arrow::{
    array::{
        BooleanArray, BooleanBuilder, Float64Array, Int64Array, Int64Builder, LargeListArray,
        LargeListBuilder, StringArray, StringBuilder,
    },
    record_batch::RecordBatch,
};
use connectorx::{
    constants::RECORD_BATCH_SIZE,
    destinations::arrow::{ArrowDestination, ArrowTypeSystem},
    prelude::*,
    sources::{
        dummy::{DummySource, DummyTypeSystem},
        postgres::{rewrite_tls_args, BinaryProtocol, PostgresSource},
    },
    sql::CXQuery,
    transports::{DummyArrowTransport, PostgresArrowTransport},
};
use postgres::NoTls;
use std::env;
use url::Url;

#[test]
#[should_panic]
fn arrow_destination_col_major() {
    let mut dw = ArrowDestination::new();
    dw.allocate(
        11,
        &["a", "b", "c"],
        &[
            ArrowTypeSystem::Int64(false),
            ArrowTypeSystem::Float64(true),
            ArrowTypeSystem::LargeUtf8(true),
        ],
        DataOrder::ColumnMajor,
    )
    .unwrap();
}

#[test]
fn test_arrow() {
    let schema = [
        DummyTypeSystem::I64(true),
        DummyTypeSystem::F64(true),
        DummyTypeSystem::Bool(false),
        DummyTypeSystem::String(true),
        DummyTypeSystem::F64(false),
    ];
    let nrows = [4, 7];
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
        None,
    );
    dispatcher.run().expect("run dispatcher");

    let records: Vec<RecordBatch> = destination.arrow().unwrap();
    assert_eq!(2, records.len());

    for r in records {
        match r.num_rows() {
            4 => {
                assert!(r
                    .column(0)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![0, 1, 2, 3])));

                assert!(r
                    .column(1)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![0.0, 1.0, 2.0, 3.0])));
                assert!(r
                    .column(2)
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .unwrap()
                    .eq(&BooleanArray::from(vec![true, false, true, false])));
                assert!(r
                    .column(3)
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .eq(&StringArray::from(vec!["0", "1", "2", "3"])));
                assert!(r
                    .column(4)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![0.0, 1.0, 2.0, 3.0])));
            }
            7 => {
                assert!(r
                    .column(0)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![0, 1, 2, 3, 4, 5, 6])));
                assert!(r
                    .column(1)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0])));

                assert!(r
                    .column(2)
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .unwrap()
                    .eq(&BooleanArray::from(vec![
                        true, false, true, false, true, false, true
                    ])));
                assert!(r
                    .column(3)
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .eq(&StringArray::from(vec!["0", "1", "2", "3", "4", "5", "6"])));
                assert!(r
                    .column(4)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0])));
            }
            _ => {
                println!("got a batch record with {} rows", r.num_rows());
                unreachable!();
            }
        }
    }
}

#[test]
fn test_arrow_large() {
    let schema = [
        DummyTypeSystem::I64(true),
        DummyTypeSystem::F64(true),
        DummyTypeSystem::Bool(false),
        DummyTypeSystem::String(true),
        DummyTypeSystem::F64(false),
    ];
    let nrows = [RECORD_BATCH_SIZE * 2 + 1, RECORD_BATCH_SIZE * 2 - 1];
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
        None,
    );
    dispatcher.run().expect("run dispatcher");

    let records: Vec<RecordBatch> = destination.arrow().unwrap();
    assert_eq!(5, records.len());
    let mut rsizes = vec![];
    for r in records {
        rsizes.push(r.num_rows());
    }
    rsizes.sort();
    assert_eq!(
        vec![
            1,
            RECORD_BATCH_SIZE - 1,
            RECORD_BATCH_SIZE,
            RECORD_BATCH_SIZE,
            RECORD_BATCH_SIZE
        ],
        rsizes
    );
}

#[test]
fn test_postgres_arrow() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [
        CXQuery::naked("select * from test_table where test_int < 2"),
        CXQuery::naked("select * from test_table where test_int >= 2"),
    ];
    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        Some("select * from test_table".to_string()),
    );
    dispatcher.run().expect("run dispatcher");

    let records: Vec<RecordBatch> = destination.arrow().unwrap();
    assert_eq!(2, records.len());

    for r in records {
        match r.num_rows() {
            2 => {
                assert!(r
                    .column(0)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![1, 0])));
                assert!(r
                    .column(1)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![3, 5])));
                assert!(r
                    .column(2)
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .eq(&StringArray::from(vec!["str1", "a"])));
                assert!(r
                    .column(3)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![None, Some(3.1)])));
                assert!(r
                    .column(4)
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .unwrap()
                    .eq(&BooleanArray::from(vec![Some(true), None])));
            }
            4 => {
                assert!(r
                    .column(0)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![2, 3, 4, 1314])));
                assert!(r
                    .column(1)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![None, Some(7), Some(9), Some(2)])));
                assert!(r
                    .column(2)
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .eq(&StringArray::from(vec![
                        Some("str2"),
                        Some("b"),
                        Some("c"),
                        None
                    ])));
                assert!(r
                    .column(3)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![2.2, 3., 7.8, -10.])));
                assert!(r
                    .column(4)
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
}

#[test]
fn test_postgres_boolarray_arrow() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [CXQuery::naked(
        "select test_boolarray from test_types where test_boolarray is not null",
    )];
    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        Some("select * from test_table".to_string()),
    );
    dispatcher.run().expect("run dispatcher");

    let records: Vec<RecordBatch> = destination.arrow().unwrap();
    assert_eq!(1, records.len());

    for r in records {
        let mut builder: LargeListBuilder<BooleanBuilder> =
            LargeListBuilder::with_capacity(BooleanBuilder::new(), 3);
        builder.append_value([Some(true), Some(false)]);
        builder.append_value([]);
        builder.append_value([Some(true)]);
        let val = builder.finish();
        assert!(r
            .column(0)
            .as_any()
            .downcast_ref::<LargeListArray>()
            .unwrap()
            .eq(&val));
    }
}

#[test]
fn test_postgres_varchararray_arrow() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [CXQuery::naked("select test_varchararray from test_types")];
    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        Some("select * from test_table".to_string()),
    );
    dispatcher.run().expect("run dispatcher");

    let records: Vec<RecordBatch> = destination.arrow().unwrap();
    assert_eq!(1, records.len());

    for r in records {
        let mut builder: LargeListBuilder<StringBuilder> =
            LargeListBuilder::with_capacity(StringBuilder::new(), 4);
        builder.append_value([Some("str1"), Some("str2")]);
        builder.append_value([
            Some("0123456789"),
            Some("abcdefghijklmnopqrstuvwxyz"),
            Some("!@#$%^&*()_-+=~`:;<>?/"),
        ]);
        builder.append_value([Some(""), Some("  ")]);
        builder.append_value::<Vec<Option<String>>, String>(vec![]);
        let val = builder.finish();

        assert!(r
            .column(0)
            .as_any()
            .downcast_ref::<LargeListArray>()
            .unwrap()
            .eq(&val));
    }
}

#[test]
fn test_postgres_textarray_arrow() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [CXQuery::naked("select test_textarray from test_types")];
    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        Some("select * from test_table".to_string()),
    );
    dispatcher.run().expect("run dispatcher");

    let records: Vec<RecordBatch> = destination.arrow().unwrap();
    assert_eq!(1, records.len());

    for r in records {
        let mut builder: LargeListBuilder<StringBuilder> =
            LargeListBuilder::with_capacity(StringBuilder::new(), 4);
        builder.append_value([Some("text1"), Some("text2")]);
        builder.append_value([
            Some("0123456789"),
            Some("abcdefghijklmnopqrstuvwxyz"),
            Some("!@#$%^&*()_-+=~`:;<>?/"),
        ]);
        builder.append_value([Some(""), Some("  ")]);
        builder.append_value::<Vec<Option<String>>, String>(vec![]);
        let val = builder.finish();

        assert!(r
            .column(0)
            .as_any()
            .downcast_ref::<LargeListArray>()
            .unwrap()
            .eq(&val));
    }
}

#[test]
fn test_postgres_intarray_arrow() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [CXQuery::naked(
        "select test_i2array, test_i4array, test_i8array from test_types",
    )];
    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        Some("select * from test_table".to_string()),
    );
    dispatcher.run().expect("run dispatcher");

    let records: Vec<RecordBatch> = destination.arrow().unwrap();
    assert_eq!(1, records.len());

    for r in records {
        let mut builder: LargeListBuilder<Int64Builder> =
            LargeListBuilder::with_capacity(Int64Builder::new(), 4);
        builder.append_value([Some(-1), Some(0), Some(1)]);
        builder.append_value::<Vec<Option<i64>>, i64>(vec![]);
        builder.append_value([Some(-32768), Some(32767)]);
        builder.append_null();
        let val = builder.finish();

        assert!(r
            .column(0)
            .as_any()
            .downcast_ref::<LargeListArray>()
            .unwrap()
            .eq(&val));

        let mut builder: LargeListBuilder<Int64Builder> =
            LargeListBuilder::with_capacity(Int64Builder::new(), 4);
        builder.append_value([Some(-1), Some(0), Some(1123)]);
        builder.append_value::<Vec<Option<i64>>, i64>(vec![]);
        builder.append_value([Some(-2147483648), Some(2147483647)]);
        builder.append_null();
        let val = builder.finish();

        assert!(r
            .column(1)
            .as_any()
            .downcast_ref::<LargeListArray>()
            .unwrap()
            .eq(&val));

        let mut builder: LargeListBuilder<Int64Builder> =
            LargeListBuilder::with_capacity(Int64Builder::new(), 2);
        builder.append_value([Some(-9223372036854775808), Some(9223372036854775807)]);
        builder.append_value::<Vec<Option<i64>>, i64>(vec![]);
        builder.append_value([Some(0)]);
        builder.append_null();
        let val = builder.finish();

        assert!(r
            .column(2)
            .as_any()
            .downcast_ref::<LargeListArray>()
            .unwrap()
            .eq(&val));
    }
}
