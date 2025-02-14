use connectorx::{
    constants::RECORD_BATCH_SIZE,
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

    println!("{}", df);

    assert!(df.equals_missing(&expected) || df.equals_missing(&expected2));
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
    let nrows = [RECORD_BATCH_SIZE * 2 - 1, RECORD_BATCH_SIZE * 2 + 10];
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
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        Some("select * from test_table".to_string()),
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

    // order of each batch is not guaranteed
    let expected2 = df!(
        "test_int" => &[2, 3, 4, 1314, 1, 0],
        "test_nullint" => &[None, Some(7), Some(9), Some(2), Some(3), Some(5)],
        "test_str" => &[Some("str2"), Some("b"), Some("c"), None, Some("str1"), Some("a")],
        "test_float" => &[Some(2.2), Some(3.), Some(7.8), Some(-10.), None, Some(3.1)],
        "test_bool" => &[Some(false), Some(false), None, Some(true), Some(true), None]
    )
    .unwrap();

    assert!(df.equals_missing(&expected) || df.equals_missing(&expected2));
}

#[test]
fn test_polars_name() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [CXQuery::naked("select test_name from test_types")];
    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        Some("select test_name from test_types".to_string()),
    );

    dispatcher.run().expect("run dispatcher");

    let s1 = "0";
    let s2 = "21";
    let s3 = "someName";
    let s4 = "101203203-1212323-22131235";

    let df: DataFrame = destination.polars().unwrap();
    let test_df: DataFrame = df!(
        "test_name" => &[Some(s1),Some(s2),Some(s3),Some(s4),None]
    )
    .unwrap();

    assert_eq!(df, test_df);
}

#[test]
fn test_polars_boolarray() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [CXQuery::naked("select test_boolarray from test_types")];
    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        Some("select test_boolarray from test_types".to_string()),
    );

    dispatcher.run().expect("run dispatcher");

    let s1 = Series::new(PlSmallStr::from("a"), [true, false]);
    let empty_vec: Vec<bool> = vec![];
    let s2 = Series::new(PlSmallStr::from("b"), empty_vec);
    let s3 = Series::new(PlSmallStr::from("c"), [true]);
    let s4 = Series::new(PlSmallStr::from("c"), [Some(true), Some(false), None]);

    let df: DataFrame = destination.polars().unwrap();
    let test_df: DataFrame = df!(
        "test_boolarray" => &[Some(s1),Some(s2),Some(s3),Some(s4),None]
    )
    .unwrap();

    assert_eq!(df, test_df);
}

#[test]
fn test_polars_utf8array() {
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
        Some("select test_varchararray from test_types".to_string()),
    );
    dispatcher.run().expect("run dispatcher");

    let df: DataFrame = destination.polars().unwrap();

    let s1 = Series::new(PlSmallStr::from("a"), ["str1", "str2"]);
    let s2 = Series::new(
        PlSmallStr::from("b"),
        [
            "0123456789",
            "abcdefghijklmnopqrstuvwxyz",
            "!@#$%^&*()_-+=~`:;<>?/",
        ],
    );
    let s3 = Series::new(PlSmallStr::from("c"), ["", "  "]);
    let s4 = Series::new(PlSmallStr::from("d"), [Some("👨‍🍳👨‍🍳👨‍🍳👨"), Some(""), None]);
    let test_df: DataFrame = df!(
        "test_varchararray" => &[Some(s1),Some(s2),Some(s3),Some(s4),None]
    )
    .unwrap();

    assert_eq!(df, test_df);
}

#[test]
fn test_polars_intarray() {
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
        Some("select test_i2array, test_i4array, test_i8array from test_types".to_string()),
    );
    dispatcher.run().expect("run dispatcher");

    let df: DataFrame = destination.polars().unwrap();

    let v1_s1 = Series::new(PlSmallStr::from("a"), [12i16]);
    let empty_vec: Vec<i16> = vec![];
    let v1_s2 = Series::new(PlSmallStr::from("b"), empty_vec);
    let v1_s3 = Series::new(PlSmallStr::from("c"), [-32768i16, 32767]);
    let v1_s4 = Series::new(PlSmallStr::from("d"), [Some(-1i16), Some(0), Some(1), None]);

    let v2_s1 = Series::new(PlSmallStr::from("a"), [-1i32]);
    let empty_vec: Vec<i32> = vec![];
    let v2_s2 = Series::new(PlSmallStr::from("b"), empty_vec);
    let v2_s3 = Series::new(PlSmallStr::from("c"), [-2147483648i32, 2147483647]);
    let v2_s4 = Series::new(
        PlSmallStr::from("d"),
        [Some(-1i32), Some(0), Some(1123), None],
    );

    let v3_s1 = Series::new(
        PlSmallStr::from("a"),
        [-9223372036854775808i64, 9223372036854775807],
    );
    let empty_vec: Vec<i64> = vec![];
    let v3_s2 = Series::new(PlSmallStr::from("b"), empty_vec);
    let v3_s3 = Series::new(PlSmallStr::from("c"), [0i64]);
    let v3_s4 = Series::new(PlSmallStr::from("d"), [Some(-1i64), Some(0), Some(1), None]);

    let test_df: DataFrame = df!(
        "test_i2array" => &[Some(v1_s1),Some(v1_s2),Some(v1_s3),Some(v1_s4),None],
        "test_i4array" =>  &[Some(v2_s1),Some(v2_s2),Some(v2_s3),Some(v2_s4),None],
        "test_i8array" =>  &[Some(v3_s1),Some(v3_s2),Some(v3_s3),Some(v3_s4),None],
    )
    .unwrap();

    assert_eq!(df, test_df);
}
