use connectorx::prelude::*;
use connectorx::sources::oracle::OracleSource;
use connectorx::sql::CXQuery;

mod test_db;

#[test]
#[ignore]
fn test_types() {
    let _ = env_logger::builder().is_test(true).try_init();
    let dburl = test_db::oracle_url();
    let mut source = OracleSource::new(&dburl, 1).unwrap();
    #[derive(Debug, PartialEq)]
    struct Row(i64, Option<String>, Option<f64>);

    source.set_queries(&[CXQuery::naked(
        "select test_int, test_char, test_float from admin.test_table order by test_int",
    )]);
    source.fetch_metadata().unwrap();
    let mut partitions = source.partition().unwrap();
    assert!(partitions.len() == 1);
    let mut partition = partitions.remove(0);
    partition.result_rows().expect("run query");
    assert_eq!(5, partition.nrows());
    assert_eq!(3, partition.ncols());

    let mut parser = partition.parser().unwrap();

    let mut rows: Vec<Row> = Vec::new();
    loop {
        let (n, is_last) = parser.fetch_next().unwrap();
        for _i in 0..n {
            rows.push(Row(
                parser.produce().unwrap(),
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
            Row(1, Some("str1 ".to_string()), Some(1.1)),
            Row(2, Some("str2 ".to_string()), Some(2.2)),
            Row(4, None, Some(-4.44)),
            Row(5, Some("str05".to_string()), None),
            Row(2333, None, None),
        ],
        rows
    );
}

/// UTC instants of `admin.test_types` (scripts/oracle.sql), sorted, NULL last, in microseconds or nanoseconds. The
/// offset is applied: `1999-12-01 11:00:00 -8:00` is `1999-12-01 19:00:00 UTC`.
#[cfg(feature = "dst_arrow")]
fn expected_instants(values: &[&str], nanos: bool) -> Vec<Option<i64>> {
    use chrono::DateTime;
    values
        .iter()
        .map(|s| {
            let instant = DateTime::parse_from_rfc3339(s).unwrap();
            Some(if nanos {
                instant.timestamp_nanos_opt().unwrap()
            } else {
                instant.timestamp_micros()
            })
        })
        .chain([None])
        .collect()
}

#[cfg(feature = "dst_arrow")]
fn expected_timestamptz() -> Vec<Option<i64>> {
    expected_instants(
        &[
            "1899-12-01T10:00:00Z", // 1899-12-01 11:00:00 +1:00
            "1999-12-01T19:00:00Z", // 1999-12-01 11:00:00 -8:00
            "9999-12-31T10:00:00Z", // 9999-12-31 11:00:00 +1:00
        ],
        false,
    )
}

#[cfg(feature = "dst_arrow")]
const TIMESTAMPTZ_QUERY: &str =
    "select test_timestamptz from admin.test_types order by test_timestamptz nulls last";

/// Reads the single `Timestamp(Microsecond, "+00:00")` column of `batches` as microseconds.
#[cfg(feature = "dst_arrow")]
fn timestamptz_values(batches: &[arrow::record_batch::RecordBatch]) -> Vec<Option<i64>> {
    use arrow::array::{Array, TimestampMicrosecondArray};
    use arrow::datatypes::{DataType, TimeUnit};
    let mut values = Vec::new();
    for batch in batches {
        let column = batch.column(0);
        assert_eq!(
            column.data_type(),
            &DataType::Timestamp(TimeUnit::Microsecond, Some("+00:00".into()))
        );
        let column = column
            .as_any()
            .downcast_ref::<TimestampMicrosecondArray>()
            .unwrap();
        values.extend((0..column.len()).map(|i| column.is_valid(i).then(|| column.value(i))));
    }
    values
}

/// `TIMESTAMP WITH TIME ZONE` through the streaming Arrow destination (used to fail with "Data type unexpected:
/// DateTimeTz(true) expected, DateTimeWrapperMicro found").
#[test]
#[ignore]
#[cfg(feature = "dst_arrow")]
fn test_oracle_timestamptz_arrow_stream() {
    use connectorx::get_arrow::new_record_batch_iter;

    let source_conn = SourceConn::try_from(test_db::oracle_url().as_str()).unwrap();
    let queries = [CXQuery::naked(TIMESTAMPTZ_QUERY)];
    let mut iter = new_record_batch_iter(&source_conn, None, &queries, 2, None).unwrap();
    iter.prepare();
    let mut batches = Vec::new();
    while let Some(batch) = iter.next_batch() {
        batches.push(batch.unwrap());
    }
    assert_eq!(timestamptz_values(&batches), expected_timestamptz());
}

/// Same values through the batch Arrow destination.
#[test]
#[ignore]
#[cfg(feature = "dst_arrow")]
fn test_oracle_timestamptz_arrow() {
    use connectorx::get_arrow::get_arrow;

    let source_conn = SourceConn::try_from(test_db::oracle_url().as_str()).unwrap();
    let queries = [CXQuery::naked(TIMESTAMPTZ_QUERY)];
    let batches = get_arrow(&source_conn, None, &queries, None)
        .unwrap()
        .arrow()
        .unwrap();
    assert_eq!(timestamptz_values(&batches), expected_timestamptz());
}

/// `TIMESTAMP(9) WITH TIME ZONE` through the batch Arrow destination, at nanosecond precision.
#[test]
#[ignore]
#[cfg(feature = "dst_arrow")]
fn test_oracle_timestamptz9_arrow() {
    use arrow::array::{Array, TimestampNanosecondArray};
    use connectorx::get_arrow::get_arrow;

    let source_conn = SourceConn::try_from(test_db::oracle_url().as_str()).unwrap();
    let queries = [CXQuery::naked(
        "select test_timestamptz9 from admin.test_types order by test_timestamptz9 nulls last",
    )];
    let batches = get_arrow(&source_conn, None, &queries, None)
        .unwrap()
        .arrow()
        .unwrap();
    let mut values = Vec::new();
    for batch in &batches {
        let column = batch
            .column(0)
            .as_any()
            .downcast_ref::<TimestampNanosecondArray>()
            .unwrap();
        values.extend((0..column.len()).map(|i| column.is_valid(i).then(|| column.value(i))));
    }
    assert_eq!(
        values,
        expected_instants(
            &[
                "1899-12-01T10:00:00.123456789Z", // 1899-12-01 11:00:00.123456789 +1:00
                "1999-12-01T19:00:00.987654321Z", // 1999-12-01 11:00:00.987654321 -8:00
                "2021-01-01T02:04:05.999999999Z", // 2021-01-01 03:04:05.999999999 +1:00
            ],
            true,
        )
    );
}
