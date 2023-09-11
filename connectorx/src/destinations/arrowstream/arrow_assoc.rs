use super::errors::{ArrowDestinationError, Result};
use crate::constants::SECONDS_IN_DAY;
use arrow::array::{
    ArrayBuilder, BooleanBuilder, Date32Builder, Date64Builder, Float32Builder, Float64Builder,
    Int32Builder, Int64Builder, LargeBinaryBuilder, StringBuilder, Time64NanosecondBuilder,
    TimestampNanosecondBuilder, UInt32Builder, UInt64Builder,
};
use arrow::datatypes::Field;
use arrow::datatypes::{DataType as ArrowDataType, TimeUnit};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
use fehler::throws;

/// Associate arrow builder with native type
pub trait ArrowAssoc {
    type Builder: ArrayBuilder + Send;

    fn builder(nrows: usize) -> Self::Builder;
    fn append(builder: &mut Self::Builder, value: Self) -> Result<()>;
    fn field(header: &str) -> Field;
}

macro_rules! impl_arrow_assoc {
    ($T:ty, $AT:expr, $B:ty) => {
        impl ArrowAssoc for $T {
            type Builder = $B;

            fn builder(nrows: usize) -> Self::Builder {
                Self::Builder::with_capacity(nrows)
            }

            #[throws(ArrowDestinationError)]
            fn append(builder: &mut Self::Builder, value: Self) {
                builder.append_value(value);
            }

            fn field(header: &str) -> Field {
                Field::new(header, $AT, false)
            }
        }

        impl ArrowAssoc for Option<$T> {
            type Builder = $B;

            fn builder(nrows: usize) -> Self::Builder {
                Self::Builder::with_capacity(nrows)
            }

            #[throws(ArrowDestinationError)]
            fn append(builder: &mut Self::Builder, value: Self) {
                builder.append_option(value);
            }

            fn field(header: &str) -> Field {
                Field::new(header, $AT, true)
            }
        }
    };
}

impl_arrow_assoc!(u32, ArrowDataType::UInt32, UInt32Builder);
impl_arrow_assoc!(u64, ArrowDataType::UInt64, UInt64Builder);
impl_arrow_assoc!(i32, ArrowDataType::Int32, Int32Builder);
impl_arrow_assoc!(i64, ArrowDataType::Int64, Int64Builder);
impl_arrow_assoc!(f32, ArrowDataType::Float32, Float32Builder);
impl_arrow_assoc!(f64, ArrowDataType::Float64, Float64Builder);
impl_arrow_assoc!(bool, ArrowDataType::Boolean, BooleanBuilder);

impl ArrowAssoc for &str {
    type Builder = StringBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        StringBuilder::with_capacity(1024, nrows)
    }

    #[throws(ArrowDestinationError)]
    fn append(builder: &mut Self::Builder, value: Self) {
        builder.append_value(value);
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Utf8, false)
    }
}

impl ArrowAssoc for Option<&str> {
    type Builder = StringBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        StringBuilder::with_capacity(1024, nrows)
    }

    #[throws(ArrowDestinationError)]
    fn append(builder: &mut Self::Builder, value: Self) {
        match value {
            Some(s) => builder.append_value(s),
            None => builder.append_null(),
        }
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Utf8, true)
    }
}

impl ArrowAssoc for String {
    type Builder = StringBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        StringBuilder::with_capacity(1024, nrows)
    }

    #[throws(ArrowDestinationError)]
    fn append(builder: &mut Self::Builder, value: String) {
        builder.append_value(value.as_str());
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Utf8, false)
    }
}

impl ArrowAssoc for Option<String> {
    type Builder = StringBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        StringBuilder::with_capacity(1024, nrows)
    }

    #[throws(ArrowDestinationError)]
    fn append(builder: &mut Self::Builder, value: Self) {
        match value {
            Some(s) => builder.append_value(s.as_str()),
            None => builder.append_null(),
        }
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Utf8, true)
    }
}

impl ArrowAssoc for DateTime<Utc> {
    type Builder = TimestampNanosecondBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        TimestampNanosecondBuilder::with_capacity(nrows)
    }

    #[throws(ArrowDestinationError)]
    fn append(builder: &mut Self::Builder, value: DateTime<Utc>) {
        builder.append_value(value.timestamp_nanos())
    }

    fn field(header: &str) -> Field {
        Field::new(
            header,
            ArrowDataType::Timestamp(TimeUnit::Nanosecond, None),
            false,
        )
    }
}

