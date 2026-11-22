// End-to-end tests of the Trino source against the in-process `FakeTrino`
// server: type mapping, paging, row counts, partitioning and error reporting,
// all through the public entry points (`get_arrow`, `partition`).

use super::fake_server::{every_type_table, FakeTrino, Table};
use crate::get_arrow::get_arrow;
use crate::partition::{partition, PartitionQuery};
use crate::source_router::SourceConn;
use crate::sql::CXQuery;
use arrow::array::AsArray;
use arrow::compute::{cast, sort_to_indices, take_record_batch};
use arrow::datatypes::{DataType, Float64Type, Int64Type};
use arrow::record_batch::RecordBatch;
use serde_json::json;

fn conn(server: &FakeTrino) -> SourceConn {
    SourceConn::try_from(server.url().as_str()).unwrap()
}

fn naked(queries: &[&str]) -> Vec<CXQuery<String>> {
    queries.iter().map(|q| CXQuery::naked(*q)).collect()
}

/// All batches as one, sorted by the `id` column.
fn read(server: &FakeTrino, queries: &[CXQuery<String>], origin: Option<&str>) -> RecordBatch {
    let batches = get_arrow(&conn(server), origin.map(str::to_string), queries, None)
        .unwrap()
        .arrow()
        .unwrap();
    let rb = arrow::compute::concat_batches(&batches[0].schema(), &batches).unwrap();
    let order = sort_to_indices(rb.column_by_name("id").unwrap(), None, None).unwrap();
    take_record_batch(&rb, &order).unwrap()
}

fn ints(rb: &RecordBatch, name: &str) -> Vec<Option<i64>> {
    rb.column_by_name(name)
        .unwrap()
        .as_primitive::<Int64Type>()
        .iter()
        .collect()
}

fn floats(rb: &RecordBatch, name: &str) -> Vec<Option<f64>> {
    rb.column_by_name(name)
        .unwrap()
        .as_primitive::<Float64Type>()
        .iter()
        .collect()
}

/// Any column rendered as text by Arrow's cast kernel.
fn text(rb: &RecordBatch, name: &str) -> Vec<Option<String>> {
    let col = cast(rb.column_by_name(name).unwrap(), &DataType::LargeUtf8).unwrap();
    col.as_string::<i64>()
        .iter()
        .map(|s| s.map(str::to_string))
        .collect()
}

fn strs(values: &[Option<&str>]) -> Vec<Option<String>> {
    values.iter().map(|s| s.map(str::to_string)).collect()
}

