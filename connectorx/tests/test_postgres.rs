use connectorx::{
    destinations::memory::MemoryDestination,
    sources::{
        postgres::{Binary, PostgresSource, CSV},
        Produce, Source, SourcePartition,
    },
    sql::CXQuery,
    transports::PostgresMemoryTransport,
    Dispatcher,
};
use ndarray::array;
use std::env;

#[test]
fn load_and_parse() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();
    #[derive(Debug, PartialEq)]
    struct Row(i32, Option<i32>, Option<String>, Option<f64>, Option<bool>);

    let mut source = PostgresSource::<Binary>::new(&dburl, 1).unwrap();
    source.set_queries(&[CXQuery::naked("select * from test_table")]);
    source.fetch_metadata().unwrap();

    let mut partitions = source.partition().unwrap();
    assert!(partitions.len() == 1);
    let mut partition = partitions.remove(0);
    partition.prepare().expect("run query");

    assert_eq!(6, partition.nrows());
    assert_eq!(5, partition.ncols());

    let mut parser = partition.parser().unwrap();

    let mut rows: Vec<Row> = Vec::new();
    for _i in 0..6 {
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
    let builder = PostgresSource::new(&dburl, 2).unwrap();
    let mut destination = MemoryDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresMemoryTransport<Binary>>::new(
        builder,
        &mut destination,
        &queries,
    );

    dispatcher.run().expect("run dispatcher");
    assert_eq!(
        array![Some(1), Some(0), Some(2), Some(3), Some(4), Some(1314)],
        destination.column_view::<Option<i64>>(0).unwrap()
    );
    assert_eq!(
        array![Some(3), Some(5), None, Some(7), Some(9), Some(2)],
        destination.column_view::<Option<i64>>(1).unwrap()
    );
    assert_eq!(
        array![
            Some("str1".to_string()),
            Some("a".to_string()),
            Some("str2".to_string()),
            Some("b".to_string()),
            Some("c".to_string()),
            None
        ],
        destination.column_view::<Option<String>>(2).unwrap()
    );

    assert_eq!(
        array![
            None,
            Some(3.1 as f64),
            Some(2.2 as f64),
            Some(3 as f64),
            Some(7.8 as f64),
            Some(-10 as f64)
        ],
        destination.column_view::<Option<f64>>(3).unwrap()
    );

    assert_eq!(
        array![Some(true), None, Some(false), Some(false), None, Some(true)],
        destination.column_view::<Option<bool>>(4).unwrap()
    );
}

#[test]
fn test_postgres_agg() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [CXQuery::naked(
        "SELECT test_bool, SUM(test_float) FROM test_table GROUP BY test_bool",
    )];
    let builder = PostgresSource::new(&dburl, 1).unwrap();
    let mut destination = MemoryDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresMemoryTransport<Binary>>::new(
        builder,
        &mut destination,
        &queries,
    );

    dispatcher.run().expect("run dispatcher");
    assert_eq!(
        array![None, Some(false), Some(true)],
        destination.column_view::<Option<bool>>(0).unwrap()
    );
    assert_eq!(
        array![Some(10.9), Some(5.2), Some(-10.0)],
        destination.column_view::<Option<f64>>(1).unwrap()
    );
}

#[test]
fn load_and_parse_csv() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();
    #[derive(Debug, PartialEq)]
    struct Row(i32, Option<i32>, Option<String>, Option<f64>, Option<bool>);

    let mut source = PostgresSource::<CSV>::new(&dburl, 1).unwrap();
    source.set_queries(&[CXQuery::naked("select * from test_table")]);
    source.fetch_metadata().unwrap();

    let mut partitions = source.partition().unwrap();
    assert!(partitions.len() == 1);
    let mut partition = partitions.remove(0);
    partition.prepare().expect("run query");

    assert_eq!(6, partition.nrows());
    assert_eq!(5, partition.ncols());

    let mut parser = partition.parser().unwrap();

    let mut rows: Vec<Row> = Vec::new();
    for _i in 0..6 {
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
fn test_postgres_csv() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [
        CXQuery::naked("select * from test_table where test_int < 2"),
        CXQuery::naked("select * from test_table where test_int >= 2"),
    ];
    let builder = PostgresSource::<CSV>::new(&dburl, 2).unwrap();
    let mut dst = MemoryDestination::new();
    let dispatcher =
        Dispatcher::<_, _, PostgresMemoryTransport<CSV>>::new(builder, &mut dst, &queries);

    dispatcher.run().expect("run dispatcher");
    assert_eq!(
        array![Some(1), Some(0), Some(2), Some(3), Some(4), Some(1314)],
        dst.column_view::<Option<i64>>(0).unwrap()
    );
    assert_eq!(
        array![Some(3), Some(5), None, Some(7), Some(9), Some(2)],
        dst.column_view::<Option<i64>>(1).unwrap()
    );
    assert_eq!(
        array![
            Some("str1".to_string()),
            Some("a".to_string()),
            Some("str2".to_string()),
            Some("b".to_string()),
            Some("c".to_string()),
            None
        ],
        dst.column_view::<Option<String>>(2).unwrap()
    );

    assert_eq!(
        array![
            None,
            Some(3.1 as f64),
            Some(2.2 as f64),
            Some(3 as f64),
            Some(7.8 as f64),
            Some(-10 as f64)
        ],
        dst.column_view::<Option<f64>>(3).unwrap()
    );

    assert_eq!(
        array![Some(true), None, Some(false), Some(false), None, Some(true)],
        dst.column_view::<Option<bool>>(4).unwrap()
    );
}
