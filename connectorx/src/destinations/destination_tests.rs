// Shared unit tests for the two Arrow destinations. `destinations/arrow` and
// `destinations/arrowstream` expose the same `ArrowDestination` /
// `ArrowTypeSystem` API (one collects batches in a Vec, the other streams them
// through a channel); both include this file with `#[path]`.

use super::typesystem::{DateTimeWrapperMicro, NaiveDateTimeWrapperMicro, NaiveTimeWrapperMicro};
use super::{ArrowDestination, ArrowTypeSystem};
use crate::data_order::DataOrder;
use crate::destinations::{Consume, Destination, DestinationPartition};
use arrow::array::{Array, AsArray, Int64Array};
use arrow::datatypes::Int64Type;
use arrow::record_batch::RecordBatch;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use rust_decimal::Decimal;

fn ts() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2020, 2, 3, 4, 5, 6).unwrap()
}

fn ndt() -> NaiveDateTime {
    ts().naive_utc()
}

fn nt() -> NaiveTime {
    NaiveTime::from_hms_opt(12, 34, 56).unwrap()
}

/// Runs `$m!(Variant, RustType, sample_value)` for every `ArrowTypeSystem`
/// variant, so a variant added without a case here fails to compile in
/// `every_variant_is_listed`.
macro_rules! for_each_variant {
    ($m:ident) => {
        $m!(Int8, i8, -8i8);
        $m!(Int16, i16, -16i16);
        $m!(Int32, i32, -32i32);
        $m!(Int64, i64, -64i64);
        $m!(UInt8, u8, 8u8);
        $m!(UInt16, u16, 16u16);
        $m!(UInt32, u32, 32u32);
        $m!(UInt64, u64, 64u64);
        $m!(Float32, f32, 0.5f32);
        $m!(Float64, f64, 0.25f64);
        $m!(Decimal, Decimal, Decimal::new(12345, 2));
        $m!(Boolean, bool, true);
        $m!(LargeUtf8, String, "s".to_string());
        $m!(LargeBinary, Vec<u8>, vec![1u8, 2]);
        $m!(
            Date32,
            NaiveDate,
            NaiveDate::from_ymd_opt(2020, 2, 3).unwrap()
        );
        $m!(Date64, NaiveDateTime, ndt());
        $m!(
            Date64Micro,
            NaiveDateTimeWrapperMicro,
            NaiveDateTimeWrapperMicro(ndt())
        );
        $m!(Time64, NaiveTime, nt());
        $m!(
            Time64Micro,
            NaiveTimeWrapperMicro,
            NaiveTimeWrapperMicro(nt())
        );
        $m!(DateTimeTz, DateTime<Utc>, ts());
        $m!(
            DateTimeTzMicro,
            DateTimeWrapperMicro,
            DateTimeWrapperMicro(ts())
        );
        $m!(BoolArray, Vec<Option<bool>>, vec![Some(true), None]);
        $m!(
            Utf8Array,
            Vec<Option<String>>,
            vec![Some("a".to_string()), None]
        );
        $m!(Int16Array, Vec<Option<i16>>, vec![Some(1i16), None]);
        $m!(Int32Array, Vec<Option<i32>>, vec![Some(1i32), None]);
        $m!(Int64Array, Vec<Option<i64>>, vec![Some(1i64), None]);
        $m!(UInt16Array, Vec<Option<u16>>, vec![Some(1u16), None]);
        $m!(UInt32Array, Vec<Option<u32>>, vec![Some(1u32), None]);
        $m!(UInt64Array, Vec<Option<u64>>, vec![Some(1u64), None]);
        $m!(Float32Array, Vec<Option<f32>>, vec![Some(1f32), None]);
        $m!(Float64Array, Vec<Option<f64>>, vec![Some(1f64), None]);
        $m!(
            DecimalArray,
            Vec<Option<Decimal>>,
            vec![Some(Decimal::ONE), None]
        );
    };
}

