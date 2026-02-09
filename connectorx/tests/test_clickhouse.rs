#![cfg(all(feature = "src_clickhouse", feature = "dst_arrow"))]

use arrow::array::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use connectorx::{
    destinations::arrow::ArrowDestination, prelude::*, sources::clickhouse::ClickHouseSource,
    sql::CXQuery, transports::ClickHouseArrowTransport,
};
use rust_decimal::Decimal;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tokio::runtime::Runtime;

macro_rules! col {
    ($batch:expr, $idx:expr, $ty:ty) => {
        $batch.column($idx).as_any().downcast_ref::<$ty>().unwrap()
    };
}

macro_rules! assert_values {
    ($batch:expr, $idx:expr, $ty:ty, $expected:expr) => {{
        let arr = col!($batch, $idx, $ty);
        assert_eq!(arr.values(), $expected);
    }};
}

macro_rules! assert_strings {
    ($batch:expr, $idx:expr, $ty:ty, $expected:expr) => {{
        let arr = col!($batch, $idx, $ty);
        for (i, v) in $expected.iter().enumerate() {
            assert_eq!(arr.value(i), *v, "row {}", i);
        }
    }};
}

macro_rules! assert_binary {
    ($batch:expr, $idx:expr, $expected:expr) => {{
        let arr = col!($batch, $idx, LargeBinaryArray);
        for (i, v) in $expected.iter().enumerate() {
            assert_eq!(arr.value(i), *v, "row {}", i);
        }
    }};
}

macro_rules! assert_floats {
    ($batch:expr, $idx:expr, $ty:ty, $expected:expr, $eps:expr) => {{
        let arr = col!($batch, $idx, $ty).values();
        for (i, v) in $expected.iter().enumerate() {
            assert!((arr[i] - v).abs() < $eps, "row {}", i);
        }
    }};
}

macro_rules! assert_decimal {
    ($batch:expr, $idx:expr, $expected:expr) => {{
        let arr = col!($batch, $idx, Decimal128Array);
        let scale = arr.scale() as u32;
        for (i, v) in $expected.iter().enumerate() {
            let actual = Decimal::new(arr.value(i) as i64, scale);
            assert_eq!(actual, *v, "row {}", i);
        }
    }};
}

macro_rules! list_col {
    ($batch:expr, $idx:expr) => {
        col!($batch, $idx, LargeListArray)
    };
}

macro_rules! assert_list_values {
    ($batch:expr, $idx:expr, $ty:ty, $expected:expr) => {{
        let col = list_col!($batch, $idx);
        assert_eq!(col.len(), $expected.len(), "row count mismatch");

        for (row_idx, expected_row) in $expected.iter().enumerate() {
            let value = col.value(row_idx);
            let arr = value.as_any().downcast_ref::<$ty>().unwrap();

            assert_eq!(
                arr.len(),
                expected_row.len(),
                "len mismatch at row {}",
                row_idx
            );

            for (i, v) in expected_row.iter().enumerate() {
                assert_eq!(
                    arr.value(i),
                    *v,
                    "value mismatch at row {}, idx {}",
                    row_idx,
                    i
                );
            }
        }
    }};
}

macro_rules! assert_list_floats {
    ($batch:expr, $idx:expr, $ty:ty, $expected:expr, $eps:expr) => {{
        let col = list_col!($batch, $idx);
        assert_eq!(col.len(), $expected.len());

        for (row_idx, expected_row) in $expected.iter().enumerate() {
            let value = col.value(row_idx);
            let arr = value.as_any().downcast_ref::<$ty>().unwrap();

            assert_eq!(arr.len(), expected_row.len());

            for (i, v) in expected_row.iter().enumerate() {
                assert!(
                    (arr.value(i) - v).abs() < $eps,
                    "float mismatch at row {}, idx {}",
                    row_idx,
                    i
                );
            }
        }
    }};
}

macro_rules! assert_list_strings {
    ($batch:expr, $idx:expr, $expected:expr) => {{
        let col = list_col!($batch, $idx);

        for (row_idx, expected_row) in $expected.iter().enumerate() {
            let value = col.value(row_idx);
            let arr = value.as_any().downcast_ref::<StringArray>().unwrap();

            assert_eq!(arr.len(), expected_row.len());

            for (i, v) in expected_row.iter().enumerate() {
                assert_eq!(
                    arr.value(i),
                    *v,
                    "string mismatch at row {}, idx {}",
                    row_idx,
                    i
                );
            }
        }
    }};
}

fn run_clickhouse_query(query: &str) -> Vec<RecordBatch> {
    let dburl = env::var("CLICKHOUSE_URL").unwrap();
    let rt = Arc::new(Runtime::new().unwrap());
    let builder = ClickHouseSource::new(rt, &dburl).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, ClickHouseArrowTransport>::new(
        builder,
        &mut destination,
        &vec![CXQuery::naked(query)],
        Some(String::from(query)),
    );
    dispatcher.run().unwrap();
    destination.arrow().unwrap()
}