#[test]
fn reads_every_type_across_pages() {
    // Two rows per page, so the parser has to follow nextUri for the rest.
    let server = FakeTrino::start(every_type_table(), 2);
    let rb = read(&server, &naked(&["SELECT * FROM t"]), None);

    assert_eq!(rb.num_rows(), 5);
    let schema = rb.schema();
    let types: Vec<_> = schema
        .fields()
        .iter()
        .map(|f| f.data_type().clone())
        .collect();
    assert_eq!(
        &types[..11],
        &[
            DataType::Int64,
            DataType::Int64,
            DataType::Int64,
            DataType::Int64,
            DataType::Float64,
            DataType::Float64,
            DataType::Float64,
            DataType::Boolean,
            DataType::Utf8,
            DataType::Utf8,
            DataType::Utf8,
        ]
    );
    assert_eq!(types[11], DataType::Date32);
    assert!(schema.fields().iter().all(|f| f.is_nullable()));

    assert_eq!(
        ints(&rb, "id"),
        [Some(0), Some(1), Some(2), Some(3), Some(4)]
    );
    assert_eq!(
        ints(&rb, "i"),
        [
            Some(1),
            Some(-1),
            None,
            Some(i32::MAX as i64),
            Some(i32::MIN as i64)
        ]
    );
    assert_eq!(
        ints(&rb, "s"),
        [
            Some(2),
            Some(-2),
            None,
            Some(i16::MAX as i64),
            Some(i16::MIN as i64)
        ]
    );
    assert_eq!(
        ints(&rb, "t"),
        [
            Some(3),
            Some(-3),
            None,
            Some(i8::MAX as i64),
            Some(i8::MIN as i64)
        ]
    );
    assert_eq!(
        floats(&rb, "d"),
        [Some(1.5), Some(-1.5), None, Some(1e300), Some(0.125)]
    );
    assert_eq!(
        floats(&rb, "r"),
        [Some(2.5), Some(0.25), None, Some(-0.5), Some(8.5)]
    );
    // Trino sends decimals as strings; the source parses them into doubles.
    assert_eq!(
        floats(&rb, "dec"),
        [Some(12.34), Some(-0.5), None, Some(0.0), Some(99999999.99)]
    );
    let b: Vec<_> = rb
        .column_by_name("b")
        .unwrap()
        .as_boolean()
        .iter()
        .collect();
    assert_eq!(b, [Some(true), Some(false), None, Some(true), Some(false)]);
    assert_eq!(
        text(&rb, "v"),
        strs(&[Some("a"), Some("bb"), None, Some(""), Some("üñí")])
    );
    assert_eq!(
        text(&rb, "c"),
        strs(&[Some("x"), Some("y"), None, Some(" "), Some("z")])
    );
    // Arrays have no Arrow mapping yet and arrive as their JSON text.
    assert_eq!(
        text(&rb, "arr"),
        strs(&[
            Some("[1,2]"),
            Some("[]"),
            None,
            Some("[3]"),
            Some("[4,5,6]")
        ])
    );
    assert_eq!(
        text(&rb, "dt"),
        strs(&[
            Some("2021-01-02"),
            Some("1999-12-31"),
            None,
            Some("1970-01-01"),
            Some("2038-01-19")
        ])
    );
    assert_eq!(
        text(&rb, "tm"),
        strs(&[
            Some("01:02:03"),
            Some("23:59:59.999"),
            None,
            Some("00:00:00"),
            Some("12:00:00.500")
        ])
    );
    assert_eq!(
        text(&rb, "ts"),
        strs(&[
            Some("2021-01-02T03:04:05"),
            Some("1999-12-31T23:59:59.999"),
            None,
            Some("1970-01-01T00:00:00"),
            Some("2038-01-19T03:14:07.500"),
        ])
    );

    // The schema comes from a LIMIT 0 query, run before the data query.
    let statements = server.statements();
    assert_eq!(statements.len(), 2, "{statements:?}");
    assert!(
        statements[0].to_uppercase().contains("LIMIT 0"),
        "{statements:?}"
    );
    assert_eq!(statements[1], "SELECT * FROM t");
}

#[test]
fn partitions_on_the_range_reported_by_the_server() {
    let server = FakeTrino::start(every_type_table(), 10);
    let conn = conn(&server);

    // min/max come from the server (trino_get_partition_range).
    let part = PartitionQuery::new("SELECT * FROM t", "id", None, None, 2);
    let queries = partition(&part, &conn).unwrap();
    assert_eq!(queries.len(), 2);
    assert!(server.statements()[0].to_uppercase().contains("MIN("));
    let sql: Vec<String> = queries.iter().map(|q| q.to_string()).collect();
    assert!(
        sql[0].contains("0 <= CXTMPTAB_PART.id AND CXTMPTAB_PART.id < 2"),
        "{sql:?}"
    );
    assert!(
        sql[1].contains("2 <= CXTMPTAB_PART.id AND CXTMPTAB_PART.id < 5"),
        "{sql:?}"
    );

    // Each partition reads its own slice; together they are the whole table.
    let rb = read(&server, &queries, Some("SELECT * FROM t"));
    assert_eq!(
        ints(&rb, "id"),
        [Some(0), Some(1), Some(2), Some(3), Some(4)]
    );
}