#[test]
fn every_variant_is_listed() {
    fn covered(t: ArrowTypeSystem) -> bool {
        macro_rules! arm {
            ($V:ident, $T:ty, $v:expr) => {
                if let ArrowTypeSystem::$V(_) = t {
                    return true;
                }
            };
        }
        for_each_variant!(arm);
        false
    }
    // Exhaustive match: adding a variant breaks this until it is listed above.
    let all = |t: ArrowTypeSystem| match t {
        ArrowTypeSystem::Int8(_)
        | ArrowTypeSystem::Int16(_)
        | ArrowTypeSystem::Int32(_)
        | ArrowTypeSystem::Int64(_)
        | ArrowTypeSystem::UInt8(_)
        | ArrowTypeSystem::UInt16(_)
        | ArrowTypeSystem::UInt32(_)
        | ArrowTypeSystem::UInt64(_)
        | ArrowTypeSystem::Float32(_)
        | ArrowTypeSystem::Float64(_)
        | ArrowTypeSystem::Decimal(_)
        | ArrowTypeSystem::Boolean(_)
        | ArrowTypeSystem::LargeUtf8(_)
        | ArrowTypeSystem::LargeBinary(_)
        | ArrowTypeSystem::Date32(_)
        | ArrowTypeSystem::Date64(_)
        | ArrowTypeSystem::Date64Micro(_)
        | ArrowTypeSystem::Time64(_)
        | ArrowTypeSystem::Time64Micro(_)
        | ArrowTypeSystem::DateTimeTz(_)
        | ArrowTypeSystem::DateTimeTzMicro(_)
        | ArrowTypeSystem::BoolArray(_)
        | ArrowTypeSystem::Utf8Array(_)
        | ArrowTypeSystem::Int16Array(_)
        | ArrowTypeSystem::Int32Array(_)
        | ArrowTypeSystem::Int64Array(_)
        | ArrowTypeSystem::UInt16Array(_)
        | ArrowTypeSystem::UInt32Array(_)
        | ArrowTypeSystem::UInt64Array(_)
        | ArrowTypeSystem::Float32Array(_)
        | ArrowTypeSystem::Float64Array(_)
        | ArrowTypeSystem::DecimalArray(_) => covered(t),
    };
    assert!(all(ArrowTypeSystem::DecimalArray(true)));
}

/// One non-null and one nullable column per variant.
fn full_schema() -> (Vec<String>, Vec<ArrowTypeSystem>) {
    let mut names = vec![];
    let mut schema = vec![];
    macro_rules! col {
        ($V:ident, $T:ty, $v:expr) => {
            names.push(format!("{}", stringify!($V)));
            schema.push(ArrowTypeSystem::$V(false));
            names.push(format!("{}_null", stringify!($V)));
            schema.push(ArrowTypeSystem::$V(true));
        };
    }
    for_each_variant!(col);
    (names, schema)
}

/// Writes one row of `full_schema`: the non-null column always gets the
/// sample value; the nullable one gets `Some(sample)` on even rows, else `None`.
macro_rules! consume_row {
    ($w:expr, $row:expr) => {{
        macro_rules! val {
            ($V:ident, $T:ty, $v:expr) => {
                Consume::<$T>::consume($w, $v).unwrap();
                let opt: Option<$T> = if $row % 2 == 0 { Some($v) } else { None };
                Consume::<Option<$T>>::consume($w, opt).unwrap();
            };
        }
        for_each_variant!(val);
    }};
}

fn collect(dst: ArrowDestination) -> Vec<RecordBatch> {
    dst.arrow().unwrap()
}

