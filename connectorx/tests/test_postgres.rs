use arrow::{
    array::{
        Array, BooleanArray, BooleanBuilder, Date32Array, Decimal128Array, Decimal128Builder,
        Float32Array, Float64Array, Int16Array, Int32Array, Int64Array, LargeBinaryArray,
        LargeListArray, LargeListBuilder, StringArray, StringBuilder, Time64MicrosecondArray,
        TimestampMicrosecondArray,
    },
    datatypes::{Float32Type, Float64Type, Int16Type, Int32Type, Int64Type},
    record_batch::RecordBatch,
};
use chrono::naive::NaiveDate;
use connectorx::{
    destinations::arrow::ArrowDestination,
    prelude::*,
    sources::{
        postgres::{
            rewrite_tls_args, BinaryProtocol, CSVProtocol, CursorProtocol, PostgresSource,
            SimpleProtocol,
        },
        PartitionParser,
    },
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
fn test_postgres_binary() {
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
    println!("{:?}", result);
    verify_arrow_results(result);
}

#[test]
fn test_postgres_cursor() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [
        CXQuery::naked("select * from test_table where test_int < 2"),
        CXQuery::naked("select * from test_table where test_int >= 2"),
    ];
    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<CursorProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut dst = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<CursorProtocol, NoTls>>::new(
        builder, &mut dst, &queries, None,
    );

    dispatcher.run().expect("run dispatcher");
    let result = dst.arrow().unwrap();
    println!("{:?}", result);
    verify_arrow_results(result);
}

#[test]
fn test_postgres_simple() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [
        CXQuery::naked("select * from test_table where test_int < 2"),
        CXQuery::naked("select * from test_table where test_int >= 2"),
    ];
    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<SimpleProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut dst = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<SimpleProtocol, NoTls>>::new(
        builder, &mut dst, &queries, None,
    );

    dispatcher.run().expect("run dispatcher");
    let result = dst.arrow().unwrap();
    println!("{:?}", result);
    verify_arrow_results(result);
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

macro_rules! test_types {
    ($protocol: expr, $sql: expr, $P: ty, $verify: expr) => {
        let _ = env_logger::builder().is_test(true).try_init();
        let dburl = env::var("POSTGRES_URL").unwrap();
        let queries = [CXQuery::naked($sql)];
        let url = Url::parse(dburl.as_str()).unwrap();
        let (config, _tls) = rewrite_tls_args(&url).unwrap();
        let builder = PostgresSource::<$P, NoTls>::new(config, NoTls, 2).unwrap();
        let mut destination = ArrowDestination::new();
        let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<$P, NoTls>>::new(
            builder,
            &mut destination,
            &queries,
            Some(String::from("select * from test_types")),
        );

        dispatcher.run().expect("run dispatcher");

        let result = destination.arrow().unwrap();
        $verify(result, $protocol);
    };
}

#[test]
fn test_types_binary_postgres() {
    test_types!(
        "binary",
        "select test_bool,test_date,test_timestamp,test_timestamptz,test_int2,test_int4,test_int8,test_float4,test_float8,test_numeric,test_bpchar,test_char,test_varchar,test_uuid,test_time,test_json,test_jsonb,test_bytea,test_enum,test_f4array,test_f8array,test_narray,test_boolarray,test_i2array,test_i4array,test_i8array,test_citext,test_ltree,test_lquery,test_ltxtquery,test_varchararray,test_textarray,test_name,test_inet from test_types",
        BinaryProtocol,
        verify_arrow_type_results
    );
}

#[test]
fn test_pgvector_types_binary_postgres() {
    test_types!(
        "binary",
        "select * from vector_types",
        BinaryProtocol,
        verfiy_pgvector_results
    );
}

