// Shared unit tests for the `ArrowAssoc` impls. `destinations/arrow` and
// `destinations/arrowstream` each have their own (near-identical) copy of
// `arrow_assoc.rs`; both include this file with `#[path]`, so `super::*`
// resolves to whichever copy is being tested.
//
// Every case checks the invariant the destinations rely on: the `Field`
// returned by `T::field` has the same data type and nullability as the array
// that `T::builder` + `T::append` produce, because `RecordBatch::try_new`
// rejects a column whose type differs from its schema field.

use super::*;
use arrow::array::{
    Array, ArrayRef, AsArray, BooleanArray, Date32Array, Decimal128Array, Float32Array,
    Float64Array, Int16Array, Int32Array, Int64Array, Int8Array, LargeBinaryArray, LargeListArray,
    StringArray, Time64MicrosecondArray, Time64NanosecondArray, TimestampMicrosecondArray,
    TimestampNanosecondArray, UInt16Array, UInt32Array, UInt64Array, UInt8Array,
};
use chrono::TimeZone;
use std::str::FromStr;

fn build<T: ArrowAssoc>(vals: Vec<T>) -> (Field, ArrayRef) {
    let mut builder = T::builder(vals.len());
    for v in vals {
        T::append(&mut builder, v).unwrap();
    }
    (T::field("c"), builder.finish())
}

/// Build the column and check that it agrees with its declared field.
fn check<T: ArrowAssoc>(vals: Vec<T>, nulls: usize) -> ArrayRef {
    let n = vals.len();
    let (field, arr) = build(vals);
    assert_eq!(field.name(), "c");
    assert_eq!(field.data_type(), arr.data_type(), "field vs array type");
    assert_eq!(arr.len(), n);
    assert_eq!(arr.null_count(), nulls);
    if nulls > 0 {
        assert!(field.is_nullable(), "column with nulls on non-null field");
    }
    arr
}

fn dec(s: &str) -> Decimal {
    Decimal::from_str(s).unwrap()
}

macro_rules! primitive_case {
    ($name:ident, $T:ty, $A:ty, $a:expr, $b:expr) => {
        #[test]
        fn $name() {
            let arr = check::<$T>(vec![$a, $b], 0);
            let arr = arr.as_any().downcast_ref::<$A>().unwrap();
            assert_eq!(arr.value(0), $a);
            assert_eq!(arr.value(1), $b);
            assert!(!<$T>::field("c").is_nullable());

            let arr = check::<Option<$T>>(vec![Some($a), None], 1);
            let arr = arr.as_any().downcast_ref::<$A>().unwrap();
            assert_eq!(arr.value(0), $a);
            assert!(arr.is_null(1));
            assert!(<Option<$T>>::field("c").is_nullable());
        }
    };
}

primitive_case!(u8_column, u8, UInt8Array, 0u8, u8::MAX);
primitive_case!(u16_column, u16, UInt16Array, 1u16, u16::MAX);
primitive_case!(u32_column, u32, UInt32Array, 2u32, u32::MAX);
primitive_case!(u64_column, u64, UInt64Array, 3u64, u64::MAX);
primitive_case!(i8_column, i8, Int8Array, i8::MIN, i8::MAX);
primitive_case!(i16_column, i16, Int16Array, i16::MIN, i16::MAX);
primitive_case!(i32_column, i32, Int32Array, i32::MIN, i32::MAX);
primitive_case!(i64_column, i64, Int64Array, i64::MIN, i64::MAX);
primitive_case!(f32_column, f32, Float32Array, -1.5f32, 2.25f32);
primitive_case!(f64_column, f64, Float64Array, -1.5f64, 2.25f64);
primitive_case!(bool_column, bool, BooleanArray, true, false);

#[test]
fn decimal_column_is_rescaled_to_default_scale() {
    let arr = check::<Decimal>(vec![dec("1.5"), dec("-0.0000000001")], 0);
    let arr = arr.as_any().downcast_ref::<Decimal128Array>().unwrap();
    assert_eq!(arr.data_type(), &DEFAULT_ARROW_DECIMAL);
    // Stored as mantissa at DEFAULT_ARROW_DECIMAL_SCALE (10).
    assert_eq!(arr.value(0), 15_000_000_000);
    assert_eq!(arr.value(1), -1);
    assert_eq!(arr.value_as_string(0), "1.5000000000");

    let arr = check::<Option<Decimal>>(vec![None, Some(dec("2"))], 1);
    let arr = arr.as_any().downcast_ref::<Decimal128Array>().unwrap();
    assert!(arr.is_null(0));
    assert_eq!(arr.value(1), 20_000_000_000);
}

