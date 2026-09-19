//! Phase 1 characterization tests for the MsSQL source (issue #942 migration plan).
//!
//! These tests pin down the *current* (Tiberius-backed) MsSQL -> Arrow behavior against
//! the existing `test_types` fixture (see scripts/mssql.sql), so that a future mssql-tds
//! backed implementation can be checked against the same expectations. Values below were
//! captured directly from a live SQL Server container, not derived from documentation, to
//! avoid encoding assumptions that don't match actual driver behavior (e.g. `FLOAT(18)` is
//! stored by SQL Server as a 4-byte `real` because 18 <= 24, so it round-trips with float32
//! precision even though the destination column is `Float64`).
use arrow::array::{
    Array, Date32Array, Decimal128Array, Float64Array, Int64Array, LargeBinaryArray, StringArray,
    Time64MicrosecondArray, TimestampMicrosecondArray,
};
use connectorx::{
    destinations::arrow::ArrowDestination, prelude::*, sources::mssql::MsSQLSource, sql::CXQuery,
    transports::MsSQLArrowTransport,
};
use std::sync::Arc;
use tokio::runtime::Runtime;

mod test_db;

fn run_query(query: &str) -> Vec<arrow::record_batch::RecordBatch> {
    let _ = env_logger::builder().is_test(true).try_init();
    let dburl = test_db::mssql_url();
    let queries = [CXQuery::naked(query)];
    let rt = Arc::new(Runtime::new().unwrap());
    let builder = MsSQLSource::new(rt, &dburl, 1).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher =
        Dispatcher::<_, _, MsSQLArrowTransport>::new(builder, &mut destination, &queries, None);
    dispatcher.run().unwrap();
    destination.arrow().unwrap()
}

/// Full type-matrix characterization: every column of `test_types`, ordered so nulls in
/// `test_int1` sort first (SQL Server's default NULLS FIRST for ASC).
#[test]
fn test_mssql_types_matrix() {
    let result = run_query("select * from test_types order by test_int1");
    assert_eq!(result.len(), 1);
    let rb = &result[0];
    assert_eq!(rb.num_rows(), 3);
    assert_eq!(rb.columns().len(), 25);

    macro_rules! col {
        ($idx:expr, $arr:ty) => {
            rb.column($idx).as_any().downcast_ref::<$arr>().unwrap()
        };
    }

    // tinyint / smallint / int / bigint -> Int64, nulls-first ordering
    assert!(col!(0, Int64Array).eq(&Int64Array::from(vec![None, Some(0), Some(255)])));
    assert!(col!(1, Int64Array).eq(&Int64Array::from(vec![None, Some(-32768), Some(32767)])));
    assert!(col!(2, Int64Array).eq(&Int64Array::from(vec![
        None,
        Some(-2147483648),
        Some(2147483647)
    ])));
    assert!(col!(3, Int64Array).eq(&Int64Array::from(vec![
        None,
        Some(-9223372036854775808),
        Some(9223372036854775807)
    ])));

    // real (test_float24) -> Float64, but stored/round-tripped as float32 precision
    let float24 = col!(4, Float64Array);
    assert_eq!(float24.value(0), 3.4e38_f32 as f64);
    assert!(float24.is_null(1));
    assert_eq!(float24.value(2), -1.18e-38_f32 as f64);

    // float(53) (test_float53) -> Float64, full double precision
    let float53 = col!(5, Float64Array);
    assert_eq!(float53.value(0), 1.79e308_f64);
    assert!(float53.is_null(1));
    assert_eq!(float53.value(2), -2.23e-308_f64);

    // float(18) (test_floatn) -> Float64, but SQL Server implements FLOAT(n<=24) as `real`,
    // so the value comes back with float32 precision loss (123.1234567 -> 123.12345886230469).
    let floatn = col!(6, Float64Array);
    assert_eq!(floatn.value(0), 123.1234567_f32 as f64);
    assert!(floatn.is_null(1));
    assert_eq!(floatn.value(2), 0.0_f64);

    // date -> Date32 (days since epoch)
    let date = col!(7, Date32Array);
    assert_eq!(date.value_as_date(0).unwrap().to_string(), "9999-12-31");
    assert_eq!(date.value_as_date(1).unwrap().to_string(), "1999-07-25");
    assert!(date.is_null(2));

    // time -> Time64Micro (microseconds since midnight)
    let time = col!(8, Time64MicrosecondArray);
    assert!(time.is_null(0));
    assert_eq!(time.value(1), 0);
    assert_eq!(time.value(2), 86_399_000_000);

    // datetimeoffset -> DateTimeTzMicro, normalized to UTC ("+00:00")
    let dto = col!(9, TimestampMicrosecondArray);
    assert_eq!(dto.value(0), DTO_ROW0);
    assert!(dto.is_null(1));
    assert_eq!(dto.value(2), DTO_ROW2);

    // smalldatetime -> Date64Micro (no timezone)
    let smalldt = col!(10, TimestampMicrosecondArray);
    assert_eq!(smalldt.value(0), SMALLDT_ROW0);
    assert_eq!(smalldt.value(1), SMALLDT_ROW1);
    assert!(smalldt.is_null(2));

    // datetime (old DATETIME type) -> Date64Micro
    let dt = col!(11, TimestampMicrosecondArray);
    assert!(dt.is_null(0));
    assert_eq!(dt.value(1), DT_ROW1);
    assert_eq!(dt.value(2), DT_ROW2);

    // datetime2 -> Date64Micro, sub-second precision preserved to microseconds
    let dt2 = col!(12, TimestampMicrosecondArray);
    assert_eq!(dt2.value(0), DT2_ROW0);
    assert_eq!(dt2.value(1), DT2_ROW1);
    assert!(dt2.is_null(2));

    // numeric(5,2) / decimal (default DECIMAL(18,0)) -> Decimal128(38, 10)
    let numeric = col!(13, Decimal128Array);
    assert!(numeric.is_null(0));
    assert_eq!(numeric.value(1), 11_000_000_000);
    assert_eq!(numeric.value(2), 22_000_000_000);

    let decimal = col!(14, Decimal128Array);
    assert!(decimal.is_null(0));
    assert_eq!(decimal.value(1), 10_000_000_000);
    assert_eq!(decimal.value(2), 20_000_000_000);

    // varchar / char (fixed-width, space-padded) -> Utf8
    assert!(col!(15, StringArray).eq(&StringArray::from(vec![
        Some("varchar3"),
        None,
        Some("varchar2"),
    ])));
    assert!(col!(16, StringArray).eq(&StringArray::from(vec![
        Some("char3     "),
        None,
        Some("char2     "),
    ])));

    // varbinary / binary (fixed-width, zero-padded) -> LargeBinary
    let varbinary = col!(17, LargeBinaryArray);
    assert_eq!(varbinary.value(0), Vec::<u8>::new().as_slice());
    assert!(varbinary.is_null(1));
    assert_eq!(varbinary.value(2), [49u8, 50, 51, 52]);

    let binary = col!(18, LargeBinaryArray);
    assert_eq!(binary.value(0), [0u8, 0, 0, 0, 0]);
    assert!(binary.is_null(1));
    assert_eq!(binary.value(2), [49u8, 50, 0, 0, 0]);

    // nchar / text / ntext -> Utf8
    assert!(col!(19, StringArray).eq(&StringArray::from(vec![
        Some("12  "),
        Some("1234"),
        None,
    ])));
    assert!(col!(20, StringArray).eq(&StringArray::from(vec![None, Some("text"), Some("t")])));
    assert!(col!(21, StringArray).eq(&StringArray::from(vec![None, Some("ntext"), Some("nt")])));

    // uniqueidentifier -> Utf8 (lowercase, hyphenated string form)
    assert!(col!(22, StringArray).eq(&StringArray::from(vec![
        Some("86b49b84-96b2-11eb-9298-3e22fbb9fe9d"),
        Some("86b494cc-96b2-11eb-9298-3e22fbb9fe9d"),
        None,
    ])));

    // money / smallmoney -> Float64 / Float32(as Float64 in this schema), no exact-decimal path
    let money = col!(23, Float64Array);
    assert_eq!(money.value(0), -922337203685477.6_f64);
    assert!(money.is_null(1));
    assert_eq!(money.value(2), 922337203685477.6_f64);

    let smallmoney = col!(24, Float64Array);
    assert_eq!(smallmoney.value(0), -214748.3648_f64);
    assert!(smallmoney.is_null(1));
    assert_eq!(smallmoney.value(2), 214748.3647_f64);
}