#[test]
fn test_types_csv_postgres() {
    test_types!(
        "csv",
        "select test_bool,test_date,test_timestamp,test_timestamptz,test_int2,test_int4,test_int8,test_float4,test_float8,test_numeric,test_bpchar,test_char,test_varchar,test_uuid,test_time,test_json,test_jsonb,test_bytea,test_enum,test_f4array,test_f8array,test_narray,test_boolarray,test_i2array,test_i4array,test_i8array,test_citext,test_ltree,test_lquery,test_ltxtquery,test_varchararray,test_textarray,test_name,test_inet from test_types",
        CSVProtocol,
        verify_arrow_type_results
    );
}

#[test]
fn test_types_cursor_postgres() {
    test_types!(
        "cursor",
        "select test_bool,test_date,test_timestamp,test_timestamptz,test_int2,test_int4,test_int8,test_float4,test_float8,test_numeric,test_bpchar,test_char,test_varchar,test_uuid,test_time,test_json,test_jsonb,test_bytea,test_f4array,test_f8array,test_narray,test_boolarray,test_i2array,test_i4array,test_i8array,test_citext,test_ltree,test_lquery,test_ltxtquery,test_varchararray,test_textarray,test_name,test_inet from test_types",
        CursorProtocol,
        verify_arrow_type_results
    );
}

#[test]
fn test_types_simple_postgres() {
    test_types!(
        "simple",
        "select test_bool,test_date,test_timestamp,test_timestamptz,test_int2,test_int4,test_int8,test_float4,test_float8,test_numeric,test_bpchar,test_char,test_varchar,test_uuid,test_time,test_bytea,test_f4array,test_f8array,test_narray,test_boolarray,test_i2array,test_i4array,test_i8array,test_citext,test_ltree,test_lquery,test_ltxtquery,test_varchararray,test_textarray,test_name,test_inet from test_types",
        SimpleProtocol,
        verify_arrow_type_results
    );
}

