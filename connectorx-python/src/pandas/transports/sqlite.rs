use crate::pandas::destination::PandasDestination;
use crate::pandas::types::PandasTypeSystem;
use connectorx::{
    impl_transport,
    sources::sqlite::{SqliteSource, SqliteTypeSystem},
    typesystem::TypeConversion,
};

pub struct SqlitePandasTransport<'py>(&'py ());

impl_transport!(
    name = SqlitePandasTransport<'tp>,
    systems = SqliteTypeSystem => PandasTypeSystem,
    route = SqliteSource => PandasDestination<'tp>,
    mappings = {
        { Bool[bool]                 => Bool[bool]              | conversion all }
        { Integer[i64]               => I64[i64]                | conversion all }
        { Real[f64]                  => F64[f64]                | conversion all }
        { Text[Box<str>]             => BoxStr[Box<str>]        | conversion all }
    }
);
