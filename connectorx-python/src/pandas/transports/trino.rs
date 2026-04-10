use crate::errors::ConnectorXPythonError;
use crate::pandas::destination::PandasDestination;
use crate::pandas::typesystem::{DateTimeWrapperMicro, PandasTypeSystem};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use connectorx::{
    impl_transport,
    sources::trino::{TrinoSource, TrinoTypeSystem},
    typesystem::TypeConversion,
};

#[allow(dead_code)]
pub struct TrinoPandasTransport<'py>(&'py ());

impl_transport!(
    name = TrinoPandasTransport<'tp>,
    error = ConnectorXPythonError,
    systems = TrinoTypeSystem => PandasTypeSystem,
    route = TrinoSource => PandasDestination<'tp>,
    mappings = {
        { Date[NaiveDate]            => DateTimeMicro[DateTimeWrapperMicro] | conversion option }
        { Time[NaiveTime]            => String[String]          | conversion option }
        { Timestamp[NaiveDateTime]   => DateTimeMicro[DateTimeWrapperMicro] | conversion option }
        { Boolean[bool]              => Bool[bool]              | conversion auto }
        { Bigint[i32]                => I64[i64]                | conversion auto }
        { Integer[i32]               => I64[i64]                | conversion none }
        { Smallint[i16]              => I64[i64]                | conversion auto }
        { Tinyint[i8]                => I64[i64]                | conversion auto }
        { Double[f64]                => F64[f64]                | conversion auto }
        { Real[f32]                  => F64[f64]                | conversion auto }
        { Varchar[String]            => String[String]          | conversion auto }
        { Char[String]               => String[String]          | conversion none }
    }
);

impl<'py> TypeConversion<NaiveDate, DateTimeWrapperMicro> for TrinoPandasTransport<'py> {
    fn convert(val: NaiveDate) -> DateTimeWrapperMicro {
        DateTimeWrapperMicro(DateTime::from_naive_utc_and_offset(
            val.and_hms_opt(0, 0, 0)
                .unwrap_or_else(|| panic!("and_hms_opt got None from {:?}", val)),
            Utc,
        ))
    }
}

impl<'py> TypeConversion<NaiveTime, String> for TrinoPandasTransport<'py> {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}

impl<'py> TypeConversion<NaiveDateTime, DateTimeWrapperMicro> for TrinoPandasTransport<'py> {
    fn convert(val: NaiveDateTime) -> DateTimeWrapperMicro {
        DateTimeWrapperMicro(DateTime::from_naive_utc_and_offset(val, Utc))
    }
}