#[test]
fn decimal_with_more_than_default_scale_is_rounded() {
    // rust_decimal::rescale rounds when narrowing the scale.
    let (_, arr) = build::<Decimal>(vec![dec("0.12345678901")]);
    let arr = arr.as_any().downcast_ref::<Decimal128Array>().unwrap();
    assert_eq!(arr.value(0), 1_234_567_890);
}

#[test]
fn decimal_that_cannot_be_rescaled_is_an_error() {
    // 28 integer digits leave no room for 10 fractional ones in rust_decimal's
    // 96-bit mantissa, so rescale stops short and append must fail instead of
    // writing a value at the wrong scale.
    let big = dec("1234567890123456789012345678");
    let mut builder = <Decimal as ArrowAssoc>::builder(1);
    assert!(<Decimal as ArrowAssoc>::append(&mut builder, big).is_err());
    let mut builder = <Option<Decimal> as ArrowAssoc>::builder(1);
    assert!(<Option<Decimal> as ArrowAssoc>::append(&mut builder, Some(big)).is_err());
}

#[test]
fn string_columns() {
    let arr = check::<&str>(vec!["a", ""], 0);
    let arr = arr.as_any().downcast_ref::<StringArray>().unwrap();
    assert_eq!((arr.value(0), arr.value(1)), ("a", ""));

    let arr = check::<Option<&str>>(vec![Some("ü"), None], 1);
    assert_eq!(arr.as_string::<i32>().value(0), "ü");

    let arr = check::<String>(vec!["x".to_string()], 0);
    assert_eq!(arr.as_string::<i32>().value(0), "x");

    let arr = check::<Option<String>>(vec![None, Some("y".to_string())], 1);
    assert_eq!(arr.as_string::<i32>().value(1), "y");
}

#[test]
fn binary_columns() {
    let arr = check::<Vec<u8>>(vec![vec![0, 1, 2], vec![]], 0);
    let arr = arr.as_any().downcast_ref::<LargeBinaryArray>().unwrap();
    assert_eq!(arr.value(0), &[0, 1, 2]);
    assert!(arr.value(1).is_empty());

    let arr = check::<Option<Vec<u8>>>(vec![None, Some(vec![255])], 1);
    let arr = arr.as_any().downcast_ref::<LargeBinaryArray>().unwrap();
    assert_eq!(arr.value(1), &[255]);
}

#[test]
fn datetime_utc_is_nanoseconds() {
    let t = Utc.with_ymd_and_hms(2021, 3, 4, 5, 6, 7).unwrap();
    let arr = check::<DateTime<Utc>>(vec![t], 0);
    let arr = arr
        .as_any()
        .downcast_ref::<TimestampNanosecondArray>()
        .unwrap();
    assert_eq!(arr.value(0), t.timestamp_nanos_opt().unwrap());

    let arr = check::<Option<DateTime<Utc>>>(vec![None, Some(t)], 1);
    let arr = arr
        .as_any()
        .downcast_ref::<TimestampNanosecondArray>()
        .unwrap();
    assert_eq!(arr.value(1), t.timestamp_nanos_opt().unwrap());
}

#[test]
#[should_panic(expected = "out of range DateTime")]
fn datetime_utc_outside_nanosecond_range_panics() {
    // i64 nanoseconds only reach years 1677..2262.
    let t = Utc.with_ymd_and_hms(3000, 1, 1, 0, 0, 0).unwrap();
    let _ = build::<DateTime<Utc>>(vec![t]);
}