// Timestamp constants (microseconds since Unix epoch), computed with chrono from the same
// wall-clock values in scripts/mssql.sql so the intent (which wall-clock time we expect) stays
// readable, rather than hand-computed epoch arithmetic which is easy to get subtly wrong.
const DTO_ROW0: i64 = 1611829830_000_000; // 2021-01-28T12:30:30+01:00 -> 2021-01-28T11:30:30Z
const DTO_ROW2: i64 = 1609459199_000_000; // 2020-12-31T23:59:59+00:00
const SMALLDT_ROW0: i64 = 3453231600_000_000; // 2079-06-05T23:00:00
const SMALLDT_ROW1: i64 = 631188000_000_000; // 1990-01-01T10:00:00
const DT_ROW1: i64 = -6847761600_000_000; // 1753-01-01T12:00:00
const DT_ROW2: i64 = 2177370000_000_000; // 2038-12-31T01:00:00
const DT2_ROW0: i64 = 253402266630_543210; // 9999-12-31T14:30:30.543210
const DT2_ROW1: i64 = -2208945599_876550; // 1900-01-01T12:00:00.123450

/// A query that returns zero rows should still yield a well-formed (empty) result with the
/// correct schema, rather than an error or a missing RecordBatch.
#[test]
fn test_mssql_empty_result() {
    let result = run_query("select * from test_types where 1 = 0");
    assert_eq!(result.len(), 1);
    let rb = &result[0];
    assert_eq!(rb.num_rows(), 0);
    assert_eq!(rb.columns().len(), 25);
}

/// An invalid query should surface as an error from `dispatcher.run()`, not a panic.
#[test]
fn test_mssql_sql_error() {
    let _ = env_logger::builder().is_test(true).try_init();
    let dburl = test_db::mssql_url();
    let queries = [CXQuery::naked("select * from table_does_not_exist")];
    let rt = Arc::new(Runtime::new().unwrap());
    let builder = MsSQLSource::new(rt, &dburl, 1).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher =
        Dispatcher::<_, _, MsSQLArrowTransport>::new(builder, &mut destination, &queries, None);
    assert!(dispatcher.run().is_err());
}