pub fn verify_arrow_type_results(result: Vec<RecordBatch>, protocol: &str) {
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
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<TimestampMicrosecondArray>()
        .unwrap()
        .eq(&TimestampMicrosecondArray::from(vec![
            Some(time_to_arrow(1970, 1, 1, 0, 0, 1, 0) * 1000000),
            Some(time_to_arrow(2000, 2, 28, 12, 0, 10, 4) * 1000000),
            Some(time_to_arrow(2038, 1, 18, 23, 59, 59, -8) * 1000000),
            Some(time_to_arrow(1901, 12, 14, 0, 0, 0, 12) * 1000000 + 62547),
            None,
        ])
        .with_timezone("+00:00")));

    // test_int2
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<Int16Array>()
        .unwrap()
        .eq(&Int16Array::from(vec![
            Some(-32768),
            Some(0),
            Some(1),
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
    let actual = result[0]
        .column(col)
        .as_any()
        .downcast_ref::<Decimal128Array>()
        .unwrap();
    let expected = build_decimal_array(vec![
        Some(100000000),
        Some(5213400000000),
        Some(0),
        Some(-1123000000000),
        None,
    ]);
    assert_eq!(actual, &expected);

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
    if protocol == "csv" {
        assert!(result[0]
            .column(col)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap()
            .eq(&StringArray::from(vec![
                Some("abcdefghij"),
                None, // CSV Protocol can't distinguish "" from NULL
                Some("👨‍🍳👨‍🍳👨‍🍳👨"),
                Some("@"),
                None,
            ])));
    } else {
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
    }

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

    if protocol != "simple" {
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
                Some(
                    "{\"customer\":\"Josh William\",\"items\":{\"product\":\"Toy Car\",\"qty\":1}}"
                ),
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
                Some(
                    "{\"customer\":\"Josh William\",\"items\":{\"product\":\"Toy Car\",\"qty\":1}}"
                ),
                Some("{}"),
                None,
            ])));
    }

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
    if protocol != "cursor" && protocol != "simple" {
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
    }

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
                Some(vec![Some(1.0), Some(-2.0), Some(-12345.1)]),
                Some(vec![Some(2.123456), None, Some(123.123)]),
                None,
            ]
        )));

    // test_f8array
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<LargeListArray>()
        .unwrap()
        .eq(&LargeListArray::from_iter_primitive::<Float64Type, _, _>(
            vec![
                Some(vec![Some(-1.1), Some(0.00)]),
                Some(vec![]),
                Some(vec![Some(2.12345678901), Some(-12345678901.1)]),
                Some(vec![Some(2.123456), None, Some(123.123)]),
                None,
            ]
        )));

    // test_narray
    col += 1;
    let actual = result[0]
        .column(col)
        .as_any()
        .downcast_ref::<LargeListArray>()
        .unwrap();

    let mut expected = LargeListBuilder::new(
        Decimal128Builder::new()
            .with_precision_and_scale(38, 10)
            .unwrap(),
    );
    expected.append_value(vec![Some(100000000), Some(5212300000000)]);
    expected.append_value(vec![
        Some(1200000000),
        Some(3333300000000),
        Some(222200000000),
    ]);
    expected.append_value(vec![]);
    expected.append_value(vec![Some(0), None, Some(-1121000000000)]);
    expected.append_null();

    assert!(actual.eq(&expected.finish()));

    // test_boolarray (from_iter_primitive not available for boolean)
    col += 1;
    let b0: BooleanArray = vec![Some(true), Some(false)].into();
    let empty_vec: Vec<bool> = vec![];
    let b1: BooleanArray = empty_vec.into();
    let b2: BooleanArray = vec![Some(true)].into();
    let b3: BooleanArray = vec![Some(true), Some(false), None].into();

    let mut builder: LargeListBuilder<BooleanBuilder> =
        LargeListBuilder::with_capacity(BooleanBuilder::new(), 5);
    builder.append_value(&b0);
    builder.append_value(&b1);
    builder.append_value(&b2);
    builder.append_value(&b3);
    builder.append_null();

    let bool_array = builder.finish();

    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<LargeListArray>()
        .unwrap()
        .eq(&bool_array));

    // test_i2array
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<LargeListArray>()
        .unwrap()
        .eq(&LargeListArray::from_iter_primitive::<Int16Type, _, _>(
            vec![
                Some(vec![Some(12)]),
                Some(vec![]),
                Some(vec![Some(-32768), Some(32767)]),
                Some(vec![Some(-1), Some(0), Some(1), None]),
                None,
            ]
        )));

    // test_i4array
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<LargeListArray>()
        .unwrap()
        .eq(&LargeListArray::from_iter_primitive::<Int32Type, _, _>(
            vec![
                Some(vec![Some(-1)]),
                Some(vec![]),
                Some(vec![Some(-2147483648), Some(2147483647)]),
                Some(vec![Some(-1), Some(0), Some(1123), None]),
                None,
            ]
        )));

    // test_i8array
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<LargeListArray>()
        .unwrap()
        .eq(&LargeListArray::from_iter_primitive::<Int64Type, _, _>(
            vec![
                Some(vec![Some(-9223372036854775808), Some(9223372036854775807)]),
                Some(vec![]),
                Some(vec![Some(-0)]),
                Some(vec![Some(-1), Some(0), Some(1), None]),
                None,
            ]
        )));

    // test_citext
    col += 1;
    if protocol == "csv" {
        assert!(result[0]
            .column(col)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap()
            .eq(&StringArray::from(vec![
                Some("str_citext"),
                None, // CSV Protocol can't distinguish "" from NULL
                Some("abcdef"),
                Some("1234"),
                None,
            ])));
    } else {
        assert!(result[0]
            .column(col)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap()
            .eq(&StringArray::from(vec![
                Some("str_citext"),
                Some(""),
                Some("abcdef"),
                Some("1234"),
                None,
            ])));
    }

    // test_ltree
    col += 1;
    if protocol == "csv" {
        assert!(result[0]
            .column(col)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap()
            .eq(&StringArray::from(vec![
                Some("A.B.C.D"),
                Some("A.B.E"),
                Some("A"),
                None, // CSV Protocol can't distinguish "" from NULL
                None,
            ])));
    } else {
        assert!(result[0]
            .column(col)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap()
            .eq(&StringArray::from(vec![
                Some("A.B.C.D"),
                Some("A.B.E"),
                Some("A"),
                Some(""),
                None,
            ])));
    }

    // test_lquery
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap()
        .eq(&StringArray::from(vec![
            Some("*.B.*"),
            Some("A.*"),
            Some("*"),
            Some("*.A"),
            None,
        ])));

    // test_ltxtquery
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap()
        .eq(&StringArray::from(vec![
            Some("A & B*"),
            Some("A | B"),
            Some("A@"),
            Some("A & B*"),
            None,
        ])));

    // test_varchararray (does not implement from_iter_primitive)
    col += 1;
    let b0: StringArray = vec![Some("str1"), Some("str2")].into();
    let b1: StringArray = vec![
        Some("0123456789"),
        Some("abcdefghijklmnopqrstuvwxyz"),
        Some("!@#$%^&*()_-+=~`:;<>?/"),
    ]
    .into();
    let b2: StringArray;
    let b3: StringArray;
    if protocol == "csv" || protocol == "simple" {
        // Interpret the '' as \"
        b2 = vec![Some("\"\""), Some("\"  \"")].into();
        b3 = vec![Some("👨‍🍳👨‍🍳👨‍🍳👨"), Some("\"\""), None].into();
    } else {
        b2 = vec![Some(""), Some("  ")].into();
        b3 = vec![Some("👨‍🍳👨‍🍳👨‍🍳👨"), Some(""), None].into();
    }
    let mut builder: LargeListBuilder<StringBuilder> =
        LargeListBuilder::with_capacity(StringBuilder::new(), 5);
    builder.append_value(&b0);
    builder.append_value(&b1);
    builder.append_value(&b2);
    builder.append_value(&b3);
    builder.append_null();

    let array = builder.finish();

    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<LargeListArray>()
        .unwrap()
        .eq(&array));

    // test_textarray (does not implement from_iter_primitive)
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<LargeListArray>()
        .unwrap()
        .eq(&array));

    // test_name (does not implement from_iter_primitive)
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap()
        .eq(&StringArray::from(vec![
            Some("0"), // 👨‍🍳 is 3 char
            Some("21"),
            Some("someName"),
            Some("101203203-1212323-22131235"),
            None,
        ])));

    // test_inet
    col += 1;
    assert!(result[0]
        .column(col)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap()
        .eq(&StringArray::from(vec![
            Some("192.168.1.1"),
            Some("10.0.0.0/24"),
            Some("2001:db8::1"),
            Some("2001:db8::/32"),
            None,
        ])));
}

