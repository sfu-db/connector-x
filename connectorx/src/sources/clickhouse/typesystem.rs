use std::net::IpAddr;

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use chrono_tz::Tz;
use regex::Regex;
use rust_decimal::Decimal;
use rustc_hash::FxHashMap;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum DataType {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
    Decimal(Decimal),
    String(String),
    FixedString(Vec<u8>),
    Date(NaiveDate),
    Date32(NaiveDate),
    Time(NaiveTime),
    Time64(NaiveTime),
    DateTime(DateTime<Utc>),
    DateTime64(DateTime<Utc>),
    Enum8(String),
    Enum16(String),
    UUID(Uuid),
    IPv4(IpAddr),
    IPv6(IpAddr),
    Bool(bool),
    Null,
    ArrayBool(Vec<Option<bool>>),
    ArrayString(Vec<Option<String>>),
    ArrayInt8(Vec<Option<i8>>),
    ArrayInt16(Vec<Option<i16>>),
    ArrayInt32(Vec<Option<i32>>),
    ArrayInt64(Vec<Option<i64>>),
    ArrayUInt8(Vec<Option<u8>>),
    ArrayUInt16(Vec<Option<u16>>),
    ArrayUInt32(Vec<Option<u32>>),
    ArrayUInt64(Vec<Option<u64>>),
    ArrayFloat32(Vec<Option<f32>>),
    ArrayFloat64(Vec<Option<f64>>),
    ArrayDecimal(Vec<Option<Decimal>>),
}

#[derive(Clone, Copy, Debug)]
pub enum ClickHouseTypeSystem {
    Int8(bool),
    Int16(bool),
    Int32(bool),
    Int64(bool),
    UInt8(bool),
    UInt16(bool),
    UInt32(bool),
    UInt64(bool),
    Float32(bool),
    Float64(bool),
    Decimal(bool),
    String(bool),
    FixedString(bool),
    Date(bool),
    Date32(bool),
    Time(bool),
    Time64(bool),
    DateTime(bool),
    DateTime64(bool),
    Enum8(bool),
    Enum16(bool),
    UUID(bool),
    IPv4(bool),
    IPv6(bool),
    Bool(bool),
    // ClickHouse supports arrays of any type,
    // but for simplicity we only define arrays of primitive types here.
    ArrayBool(bool),
    ArrayString(bool),
    ArrayInt8(bool),
    ArrayInt16(bool),
    ArrayInt32(bool),
    ArrayInt64(bool),
    ArrayUInt8(bool),
    ArrayUInt16(bool),
    ArrayUInt32(bool),
    ArrayUInt64(bool),
    ArrayFloat32(bool),
    ArrayFloat64(bool),
    ArrayDecimal(bool),
}

#[derive(Clone, Debug, Default)]
pub struct TypeMetadata {
    /// Decimal
    pub scale: u8,
    /// Decimal, Time64, DateTime64
    pub precision: u8,
    /// DateTime, DateTime64
    pub timezone: Option<Tz>,
    /// Enum8, Enum16
    pub named_values: Option<FxHashMap<i16, String>>,
    /// FixedString
    pub length: usize,
}

