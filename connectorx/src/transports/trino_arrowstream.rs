//! Transport from Trino Source to Arrow Destination.

use crate::{
    destinations::arrowstream::{
        typesystem::{ArrowTypeSystem, NaiveDateTimeWrapperMicro, NaiveTimeWrapperMicro},
        ArrowDestination, ArrowDestinationError,
    },
    sources::trino::{TrinoSource, TrinoSourceError, TrinoTypeSystem},
    typesystem::TypeConversion,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use num_traits::ToPrimitive;
use rust_decimal::Decimal;
use serde_json::{to_string, Value};
use thiserror::Error;

/// Errors raised while moving data from the Trino source to the streaming Arrow destination.
#[derive(Error, Debug)]
pub enum TrinoArrowTransportError {
    /// Error from the Trino source.
    #[error(transparent)]
    Source(#[from] TrinoSourceError),

    /// Error from the streaming Arrow destination.
    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    /// Generic ConnectorX error.
    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

/// Convert Trino data types to Arrow data types.
pub struct TrinoArrowTransport();

impl_transport!(
    name = TrinoArrowTransport,
    error = TrinoArrowTransportError,
    systems = TrinoTypeSystem => ArrowTypeSystem,
    route = TrinoSource => ArrowDestination,
    mappings = {
        { Date[NaiveDate]            => Date32[NaiveDate]       | conversion auto }
        { Time[NaiveTime]            => Time64Micro[NaiveTimeWrapperMicro]       | conversion option }
        { Timestamp[NaiveDateTime]   => Date64Micro[NaiveDateTimeWrapperMicro]   | conversion option }
        { Boolean[bool]              => Boolean[bool]           | conversion auto }
        { Bigint[i64]                => Int64[i64]              | conversion auto }
        { Integer[i32]               => Int64[i64]              | conversion auto }
        { Smallint[i16]              => Int64[i64]              | conversion auto }
        { Tinyint[i8]                => Int64[i64]              | conversion auto }
        { Double[f64]                => Float64[f64]            | conversion auto }
        { Real[f32]                  => Float64[f64]            | conversion auto }
        { Varchar[String]            => LargeUtf8[String]       | conversion auto }
        { Char[String]               => LargeUtf8[String]       | conversion none }
    }
);

impl TypeConversion<Decimal, f64> for TrinoArrowTransport {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}

impl TypeConversion<Value, String> for TrinoArrowTransport {
    fn convert(val: Value) -> String {
        to_string(&val).unwrap()
    }
}

impl TypeConversion<NaiveTime, NaiveTimeWrapperMicro> for TrinoArrowTransport {
    fn convert(val: NaiveTime) -> NaiveTimeWrapperMicro {
        NaiveTimeWrapperMicro(val)
    }
}

impl TypeConversion<NaiveDateTime, NaiveDateTimeWrapperMicro> for TrinoArrowTransport {
    fn convert(val: NaiveDateTime) -> NaiveDateTimeWrapperMicro {
        NaiveDateTimeWrapperMicro(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bigint_conversion_preserves_full_i64_range() {
        for value in [
            i64::MIN,
            -(1_i64 << 53) - 1,
            (1_i64 << 53) + 1,
            2_518_422_941_645_303_032,
            i64::MAX,
        ] {
            assert_eq!(
                <TrinoArrowTransport as TypeConversion<i64, i64>>::convert(value),
                value
            );
        }
    }

    /// The streaming destination gets the same rows as the batch one: every
    /// column of the fake server's table, read through the dispatcher.
    #[test]
    fn dispatcher_streams_every_trino_type() {
        use crate::dispatcher::Dispatcher;
        use crate::sources::trino::fake_server::{every_type_table, FakeTrino};
        use crate::sql::CXQuery;
        use std::sync::Arc;

        let server = FakeTrino::start(every_type_table(), 2);
        let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
        let source = TrinoSource::new(rt, &server.url()).unwrap();
        let mut dst = ArrowDestination::new();
        let queries = [
            CXQuery::naked("SELECT * FROM t WHERE id >= 0 AND id < 2"),
            CXQuery::naked("SELECT * FROM t WHERE id >= 2 AND id < 5"),
        ];
        Dispatcher::<_, _, TrinoArrowTransport>::new(source, &mut dst, &queries, None)
            .run()
            .unwrap();

        let batches = dst.arrow().unwrap();
        assert_eq!(batches.iter().map(|b| b.num_rows()).sum::<usize>(), 5);
        let schema = batches[0].schema();
        let names: Vec<_> = schema.fields().iter().map(|f| f.name().as_str()).collect();
        assert_eq!(
            names,
            ["id", "i", "s", "t", "d", "r", "dec", "b", "v", "c", "arr", "dt", "tm", "ts"]
        );
    }
}