fn verfiy_pgvector_results(result: Vec<RecordBatch>, _protocol: &str) {
    let rb = &result[0];
    let mut col = 0;

    assert!(result.len() == 1);
    assert!(rb.columns().len() == 5); // id + 4 vector types

    // Verify id column
    assert!(rb
        .column(col)
        .as_any()
        .downcast_ref::<Int32Array>()
        .unwrap()
        .eq(&Int32Array::from(vec![1, 2])));

    // Verify dense_vector column
    col += 1;
    let dense_vector = rb
        .column(col)
        .as_any()
        .downcast_ref::<LargeListArray>()
        .unwrap();
    let dense_vector_value = dense_vector.value(0);
    let dense_vector_values = dense_vector_value
        .as_any()
        .downcast_ref::<Float32Array>()
        .unwrap();
    let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    for (i, val) in expected.iter().enumerate() {
        assert_eq!(dense_vector_values.value(i), *val);
    }
    assert!(dense_vector.is_null(1));

    // Verify half_vector column
    col += 1;
    let half_vector = rb
        .column(col)
        .as_any()
        .downcast_ref::<LargeListArray>()
        .unwrap();
    let half_vector_value = half_vector.value(0);
    let half_vector_values = half_vector_value
        .as_any()
        .downcast_ref::<Float32Array>()
        .unwrap();
    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    for (i, val) in expected.iter().enumerate() {
        assert_eq!(half_vector_values.value(i), *val);
    }
    assert!(half_vector.is_null(1));

    // Verify binary_vector column
    col += 1;
    let binary_vector = rb
        .column(col)
        .as_any()
        .downcast_ref::<LargeBinaryArray>()
        .unwrap();
    let binary_vector_value = binary_vector.value(0);
    let expected = vec![170, 128];
    for (i, val) in expected.iter().enumerate() {
        assert_eq!(binary_vector_value[i], *val);
    }
    assert!(binary_vector.is_null(1));

    // Verify sparse_vector column
    col += 1;
    let sparse_vector = rb
        .column(col)
        .as_any()
        .downcast_ref::<LargeListArray>()
        .unwrap();
    let sparse_vector_value = sparse_vector.value(0);
    let sparse_vector_value = sparse_vector_value
        .as_any()
        .downcast_ref::<Float32Array>()
        .unwrap();
    let expected = vec![1.0, 0.0, 2.0, 0.0, 3.0];
    for (i, val) in expected.iter().enumerate() {
        assert_eq!(sparse_vector_value.value(i), *val);
    }
    assert!(sparse_vector.is_null(1));
}