/// The Arrow destinations never ask for row counts, so this drives the
/// `Source` API directly, as a destination that preallocates would.
#[test]
fn row_counts_come_from_count_queries() {
    use crate::sources::{Source, SourcePartition};
    use std::sync::Arc;

    let server = FakeTrino::start(every_type_table(), 10);
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let mut source = super::TrinoSource::new(rt, &server.url()).unwrap();
    source.set_queries(&naked(&[
        "SELECT * FROM t WHERE id >= 0 AND id < 3",
        "SELECT * FROM t WHERE id >= 3 AND id < 5",
    ]));
    source.set_origin_query(Some("SELECT * FROM t".into()));
    source.fetch_metadata().unwrap();
    assert_eq!(source.names().len(), 14);

    // The origin query counts the whole table, each partition its own slice.
    assert_eq!(source.result_rows().unwrap(), Some(5));
    let mut parts = source.partition().unwrap();
    for part in &mut parts {
        part.result_rows().unwrap();
    }
    let counts: Vec<usize> = parts.iter().map(|p| p.nrows()).collect();
    assert_eq!(counts, [3, 2]);
    assert!(parts.iter().all(|p| p.ncols() == 14));
    assert!(server
        .statements()
        .iter()
        .any(|s| s.to_uppercase().contains("COUNT(*)")));
}

#[test]
fn empty_result_still_has_the_schema() {
    let server = FakeTrino::start(every_type_table(), 10);
    // id >= 10 AND id < 20 matches nothing in the fake table.
    let queries = naked(&["SELECT * FROM t WHERE id >= 10 AND id < 20"]);
    let batches = get_arrow(&conn(&server), None, &queries, None)
        .unwrap()
        .arrow()
        .unwrap();
    assert_eq!(batches.iter().map(RecordBatch::num_rows).sum::<usize>(), 0);
}

#[test]
fn server_errors_are_reported() {
    let server = FakeTrino::start(every_type_table(), 10);
    let err = get_arrow(
        &conn(&server),
        None,
        &naked(&["SELECT * FROM no_such_table"]),
        None,
    )
    .err()
    .expect("a 400 from the server must fail the read");
    assert!(err.to_string().contains("Table does not exist"), "{err}");
}

/// Values that do not fit the declared column type fail the read rather
/// than being silently truncated or nulled.
#[test]
fn values_that_do_not_match_the_column_type_are_errors() {
    for (ty, value) in [
        ("tinyint", json!(300)),
        ("integer", json!("12")),
        ("integer", json!(1.5)),
        ("double", json!("abc")),
        ("double", json!(true)),
        ("boolean", json!("true")),
        ("date", json!("2021-13-01")),
        ("date", json!(20210101)),
        ("time(3)", json!("25:00:00")),
        ("time(3)", json!(1)),
        ("timestamp(3)", json!("yesterday")),
        ("timestamp(3)", json!(1)),
    ] {
        let server = FakeTrino::start(
            Table {
                columns: vec![("id", "bigint"), ("x", ty)],
                rows: vec![vec![json!(0), value.clone()]],
            },
            10,
        );
        let result = get_arrow(&conn(&server), None, &naked(&["SELECT * FROM t"]), None);
        assert!(result.is_err(), "{ty} = {value} should not parse");
    }
}

/// Every Trino column is reported as nullable, so the dispatcher only ever
/// asks for `Option<T>`. This drives the partition parser directly to cover
/// the non-null `Produce<T>` impls too.
mod parser {
    use super::super::{build_client_from_url, TrinoSourcePartitionParser, TrinoTypeSystem};
    use super::*;
    use crate::sources::{PartitionParser, Produce};
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use std::sync::Arc;

