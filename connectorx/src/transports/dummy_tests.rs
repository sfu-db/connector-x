// Shared unit tests for the Dummy -> Arrow transports. `dummy_arrow.rs` and
// `dummy_arrowstream.rs` define the same `DummyArrowTransport` against the two
// Arrow destinations; both include this file with `#[path]`.
//
// The dummy source needs no database, so these tests drive the whole
// `Dispatcher` (prepare, partition, parse, convert, write) in `cargo test --lib`.

use super::*;
use crate::dispatcher::Dispatcher;
use crate::sql::CXQuery;
use crate::typesystem::Transport;
use arrow::array::{Array, AsArray, BooleanArray};
use arrow::datatypes::{DataType, Float64Type, Int64Type, TimeUnit};
use arrow::record_batch::RecordBatch;

fn queries(nrows: &[usize], ncols: usize) -> Vec<CXQuery> {
    nrows
        .iter()
        .map(|n| CXQuery::naked(format!("{n},{ncols}")))
        .collect()
}

fn run(names: &[&str], schema: &[DummyTypeSystem], nrows: &[usize]) -> Vec<RecordBatch> {
    let mut dst = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, DummyArrowTransport>::new(
        DummySource::new(names, schema),
        &mut dst,
        &queries(nrows, schema.len()),
        None,
    );
    dispatcher.run().expect("run dispatcher");
    let mut batches = dst.arrow().unwrap();
    batches.sort_by_key(RecordBatch::num_rows);
    batches
}

#[test]
fn dispatcher_writes_every_dummy_type() {
    let schema = [
        DummyTypeSystem::I64(false),
        DummyTypeSystem::F64(true),
        DummyTypeSystem::Bool(false),
        DummyTypeSystem::Bool(true),
        DummyTypeSystem::String(true),
        DummyTypeSystem::DateTime(false),
    ];
    let names = ["i", "f", "b", "bn", "s", "t"];
    let batches = run(&names, &schema, &[2, 3]);
    assert_eq!(batches.len(), 2);

    let rb = &batches[1];
    assert_eq!(rb.num_rows(), 3);
    let fields: Vec<_> = rb
        .schema()
        .fields()
        .iter()
        .map(|f| f.name().clone())
        .collect();
    assert_eq!(fields, names);

    // The dummy parser yields the row index for every column.
    assert_eq!(
        rb.column(0).as_primitive::<Int64Type>().values(),
        &[0, 1, 2]
    );
    assert_eq!(
        rb.column(1).as_primitive::<Float64Type>().values(),
        &[0.0, 1.0, 2.0]
    );
    assert_eq!(
        rb.column(2).as_boolean(),
        &BooleanArray::from(vec![true, false, true])
    );
    // Option<bool> cycles Some(true), Some(false), None.
    assert_eq!(
        rb.column(3).as_boolean(),
        &BooleanArray::from(vec![Some(true), Some(false), None])
    );
    let s = rb.column(4).as_string::<i32>();
    assert_eq!((s.value(0), s.value(2)), ("0", "2"));
    // DateTime goes through the `option` conversion to Date64Micro.
    assert_eq!(
        rb.column(5).data_type(),
        &DataType::Timestamp(TimeUnit::Microsecond, None)
    );
    assert_eq!(rb.column(5).null_count(), 0);
}

#[test]
#[should_panic(expected = "pre_execution_queries is not implemented")]
fn pre_execution_queries_are_forwarded_to_the_source() {
    // The dispatcher hands them to the source; the dummy source keeps the
    // default `Source::set_pre_execution_queries`, which refuses them.
    let mut dst = ArrowDestination::new();
    let mut dispatcher = Dispatcher::<_, _, DummyArrowTransport>::new(
        DummySource::new(&["a"], &[DummyTypeSystem::I64(false)]),
        &mut dst,
        &queries(&[1], 1),
        None,
    );
    dispatcher.set_pre_execution_queries(Some(&["SET x = 1".to_string()]));
}

#[test]
fn nullable_datetime_produces_nulls() {
    let batches = run(&["t"], &[DummyTypeSystem::DateTime(true)], &[4]);
    let col = batches[0].column(0);
    assert_eq!(col.len(), 4);
    assert!(col.null_count() > 0);
    assert!(batches[0].schema().field(0).is_nullable());
}

#[test]
fn get_meta_allocates_the_schema_without_rows() {
    let schema = [DummyTypeSystem::I64(true), DummyTypeSystem::String(false)];
    let mut dst = ArrowDestination::new();
    let mut dispatcher = Dispatcher::<_, _, DummyArrowTransport>::new(
        DummySource::new(&["a", "b"], &schema),
        &mut dst,
        &queries(&[5], 2),
        Some("SELECT origin".to_string()),
    );
    dispatcher.get_meta().unwrap();
    assert_eq!(dst.names(), &["a".to_string(), "b".to_string()]);
    let empty = dst.empty_batch();
    assert_eq!(empty.num_rows(), 0);
    assert_eq!(empty.schema().field(0).data_type(), &DataType::Int64);
    assert!(empty.schema().field(0).is_nullable());
    // ArrowTypeSystem::LargeUtf8 is written with a StringBuilder, i.e. Arrow Utf8.
    assert_eq!(empty.schema().field(1).data_type(), &DataType::Utf8);
    assert!(!empty.schema().field(1).is_nullable());
}

#[test]
fn typesystem_mapping_keeps_nullability() {
    let cases = [
        (DummyTypeSystem::F64(true), ArrowTypeSystem::Float64(true)),
        (DummyTypeSystem::I64(false), ArrowTypeSystem::Int64(false)),
        (DummyTypeSystem::Bool(true), ArrowTypeSystem::Boolean(true)),
        (
            DummyTypeSystem::String(false),
            ArrowTypeSystem::LargeUtf8(false),
        ),
        (
            DummyTypeSystem::DateTime(true),
            ArrowTypeSystem::Date64Micro(true),
        ),
    ];
    for (src, dst) in cases {
        assert_eq!(
            <DummyArrowTransport as Transport>::convert_typesystem(src).unwrap(),
            dst
        );
    }
}

#[test]
fn date_conversions() {
    let dt = NaiveDate::from_ymd_opt(2001, 2, 3)
        .unwrap()
        .and_hms_opt(4, 5, 6)
        .unwrap();
    let utc: DateTime<Utc> = <DummyArrowTransport as TypeConversion<_, _>>::convert(dt);
    assert_eq!(utc.naive_utc(), dt);
    let wrapped: NaiveDateTimeWrapperMicro =
        <DummyArrowTransport as TypeConversion<_, _>>::convert(utc);
    assert_eq!(wrapped.0, dt);
    let midnight: DateTime<Utc> = <DummyArrowTransport as TypeConversion<_, _>>::convert(dt.date());
    assert_eq!(
        midnight.naive_utc(),
        dt.date().and_hms_opt(0, 0, 0).unwrap()
    );
}