#[test]
fn test_clickhouse_basic_types() {
    let _ = env_logger::builder().is_test(true).try_init();

    let batch = &run_clickhouse_query("SELECT * FROM test_basic_types ORDER BY id")[0];

    assert_values!(batch, 0, UInt32Array, &[1, 2, 3, 4, 5]);
    assert_values!(batch, 1, Int16Array, &[-128, 127, 0, 42, -1]);
    assert_values!(batch, 2, Int16Array, &[-32768, 32767, 0, 1000, -1]);
    assert_values!(
        batch,
        3,
        Int32Array,
        &[-2147483648, 2147483647, 0, 100000, -1]
    );
    assert_values!(
        batch,
        4,
        Int64Array,
        &[-9223372036854775808, 9223372036854775807, 0, 1000000000, -1]
    );

    assert_floats!(
        batch,
        9,
        Float32Array,
        &[-3.14, 3.14, 0.0, 2.718, -0.5],
        0.001
    );
    assert_floats!(
        batch,
        10,
        Float64Array,
        &[
            -3.141592653589793,
            3.141592653589793,
            0.0,
            2.718281828459045,
            -0.123456789012345
        ],
        1e-12
    );

    assert_decimal!(
        batch,
        11,
        &[
            Decimal::from_str("-12345.6789").unwrap(),
            Decimal::from_str("12345.6789").unwrap(),
            Decimal::from_str("0").unwrap(),
            Decimal::from_str("9999.9999").unwrap(),
            Decimal::from_str("-0.0001").unwrap(),
        ]
    );

    assert_decimal!(
        batch,
        12,
        &[
            Decimal::from_str("-123456789.12345678").unwrap(),
            Decimal::from_str("123456789.12345678").unwrap(),
            Decimal::from_str("0").unwrap(),
            Decimal::from_str("99999999.99999999").unwrap(),
            Decimal::from_str("-0.00000001").unwrap(),
        ]
    );
}

#[test]
fn test_clickhouse_string_types() {
    let _ = env_logger::builder().is_test(true).try_init();

    let batch = &run_clickhouse_query("SELECT * FROM test_string_types ORDER BY id")[0];

    assert_strings!(
        batch,
        1,
        StringArray,
        &["Hello, World!", "ConnectorX ClickHouse Test"]
    );

    assert_binary!(batch, 2, &[b"FixedStr16bytes!", b"ABCDEFGHIJKLMNOP"]);
}

