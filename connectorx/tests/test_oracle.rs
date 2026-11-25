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

/// `TIMESTAMP WITH TIME ZONE` values of `admin.test_types` (scripts/oracle.sql), sorted, NULL last. The source
/// returns the wall-clock time without applying the offset, as the pandas destination does (`test_oracle_types` in
/// connectorx-python): `1999-12-01 11:00:00 -8:00` reads as `1999-12-01 11:00:00`.
#[cfg(feature = "dst_arrow")]
fn expected_timestamptz() -> Vec<Option<i64>> {
    use chrono::DateTime;
    [
        "1899-12-01T11:00:00Z",
        "1999-12-01T11:00:00Z",
        "9999-12-31T11:00:00Z",
    ]
    .iter()
    .map(|s| Some(DateTime::parse_from_rfc3339(s).unwrap().timestamp_micros()))
    .chain([None])
    .collect()
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
    let queries = [CXQuery::naked(TIMESTAMPTZ_QUERY.to_string())];
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
    let queries = [CXQuery::naked(TIMESTAMPTZ_QUERY.to_string())];
    let batches = get_arrow(&source_conn, None, &queries, None)
        .unwrap()
        .arrow()
        .unwrap();
    assert_eq!(timestamptz_values(&batches), expected_timestamptz());
}