    fn parser<'a>(server: &FakeTrino, query: &str, ncols: usize) -> TrinoSourcePartitionParser<'a> {
        let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
        let client = Arc::new(build_client_from_url(&server.url().parse().unwrap()).unwrap());
        let schema = vec![TrinoTypeSystem::Bigint(false); ncols];
        TrinoSourcePartitionParser::new(rt, client, CXQuery::naked(query), &schema).unwrap()
    }

    /// Reads the next value as a `$t` (not `Option<$t>`).
    macro_rules! next {
        ($p:expr, $t:ty) => {{
            let v: $t = $p.produce().unwrap();
            v
        }};
    }

    #[test]
    fn produces_every_non_null_type_across_pages() {
        // One row per page; rows 0 and 1 have no NULLs.
        let server = FakeTrino::start(every_type_table(), 1);
        let mut p = parser(&server, "SELECT * FROM t WHERE id >= 0 AND id < 2", 14);
        let mut rows = vec![];
        loop {
            let (n, last) = p.fetch_next().unwrap();
            for _ in 0..n {
                rows.push((
                    next!(p, i64),
                    next!(p, i32),
                    next!(p, i16),
                    next!(p, i8),
                    next!(p, f64),
                    next!(p, f32),
                    next!(p, f64),
                    next!(p, bool),
                    next!(p, String),
                    next!(p, char),
                    next!(p, String),
                    next!(p, NaiveDate),
                    next!(p, NaiveTime),
                    next!(p, NaiveDateTime),
                ));
            }
            if last {
                break;
            }
        }
        assert_eq!(rows.len(), 2);
        let r = &rows[1];
        assert_eq!((r.0, r.1, r.2, r.3), (1, -1, -2, -3));
        assert_eq!((r.4, r.5, r.6, r.7), (-1.5, 0.25, -0.5, false));
        assert_eq!((r.8.as_str(), r.9, r.10.as_str()), ("bb", 'y', "[]"));
        assert_eq!(r.11, NaiveDate::from_ymd_opt(1999, 12, 31).unwrap());
        assert_eq!(
            r.12,
            NaiveTime::from_hms_milli_opt(23, 59, 59, 999).unwrap()
        );
        assert_eq!(
            r.13,
            NaiveDate::from_ymd_opt(1999, 12, 31)
                .unwrap()
                .and_hms_milli_opt(23, 59, 59, 999)
                .unwrap()
        );
    }

    /// A NULL or mistyped value where a non-null one is expected is an error.
    #[test]
    fn non_null_reads_reject_nulls_and_wrong_types() {
        /// Serves one `ty` column holding `value` and expects reading it
        /// as a non-null `$t` to fail.
        macro_rules! rejects {
            ($t:ty, $ty:expr, $value:expr) => {{
                let value = $value;
                let server = FakeTrino::start(
                    Table {
                        columns: vec![("id", $ty)],
                        rows: vec![vec![value.clone()]],
                    },
                    10,
                );
                let mut p = parser(&server, "SELECT * FROM t", 1);
                p.fetch_next().unwrap();
                let r: Result<$t, _> = p.produce();
                assert!(
                    r.is_err(),
                    "{} = {} should not parse as non-null",
                    $ty,
                    value
                );
            }};
        }
        for v in [json!(null), json!("1"), json!(1.5), json!(300)] {
            rejects!(i8, "tinyint", v);
        }
        for v in [json!(null), json!(true), json!("x")] {
            rejects!(f64, "double", v);
        }
        for v in [json!(null), json!(1)] {
            rejects!(bool, "boolean", v);
        }
        rejects!(String, "varchar", json!(null));
        rejects!(char, "char(1)", json!("ab"));
        for v in [json!(null), json!(1), json!("2021-02-30")] {
            rejects!(NaiveDate, "date", v);
        }
        for v in [json!(null), json!(1), json!("24:00:00")] {
            rejects!(NaiveTime, "time(3)", v);
        }
        for v in [json!(null), json!(1), json!("2021-01-01")] {
            rejects!(NaiveDateTime, "timestamp(3)", v);
        }
    }
}
