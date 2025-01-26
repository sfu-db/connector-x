use arrow::{
    array::{
        BooleanArray, Date32Array, Float32Array, Float64Array, Int16Array, Int32Array, Int64Array,
        LargeBinaryArray, LargeListArray, StringArray, Time64MicrosecondArray,
        TimestampMicrosecondArray,
    },
    datatypes::Float32Type,
    record_batch::RecordBatch,
};
use chrono::naive::NaiveDate;
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

// #[test]
// fn test_postgres_csv() {
//     let _ = env_logger::builder().is_test(true).try_init();

//     let dburl = env::var("POSTGRES_URL").unwrap();

//     let queries = [
//         CXQuery::naked("select * from test_table where test_int < 2"),
//         CXQuery::naked("select * from test_table where test_int >= 2"),
//     ];
//     let url = Url::parse(dburl.as_str()).unwrap();
//     let (config, _tls) = rewrite_tls_args(&url).unwrap();
//     let builder = PostgresSource::<CSVProtocol, NoTls>::new(config, NoTls, 2).unwrap();
//     let mut dst = ArrowDestination::new();
//     let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<CSVProtocol, NoTls>>::new(
//         builder, &mut dst, &queries, None,
//     );

//     dispatcher.run().expect("run dispatcher");
//     let result = dst.arrow().unwrap();
//     println!("{:?}", result);
//     verify_arrow_results(result);
// }

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
                    .downcast_ref::<Int32Array>()
                    .unwrap()
                    .eq(&Int32Array::from(vec![1, 0])));
                assert!(r
                    .column(1)
                    .as_any()
                    .downcast_ref::<Int32Array>()
                    .unwrap()
                    .eq(&Int32Array::from(vec![3, 5])));
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
                    .downcast_ref::<Int32Array>()
                    .unwrap()
                    .eq(&Int32Array::from(vec![2, 3, 4, 1314])));
                assert!(r
                    .column(1)
                    .as_any()
                    .downcast_ref::<Int32Array>()
                    .unwrap()
                    .eq(&Int32Array::from(vec![None, Some(7), Some(9), Some(2)])));
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
fn test_types_postgres() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let vars = vec![
        "test_bool",
        "test_date",
        "test_timestamp",
        "test_timestamptz",
        "test_int2",
        "test_int4",
        "test_int8",
        "test_float4",
        "test_float8",
        "test_numeric",
        "test_bpchar",
        "test_char",
        "test_varchar",
        "test_uuid",
        "test_time",
        "test_json",
        "test_jsonb",
        "test_bytea",
        "test_enum",
        "test_f4array",
        "test_f8array",
        "test_narray",
        "test_boolarray",
        "test_i2array",
        "test_i4array",
        "test_i8array",
        "test_citext",
        "test_ltree",
        "test_lquery",
        "test_ltxtquery",
        "test_varchararray",
        "test_textarray",
        "test_name",
    ]
    .join(",");

    let queries = [CXQuery::naked(format!("select {vars} from test_types"))];
    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        Some(String::from("select * from test_types")),
    );

    dispatcher.run().expect("run dispatcher");

    let result = destination.arrow().unwrap();
    verify_arrow_type_results(result);
}

// #[test]
// fn test_types_postgres_csv() {
//     let _ = env_logger::builder().is_test(true).try_init();

//     let dburl = env::var("POSTGRES_URL").unwrap();

//     let queries = [
//         CXQuery::naked("select * from test_table where test_int < 2"),
//         CXQuery::naked("select * from test_table where test_int >= 2"),
//     ];
//     let url = Url::parse(dburl.as_str()).unwrap();
//     let (config, _tls) = rewrite_tls_args(&url).unwrap();
//     let builder = PostgresSource::<CSVProtocol, NoTls>::new(config, NoTls, 2).unwrap();
//     let mut dst = ArrowDestination::new();
//     let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<CSVProtocol, NoTls>>::new(
//         builder, &mut dst, &queries, None,
//     );

