// End-to-end tests for the SQLite source. SQLite is embedded, so unlike the
// other sources these need no server or container: each test writes a small
// database file to the temp directory and reads it back through the public
// entry points (`get_arrow`, `new_record_batch_iter`, `partition`).

use super::*;
use crate::get_arrow::{get_arrow, new_record_batch_iter};
use crate::partition::{partition, PartitionQuery};
use crate::source_router::SourceConn;
use arrow::array::{Array, AsArray};
use arrow::datatypes::{DataType, Date32Type, Float64Type, Int64Type, TimeUnit};
use arrow::record_batch::RecordBatch;
use rusqlite::Connection;
use std::path::PathBuf;

/// A database file that is removed when the test ends.
struct TempDb(PathBuf);

impl TempDb {
    fn new(name: &str, sql: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "connectorx_sqlite_{}_{}.db",
            name,
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        Connection::open(&path).unwrap().execute_batch(sql).unwrap();
        TempDb(path)
    }

    fn conn(&self) -> SourceConn {
        // Percent-encode the path, as the Python ConnectionUrl helper does: a raw
        // Windows path (`C:\Users\...`) does not survive URL parsing.
        let path = urlencoding::encode(&self.path()).into_owned();
        SourceConn::try_from(format!("sqlite://{}", path).as_str()).unwrap()
    }

    fn path(&self) -> String {
        self.0.display().to_string()
    }
}

