use arrow::{
    array::{BooleanArray, Float64Array, Int64Array, StringArray},
    record_batch::RecordBatch,
};
use connectorx::{
    destinations::arrow::ArrowDestination,
    prelude::*,
    sources::postgres::{rewrite_tls_args, BinaryProtocol, CSVProtocol, PostgresSource},
    sources::PartitionParser,
    sql::CXQuery,
    transports::PostgresArrowTransport,
};
use postgres::NoTls;
use std::env;
use url::Url;

#[test]
fn load_and_parse() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();
    #[derive(Debug, PartialEq)]
    struct Row(i32, Option<i32>, Option<String>, Option<f64>, Option<bool>);

    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let mut source = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 1).unwrap();
    source.set_queries(&[CXQuery::naked("select * from test_table")]);
    source.fetch_metadata().unwrap();

    let mut partitions = source.partition().unwrap();
    assert!(partitions.len() == 1);
    let mut partition = partitions.remove(0);
    partition.result_rows().expect("run query");

    let mut parser = partition.parser().unwrap();

    let mut rows: Vec<Row> = Vec::new();
    loop {
        let (n, is_last) = parser.fetch_next().unwrap();
        for _i in 0..n {
            rows.push(Row(
                parser.produce().unwrap(),
                parser.produce().unwrap(),
                Produce::<Option<&str>>::produce(&mut parser)
                    .unwrap()
                    .map(ToString::to_string),
                parser.produce().unwrap(),
                parser.produce().unwrap(),
            ));
        }
        if is_last {
            break;
        }
    }

    assert_eq!(
        vec![
            Row(1, Some(3), Some("str1".into()), None, Some(true)),
            Row(2, None, Some("str2".into()), Some(2.2), Some(false)),
            Row(0, Some(5), Some("a".into()), Some(3.1), None),
            Row(3, Some(7), Some("b".into()), Some(3.), Some(false)),
            Row(4, Some(9), Some("c".into()), Some(7.8), None),
            Row(1314, Some(2), None, Some(-10.), Some(true)),
        ],
        rows
    );
}

#[test]
fn load_and_parse_csv() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();
    #[derive(Debug, PartialEq)]
    struct Row(i32, Option<i32>, Option<String>, Option<f64>, Option<bool>);

    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let mut source = PostgresSource::<CSVProtocol, NoTls>::new(config, NoTls, 1).unwrap();
    source.set_queries(&[CXQuery::naked("select * from test_table")]);
    source.fetch_metadata().unwrap();

    let mut partitions = source.partition().unwrap();
    assert!(partitions.len() == 1);
    let mut partition = partitions.remove(0);
    partition.result_rows().expect("run query");

    assert_eq!(6, partition.nrows());
    assert_eq!(5, partition.ncols());

    let mut parser = partition.parser().unwrap();

    let mut rows: Vec<Row> = Vec::new();
    loop {
        let (n, is_last) = parser.fetch_next().unwrap();
        for _i in 0..n {
            rows.push(Row(
                parser.produce().unwrap(),
                parser.produce().unwrap(),
                Produce::<Option<&str>>::produce(&mut parser)
                    .unwrap()
                    .map(ToString::to_string),
                parser.produce().unwrap(),
                parser.produce().unwrap(),
            ));
        }
        if is_last {
            break;
        }
    }
    assert_eq!(
        vec![
            Row(1, Some(3), Some("str1".into()), None, Some(true)),
            Row(2, None, Some("str2".into()), Some(2.2), Some(false)),
            Row(0, Some(5), Some("a".into()), Some(3.1), None),
            Row(3, Some(7), Some("b".into()), Some(3.), Some(false)),
            Row(4, Some(9), Some("c".into()), Some(7.8), None),
            Row(1314, Some(2), None, Some(-10.), Some(true)),
        ],
        rows
    );
}

#[test]
fn test_postgres() {
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
        Some(String::from("select * from test_table")),
    );

    dispatcher.run().expect("run dispatcher");

    let result = destination.arrow().unwrap();
    verify_arrow_results(result);
}

#[test]
fn test_postgres_csv() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [
        CXQuery::naked("select * from test_table where test_int < 2"),
        CXQuery::naked("select * from test_table where test_int >= 2"),
    ];
    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<CSVProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut dst = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<CSVProtocol, NoTls>>::new(
        builder, &mut dst, &queries, None,
    );

    dispatcher.run().expect("run dispatcher");
    let result = dst.arrow().unwrap();
    verify_arrow_results(result);
}

#[test]
fn test_postgres_agg() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [CXQuery::naked(
        "SELECT test_bool, SUM(test_float) FROM test_table GROUP BY test_bool",
    )];

    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 1).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        Some("SELECT test_bool, SUM(test_float) FROM test_table GROUP BY test_bool".to_string()),
    );

    dispatcher.run().expect("run dispatcher");

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

    for r in result {
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