//     dispatcher.run().expect("run dispatcher");
//     let result = dst.arrow().unwrap();
//     println!("{:?}", result);
//     verify_arrow_results(result);
// }

pub fn verify_arrow_type_results(result: Vec<RecordBatch>) {
    assert!(result.len() == 1);

    let mut col = 0;

    // test_bool
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<BooleanArray>()
        .unwrap()
        .eq(&BooleanArray::from(vec![
            Some(true),
            Some(true),
            Some(false),
            Some(false),
            None,
        ])));

    fn time_to_arrow(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        tz: i64,
    ) -> i64 {
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, min, sec)
            .unwrap()
            .and_utc()
            .timestamp()
            + (tz * 60 * 60)
    }

    // test_date
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<Date32Array>()
        .unwrap()
        .eq(&Date32Array::from(vec![
            Some(time_to_arrow(1970, 1, 1, 0, 0, 0, 0) as i32 / 86_400),
            Some(time_to_arrow(2000, 2, 28, 0, 0, 0, 0) as i32 / 86_400),
            Some(time_to_arrow(2038, 1, 18, 0, 0, 0, 0) as i32 / 86_400),
            Some(time_to_arrow(1901, 12, 14, 0, 0, 0, 0) as i32 / 86_400),
            None,
        ])));

    // test_timestamp
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<TimestampMicrosecondArray>()
        .unwrap()
        .eq(&TimestampMicrosecondArray::from(vec![
            Some(time_to_arrow(1970, 1, 1, 0, 0, 1, 0) * 1_000_000),
            Some(time_to_arrow(2000, 2, 28, 12, 0, 10, 0) * 1_000_000),
            Some(time_to_arrow(2038, 1, 18, 23, 59, 59, 0) * 1_000_000),
            Some(time_to_arrow(1901, 12, 14, 0, 0, 0, 0) * 1_000_000 + 62_547),
            None,
        ])));

    // test_timestamptz
    col += 1;
    // assert!(result[0]
    //     .column(col)
    //     .as_any()
    //     .downcast_ref::<TimestampMicrosecondArray>()
    //     .unwrap()
    //     .eq(&TimestampMicrosecondArray::from(vec![
    //         Some(time_to_arrow(1970, 1, 1, 0, 0, 1, 0) * 1000000),
    //         Some(time_to_arrow(2000, 2, 28, 12, 0, 10, 4) * 1000000),
    //         Some(time_to_arrow(2038, 1, 18, 23, 59, 59, -8) * 1000000),
    //         Some(time_to_arrow(1901, 12, 14, 0, 0, 0, 12) * 1000000 + 62547),
    // None,
    //     ])
    //     .with_timezone("+00:00")));

    // test_int2
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<Int16Array>()
        .unwrap()
        .eq(&Int16Array::from(vec![
            Some(0),
            Some(1),
            Some(-32768),
            Some(32767),
            None,
        ])));

    // test_int4
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<Int32Array>()
        .unwrap()
        .eq(&Int32Array::from(vec![
            Some(0),
            Some(1),
            Some(-2147483648),
            Some(2147483647),
            None,
        ])));

    // test_int8
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<Int64Array>()
        .unwrap()
        .eq(&Int64Array::from(vec![
            Some(-9223372036854775808),
            Some(0),
            Some(9223372036854775807),
            Some(1),
            None,
        ])));

    // test_float4
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<Float32Array>()
        .unwrap()
        .eq(&Float32Array::from(vec![
            Some(-1.1),
            Some(0.00),
            Some(2.123456),
            Some(-12345.1),
            None,
        ])));

    // test_float8
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<Float64Array>()
        .unwrap()
        .eq(&Float64Array::from(vec![
            Some(-1.1),
            Some(0.00),
            Some(2.12345678901),
            Some(-12345678901.1),
            None,
        ])));

    // test_numeric
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<Float64Array>()
        .unwrap()
        .eq(&Float64Array::from(vec![
            Some(0.01),
            Some(521.34),
            Some(0.0),
            Some(-112.3),
            None,
        ])));

    // test_bpchar
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap()
        .eq(&StringArray::from(vec![
            Some("👨‍🍳  "), // 👨‍🍳 is 3 char
            Some("bb   "),
            Some("     "),
            Some("ddddd"),
            None,
        ])));

    // test_char
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap()
        .eq(&StringArray::from(vec![
            Some("a"),
            Some("ಠ"),
            Some("😃"),
            Some("@"),
            None,
        ])));

    // test_varchar
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap()
        .eq(&StringArray::from(vec![
            Some("abcdefghij"),
            Some(""),
            Some("👨‍🍳👨‍🍳👨‍🍳👨"),
            Some("@"),
            None,
        ])));

    // test_uuid
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap()
        .eq(&StringArray::from(vec![
            Some("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11"),
            Some("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11"),
            Some("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11"),
            Some("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11"),
            None,
        ])));

    // test_time
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<Time64MicrosecondArray>()
        .unwrap()
        .eq(&Time64MicrosecondArray::from(vec![
            Some(time_to_arrow(1970, 1, 1, 8, 12, 40, 0) * 1_000_000),
            Some(time_to_arrow(1970, 1, 1, 18, 30, 0, 0) * 1_000_000),
            Some(time_to_arrow(1970, 1, 1, 23, 00, 10, 0) * 1_000_000),
            Some(time_to_arrow(1970, 1, 1, 0, 0, 59, 0) * 1_000_000 + 62547),
            None,
        ])));

    // test_json
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap()
        .eq(&StringArray::from(vec![
            Some("{\"customer\":\"John Doe\",\"items\":{\"product\":\"Beer\",\"qty\":6}}"),
            Some("{\"customer\":\"Lily Bush\",\"items\":{\"product\":\"Diaper\",\"qty\":24}}"),
            Some("{\"customer\":\"Josh William\",\"items\":{\"product\":\"Toy Car\",\"qty\":1}}"),
            Some("{}"),
            None,
        ])));

    // test_jsonb
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap()
        .eq(&StringArray::from(vec![
            Some("{\"customer\":\"John Doe\",\"items\":{\"product\":\"Beer\",\"qty\":6}}"),
            Some("{\"customer\":\"Lily Bush\",\"items\":{\"product\":\"Diaper\",\"qty\":24}}"),
            Some("{\"customer\":\"Josh William\",\"items\":{\"product\":\"Toy Car\",\"qty\":1}}"),
            Some("{}"),
            None,
        ])));

    // test_bytea
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<LargeBinaryArray>()
        .unwrap()
        .eq(&LargeBinaryArray::from(vec![
            Some("\x08".as_bytes()),
            Some("Здра́вствуйте".as_bytes()),
            Some("".as_bytes()),
            Some("😜".as_bytes()),
            None,
        ])));

    // test_enum
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap()
        .eq(&StringArray::from(vec![
            Some("happy"),
            Some("very happy"),
            Some("ecstatic"),
            Some("ecstatic"),
            None,
        ])));

    // test_f4array
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<LargeListArray>()
        .unwrap()
        .eq(&LargeListArray::from_iter_primitive::<Float32Type, _, _>(
            vec![
                Some(vec![Some(-1.1), Some(0.00)]),
                Some(vec![]),
                Some(vec![Some(2.123456), None, Some(123.123)]),
                Some(vec![Some(1.0), Some(-2.0), Some(-12345.1)]),
                None,
            ]
        )));

    // test_f8array DOUBLE PRECISION[],
    // test_narray NUMERIC(5,2)[],
    // test_boolarray BOOLEAN[],
    // test_i2array SMALLINT[],
    // test_i4array Integer[],
    // test_i8array BIGINT[],
    // test_citext CITEXT,
    // test_ltree ltree,
    // test_lquery lquery,
    // test_ltxtquery ltxtquery,
    // test_varchararray VARCHAR[],
    // test_textarray TEXT[],
    // test_name NAME

    // panic!();
}