#[test]
fn writes_every_type_and_schema_matches() {
    let (names, schema) = full_schema();
    let mut dst = ArrowDestination::new();
    dst.allocate(0, &names, &schema, DataOrder::RowMajor)
        .unwrap();
    assert!(!dst.needs_count());
    assert_eq!(Destination::schema(&dst), schema.as_slice());
    assert_eq!(dst.names(), names.as_slice());

    let arrow_schema = dst.arrow_schema();
    assert_eq!(arrow_schema.fields().len(), schema.len());
    for (f, (n, t)) in arrow_schema.fields().iter().zip(names.iter().zip(&schema)) {
        assert_eq!(f.name(), n);
        let nullable = n.ends_with("_null");
        assert_eq!(f.is_nullable(), nullable, "{t:?}");
    }
    let empty = dst.empty_batch();
    assert_eq!(empty.num_rows(), 0);
    assert_eq!(empty.schema(), arrow_schema);

    {
        let mut parts = dst.partition(1).unwrap();
        let w = &mut parts[0];
        assert_eq!(w.ncols(), schema.len());
        assert_eq!(w.aquire_row(2).unwrap(), 0);
        consume_row!(w, 0);
        consume_row!(w, 1);
        w.finalize().unwrap();
    }

    let batches = collect(dst);
    assert_eq!(batches.len(), 1);
    let rb = &batches[0];
    assert_eq!(rb.num_rows(), 2);
    assert_eq!(rb.schema(), arrow_schema);
    for (i, f) in arrow_schema.fields().iter().enumerate() {
        let col = rb.column(i);
        let expected_nulls = usize::from(f.is_nullable());
        assert_eq!(col.null_count(), expected_nulls, "{}", f.name());
    }
    let i64s = rb
        .column_by_name("Int64")
        .unwrap()
        .as_primitive::<Int64Type>();
    assert_eq!(i64s, &Int64Array::from(vec![-64, -64]));
    let strs = rb.column_by_name("LargeUtf8_null").unwrap();
    assert_eq!(strs.as_string::<i32>().value(0), "s");
    assert!(strs.is_null(1));
}

#[test]
fn flushes_a_batch_every_batch_size_rows() {
    let names = ["a", "b"];
    let schema = [
        ArrowTypeSystem::Int64(false),
        ArrowTypeSystem::LargeUtf8(true),
    ];
    let mut dst = ArrowDestination::new_with_batch_size(2);
    dst.allocate(0, &names, &schema, DataOrder::RowMajor)
        .unwrap();
    {
        let mut parts = dst.partition(2).unwrap();
        for (p, w) in parts.iter_mut().enumerate() {
            for i in 0..5i64 {
                w.consume(i + 10 * p as i64).unwrap();
                w.consume(Some(i.to_string())).unwrap();
            }
            w.finalize().unwrap();
        }
    }
    let batches = collect(dst);
    // 5 rows per partition at batch size 2 -> 2 + 2 + 1, twice.
    let mut sizes: Vec<_> = batches.iter().map(RecordBatch::num_rows).collect();
    sizes.sort();
    assert_eq!(sizes, vec![1, 1, 2, 2, 2, 2]);
    let mut all: Vec<i64> = batches
        .iter()
        .flat_map(|b| b.column(0).as_primitive::<Int64Type>().values().to_vec())
        .collect();
    all.sort();
    assert_eq!(all, vec![0, 1, 2, 3, 4, 10, 11, 12, 13, 14]);
}

#[test]
fn finalize_without_rows_emits_an_empty_batch() {
    let mut dst = ArrowDestination::new();
    dst.allocate(
        0,
        &["a"],
        &[ArrowTypeSystem::Int32(true)],
        DataOrder::RowMajor,
    )
    .unwrap();
    {
        let mut parts = dst.partition(1).unwrap();
        parts[0].finalize().unwrap();
    }
    let batches = collect(dst);
    assert_eq!(batches.len(), 1);
    assert_eq!(batches[0].num_rows(), 0);
}

#[test]
fn column_major_is_rejected() {
    let mut dst = ArrowDestination::new();
    let err = dst
        .allocate(
            0,
            &["a"],
            &[ArrowTypeSystem::Int64(false)],
            DataOrder::ColumnMajor,
        )
        .unwrap_err();
    assert!(err.to_string().contains("ColumnMajor"), "{err}");
}

#[test]
fn consuming_the_wrong_type_is_an_error() {
    let mut dst = ArrowDestination::new();
    dst.allocate(
        0,
        &["a"],
        &[ArrowTypeSystem::Int64(false)],
        DataOrder::RowMajor,
    )
    .unwrap();
    let mut parts = dst.partition(1).unwrap();
    // Column is Int64(false): a nullable value, or another type, must not be
    // accepted (the builder downcast would otherwise fail later).
    assert!(Consume::<Option<i64>>::consume(&mut parts[0], Some(1)).is_err());
    assert!(Consume::<i32>::consume(&mut parts[0], 1).is_err());
}
