use connectorx::{
    constants::RECORD_BATCH_SIZE,
    destinations::arrow2::Arrow2Destination,
    prelude::*,
    sources::{
        dummy::{DummySource, DummyTypeSystem},
        postgres::{rewrite_tls_args, BinaryProtocol, PostgresSource},
    },
    sql::CXQuery,
    transports::{DummyArrow2Transport, PostgresArrow2Transport},
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
    let mut destination = Arrow2Destination::new();

    let dispatcher = Dispatcher::<_, _, DummyArrow2Transport>::new(
        DummySource::new(&["a", "b", "c", "d", "e"], &schema),
        &mut destination,
        &queries,
        None,
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

    // order of each batch is not guaranteed
    let expected2 = df!(
        "a" => &[0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3],
        "b" => &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 0.0, 1.0, 2.0, 3.0],
        "c" => &[true, false, true, false, true, false, true, true, false, true, false],
        "d" => &["0", "1", "2", "3", "4", "5", "6", "0", "1", "2", "3"],
        "e" => &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 0.0, 1.0, 2.0, 3.0]
    )
    .unwrap();

    assert!(df.frame_equal_missing(&expected) || df.frame_equal_missing(&expected2));
}

#[test]
fn test_polars_large() {
    let schema = [
        DummyTypeSystem::I64(true),
        DummyTypeSystem::F64(true),
        DummyTypeSystem::Bool(false),
        DummyTypeSystem::String(true),
        DummyTypeSystem::F64(false),
    ];
    let nrows = vec![RECORD_BATCH_SIZE * 2 - 1, RECORD_BATCH_SIZE * 2 + 10];
    let ncols = schema.len();
    let queries: Vec<CXQuery> = nrows
        .iter()
        .map(|v| CXQuery::naked(format!("{},{}", v, ncols)))
        .collect();
    let mut destination = Arrow2Destination::new();

    let dispatcher = Dispatcher::<_, _, DummyArrow2Transport>::new(
        DummySource::new(&["a", "b", "c", "d", "e"], &schema),
        &mut destination,
        &queries,
        None,
    );
    dispatcher.run().expect("run dispatcher");

    let df: DataFrame = destination.polars().unwrap();
    assert_eq!(RECORD_BATCH_SIZE * 4 + 9, df.height());
    assert_eq!(5, df.width());
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
    let mut destination = Arrow2Destination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrow2Transport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        Some(format!("select * from test_table")),
    );

    dispatcher.run().expect("run dispatcher");

    let df: DataFrame = destination.polars().unwrap();

    let expected = df!(
        "test_int" => &[1, 0, 2, 3, 4, 1314],
        "test_nullint" => &[Some(3), Some(5), None, Some(7), Some(9), Some(2)],
        "test_str" => &[Some("str1"), Some("a"), Some("str2"), Some("b"), Some("c"), None],
        "test_float" => &[None, Some(3.1), Some(2.2), Some(3.), Some(7.8), Some(-10.)],
        "test_bool" => &[Some(true), None, Some(false), Some(false), None, Some(true)]
    )
    .unwrap();

    let expected2 = df!(
        "test_int" => &[2, 3, 4, 1314, 1, 0],
        "test_nullint" => &[None, Some(7), Some(9), Some(2), Some(3), Some(5)],
        "test_str" => &[Some("str2"), Some("b"), Some("c"), None, Some("str1"), Some("a")],
        "test_float" => &[Some(2.2), Some(3.), Some(7.8), Some(-10.), None, Some(3.1)],
        "test_bool" => &[Some(false), Some(false), None, Some(true), Some(true), None]
    )
    .unwrap();

    assert!(df.frame_equal_missing(&expected) || df.frame_equal_missing(&expected2));
}

#[test]
fn test_pg_pl_bool_array() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [CXQuery::naked(
        "select test_boolarray from test_types where test_boolarray is not null",
    )];
    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut destination = Arrow2Destination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrow2Transport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        Some(format!("select * from test_types")),
    );

    dispatcher.run().expect("run dispatcher");

    let s1 = Series::new("a", [true, false]);
    let empty_vec: Vec<bool> = vec![];
    let s2 = Series::new("b", empty_vec);
    let s3 = Series::new("c", [true]);

    let df: DataFrame = destination.polars().unwrap();
    let test_df: DataFrame = df!(
        "test_boolarray" => &[s1,s2,s3]
    )
    .unwrap();

    println!("{:?}", df);
    assert_eq!(df, test_df);
}

#[test]
fn test_pg_pl_varchar_array() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [CXQuery::naked("select test_varchararray from test_types")];
    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut destination = Arrow2Destination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrow2Transport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        Some(format!("select * from test_types")),
    );

    dispatcher.run().expect("run dispatcher");

    let s1 = Series::new("a", ["str1", "str2"]);
    let s2 = Series::new(
        "b",
        [
            "0123456789",
            "abcdefghijklmnopqrstuvwxyz",
            "!@#$%^&*()_-+=~`:;<>?/",
        ],
    );
    let s3 = Series::new("c", ["", "  "]);
    let empty_vec: Vec<&str> = vec![];
    let s4 = Series::new("d", empty_vec);

    let df: DataFrame = destination.polars().unwrap();
    let test_df: DataFrame = df!(
        "test_varchararray" => &[s1,s2,s3,s4]
    )
    .unwrap();

    println!("{:?}", df);
    // panic!("spurious");
    assert_eq!(df, test_df);
}

#[test]
fn test_pg_pl_text_array() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [CXQuery::naked("select test_textarray from test_types")];
    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut destination = Arrow2Destination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrow2Transport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        Some(format!("select * from test_types")),
    );

    dispatcher.run().expect("run dispatcher");

    let s1 = Series::new("a", ["text1", "text2"]);
    let s2 = Series::new(
        "b",
        [
            "0123456789",
            "abcdefghijklmnopqrstuvwxyz",
            "!@#$%^&*()_-+=~`:;<>?/",
        ],
    );
    let s3 = Series::new("c", ["", "  "]);
    let empty_vec: Vec<&str> = vec![];
    let s4 = Series::new("d", empty_vec);

    let df: DataFrame = destination.polars().unwrap();
    let test_df: DataFrame = df!(
        "test_textarray" => &[s1,s2,s3,s4]
    )
    .unwrap();

    println!("{:?}", df);
    assert_eq!(df, test_df);
}

#[test]

fn test_pg_pl_name() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [CXQuery::naked("select test_name from test_types")];
    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut destination = Arrow2Destination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrow2Transport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        Some(format!("select * from test_types")),
    );

    dispatcher.run().expect("run dispatcher");

    let s1 = "0";
    let s2 = "21";
    let s3 = "someName";
    let s4 = "101203203-1212323-22131235";

    let df: DataFrame = destination.polars().unwrap();
    let test_df: DataFrame = df!(
        "test_name" => &[s1,s2,s3,s4]
    )
    .unwrap();

    println!("{:?}", df);
    assert_eq!(df, test_df);
}
