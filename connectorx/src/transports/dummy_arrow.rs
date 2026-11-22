//! Transport from Dummy Source to Arrow Destination.

use crate::destinations::arrow::{
    typesystem::{ArrowTypeSystem, NaiveDateTimeWrapperMicro},
    ArrowDestination, ArrowDestinationError,
};
use crate::sources::dummy::{DummySource, DummyTypeSystem};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use thiserror::Error;

/// Convert Dummy data types to Arrow data types.
pub struct DummyArrowTransport;

#[derive(Error, Debug)]
pub enum DummyArrowTransportError {
    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

impl_transport!(
    name = DummyArrowTransport,
    error = DummyArrowTransportError,
    systems = DummyTypeSystem => ArrowTypeSystem,
    route = DummySource => ArrowDestination,
    mappings = {
        { F64[f64]                => Float64[f64]               | conversion auto}
        { I64[i64]                => Int64[i64]                 | conversion auto}
        { Bool[bool]              => Boolean[bool]              | conversion auto}
        { String[String]          => LargeUtf8[String]          | conversion auto}
        { DateTime[DateTime<Utc>] => Date64Micro[NaiveDateTimeWrapperMicro]      | conversion option}
    }
);

impl TypeConversion<DateTime<Utc>, NaiveDateTimeWrapperMicro> for DummyArrowTransport {
    fn convert(val: DateTime<Utc>) -> NaiveDateTimeWrapperMicro {
        NaiveDateTimeWrapperMicro(val.naive_utc())
    }
}

impl TypeConversion<NaiveDateTime, DateTime<Utc>> for DummyArrowTransport {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_naive_utc_and_offset(val, Utc)
    }
}

impl TypeConversion<NaiveDate, DateTime<Utc>> for DummyArrowTransport {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_naive_utc_and_offset(
            val.and_hms_opt(0, 0, 0)
                .unwrap_or_else(|| panic!("and_hms_opt return None")),
            Utc,
        )
    }
}

// The arrow and arrowstream versions of this module share one API, so they
// share their tests: both include the same file.
#[cfg(test)]
#[allow(clippy::duplicate_mod)]
#[path = "dummy_tests.rs"]
mod tests;

#[cfg(all(test, feature = "dst_polars"))]
mod polars_tests {
    use super::*;
    use crate::dispatcher::Dispatcher;
    use crate::sql::CXQuery;

    #[test]
    fn arrow_destination_converts_to_polars() {
        let schema = [
            DummyTypeSystem::I64(false),
            DummyTypeSystem::F64(true),
            DummyTypeSystem::Bool(true),
            DummyTypeSystem::String(false),
        ];
        let queries = [CXQuery::naked("3,4"), CXQuery::naked("2,4")];
        let mut dst = ArrowDestination::new();
        Dispatcher::<_, _, DummyArrowTransport>::new(
            DummySource::new(&["i", "f", "b", "s"], &schema),
            &mut dst,
            &queries,
            None,
        )
        .run()
        .unwrap();

        let df = dst.polars().unwrap();
        assert_eq!(df.shape(), (5, 4));
        let names: Vec<_> = df
            .get_column_names()
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(names, ["i", "f", "b", "s"]);
        let i: Vec<i64> = df
            .column("i")
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter()
            .collect();
        let mut sorted = i.clone();
        sorted.sort();
        assert_eq!(sorted, vec![0, 0, 1, 1, 2]);
        // Option<bool> cycles Some(true), Some(false), None per partition.
        assert_eq!(df.column("b").unwrap().null_count(), 1);
    }
}
