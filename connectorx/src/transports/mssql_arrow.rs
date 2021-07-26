use crate::destinations::arrow::{ArrowDestination, ArrowDestinationError, ArrowTypeSystem};
use crate::sources::mssql::{FloatN, IntN, MsSQLSource, MsSQLSourceError, MsSQLTypeSystem};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

pub struct MsSQLArrowTransport;

#[derive(Error, Debug)]
pub enum MsSQLArrowTransportError {
    #[error(transparent)]
    Source(#[from] MsSQLSourceError),

    #[error(transparent)]
    Destination(#[from] ArrowDestinationError),

    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),
}

impl_transport!(
    name = MsSQLArrowTransport,
    error = MsSQLArrowTransportError,
    systems = MsSQLTypeSystem => ArrowTypeSystem,
    route = MsSQLSource => ArrowDestination,
    mappings = {
        { Tinyint[u8]                   => Int32[i32]                | conversion all }
        { Smallint[i16]                 => Int32[i32]                | conversion all }
        { Int[i32]                      => Int32[i32]                | conversion all }
        { Bigint[i64]                   => Int64[i64]                | conversion all }
        { Intn[IntN]                    => Int64[i64]                | conversion half }
        { Float24[f32]                  => Float32[f32]              | conversion all }
        { Float53[f64]                  => Float64[f64]              | conversion all }
        { Floatn[FloatN]                => Float64[f64]              | conversion half }
        { Bit[bool]                     => Boolean[bool]             | conversion all  }
        { Nvarchar[&'r str]             => LargeUtf8[String]         | conversion half }
        { Varchar[&'r str]              => LargeUtf8[String]         | conversion none }
        { Nchar[&'r str]                => LargeUtf8[String]         | conversion none }
        { Char[&'r str]                 => LargeUtf8[String]         | conversion none }
        { Text[&'r str]                 => LargeUtf8[String]         | conversion none }
        { Ntext[&'r str]                => LargeUtf8[String]         | conversion none }
        { Datetime[NaiveDateTime]       => Date64[NaiveDateTime]     | conversion all }
        { Datetime2[NaiveDateTime]      => Date64[NaiveDateTime]     | conversion none }
        { Smalldatetime[NaiveDateTime]  => Date64[NaiveDateTime]     | conversion none }
        { Date[NaiveDate]               => Date32[NaiveDate]         | conversion all }
        { Datetimeoffset[DateTime<Utc>] => DateTimeTz[DateTime<Utc>] | conversion all }
        { Uniqueidentifier[Uuid]        => LargeUtf8[String]         | conversion half }
    }
);

impl TypeConversion<Uuid, String> for MsSQLArrowTransport {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl<'r> TypeConversion<&'r str, String> for MsSQLArrowTransport {
    fn convert(val: &'r str) -> String {
        val.to_string()
    }
}

impl TypeConversion<IntN, i64> for MsSQLArrowTransport {
    fn convert(val: IntN) -> i64 {
        val.0
    }
}

impl TypeConversion<FloatN, f64> for MsSQLArrowTransport {
    fn convert(val: FloatN) -> f64 {
        val.0
    }
}
