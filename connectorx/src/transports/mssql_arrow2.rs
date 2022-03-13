//! Transport from MsSQL Source to Arrow2 Destination.

use crate::destinations::arrow2::{Arrow2Destination, Arrow2DestinationError, Arrow2TypeSystem};
use crate::sources::mssql::{FloatN, IntN, MsSQLSource, MsSQLSourceError, MsSQLTypeSystem};
use crate::typesystem::TypeConversion;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use num_traits::ToPrimitive;
use rust_decimal::Decimal;
use thiserror::Error;
use uuid::Uuid;

/// Convert MsSQL data types to Arrow2 data types.
pub struct MsSQLArrow2Transport;

#[derive(Error, Debug)]
pub enum MsSQLArrow2TransportError {
    #[error(transparent)]
    Source(#[from] MsSQLSourceError),

    #[error(transparent)]
    Destination(#[from] Arrow2DestinationError),

    #[error(transparent)]
    ConnectorX(#[from] crate::errors::ConnectorXError),
}

impl_transport!(
    name = MsSQLArrow2Transport,
    error = MsSQLArrow2TransportError,
    systems = MsSQLTypeSystem => Arrow2TypeSystem,
    route = MsSQLSource => Arrow2Destination,
    mappings = {
        { Tinyint[u8]                   => Int32[i32]                | conversion auto }
        { Smallint[i16]                 => Int32[i32]                | conversion auto }
        { Int[i32]                      => Int32[i32]                | conversion auto }
        { Bigint[i64]                   => Int64[i64]                | conversion auto }
        { Intn[IntN]                    => Int64[i64]                | conversion option }
        { Float24[f32]                  => Float32[f32]              | conversion auto }
        { Float53[f64]                  => Float64[f64]              | conversion auto }
        { Floatn[FloatN]                => Float64[f64]              | conversion option }
        { Bit[bool]                     => Boolean[bool]             | conversion auto  }
        { Nvarchar[&'r str]             => LargeUtf8[String]         | conversion owned }
        { Varchar[&'r str]              => LargeUtf8[String]         | conversion none }
        { Nchar[&'r str]                => LargeUtf8[String]         | conversion none }
        { Char[&'r str]                 => LargeUtf8[String]         | conversion none }
        { Text[&'r str]                 => LargeUtf8[String]         | conversion none }
        { Ntext[&'r str]                => LargeUtf8[String]         | conversion none }
        { Binary[&'r [u8]]              => LargeBinary[Vec<u8>]      | conversion owned }
        { Varbinary[&'r [u8]]           => LargeBinary[Vec<u8>]      | conversion none }
        { Image[&'r [u8]]               => LargeBinary[Vec<u8>]      | conversion none }
        { Numeric[Decimal]              => Float64[f64]              | conversion option }
        { Decimal[Decimal]              => Float64[f64]              | conversion none }
        { Datetime[NaiveDateTime]       => Date64[NaiveDateTime]     | conversion auto }
        { Datetime2[NaiveDateTime]      => Date64[NaiveDateTime]     | conversion none }
        { Smalldatetime[NaiveDateTime]  => Date64[NaiveDateTime]     | conversion none }
        { Date[NaiveDate]               => Date32[NaiveDate]         | conversion auto }
        { Datetimeoffset[DateTime<Utc>] => DateTimeTz[DateTime<Utc>] | conversion auto }
        { Uniqueidentifier[Uuid]        => LargeUtf8[String]         | conversion option }
        { Time[NaiveTime]               => Time64[NaiveTime]         | conversion auto }
        { SmallMoney[f32]               => Float32[f32]              | conversion none }
        { Money[f64]                    => Float64[f64]              | conversion none }
    }
);

impl TypeConversion<Uuid, String> for MsSQLArrow2Transport {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl TypeConversion<IntN, i64> for MsSQLArrow2Transport {
    fn convert(val: IntN) -> i64 {
        val.0
    }
}

impl TypeConversion<FloatN, f64> for MsSQLArrow2Transport {
    fn convert(val: FloatN) -> f64 {
        val.0
    }
}

impl TypeConversion<Decimal, f64> for MsSQLArrow2Transport {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}
