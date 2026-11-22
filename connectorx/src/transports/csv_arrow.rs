//! Transport from CSV Source to Arrow Destination.

use crate::destinations::arrow::{
    typesystem::{ArrowTypeSystem, DateTimeWrapperMicro},
    ArrowDestination, ArrowDestinationError,
};
use crate::sources::csv::{CSVSource, CSVSourceError, CSVTypeSystem};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, Utc};
use thiserror::Error;

/// Convert CSV data types to Arrow data types.
pub struct CSVArrowTransport;

#[derive(Error, Debug)]
pub enum CSVArrowTransportError {
    #[error(transparent)]
    Source(#[from] CSVSourceError),

    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

impl_transport!(
    name = CSVArrowTransport,
    error = CSVArrowTransportError,
    systems = CSVTypeSystem => ArrowTypeSystem,
    route = CSVSource => ArrowDestination,
    mappings = {
        { F64[f64]                => Float64[f64]              | conversion auto}
        { I64[i64]                => Int64[i64]                | conversion auto}
        { Bool[bool]              => Boolean[bool]             | conversion auto}
        { String[String]          => LargeUtf8[String]         | conversion auto}
        { DateTime[DateTime<Utc>] => DateTimeTzMicro[DateTimeWrapperMicro] | conversion option}
    }
);

impl TypeConversion<DateTime<Utc>, DateTimeWrapperMicro> for CSVArrowTransport {
    fn convert(val: DateTime<Utc>) -> DateTimeWrapperMicro {
        DateTimeWrapperMicro(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_order::DataOrder;
    use crate::dispatcher::Dispatcher;
    use crate::sources::Source;
    use crate::sql::CXQuery;
    use arrow::array::{Array, AsArray};
    use arrow::datatypes::{DataType, Float64Type, Int64Type, TimestampMicrosecondType};
    use arrow::record_batch::RecordBatch;
    use std::path::PathBuf;

    /// A CSV file in the temp directory that is removed when the test ends.
    struct TempCsv(PathBuf);

    impl TempCsv {
        fn new(name: &str, contents: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "connectorx_csv_arrow_{}_{}.csv",
                name,
                std::process::id()
            ));
            std::fs::write(&path, contents).unwrap();
            TempCsv(path)
        }

        fn query(&self) -> CXQuery {
            CXQuery::naked(self.0.display().to_string())
        }
    }

    impl Drop for TempCsv {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    fn run(
        schema: &[CSVTypeSystem],
        files: &[&TempCsv],
    ) -> Result<RecordBatch, CSVArrowTransportError> {
        let mut dst = ArrowDestination::new();
        let queries: Vec<CXQuery> = files.iter().map(|f| f.query()).collect();
        Dispatcher::<_, _, CSVArrowTransport>::new(
            CSVSource::new(schema),
            &mut dst,
            &queries,
            None,
        )
        .run()?;
        let batches = dst.arrow().unwrap();
        Ok(arrow::compute::concat_batches(&batches[0].schema(), &batches).unwrap())
    }

    const HEADER: &str = "i,f,b,s,t,ni,nf,nb,ns,nt\n";

    fn every_type() -> Vec<CSVTypeSystem> {
        use CSVTypeSystem::*;
        vec![
            I64(false),
            F64(false),
            Bool(false),
            String(false),
            DateTime(false),
            I64(true),
            F64(true),
            Bool(true),
            String(true),
            DateTime(true),
        ]
    }

    #[test]
    fn reads_every_type_from_several_files() {
        let a = TempCsv::new(
            "a",
            &format!(
                "{HEADER}1,1.5,true,x,2020-01-02T03:04:05Z,-7,0.25,false,y,1970-01-01T00:00:00Z\n"
            ),
        );
        let b = TempCsv::new(
            "b",
            &format!("{HEADER}2,-2.5,false,,2021-06-30T12:00:00.5Z,,,,,\n"),
        );
        let rb = run(&every_type(), &[&a, &b]).unwrap();

        assert_eq!(rb.num_rows(), 2);
        let types: Vec<_> = rb
            .schema()
            .fields()
            .iter()
            .map(|f| f.data_type().clone())
            .collect();
        assert_eq!(
            &types[..4],
            &[
                DataType::Int64,
                DataType::Float64,
                DataType::Boolean,
                DataType::Utf8
            ]
        );
        assert!(matches!(types[4], DataType::Timestamp(_, Some(_))));
        let nullable: Vec<_> = rb
            .schema()
            .fields()
            .iter()
            .map(|f| f.is_nullable())
            .collect();
        assert_eq!(
            nullable,
            [false, false, false, false, false, true, true, true, true, true]
        );

        // Files are read in partition order, but the destination may finish
        // them in any order, so look the rows up by the `i` column.
        let i: Vec<i64> = rb.column(0).as_primitive::<Int64Type>().values().to_vec();
        let (r1, r2) = (
            i.iter().position(|v| *v == 1).unwrap(),
            i.iter().position(|v| *v == 2).unwrap(),
        );
        let f = rb.column(1).as_primitive::<Float64Type>();
        assert_eq!((f.value(r1), f.value(r2)), (1.5, -2.5));
        let b = rb.column(2).as_boolean();
        assert_eq!((b.value(r1), b.value(r2)), (true, false));
        let s = rb.column(3).as_string::<i32>();
        assert_eq!((s.value(r1), s.value(r2)), ("x", ""));
        let t = rb.column(4).as_primitive::<TimestampMicrosecondType>();
        assert_eq!(t.value(r1), 1_577_934_245_000_000);
        assert_eq!(t.value(r2), 1_625_054_400_500_000);

        // Empty fields are NULL in the nullable columns of the second file...
        assert_eq!(rb.column(5).as_primitive::<Int64Type>().value(r1), -7);
        assert_eq!(rb.column(6).as_primitive::<Float64Type>().value(r1), 0.25);
        assert!(!rb.column(7).as_boolean().value(r1));
        assert_eq!(
            rb.column(9)
                .as_primitive::<TimestampMicrosecondType>()
                .value(r1),
            0
        );
        for c in [5, 6, 7, 9] {
            assert!(rb.column(c).is_null(r2), "column {c}");
        }
        // ...except strings: the CSV source cannot tell an empty string from
        // a missing one, so it always produces "".
        let ns = rb.column(8).as_string::<i32>();
        assert_eq!(
            (ns.value(r1), ns.is_null(r2), ns.value(r2)),
            ("y", false, "")
        );
    }

    #[test]
    fn values_that_do_not_parse_are_errors() {
        use CSVTypeSystem::*;
        for (ty, value) in [
            (I64(false), "1.5"),
            (I64(true), "x"),
            (F64(false), "one"),
            (F64(true), "1,5"),
            (Bool(false), "yes"),
            (Bool(true), "1"),
            (DateTime(false), "2020-01-02"),
            (DateTime(true), "noon"),
        ] {
            let file = TempCsv::new("bad", &format!("x\n\"{value}\"\n"));
            let err = run(&[ty], &[&file]).expect_err(&format!("{ty:?} = {value:?}"));
            assert!(err.to_string().contains(value), "{err}");
        }
    }

    #[test]
    fn missing_file_is_an_error() {
        let mut source = CSVSource::new(&[CSVTypeSystem::I64(false)]);
        source.set_queries(&[CXQuery::naked("/no/such/dir/connectorx.csv")]);
        assert!(source.fetch_metadata().is_err());
        assert!(source.partition().is_err());
    }

    #[test]
    fn only_row_major_is_supported() {
        let mut source = CSVSource::new(&[]);
        assert!(source.set_data_order(DataOrder::RowMajor).is_ok());
        assert!(source.set_data_order(DataOrder::ColumnMajor).is_err());
        assert_eq!(source.result_rows().unwrap(), None);
    }
}