impl_typesystem! {
    system = ClickHouseTypeSystem,
    mappings = {
        { Int8 => i8 }
        { Int16 => i16 }
        { Int32 => i32 }
        { Int64 => i64 }
        { UInt8 => u8 }
        { UInt16 => u16 }
        { UInt32 => u32 }
        { UInt64 => u64 }
        { Float32 => f32 }
        { Float64 => f64 }
        { Decimal => Decimal }
        { String | Enum8 | Enum16 => String }
        { FixedString => Vec<u8> }
        { Date | Date32 => NaiveDate }
        { Time | Time64 => NaiveTime }
        { DateTime | DateTime64 => DateTime<Utc> }
        { UUID => Uuid }
        { IPv4 | IPv6 => IpAddr }
        { Bool => bool }
        { ArrayBool => Vec<Option<bool>> }
        { ArrayString => Vec<Option<String>> }
        { ArrayInt8 => Vec<Option<i8>> }
        { ArrayInt16 => Vec<Option<i16>> }
        { ArrayInt32 => Vec<Option<i32>> }
        { ArrayInt64 => Vec<Option<i64>> }
        { ArrayUInt8 => Vec<Option<u8>> }
        { ArrayUInt16 => Vec<Option<u16>> }
        { ArrayUInt32 => Vec<Option<u32>> }
        { ArrayUInt64 => Vec<Option<u64>> }
        { ArrayFloat32 => Vec<Option<f32>> }
        { ArrayFloat64 => Vec<Option<f64>> }
        { ArrayDecimal => Vec<Option<Decimal>> }
    }
}

impl ClickHouseTypeSystem {
    pub fn from_type_str_with_metadata(type_str: &str) -> (Self, TypeMetadata) {
        use ClickHouseTypeSystem::*;

        let type_str = type_str.trim();
        let mut metadata = TypeMetadata::default();

        let (type_str, nullable) = Self::unwrap_nullable(type_str);

        if let Some(inner) = Self::unwrap_wrapper(type_str, "LowCardinality") {
            return Self::from_type_str_with_metadata(inner);
        }

        if let Some(inner) = Self::unwrap_wrapper(type_str, "Array") {
            if let Some(array_type) = Self::parse_array_type(inner, nullable) {
                return (array_type, metadata);
            }
            return (String(nullable), metadata);
        }

        if let Some(params) = Self::unwrap_wrapper(type_str, "Enum8") {
            metadata.named_values = Self::parse_enum_definition(params);
            return (Enum8(nullable), metadata);
        }
        if let Some(params) = Self::unwrap_wrapper(type_str, "Enum16") {
            metadata.named_values = Self::parse_enum_definition(params);
            return (Enum16(nullable), metadata);
        }

        let (base_type, params) = Self::split_type_params(type_str);

        let ts = match base_type {
            "Int8" => Int8(nullable),
            "Int16" => Int16(nullable),
            "Int32" => Int32(nullable),
            "Int64" => Int64(nullable),
            "UInt8" => UInt8(nullable),
            "UInt16" => UInt16(nullable),
            "UInt32" => UInt32(nullable),
            "UInt64" => UInt64(nullable),
            "Float32" => Float32(nullable),
            "Float64" => Float64(nullable),
            "Decimal" => {
                let (precision, scale) = Self::parse_decimal_precision_scale(params);
                metadata.precision = precision;
                metadata.scale = scale;
                Decimal(nullable)
            }
            "String" => String(nullable),
            "FixedString" => {
                metadata.length = Self::parse_length(params);
                FixedString(nullable)
            }
            "Date" => Date(nullable),
            "Date32" => Date32(nullable),
            "Time" => Time(nullable),
            "Time64" => {
                metadata.precision = Self::parse_time64_precision(params);
                Time64(nullable)
            }
            "DateTime" => {
                metadata.timezone = Self::parse_datetime_params(params).1;
                DateTime(nullable)
            }
            "DateTime64" => {
                let (precision, tz) = Self::parse_datetime_params(params);
                metadata.precision = precision.unwrap_or(3);
                metadata.timezone = tz;
                DateTime64(nullable)
            }
            "UUID" => UUID(nullable),
            "Bool" => Bool(nullable),
            "IPv4" => IPv4(nullable),
            "IPv6" => IPv6(nullable),
            "Enum8" => {
                if let Some(params) = params {
                    metadata.named_values = Self::parse_enum_definition(params);
                }
                Enum8(nullable)
            }
            "Enum16" => {
                if let Some(params) = params {
                    metadata.named_values = Self::parse_enum_definition(params);
                }
                Enum16(nullable)
            }
            _ => String(nullable),
        };

        (ts, metadata)
    }

