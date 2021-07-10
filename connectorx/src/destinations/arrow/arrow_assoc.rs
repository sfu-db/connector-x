use crate::constants::SECONDS_IN_DAY;
use crate::errors::{ConnectorAgentError, Result};
use arrow::array::{
    ArrayBuilder, BooleanBuilder, Date32Builder, Date64Builder, Float32Builder, Float64Builder,
    Int32Builder, Int64Builder, LargeStringBuilder, UInt32Builder, UInt64Builder,
};
use arrow::datatypes::Field;
use arrow::datatypes::{DataType as ArrowDataType, DateUnit};
use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, Utc};
use fehler::throws;

/// Associate arrow builder with native type
pub trait ArrowAssoc {
    type Builder: ArrayBuilder + Send;

    fn builder(nrows: usize) -> Self::Builder;
    fn append(builder: &mut Self::Builder, value: Self) -> Result<()>;
    fn field(header: &str) -> Field;
}

impl ArrowAssoc for u32 {
    type Builder = UInt32Builder;

    fn builder(nrows: usize) -> UInt32Builder {
        UInt32Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut UInt32Builder, value: u32) {
        builder.append_value(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::UInt64, false)
    }
}

impl ArrowAssoc for Option<u32> {
    type Builder = UInt32Builder;

    fn builder(nrows: usize) -> UInt32Builder {
        UInt32Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut UInt32Builder, value: Option<u32>) {
        builder.append_option(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::UInt32, true)
    }
}

impl ArrowAssoc for u64 {
    type Builder = UInt64Builder;

    fn builder(nrows: usize) -> UInt64Builder {
        UInt64Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut UInt64Builder, value: u64) {
        builder.append_value(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::UInt64, false)
    }
}

impl ArrowAssoc for Option<u64> {
    type Builder = UInt64Builder;

    fn builder(nrows: usize) -> UInt64Builder {
        UInt64Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut UInt64Builder, value: Option<u64>) {
        builder.append_option(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::UInt64, true)
    }
}

impl ArrowAssoc for i32 {
    type Builder = Int32Builder;

    fn builder(nrows: usize) -> Int32Builder {
        Int32Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Int32Builder, value: i32) {
        builder.append_value(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Int32, false)
    }
}

impl ArrowAssoc for Option<i32> {
    type Builder = Int32Builder;

    fn builder(nrows: usize) -> Int32Builder {
        Int32Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Int32Builder, value: Option<i32>) {
        builder.append_option(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Int32, true)
    }
}

impl ArrowAssoc for i64 {
    type Builder = Int64Builder;

    fn builder(nrows: usize) -> Int64Builder {
        Int64Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Int64Builder, value: i64) {
        builder.append_value(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Int64, false)
    }
}

impl ArrowAssoc for Option<i64> {
    type Builder = Int64Builder;

    fn builder(nrows: usize) -> Int64Builder {
        Int64Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Int64Builder, value: Option<i64>) {
        builder.append_option(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Int64, false)
    }
}

impl ArrowAssoc for f32 {
    type Builder = Float32Builder;

    fn builder(nrows: usize) -> Float32Builder {
        Float32Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: f32) {
        builder.append_value(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Float64, false)
    }
}

impl ArrowAssoc for Option<f32> {
    type Builder = Float32Builder;

    fn builder(nrows: usize) -> Float32Builder {
        Float32Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: Option<f32>) {
        builder.append_option(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Float32, true)
    }
}

impl ArrowAssoc for f64 {
    type Builder = Float64Builder;

    fn builder(nrows: usize) -> Float64Builder {
        Float64Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: f64) {
        builder.append_value(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Float64, false)
    }
}

impl ArrowAssoc for Option<f64> {
    type Builder = Float64Builder;

    fn builder(nrows: usize) -> Float64Builder {
        Float64Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: Option<f64>) {
        builder.append_option(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Float64, true)
    }
}

impl ArrowAssoc for bool {
    type Builder = BooleanBuilder;

    fn builder(nrows: usize) -> BooleanBuilder {
        BooleanBuilder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: bool) {
        builder.append_value(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Boolean, false)
    }
}

impl ArrowAssoc for Option<bool> {
    type Builder = BooleanBuilder;

    fn builder(nrows: usize) -> BooleanBuilder {
        BooleanBuilder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: Self) {
        builder.append_option(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Boolean, true)
    }
}

impl ArrowAssoc for String {
    type Builder = LargeStringBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        LargeStringBuilder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: String) {
        builder.append_value(value.as_str())?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::LargeUtf8, false)
    }
}