#[test]
fn datetime_micro_wrapper_is_utc_microseconds() {
    // Microsecond precision covers dates i64 nanoseconds cannot.
    let t = Utc.with_ymd_and_hms(3000, 1, 1, 0, 0, 1).unwrap();
    let arr = check::<DateTimeWrapperMicro>(vec![DateTimeWrapperMicro(t)], 0);
    let arr = arr
        .as_any()
        .downcast_ref::<TimestampMicrosecondArray>()
        .unwrap();
    assert_eq!(arr.value(0), t.timestamp_micros());
    assert_eq!(arr.timezone(), Some("+00:00"));

    let arr = check::<Option<DateTimeWrapperMicro>>(vec![Some(DateTimeWrapperMicro(t)), None], 1);
    let arr = arr
        .as_any()
        .downcast_ref::<TimestampMicrosecondArray>()
        .unwrap();
    assert_eq!(arr.value(0), t.timestamp_micros());
}

#[test]
fn naive_date_is_days_since_epoch() {
    let d = NaiveDate::from_ymd_opt(1970, 1, 11).unwrap();
    let before = NaiveDate::from_ymd_opt(1969, 12, 31).unwrap();
    let arr = check::<NaiveDate>(vec![d, before], 0);
    let arr = arr.as_any().downcast_ref::<Date32Array>().unwrap();
    assert_eq!(arr.value(0), 10);
    assert_eq!(arr.value(1), -1);

    let arr = check::<Option<NaiveDate>>(vec![None, Some(d)], 1);
    let arr = arr.as_any().downcast_ref::<Date32Array>().unwrap();
    assert_eq!(arr.value(1), 10);
}

#[test]
fn naive_datetime_is_nanoseconds_without_timezone() {
    let dt = NaiveDate::from_ymd_opt(1970, 1, 2)
        .unwrap()
        .and_hms_nano_opt(0, 0, 1, 5)
        .unwrap();
    let arr = check::<NaiveDateTime>(vec![dt], 0);
    let arr = arr
        .as_any()
        .downcast_ref::<TimestampNanosecondArray>()
        .unwrap();
    assert_eq!(arr.value(0), 86_401_000_000_005);
    assert_eq!(arr.timezone(), None);

    let arr = check::<Option<NaiveDateTime>>(vec![Some(dt), None], 1);
    let arr = arr
        .as_any()
        .downcast_ref::<TimestampNanosecondArray>()
        .unwrap();
    assert_eq!(arr.value(0), 86_401_000_000_005);
}

#[test]
fn naive_datetime_micro_wrapper_is_microseconds() {
    let dt = NaiveDate::from_ymd_opt(1970, 1, 1)
        .unwrap()
        .and_hms_micro_opt(0, 0, 2, 7)
        .unwrap();
    let arr = check::<NaiveDateTimeWrapperMicro>(vec![NaiveDateTimeWrapperMicro(dt)], 0);
    let arr = arr
        .as_any()
        .downcast_ref::<TimestampMicrosecondArray>()
        .unwrap();
    assert_eq!(arr.value(0), 2_000_007);

    let arr = check::<Option<NaiveDateTimeWrapperMicro>>(
        vec![None, Some(NaiveDateTimeWrapperMicro(dt))],
        1,
    );
    let arr = arr
        .as_any()
        .downcast_ref::<TimestampMicrosecondArray>()
        .unwrap();
    assert_eq!(arr.value(1), 2_000_007);
}

#[test]
fn naive_time_is_nanoseconds_since_midnight() {
    let t = NaiveTime::from_hms_nano_opt(1, 0, 2, 3).unwrap();
    let arr = check::<NaiveTime>(vec![t], 0);
    let arr = arr
        .as_any()
        .downcast_ref::<Time64NanosecondArray>()
        .unwrap();
    assert_eq!(arr.value(0), 3_602_000_000_003);

    let arr = check::<Option<NaiveTime>>(vec![Some(t), None], 1);
    let arr = arr
        .as_any()
        .downcast_ref::<Time64NanosecondArray>()
        .unwrap();
    assert_eq!(arr.value(0), 3_602_000_000_003);
}