    pub fn from_type_str(type_str: &str) -> Self {
        Self::from_type_str_with_metadata(type_str).0
    }

    fn unwrap_nullable(s: &str) -> (&str, bool) {
        if let Some(inner) = Self::unwrap_wrapper(s, "Nullable") {
            (inner, true)
        } else {
            (s, false)
        }
    }

    /// Unwrap "Wrapper(inner)" -> Some(inner), or None if not matching
    fn unwrap_wrapper<'a>(s: &'a str, wrapper: &str) -> Option<&'a str> {
        let prefix = format!("{}(", wrapper);
        if s.starts_with(&prefix) && s.ends_with(')') {
            Some(&s[prefix.len()..s.len() - 1])
        } else {
            None
        }
    }

    fn split_type_params(s: &str) -> (&str, Option<&str>) {
        if let Some(idx) = s.find('(') {
            if s.ends_with(')') {
                let base = &s[..idx];
                let params_str = &s[idx + 1..s.len() - 1];
                return (base, Some(params_str));
            }
        }
        (s, None)
    }

    /// Parse FixedString(N)
    fn parse_length(params: Option<&str>) -> usize {
        let params = params.and_then(|p| p.split(',').map(|i| i.trim()).collect::<Vec<_>>().into());
        params
            .and_then(|p| p.get(0).and_then(|s| s.parse::<usize>().ok()))
            .unwrap_or(1)
    }

    /// Parse DateTime(precision, 'Timezone') or DateTime('Timezone')
    fn parse_datetime_params(params: Option<&str>) -> (Option<u8>, Option<Tz>) {
        let params = params.and_then(|p| p.split(',').map(|i| i.trim()).collect::<Vec<_>>().into());
        match params {
            None => (None, None),
            Some(p) => {
                if let Some(precision) = p.get(0).and_then(|s| s.parse::<u8>().ok()) {
                    let timezone = p.get(1).and_then(|s| Self::parse_timezone(s));
                    (Some(precision), timezone)
                } else {
                    (None, p.get(0).and_then(|s| Self::parse_timezone(s)))
                }
            }
        }
    }

    /// Parse Time64(precision)
    fn parse_time64_precision(params: Option<&str>) -> u8 {
        let params = params.and_then(|p| p.split(',').map(|i| i.trim()).collect::<Vec<_>>().into());
        params
            .and_then(|p| p.get(0).and_then(|s| s.parse::<u8>().ok()))
            .unwrap_or(3)
    }

    /// Parse Decimal(precision, scale)
    fn parse_decimal_precision_scale(params: Option<&str>) -> (u8, u8) {
        let params = params.and_then(|p| p.split(',').map(|i| i.trim()).collect::<Vec<_>>().into());
        params
            .and_then(|p| {
                let precision = p.get(0).and_then(|s| s.parse::<u8>().ok());
                let scale = p.get(1).and_then(|s| s.parse::<u8>().ok());
                Some((precision.unwrap_or(0), scale.unwrap_or(0)))
            })
            .unwrap_or((0, 0))
    }

    fn parse_timezone(s: &str) -> Option<Tz> {
        s.trim_matches('\'').parse::<Tz>().ok()
    }

    /// Parse Array(InnerType) and return the corresponding ArrayXxx variant
    fn parse_array_type(inner_type: &str, nullable: bool) -> Option<Self> {
        use ClickHouseTypeSystem::*;

        let inner = inner_type.trim();

        let inner = if let Some(unwrapped) = Self::unwrap_wrapper(inner, "Nullable") {
            unwrapped
        } else {
            inner
        };

        let (base, _) = Self::split_type_params(inner);

        match base {
            "Bool" => Some(ArrayBool(nullable)),
            "String" => Some(ArrayString(nullable)),
            "Int8" => Some(ArrayInt8(nullable)),
            "Int16" => Some(ArrayInt16(nullable)),
            "Int32" => Some(ArrayInt32(nullable)),
            "Int64" => Some(ArrayInt64(nullable)),
            "UInt8" => Some(ArrayUInt8(nullable)),
            "UInt16" => Some(ArrayUInt16(nullable)),
            "UInt32" => Some(ArrayUInt32(nullable)),
            "UInt64" => Some(ArrayUInt64(nullable)),
            "Float32" => Some(ArrayFloat32(nullable)),
            "Float64" => Some(ArrayFloat64(nullable)),
            "Decimal" => Some(ArrayDecimal(nullable)),
            _ => None,
        }
    }

    /// Parse Enum8/Enum16 definitions like "'a' = 1, 'b' = 2, 'c' = 3"
    fn parse_enum_definition(params: &str) -> Option<FxHashMap<i16, String>> {
        let re = Regex::new(r"'((?:\\'|[^'])*)'\s*=\s*(-?\d+)").unwrap();

        re.captures_iter(params)
            .map(|cap| {
                let key = cap[1].replace("\\'", "'");
                let value = cap[2].parse().unwrap();
                (value, key)
            })
            .collect::<FxHashMap<_, _>>()
            .into()
    }

    pub fn is_nullable(&self) -> bool {
        match self {
            ClickHouseTypeSystem::Int8(nullable)
            | ClickHouseTypeSystem::Int16(nullable)
            | ClickHouseTypeSystem::Int32(nullable)
            | ClickHouseTypeSystem::Int64(nullable)
            | ClickHouseTypeSystem::UInt8(nullable)
            | ClickHouseTypeSystem::UInt16(nullable)
            | ClickHouseTypeSystem::UInt32(nullable)
            | ClickHouseTypeSystem::UInt64(nullable)
            | ClickHouseTypeSystem::Float32(nullable)
            | ClickHouseTypeSystem::Float64(nullable)
            | ClickHouseTypeSystem::Decimal(nullable)
            | ClickHouseTypeSystem::String(nullable)
            | ClickHouseTypeSystem::FixedString(nullable)
            | ClickHouseTypeSystem::Date(nullable)
            | ClickHouseTypeSystem::Date32(nullable)
            | ClickHouseTypeSystem::Time(nullable)
            | ClickHouseTypeSystem::Time64(nullable)
            | ClickHouseTypeSystem::DateTime(nullable)
            | ClickHouseTypeSystem::DateTime64(nullable)
            | ClickHouseTypeSystem::Enum8(nullable)
            | ClickHouseTypeSystem::Enum16(nullable)
            | ClickHouseTypeSystem::UUID(nullable)
            | ClickHouseTypeSystem::IPv4(nullable)
            | ClickHouseTypeSystem::IPv6(nullable)
            | ClickHouseTypeSystem::Bool(nullable)
            // It's not possible to have nullable arrays in ClickHouse,
            // but we keep the nullable flag for consistency with other types.
            | ClickHouseTypeSystem::ArrayBool(nullable)
            | ClickHouseTypeSystem::ArrayString(nullable)
            | ClickHouseTypeSystem::ArrayInt8(nullable)
            | ClickHouseTypeSystem::ArrayInt16(nullable)
            | ClickHouseTypeSystem::ArrayInt32(nullable)
            | ClickHouseTypeSystem::ArrayInt64(nullable)
            | ClickHouseTypeSystem::ArrayUInt8(nullable)
            | ClickHouseTypeSystem::ArrayUInt16(nullable)
            | ClickHouseTypeSystem::ArrayUInt32(nullable)
            | ClickHouseTypeSystem::ArrayUInt64(nullable)
            | ClickHouseTypeSystem::ArrayFloat32(nullable)
            | ClickHouseTypeSystem::ArrayFloat64(nullable)
            | ClickHouseTypeSystem::ArrayDecimal(nullable) => *nullable,
        }
    }
}