impl ArrowAssoc for Option<String> {
    type Builder = LargeStringBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        LargeStringBuilder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: Self) {
        match value {
            Some(s) => builder.append_value(s.as_str())?,
            None => builder.append_null()?,
        }
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::LargeUtf8, true)
    }
}

impl ArrowAssoc for &str {
    type Builder = LargeStringBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        LargeStringBuilder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: Self) {
        builder.append_value(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::LargeUtf8, false)
    }
}

impl ArrowAssoc for Option<&str> {
    type Builder = LargeStringBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        LargeStringBuilder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: Self) {
        match value {
            Some(s) => builder.append_value(s)?,
            None => builder.append_null()?,
        }
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::LargeUtf8, true)
    }
}

impl ArrowAssoc for DateTime<Utc> {
    type Builder = Float64Builder;

    fn builder(_nrows: usize) -> Float64Builder {
        unimplemented!()
    }

    fn append(_builder: &mut Self::Builder, _value: DateTime<Utc>) -> Result<()> {
        unimplemented!()
    }

    fn field(_header: &str) -> Field {
        unimplemented!()
    }
}

impl ArrowAssoc for Option<DateTime<Utc>> {
    type Builder = Float64Builder;

    fn builder(_nrows: usize) -> Float64Builder {
        unimplemented!()
    }

    fn append(_builder: &mut Self::Builder, _value: Option<DateTime<Utc>>) -> Result<()> {
        unimplemented!()
    }

    fn field(_header: &str) -> Field {
        unimplemented!()
    }
}

impl ArrowAssoc for Date<Utc> {
    type Builder = Float64Builder;

    fn builder(_nrows: usize) -> Float64Builder {
        unimplemented!()
    }

    fn append(_builder: &mut Self::Builder, _value: Date<Utc>) -> Result<()> {
        unimplemented!()
    }

    fn field(_header: &str) -> Field {
        unimplemented!()
    }
}

impl ArrowAssoc for Option<Date<Utc>> {
    type Builder = Float64Builder;

    fn builder(_nrows: usize) -> Float64Builder {
        unimplemented!()
    }

    fn append(_builder: &mut Self::Builder, _value: Option<Date<Utc>>) -> Result<()> {
        unimplemented!()
    }

    fn field(_header: &str) -> Field {
        unimplemented!()
    }
}

fn naive_date_to_arrow(nd: NaiveDate) -> i32 {
    (nd.and_hms(0, 0, 0).timestamp() / SECONDS_IN_DAY) as i32
}

fn naive_datetime_to_arrow(nd: NaiveDateTime) -> i64 {
    nd.timestamp_millis()
}

impl ArrowAssoc for Option<NaiveDate> {
    type Builder = Date32Builder;

    fn builder(nrows: usize) -> Self::Builder {
        Date32Builder::new(nrows)
    }

    fn append(builder: &mut Self::Builder, value: Option<NaiveDate>) -> Result<()> {
        builder.append_option(value.map(naive_date_to_arrow))?;
        Ok(())
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Date32(DateUnit::Day), true)
    }
}

impl ArrowAssoc for NaiveDate {
    type Builder = Date32Builder;

    fn builder(nrows: usize) -> Self::Builder {
        Date32Builder::new(nrows)
    }

    fn append(builder: &mut Self::Builder, value: NaiveDate) -> Result<()> {
        builder.append_value(naive_date_to_arrow(value))?;
        Ok(())
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Date32(DateUnit::Day), false)
    }
}

impl ArrowAssoc for Option<NaiveDateTime> {
    type Builder = Date64Builder;

    fn builder(nrows: usize) -> Self::Builder {
        Date64Builder::new(nrows)
    }

    fn append(builder: &mut Self::Builder, value: Option<NaiveDateTime>) -> Result<()> {
        builder.append_option(value.map(naive_datetime_to_arrow))?;
        Ok(())
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Date64(DateUnit::Millisecond), true)
    }
}

impl ArrowAssoc for NaiveDateTime {
    type Builder = Date64Builder;

    fn builder(nrows: usize) -> Self::Builder {
        Date64Builder::new(nrows)
    }

    fn append(builder: &mut Self::Builder, value: NaiveDateTime) -> Result<()> {
        builder.append_value(naive_datetime_to_arrow(value))?;
        Ok(())
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Date64(DateUnit::Millisecond), false)
    }
}
