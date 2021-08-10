use crate::errors::ConnectorXPythonError;
use crate::pandas::destination::PandasDestination;
use crate::pandas::typesystem::PandasTypeSystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use connectorx::{
    impl_transport,
    sources::oracle::{OracleSource, OracleTypeSystem, TextProtocol},
    typesystem::TypeConversion,
};
use rust_decimal::prelude::*;
use std::marker::PhantomData;

pub struct OraclePandasTransport<'py, P>(&'py (), PhantomData<P>);

impl_transport!(
    name = OraclePandasTransport<'tp, TextProtocol>,
    error = ConnectorXPythonError,
    systems = OracleTypeSystem => PandasTypeSystem,
    route = OracleSource<TextProtocol> => PandasDestination<'tp>,
    mappings = {
        { Float[f64]                => F64[f64]                | conversion auto }
        { Int[i64]                  => I64[i64]                | conversion auto }
        { VarChar[String]            => String[String]          | conversion auto }
    }
);