impl Drop for TempDb {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

const TYPED: &str = "
CREATE TABLE t (
    id INTEGER, i4 INT4, i2 INT2, flag BOOLEAN, name TEXT, score REAL,
    d DATE, tm TIME, ts DATETIME, raw BLOB
);
INSERT INTO t VALUES
    (1, 10, 100, 1, 'a', 0.5, '2020-01-02', '03:04:05', '2020-01-02 03:04:05', x'0102'),
    (2, 20, 200, 0, 'b', 1.5, '1970-01-01', '00:00:01', '1970-01-01 00:00:00', x''),
    (3, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL),
    (4, 40, 400, 1, 'd', -2.0, '2021-12-31', '23:59:59', '2021-12-31 23:59:59', x'ff');
";

fn all(batches: &[RecordBatch]) -> RecordBatch {
    arrow::compute::concat_batches(&batches[0].schema(), batches).unwrap()
}

fn read(db: &TempDb, queries: &[&str]) -> RecordBatch {
    let queries: Vec<CXQuery<String>> = queries.iter().map(|q| CXQuery::naked(*q)).collect();
    let batches = get_arrow(&db.conn(), None, &queries, None)
        .unwrap()
        .arrow()
        .unwrap();
    all(&batches)
}

#[test]
fn reads_every_declared_type() {
    let db = TempDb::new("typed", TYPED);
    let rb = read(&db, &["SELECT * FROM t ORDER BY id"]);
    assert_eq!(rb.num_rows(), 4);

    let types: Vec<_> = rb
        .schema()
        .fields()
        .iter()
        .map(|f| f.data_type().clone())
        .collect();
    assert_eq!(
        types,
        vec![
            DataType::Int64,
            DataType::Int64, // INT4 -> Int4 -> widened to Int64
            DataType::Int64, // INT2 -> Int2 -> widened to Int64
            DataType::Boolean,
            DataType::Utf8,
            DataType::Float64,
            DataType::Date32,
            DataType::Time64(TimeUnit::Microsecond),
            DataType::Timestamp(TimeUnit::Microsecond, None),
            DataType::LargeBinary,
        ]
    );
    // Declared types are always nullable.
    assert!(rb.schema().fields().iter().all(|f| f.is_nullable()));

    assert_eq!(rb.column(1).as_primitive::<Int64Type>().value(1), 20);
    assert_eq!(rb.column(2).as_primitive::<Int64Type>().value(3), 400);
    assert!(rb.column(3).as_boolean().value(0));
    assert!(!rb.column(3).as_boolean().value(1));
    assert_eq!(rb.column(4).as_string::<i32>().value(3), "d");
    assert_eq!(rb.column(5).as_primitive::<Float64Type>().value(3), -2.0);
    assert_eq!(rb.column(6).as_primitive::<Date32Type>().value(1), 0);
    assert_eq!(
        rb.column(7)
            .as_any()
            .downcast_ref::<arrow::array::Time64MicrosecondArray>()
            .unwrap()
            .value(1),
        1_000_000
    );
    assert_eq!(
        rb.column(8)
            .as_any()
            .downcast_ref::<arrow::array::TimestampMicrosecondArray>()
            .unwrap()
            .value(1),
        0
    );
    assert_eq!(rb.column(9).as_binary::<i64>().value(0), &[1, 2]);
    // Row 3 is NULL in every column but id.
    for c in 1..rb.num_columns() {
        assert!(rb.column(c).is_null(2), "column {c}");
    }
}

#[test]
fn infers_undeclared_types_from_the_first_row() {
    let db = TempDb::new("infer", TYPED);
    // Expressions have no declared type: fetch_metadata falls back to LIMIT 1.
    let rb = read(
        &db,
        &["SELECT id * 2 AS dbl, name || '!' AS bang, score + 1 AS s1 FROM t WHERE id < 3"],
    );
    let types: Vec<_> = rb
        .schema()
        .fields()
        .iter()
        .map(|f| f.data_type().clone())
        .collect();
    assert_eq!(
        types,
        vec![DataType::Int64, DataType::Utf8, DataType::Float64]
    );
    assert_eq!(rb.column(0).as_primitive::<Int64Type>().values(), &[2, 4]);
    assert_eq!(rb.column(1).as_string::<i32>().value(1), "b!");
}

#[test]
fn undeclared_column_that_is_null_in_every_partition_is_an_error() {
    let db = TempDb::new("allnull", TYPED);
    let queries = [CXQuery::naked("SELECT NULL AS n FROM t")];
    let Err(err) = get_arrow(&db.conn(), None, &queries, None) else {
        panic!("expected an error");
    };
    assert!(err.to_string().contains("infer type from null"), "{err}");
}

#[test]
fn undeclared_column_of_an_empty_result_defaults_to_text() {
    let db = TempDb::new("empty", TYPED);
    let rb = read(&db, &["SELECT id + 1 AS x FROM t WHERE id > 100"]);
    assert_eq!(rb.num_rows(), 0);
    assert_eq!(rb.schema().field(0).data_type(), &DataType::Utf8);
}

#[test]
fn partitions_on_an_integer_column() {
    let db = TempDb::new("partition", TYPED);
    let conn = db.conn();
    // min/max come from the table (sqlite_get_partition_range).
    let part = PartitionQuery::new("SELECT id, name FROM t", "id", None, None, 2);
    let queries = partition(&part, &conn).unwrap();
    assert_eq!(queries.len(), 2);
    assert!(queries[0].as_str().contains("id"));

    let batches = get_arrow(&conn, Some("SELECT id, name FROM t".into()), &queries, None)
        .unwrap()
        .arrow()
        .unwrap();
    let mut ids: Vec<i64> = batches
        .iter()
        .flat_map(|b| b.column(0).as_primitive::<Int64Type>().values().to_vec())
        .collect();
    ids.sort();
    assert_eq!(ids, vec![1, 2, 3, 4]);
}

#[test]
fn partition_range_rounds_real_columns_and_rejects_text() {
    let db = TempDb::new("range", TYPED);
    let conn = db.conn();
    let range = crate::partition::get_col_range(&conn, "SELECT * FROM t", "score").unwrap();
    assert_eq!(range, (-2, 1));
    let empty =
        crate::partition::get_col_range(&conn, "SELECT * FROM t WHERE id > 100", "id").unwrap();
    assert_eq!(empty, (0, 0));
    let err = crate::partition::get_col_range(&conn, "SELECT * FROM t", "name").unwrap_err();
    assert!(err.to_string().contains("integer columns"), "{err}");
}

#[test]
fn streams_batches() {
    let db = TempDb::new("stream", TYPED);
    let queries = [
        CXQuery::naked("SELECT id, name FROM t WHERE id <= 2"),
        CXQuery::naked("SELECT id, name FROM t WHERE id > 2"),
    ];
    let mut it = new_record_batch_iter(&db.conn(), None, &queries, 1, None).unwrap();
    let (empty, names) = it.get_schema();
    assert_eq!(empty.num_columns(), 2);
    assert_eq!(names, &["id".to_string(), "name".to_string()]);
    it.prepare();
    let mut rows = 0;
    while let Some(b) = it.next_batch() {
        rows += b.unwrap().num_rows();
    }
    assert_eq!(rows, 4);
}

#[test]
fn source_reports_row_count_of_the_origin_query() {
    let db = TempDb::new("count", TYPED);
    let mut src = SQLiteSource::new(&db.path(), 1).unwrap();
    assert_eq!(src.result_rows().unwrap(), None);
    src.set_origin_query(Some("SELECT * FROM t WHERE id > 1".into()));
    assert_eq!(src.result_rows().unwrap(), Some(3));

    src.set_queries(&[CXQuery::naked("SELECT id FROM t")]);
    src.fetch_metadata().unwrap();
    let mut parts = src.partition().unwrap();
    parts[0].result_rows().unwrap();
    assert_eq!(parts[0].nrows(), 4);
    assert_eq!(parts[0].ncols(), 1);
}

#[test]
fn only_row_major_is_supported() {
    let db = TempDb::new("order", TYPED);
    let mut src = SQLiteSource::new(&db.path(), 1).unwrap();
    assert!(src.set_data_order(DataOrder::RowMajor).is_ok());
    assert!(src.set_data_order(DataOrder::ColumnMajor).is_err());
}

// A Windows path must survive the trip through `SourceConn` (a `url::Url`)
// when it is percent-encoded, and the `[9..]` slice that get_arrow/partition
// take must decode back to it. A raw `C:\...` path does not survive parsing.
#[test]
fn percent_encoded_windows_path_round_trips_through_the_url() {
    let path = r"C:\Users\RUNNER~1\AppData\Local\Temp\cx test.db";
    let encoded = urlencoding::encode(path);
    let conn = SourceConn::try_from(format!("sqlite://{}", encoded).as_str()).unwrap();
    let decoded = urlencoding::decode(&conn.conn.as_str()[9..]).unwrap();
    assert_eq!(decoded, path);
}