#[test]
fn naive_time_micro_wrapper_truncates_to_microseconds() {
    let t = NaiveTime::from_hms_nano_opt(0, 0, 1, 2_999).unwrap();
    let arr = check::<NaiveTimeWrapperMicro>(vec![NaiveTimeWrapperMicro(t)], 0);
    let arr = arr
        .as_any()
        .downcast_ref::<Time64MicrosecondArray>()
        .unwrap();
    assert_eq!(arr.value(0), 1_000_002);

    let arr = check::<Option<NaiveTimeWrapperMicro>>(vec![None, Some(NaiveTimeWrapperMicro(t))], 1);
    let arr = arr
        .as_any()
        .downcast_ref::<Time64MicrosecondArray>()
        .unwrap();
    assert_eq!(arr.value(1), 1_000_002);
}

fn list(arr: &ArrayRef) -> &LargeListArray {
    arr.as_any().downcast_ref::<LargeListArray>().unwrap()
}

macro_rules! list_case {
    ($name:ident, $E:ty, $A:ty, $a:expr) => {
        #[test]
        fn $name() {
            let arr = check::<Vec<Option<$E>>>(vec![vec![Some($a), None], vec![]], 0);
            let l = list(&arr);
            assert_eq!(l.value_length(0), 2);
            assert_eq!(l.value_length(1), 0);
            let first = l.value(0);
            let inner = first.as_any().downcast_ref::<$A>().unwrap();
            assert_eq!(inner.value(0), $a);
            assert!(inner.is_null(1));

            let arr = check::<Option<Vec<Option<$E>>>>(vec![None, Some(vec![Some($a)])], 1);
            let l = list(&arr);
            assert!(l.is_null(0));
            assert_eq!(l.value_length(1), 1);
        }
    };
}

list_case!(bool_list_column, bool, BooleanArray, true);
list_case!(i16_list_column, i16, Int16Array, -2i16);
list_case!(i32_list_column, i32, Int32Array, -3i32);
list_case!(i64_list_column, i64, Int64Array, -4i64);
list_case!(u16_list_column, u16, UInt16Array, 2u16);
list_case!(u32_list_column, u32, UInt32Array, 3u32);
list_case!(u64_list_column, u64, UInt64Array, 4u64);
list_case!(f32_list_column, f32, Float32Array, 0.5f32);
list_case!(f64_list_column, f64, Float64Array, 0.25f64);

#[test]
fn string_list_column() {
    let arr = check::<Vec<Option<String>>>(vec![vec![Some("a".into()), None]], 0);
    let first = list(&arr).value(0);
    assert_eq!(first.as_string::<i32>().value(0), "a");
    assert!(first.is_null(1));

    let arr = check::<Option<Vec<Option<String>>>>(vec![None, Some(vec![])], 1);
    assert_eq!(list(&arr).value_length(1), 0);
}

#[test]
fn decimal_list_column() {
    let arr =
        check::<Option<Vec<Option<Decimal>>>>(vec![Some(vec![Some(dec("1.1")), None]), None], 1);
    let l = list(&arr);
    let first = l.value(0);
    let inner = first.as_any().downcast_ref::<Decimal128Array>().unwrap();
    assert_eq!(inner.value(0), 11_000_000_000);
    assert!(inner.is_null(1));
    assert!(l.is_null(1));

    let big = dec("1234567890123456789012345678");
    let mut b = <Option<Vec<Option<Decimal>>> as ArrowAssoc>::builder(1);
    assert!(
        <Option<Vec<Option<Decimal>>> as ArrowAssoc>::append(&mut b, Some(vec![Some(big)]))
            .is_err()
    );
}

#[test]
fn non_null_decimal_list_column() {
    // Regression: the item field used to be declared non-null, so a non-null
    // decimal[] column failed RecordBatch::try_new in both destinations.
    let arr = check::<Vec<Option<Decimal>>>(vec![vec![Some(dec("-2")), None]], 0);
    assert!(!<Vec<Option<Decimal>>>::field("c").is_nullable());
    let first = list(&arr).value(0);
    let inner = first.as_any().downcast_ref::<Decimal128Array>().unwrap();
    assert_eq!(inner.value(0), -20_000_000_000);
    assert!(inner.is_null(1));

    let big = dec("1234567890123456789012345678");
    let mut b = <Vec<Option<Decimal>> as ArrowAssoc>::builder(1);
    assert!(<Vec<Option<Decimal>> as ArrowAssoc>::append(&mut b, vec![Some(big)]).is_err());
}
