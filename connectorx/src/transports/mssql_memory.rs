use crate::destinations::memory::{MemoryDestination, MemoryDestinationError};
use crate::dummy_typesystem::DummyTypeSystem;
use crate::sources::mssql::{FloatN, IntN, MsSQLSource, MsSQLSourceError, MsSQLTypeSystem};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use thiserror::Error;
use uuid::Uuid;

pub struct MsSQLMemoryTransport;

#[derive(Error, Debug)]
pub enum MsSQLMemoryTransportError {
    #[error(transparent)]
    MsSQLSourceError(#[from] MsSQLSourceError),

    #[error(transparent)]
    MemoryDestinationError(#[from] MemoryDestinationError),

    #[error(transparent)]
    ConnectorXError(#[from] crate::errors::ConnectorXError),
}

impl_transport!(
    name = MsSQLMemoryTransport,
    error = MsSQLMemoryTransportError,
    systems = MsSQLTypeSystem => DummyTypeSystem,
    route = MsSQLSource => MemoryDestination,
    mappings = {
        { Tinyint[u8]                   => I64[i64]                | conversion all }
        { Smallint[i16]                 => I64[i64]                | conversion all }
        { Int[i32]                      => I64[i64]                | conversion all }
        { Bigint[i64]                   => I64[i64]                | conversion all }
        { Intn[IntN]                    => I64[i64]                | conversion half }
        { Float24[f32]                  => F64[f64]                | conversion all }
        { Float53[f64]                  => F64[f64]                | conversion all }
        { Floatn[FloatN]                => F64[f64]                | conversion half }
        { Bit[bool]                     => Bool[bool]              | conversion all  }
        { Nvarchar[&'r str]             => String[String]          | conversion half }
        { Varchar[&'r str]              => String[String]          | conversion none }
        { Nchar[&'r str]                => String[String]          | conversion none }
        { Char[&'r str]                 => String[String]          | conversion none }
        { Text[&'r str]                 => String[String]          | conversion none }
        { Ntext[&'r str]                => String[String]          | conversion none }
        { Datetime[NaiveDateTime]       => DateTime[DateTime<Utc>] | conversion half }
        { Datetime2[NaiveDateTime]      => DateTime[DateTime<Utc>] | conversion none }
        { Smalldatetime[NaiveDateTime]  => DateTime[DateTime<Utc>] | conversion none }
        { Date[NaiveDate]               => DateTime[DateTime<Utc>] | conversion half }
        { Datetimeoffset[DateTime<Utc>] => DateTime[DateTime<Utc>] | conversion all }
        { Uniqueidentifier[Uuid]        => String[String]          | conversion half }
    }
);

impl TypeConversion<Uuid, String> for MsSQLMemoryTransport {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl TypeConversion<NaiveTime, String> for MsSQLMemoryTransport {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}

impl<'r> TypeConversion<&'r str, String> for MsSQLMemoryTransport {
    fn convert(val: &'r str) -> String {
        val.to_string()
    }
}

impl TypeConversion<NaiveDateTime, DateTime<Utc>> for MsSQLMemoryTransport {
    fn convert(val: NaiveDateTime) -> DateTime<Utc> {
        DateTime::from_utc(val, Utc)
    }
}

impl TypeConversion<NaiveDate, DateTime<Utc>> for MsSQLMemoryTransport {
    fn convert(val: NaiveDate) -> DateTime<Utc> {
        DateTime::from_utc(val.and_hms(0, 0, 0), Utc)
    }
}

impl TypeConversion<IntN, i64> for MsSQLMemoryTransport {
    fn convert(val: IntN) -> i64 {
        val.0
    }
}

impl TypeConversion<FloatN, f64> for MsSQLMemoryTransport {
    fn convert(val: FloatN) -> f64 {
        val.0
    }
}
