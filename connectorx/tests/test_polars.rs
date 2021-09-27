use connectorx::{
    destinations::arrow::ArrowDestination,
    prelude::*,
    sources::{
        dummy::{DummySource, DummyTypeSystem},
        postgres::{rewrite_tls_args, BinaryProtocol, PostgresSource},
    },
    sql::CXQuery,
    transports::{DummyArrowTransport, PostgresArrowTransport},
};
use polars::{df, prelude::*};
use postgres::NoTls;
use std::env;
use url::Url;

#[test]
fn test_polars() {
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

    let df: DataFrame = destination.polars().unwrap();
    let expected = df!(
        "a" => &[0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 6],
        "b" => &[0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        "c" => &[true, false, true, false, true, false, true, false, true, false, true],
        "d" => &["0", "1", "2", "3", "0", "1", "2", "3", "4", "5", "6"],
        "e" => &[0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
    )
    .unwrap();

    assert!(df.frame_equal_missing(&expected));
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
    );

    dispatcher.run().expect("run dispatcher");

    let df: DataFrame = destination.polars().unwrap();
    let expected = df!(
        "a" => &[1, 0, 2, 3, 4, 1314],
        "b" => &[Some(3), Some(5), None, Some(7), Some(9), Some(2)],
        "c" => &[Some("str1"), Some("a"), Some("str2"), Some("b"), Some("c"), None],
        "d" => &[None, Some(3.1), Some(2.2), Some(3.), Some(7.8), Some(-10.)],
        "e" => &[Some(true), None, Some(false), Some(false), None, Some(true)]
    )
    .unwrap();

    assert!(df.frame_equal_missing(&expected));
}