impl ArrowAssoc for Option<DateTime<Utc>> {
    type Builder = TimestampNanosecondBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        TimestampNanosecondBuilder::with_capacity(nrows)
    }

    #[throws(ArrowDestinationError)]
    fn append(builder: &mut Self::Builder, value: Option<DateTime<Utc>>) {
        builder.append_option(value.map(|x| x.timestamp_nanos()))
    }

    fn field(header: &str) -> Field {
        Field::new(
            header,
            ArrowDataType::Timestamp(TimeUnit::Nanosecond, None),
            true,
        )
    }
}

fn naive_date_to_arrow(nd: NaiveDate) -> i32 {
    match nd.and_hms_opt(0, 0, 0) {
        Some(dt) => (dt.timestamp() / SECONDS_IN_DAY) as i32,
        None => panic!("and_hms_opt got None from {:?}", nd),
    }
}

fn naive_datetime_to_arrow(nd: NaiveDateTime) -> i64 {
    nd.timestamp_millis()
}

impl ArrowAssoc for Option<NaiveDate> {
    type Builder = Date32Builder;

    fn builder(nrows: usize) -> Self::Builder {
        Date32Builder::with_capacity(nrows)
    }

    fn append(builder: &mut Self::Builder, value: Option<NaiveDate>) -> Result<()> {
        builder.append_option(value.map(naive_date_to_arrow));
        Ok(())
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Date32, true)
    }
}

impl ArrowAssoc for NaiveDate {
    type Builder = Date32Builder;

    fn builder(nrows: usize) -> Self::Builder {
        Date32Builder::with_capacity(nrows)
    }

    fn append(builder: &mut Self::Builder, value: NaiveDate) -> Result<()> {
        builder.append_value(naive_date_to_arrow(value));
        Ok(())
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Date32, false)
    }
}

impl ArrowAssoc for Option<NaiveDateTime> {
    type Builder = Date64Builder;

    fn builder(nrows: usize) -> Self::Builder {
        Date64Builder::with_capacity(nrows)
    }

    fn append(builder: &mut Self::Builder, value: Option<NaiveDateTime>) -> Result<()> {
        builder.append_option(value.map(naive_datetime_to_arrow));
        Ok(())
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Date64, true)
    }
}

impl ArrowAssoc for NaiveDateTime {
    type Builder = Date64Builder;

    fn builder(nrows: usize) -> Self::Builder {
        Date64Builder::with_capacity(nrows)
    }

    fn append(builder: &mut Self::Builder, value: NaiveDateTime) -> Result<()> {
        builder.append_value(naive_datetime_to_arrow(value));
        Ok(())
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Date64, false)
    }
}

impl ArrowAssoc for Option<NaiveTime> {
    type Builder = Time64NanosecondBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        Time64NanosecondBuilder::with_capacity(nrows)
    }

    fn append(builder: &mut Self::Builder, value: Option<NaiveTime>) -> Result<()> {
        builder.append_option(
            value.map(|t| {
                t.num_seconds_from_midnight() as i64 * 1_000_000_000 + t.nanosecond() as i64
            }),
        );
        Ok(())
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Time64(TimeUnit::Nanosecond), true)
    }
}

impl ArrowAssoc for NaiveTime {
    type Builder = Time64NanosecondBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        Time64NanosecondBuilder::with_capacity(nrows)
    }

    fn append(builder: &mut Self::Builder, value: NaiveTime) -> Result<()> {
        builder.append_value(
            value.num_seconds_from_midnight() as i64 * 1_000_000_000 + value.nanosecond() as i64,
        );
        Ok(())
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Time64(TimeUnit::Nanosecond), false)
    }
}

impl ArrowAssoc for Option<Vec<u8>> {
    type Builder = LargeBinaryBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        LargeBinaryBuilder::with_capacity(1024, nrows)
    }

    fn append(builder: &mut Self::Builder, value: Self) -> Result<()> {
        match value {
            Some(v) => builder.append_value(v),
            None => builder.append_null(),
        };
        Ok(())
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::LargeBinary, true)
    }
}

impl ArrowAssoc for Vec<u8> {
    type Builder = LargeBinaryBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        LargeBinaryBuilder::with_capacity(1024, nrows)
    }

    fn append(builder: &mut Self::Builder, value: Self) -> Result<()> {
        builder.append_value(value);
        Ok(())
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::LargeBinary, false)
    }
}