#[test]
fn test_postgres_pre_execution_queries() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [
        CXQuery::naked("SELECT CAST(name AS TEXT) AS name, CAST(setting AS INTEGER) AS setting FROM pg_settings WHERE name IN ('statement_timeout', 'idle_in_transaction_session_timeout') ORDER BY name"),
    ];

    let pre_execution_queries = [
        String::from("SET SESSION statement_timeout = 2151"),
        String::from("SET SESSION idle_in_transaction_session_timeout = 2252"),
    ];

    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let mut dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        None,
    );

    dispatcher.set_pre_execution_queries(Some(&pre_execution_queries));

    dispatcher.run().expect("run dispatcher");

    let result = destination.arrow().unwrap();

    assert!(result.len() == 1);

    assert!(result[0]
        .column(1)
        .as_any()
        .downcast_ref::<Int32Array>()
        .unwrap()
        .eq(&Int32Array::from(vec![2252, 2151])));
}

#[test]
fn test_postgres_partitioned_pre_execution_queries() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("POSTGRES_URL").unwrap();

    let queries = [
        "SELECT CAST(name AS TEXT) AS name, CAST(setting AS INTEGER) AS setting FROM pg_settings WHERE name = 'statement_timeout'", 
        "SELECT CAST(name AS TEXT) AS name, CAST(setting AS INTEGER) AS setting FROM pg_settings WHERE name = 'idle_in_transaction_session_timeout'"
    ];

    let pre_execution_queries = [
        String::from("SET SESSION statement_timeout = 2151"),
        String::from("SET SESSION idle_in_transaction_session_timeout = 2252"),
    ];

    let url = Url::parse(dburl.as_str()).unwrap();
    let (config, _tls) = rewrite_tls_args(&url).unwrap();
    let builder = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let mut dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
        builder,
        &mut destination,
        &queries,
        None,
    );

    dispatcher.set_pre_execution_queries(Some(&pre_execution_queries));

    dispatcher.run().expect("run dispatcher");

    let result = destination.arrow().unwrap();

    assert!(result.len() == 2);

    let mut result_map = std::collections::HashMap::new();
    for record_batch in result {
        let name = record_batch
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap()
            .value(0)
            .to_string();
        let setting = record_batch
            .column(1)
            .as_any()
            .downcast_ref::<Int32Array>()
            .unwrap()
            .value(0);
        result_map.insert(name, setting);
    }

    assert_eq!(result_map.get("statement_timeout"), Some(&2151));
    assert_eq!(
        result_map.get("idle_in_transaction_session_timeout"),
        Some(&2252)
    );
}

fn build_decimal_array(vals: Vec<Option<i128>>) -> Decimal128Array {
    let mut builder = Decimal128Builder::new()
        .with_precision_and_scale(38, 10)
        .unwrap();

    for val in vals {
        match val {
            Some(v) => builder.append_value(v),
            None => builder.append_null(),
        }
    }

    builder.finish()
}
