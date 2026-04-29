use crate::errors::ConnectorXPythonError;
use crate::pandas::destination::PandasDestination;
use crate::pandas::typesystem::{DateTimeWrapperMicro, PandasTypeSystem};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use connectorx::{
    impl_transport,
    sources::clickhouse::{ClickHouseSource, ClickHouseTypeSystem},
    typesystem::TypeConversion,
};
use rust_decimal::prelude::*;
use std::net::IpAddr;
use uuid::Uuid;

#[allow(dead_code)]
pub struct ClickHousePandasTransport<'py>(&'py ());

impl_transport!(
    name = ClickHousePandasTransport<'tp>,
    error = ConnectorXPythonError,
    systems = ClickHouseTypeSystem => PandasTypeSystem,
    route = ClickHouseSource => PandasDestination<'tp>,
    mappings = {
        { Int8[i8]                   => I64[i64]                                | conversion auto }
        { Int16[i16]                 => I64[i64]                                | conversion auto }
        { Int32[i32]                 => I64[i64]                                | conversion auto }
        { Int64[i64]                 => I64[i64]                                | conversion auto }

        { UInt8[u8]                  => I64[i64]                             | conversion auto }
        { UInt16[u16]                => I64[i64]                             | conversion auto }
        { UInt32[u32]                => I64[i64]                             | conversion auto }
        { UInt64[u64]                => I64[i64]                             | conversion auto }

        { Float32[f32]               => F64[f64]                            | conversion auto }
        { Float64[f64]               => F64[f64]                            | conversion auto }
        { Decimal[Decimal]           => F64[f64]                            | conversion option }

        { String[String]             => String[String]                       | conversion auto }
        { FixedString[Vec<u8>]       => Bytes[Vec<u8>]                       | conversion auto }
        { Enum8[String]              => String[String]                       | conversion none }
        { Enum16[String]             => String[String]                       | conversion none }

        { Date[NaiveDate]            => DateTimeMicro[DateTimeWrapperMicro]  | conversion option }
        { Date32[NaiveDate]          => DateTimeMicro[DateTimeWrapperMicro]  | conversion none }
        { DateTime[DateTime<Utc>]    => DateTimeMicro[DateTimeWrapperMicro]  | conversion option }
        { DateTime64[DateTime<Utc>]  => DateTimeMicro[DateTimeWrapperMicro]  | conversion none }
        { Time[NaiveTime]            => String[String]                       | conversion option }
        { Time64[NaiveTime]          => String[String]                       | conversion none }

        { UUID[Uuid]                 => String[String]                       | conversion option }
        { IPv4[IpAddr]               => String[String]                       | conversion option }
        { IPv6[IpAddr]               => String[String]                       | conversion none }
        { Bool[bool]                 => Bool[bool]                           | conversion auto }

        { ArrayBool[Vec<Option<bool>>]         => BoolArray[Vec<bool>]                   | conversion option }
        { ArrayInt8[Vec<Option<i8>>]           => I64Array[Vec<i64>]                     | conversion option }
        { ArrayInt16[Vec<Option<i16>>]         => I64Array[Vec<i64>]                     | conversion option }
        { ArrayInt32[Vec<Option<i32>>]         => I64Array[Vec<i64>]                     | conversion option }
        { ArrayInt64[Vec<Option<i64>>]         => I64Array[Vec<i64>]                     | conversion option }
        { ArrayUInt8[Vec<Option<u8>>]          => I64Array[Vec<i64>]                     | conversion option }
        { ArrayUInt16[Vec<Option<u16>>]        => I64Array[Vec<i64>]                     | conversion option }
        { ArrayUInt32[Vec<Option<u32>>]        => I64Array[Vec<i64>]                     | conversion option }
        { ArrayUInt64[Vec<Option<u64>>]        => I64Array[Vec<i64>]                     | conversion option }
        { ArrayFloat32[Vec<Option<f32>>]       => F64Array[Vec<f64>]                     | conversion option }
        { ArrayFloat64[Vec<Option<f64>>]       => F64Array[Vec<f64>]                     | conversion option }
        { ArrayDecimal[Vec<Option<Decimal>>]   => F64Array[Vec<f64>]                     | conversion option }
    }
);

impl<'py> TypeConversion<Decimal, f64> for ClickHousePandasTransport<'py> {
    fn convert(val: Decimal) -> f64 {
        val.to_f64()
            .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", val))
    }
}

impl<'py> TypeConversion<NaiveDate, DateTimeWrapperMicro> for ClickHousePandasTransport<'py> {
    fn convert(val: NaiveDate) -> DateTimeWrapperMicro {
        DateTimeWrapperMicro(DateTime::from_naive_utc_and_offset(
            val.and_hms_opt(0, 0, 0)
                .unwrap_or_else(|| panic!("and_hms_opt got None from {:?}", val)),
            Utc,
        ))
    }
}

impl<'py> TypeConversion<DateTime<Utc>, DateTimeWrapperMicro> for ClickHousePandasTransport<'py> {
    fn convert(val: DateTime<Utc>) -> DateTimeWrapperMicro {
        DateTimeWrapperMicro(val)
    }
}

impl<'py> TypeConversion<NaiveTime, String> for ClickHousePandasTransport<'py> {
    fn convert(val: NaiveTime) -> String {
        val.to_string()
    }
}

impl<'py> TypeConversion<Uuid, String> for ClickHousePandasTransport<'py> {
    fn convert(val: Uuid) -> String {
        val.to_string()
    }
}