#[test]
fn test_clickhouse_datetime_types() {
    let batch = &run_clickhouse_query("SELECT * FROM test_datetime_types ORDER BY id")[0];

    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

    let times = ["14:30:25", "14:35:25", "23:59:59", "12:00:00", "08:15:30"]
        .map(|t| NaiveTime::parse_from_str(t, "%H:%M:%S").unwrap());

    let arr = col!(batch, 1, Time64NanosecondArray);
    for (i, t) in times.iter().enumerate() {
        let actual = NaiveTime::from_num_seconds_from_midnight_opt(
            (arr.value(i) / 1_000_000_000) as u32,
            (arr.value(i) % 1_000_000_000) as u32,
        )
        .unwrap();
        assert_eq!(actual, *t);
    }

    let dates = [
        "2024-01-15",
        "1970-01-01",
        "2099-12-31",
        "2000-06-15",
        "2023-11-28",
    ]
    .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").unwrap());

    let arr = col!(batch, 3, Date32Array);
    for (i, d) in dates.iter().enumerate() {
        assert_eq!(epoch + chrono::Duration::days(arr.value(i) as i64), *d);
    }

    let datetimes = [
        "2024-01-15 10:30:45",
        "1970-01-01 00:00:00",
        "2099-12-31 23:59:59",
        "2000-06-15 12:00:00",
        "2023-11-28 08:15:30",
    ]
    .map(|d| {
        NaiveDateTime::parse_from_str(d, "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc()
    });

    let arr = col!(batch, 5, TimestampNanosecondArray);
    for (i, dt) in datetimes.iter().enumerate() {
        assert_eq!(Utc.timestamp_nanos(arr.value(i)), *dt);
    }
}

#[test]
fn test_clickhouse_enum_types() {
    let batch = &run_clickhouse_query("SELECT * FROM test_enum_types ORDER BY id")[0];

    assert_strings!(
        batch,
        1,
        StringArray,
        &["small", "medium", "large", "small", "large"]
    );

    assert_strings!(
        batch,
        2,
        StringArray,
        &["red", "green", "blue", "yellow", "red"]
    );
}

#[test]
fn test_clickhouse_network_types() {
    let _ = env_logger::builder().is_test(true).try_init();

    let batch = &run_clickhouse_query("SELECT * FROM test_network_types ORDER BY id")[0];

    assert_strings!(
        batch,
        1,
        StringArray,
        &[
            "550e8400-e29b-41d4-a716-446655440000",
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
            "00000000-0000-0000-0000-000000000000",
            "ffffffff-ffff-ffff-ffff-ffffffffffff",
            "123e4567-e89b-12d3-a456-426614174000",
        ]
    );

    assert_strings!(
        batch,
        2,
        StringArray,
        &[
            "192.168.1.1",
            "10.0.0.1",
            "0.0.0.0",
            "255.255.255.255",
            "127.0.0.1",
        ]
    );

    assert_strings!(
        batch,
        3,
        StringArray,
        &[
            "2001:db8:85a3::8a2e:370:7334",
            "fe80::1",
            "::",
            "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
            "::1",
        ]
    );
}

#[test]
fn test_clickhouse_array_types() {
    let _ = env_logger::builder().is_test(true).try_init();

    let batch = &run_clickhouse_query("SELECT * FROM test_array_types ORDER BY id")[0];

    assert_eq!(batch.num_rows(), 5);
    assert_eq!(batch.num_columns(), 13);

    assert_values!(batch, 0, UInt32Array, &[1, 2, 3, 4, 5]);

    assert_list_values!(
        batch,
        1,
        BooleanArray,
        &[
            vec![true, false, true],
            vec![],
            vec![false],
            vec![true, true, true, true, true],
            vec![false, true],
        ]
    );

    assert_list_strings!(
        batch,
        2,
        &[
            vec!["a", "b", "c"],
            vec![],
            vec!["single"],
            vec!["one", "two", "three", "four", "five"],
            vec!["hello", "world"],
        ]
    );

    assert_list_values!(
        batch,
        3,
        Int16Array,
        &[
            vec![-1, 0, 1],
            vec![],
            vec![42],
            vec![-128, -64, 0, 64, 127],
            vec![10, 20],
        ]
    );

    assert_list_values!(
        batch,
        4,
        Int16Array,
        &[
            vec![-100, 0, 100],
            vec![],
            vec![42],
            vec![-32768, -16384, 0, 16384, 32767],
            vec![100, 200],
        ]
    );

    assert_list_values!(
        batch,
        5,
        Int32Array,
        &[
            vec![-1000, 0, 1000],
            vec![],
            vec![42],
            vec![-2147483648, -1073741824, 0, 1073741824, 2147483647],
            vec![1000, 2000],
        ]
    );

    assert_list_values!(
        batch,
        6,
        Int64Array,
        &[
            vec![-10000, 0, 10000],
            vec![],
            vec![42],
            Vec::<i64>::from([
                -9223372036854775808,
                -4611686018427387904,
                0,
                4611686018427387904,
                9223372036854775807
            ]),
            vec![10000, 20000],
        ]
    );

    assert_list_values!(
        batch,
        7,
        UInt16Array,
        &[
            vec![0, 128, 255],
            vec![],
            vec![42],
            vec![0, 64, 128, 192, 255],
            vec![10, 20],
        ]
    );

    assert_list_values!(
        batch,
        8,
        UInt16Array,
        &[
            vec![0, 32768, 65535],
            vec![],
            vec![42],
            vec![0, 16384, 32768, 49152, 65535],
            vec![100, 200],
        ]
    );

    assert_list_values!(
        batch,
        9,
        UInt32Array,
        &[
            Vec::<u32>::from([0, 1_000_000, 4_294_967_295]),
            vec![],
            vec![42],
            Vec::<u32>::from([
                0,
                1_073_741_824,
                2_147_483_648,
                3_221_225_472,
                4_294_967_295
            ]),
            vec![1000, 2000],
        ]
    );

    assert_list_values!(
        batch,
        10,
        UInt64Array,
        &[
            Vec::<u64>::from([0, 1_000_000, 18_446_744_073_709_551_615]),
            vec![],
            vec![42],
            Vec::<u64>::from([
                0,
                4_611_686_018_427_387_904,
                9_223_372_036_854_775_808,
                13_835_058_055_282_163_712,
                18_446_744_073_709_551_615
            ]),
            vec![10000, 20000],
        ]
    );

    assert_list_floats!(
        batch,
        11,
        Float32Array,
        &[
            vec![1.1, 2.2, 3.3],
            vec![],
            vec![42.0],
            vec![-3.14, -1.57, 0.0, 1.57, 3.14],
            vec![1.5, 2.5],
        ],
        1e-3
    );

    assert_list_floats!(
        batch,
        12,
        Float64Array,
        &[
            vec![1.111, 2.222, 3.333],
            vec![],
            vec![42.0],
            Vec::<f64>::from([
                -3.141592653589793,
                -1.5707963267948966,
                0.0,
                1.5707963267948966,
                3.141592653589793
            ]),
            vec![1.55, 2.55],
        ],
        1e-12
    );
}
