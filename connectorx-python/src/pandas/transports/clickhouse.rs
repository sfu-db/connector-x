use crate::errors::ConnectorXPythonError;
use crate::pandas::destination::PandasDestination;
use crate::pandas::typesystem::PandasTypeSystem;
use chrono::{DateTime, Utc};
use connectorx::{
    impl_transport,
    sources::clickhouse::{ClickHouseSource, ClickHouseTypeSystem},
    typesystem::TypeConversion,
};

#[allow(dead_code)]
pub struct ClickHousePandasTransport<'py>(&'py ());

impl_transport!(
    name = ClickHousePandasTransport<'tp>,
    error = ConnectorXPythonError,
    systems = ClickHouseTypeSystem => PandasTypeSystem,
    route = ClickHouseSource => PandasDestination<'tp>,
    mappings = {
        { Int8[i8]                   => I64[i64]                              | conversion auto }
        { Int16[i16]                 => I64[i64]                              | conversion auto }
        { Int32[i32]                 => I64[i64]                              | conversion auto }
        { Int64[i64]                 => I64[i64]                              | conversion auto }

        { UInt8[u8]                  => I64[i64]                             | conversion auto }
        { UInt16[u16]                => I64[i64]                             | conversion auto }
        { UInt32[u32]                => I64[i64]                             | conversion auto }

        { Float32[f32]               => F64[f64]                            | conversion auto }
        { Float64[f64]               => F64[f64]                            | conversion auto }

        { String[String]             => String[String]                       | conversion auto }
        { DateTime[DateTime<Utc>]    => DateTime[DateTime<Utc>]              | conversion auto }
        { DateTime64[DateTime<Utc>]  => DateTime[DateTime<Utc>]              | conversion none }

        { Enum8[String]              => String[String]                       | conversion none }
        { Enum16[String]             => String[String]                       | conversion none }

        { Bool[bool]                 => Bool[bool]                           | conversion auto }
    }
);