impl<'py> TypeConversion<IpAddr, String> for ClickHousePandasTransport<'py> {
    fn convert(val: IpAddr) -> String {
        val.to_string()
    }
}

impl<'py> TypeConversion<Vec<Option<Decimal>>, Vec<f64>> for ClickHousePandasTransport<'py> {
    fn convert(val: Vec<Option<Decimal>>) -> Vec<f64> {
        val.into_iter()
            .map(|v| match v {
                Some(v) => v
                    .to_f64()
                    .unwrap_or_else(|| panic!("cannot convert decimal {:?} to float64", v)),
                None => {
                    unimplemented!("In-array nullable not implemented for Vec<Decimal> for Pandas")
                }
            })
            .collect()
    }
}

impl<'py> TypeConversion<Vec<Option<bool>>, Vec<bool>> for ClickHousePandasTransport<'py> {
    fn convert(val: Vec<Option<bool>>) -> Vec<bool> {
        val.into_iter()
            .map(|v| match v {
                Some(v) => v,
                None => {
                    unimplemented!("In-array nullable not implemented for Vec<bool> for Pandas")
                }
            })
            .collect()
    }
}

impl<'py> TypeConversion<Vec<Option<f32>>, Vec<f64>> for ClickHousePandasTransport<'py> {
    fn convert(val: Vec<Option<f32>>) -> Vec<f64> {
        val.into_iter()
            .map(|v| match v {
                Some(v) => v as f64,
                None => {
                    unimplemented!("In-array nullable not implemented for Vec<f32> for Pandas")
                }
            })
            .collect()
    }
}

impl<'py> TypeConversion<Vec<Option<f64>>, Vec<f64>> for ClickHousePandasTransport<'py> {
    fn convert(val: Vec<Option<f64>>) -> Vec<f64> {
        val.into_iter()
            .map(|v| match v {
                Some(v) => v,
                None => {
                    unimplemented!("In-array nullable not implemented for Vec<f64> for Pandas")
                }
            })
            .collect()
    }
}

impl<'py> TypeConversion<Vec<Option<i8>>, Vec<i64>> for ClickHousePandasTransport<'py> {
    fn convert(val: Vec<Option<i8>>) -> Vec<i64> {
        val.into_iter()
            .map(|v| match v {
                Some(v) => v as i64,
                None => {
                    unimplemented!("In-array nullable not implemented for Vec<i8> for Pandas")
                }
            })
            .collect()
    }
}

impl<'py> TypeConversion<Vec<Option<i16>>, Vec<i64>> for ClickHousePandasTransport<'py> {
    fn convert(val: Vec<Option<i16>>) -> Vec<i64> {
        val.into_iter()
            .map(|v| match v {
                Some(v) => v as i64,
                None => {
                    unimplemented!("In-array nullable not implemented for Vec<i16> for Pandas")
                }
            })
            .collect()
    }
}

impl<'py> TypeConversion<Vec<Option<i32>>, Vec<i64>> for ClickHousePandasTransport<'py> {
    fn convert(val: Vec<Option<i32>>) -> Vec<i64> {
        val.into_iter()
            .map(|v| match v {
                Some(v) => v as i64,
                None => {
                    unimplemented!("In-array nullable not implemented for Vec<i32> for Pandas")
                }
            })
            .collect()
    }
}

impl<'py> TypeConversion<Vec<Option<i64>>, Vec<i64>> for ClickHousePandasTransport<'py> {
    fn convert(val: Vec<Option<i64>>) -> Vec<i64> {
        val.into_iter()
            .map(|v| match v {
                Some(v) => v,
                None => {
                    unimplemented!("In-array nullable not implemented for Vec<i64> for Pandas")
                }
            })
            .collect()
    }
}

impl<'py> TypeConversion<Vec<Option<u8>>, Vec<i64>> for ClickHousePandasTransport<'py> {
    fn convert(val: Vec<Option<u8>>) -> Vec<i64> {
        val.into_iter()
            .map(|v| match v {
                Some(v) => v as i64,
                None => {
                    unimplemented!("In-array nullable not implemented for Vec<u8> for Pandas")
                }
            })
            .collect()
    }
}

impl<'py> TypeConversion<Vec<Option<u16>>, Vec<i64>> for ClickHousePandasTransport<'py> {
    fn convert(val: Vec<Option<u16>>) -> Vec<i64> {
        val.into_iter()
            .map(|v| match v {
                Some(v) => v as i64,
                None => {
                    unimplemented!("In-array nullable not implemented for Vec<u16> for Pandas")
                }
            })
            .collect()
    }
}

impl<'py> TypeConversion<Vec<Option<u32>>, Vec<i64>> for ClickHousePandasTransport<'py> {
    fn convert(val: Vec<Option<u32>>) -> Vec<i64> {
        val.into_iter()
            .map(|v| match v {
                Some(v) => v as i64,
                None => {
                    unimplemented!("In-array nullable not implemented for Vec<u32> for Pandas")
                }
            })
            .collect()
    }
}

impl<'py> TypeConversion<Vec<Option<u64>>, Vec<i64>> for ClickHousePandasTransport<'py> {
    fn convert(val: Vec<Option<u64>>) -> Vec<i64> {
        val.into_iter()
            .map(|v| match v {
                Some(v) => v as i64,
                None => {
                    unimplemented!("In-array nullable not implemented for Vec<u64> for Pandas")
                }
            })
            .collect()
    }
}
