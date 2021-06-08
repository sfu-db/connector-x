use super::destination::PandasDestination;
use super::types::PandasTypeSystem;
use connectorx::{
    impl_transport,
    sources::mysql::{MysqlSource, MysqlTypeSystem},
    typesystem::TypeConversion,
};
use rust_decimal::prelude::*;
use serde_json::{to_string, Value};
use uuid::Uuid;

pub struct MysqlPandasTransport<'py>(&'py ());

impl_transport!(
    name = MysqlPandasTransport<'tp>,
    systems = MysqlTypeSystem => PandasTypeSystem,
    route = MysqlSource => PandasDestination<'tp>,
    mappings = {
        { Double[f64]                => F64[f64]                | conversion all }
        { Long[i64]                  => I64[i64]                | conversion all }
    }
);
