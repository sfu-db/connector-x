use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use rust_decimal::Decimal;
#[cfg(feature = "src_mssql_tiberius")]
use tiberius::{ColumnData, ColumnType, FromSql};
use uuid_old::Uuid;

// https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-tds/ce3183a6-9d89-47e8-a02f-de5a1a1303de
#[derive(Copy, Clone, Debug)]
pub enum MsSQLTypeSystem {
    Tinyint(bool),
    Smallint(bool),
    Int(bool),
    Bigint(bool),
    Intn(bool),
    Float24(bool),
    Float53(bool),
    Floatn(bool),
    Bit(bool),
    Nvarchar(bool),
    Varchar(bool),
    Nchar(bool),
    Char(bool),
    Ntext(bool),
    Text(bool),
    Binary(bool),
    Varbinary(bool),
    Image(bool),
    Uniqueidentifier(bool),
    Numeric(bool),
    Decimal(bool),
    Datetime(bool),
    Datetime2(bool),
    Smalldatetime(bool),
    Date(bool),
    Time(bool),
    Datetimeoffset(bool),
    Money(bool),
    SmallMoney(bool),
}

impl_typesystem! {
    system = MsSQLTypeSystem,
    mappings = {
        { Tinyint  => u8 }
        { Smallint => i16 }
        { Int => i32 }
        { Bigint => i64 }
        { Intn => IntN }
        { Float24 | SmallMoney => f32 }
        { Float53 | Money => f64 }
        { Floatn => FloatN }
        { Bit => bool }
        { Nvarchar | Varchar | Nchar | Char | Text | Ntext => &'r str }
        { Binary | Varbinary | Image => &'r [u8] }
        { Uniqueidentifier => Uuid }
        { Numeric | Decimal => Decimal }
        { Datetime | Datetime2 | Smalldatetime => NaiveDateTime }
        { Date => NaiveDate }
        { Time => NaiveTime }
        { Datetimeoffset => DateTime<Utc> }
    }
}

#[cfg(feature = "src_mssql_tiberius")]
impl<'a> From<&'a ColumnType> for MsSQLTypeSystem {
    fn from(ty: &'a ColumnType) -> MsSQLTypeSystem {
        use MsSQLTypeSystem::*;

        match ty {
            ColumnType::Int1 => Tinyint(false),
            ColumnType::Int2 => Smallint(false),
            ColumnType::Int4 => Int(false),
            ColumnType::Int8 => Bigint(false),
            ColumnType::Intn => Intn(true),
            ColumnType::Float4 => Float24(false),
            ColumnType::Float8 => Float53(false),
            ColumnType::Floatn => Floatn(true),
            ColumnType::Bit => Bit(false),
            ColumnType::Bitn => Bit(true), // nullable int, var-length
            ColumnType::NVarchar => Nvarchar(true),
            ColumnType::BigVarChar => Varchar(true),
            ColumnType::NChar => Nchar(true),
            ColumnType::BigChar => Char(true),
            ColumnType::NText => Ntext(true),
            ColumnType::Text => Text(true),
            ColumnType::BigBinary => Binary(true),
            ColumnType::BigVarBin => Varbinary(true),
            ColumnType::Image => Image(true),
            ColumnType::Guid => Uniqueidentifier(true),
            ColumnType::Decimaln => Decimal(true),
            ColumnType::Numericn => Numeric(true),
            ColumnType::Datetime => Datetime(false),
            ColumnType::Datetime2 => Datetime2(true),
            ColumnType::Datetimen => Datetime(true),
            ColumnType::Datetime4 => Datetime(false),
            ColumnType::Daten => Date(true),
            ColumnType::Timen => Time(true),
            ColumnType::DatetimeOffsetn => Datetimeoffset(true),
            ColumnType::Money => Money(true),
            ColumnType::Money4 => SmallMoney(true),
            _ => unimplemented!("{}", format!("{:?}", ty)),
        }
    }
}

/// `mssql-tds`'s wire-level metadata is coarser than Tiberius's: nullable
/// int/float columns are all reported as a single `IntN`/`FltN` variant
/// regardless of byte width (the actual width is only known once a row is
/// decoded into a [`mssql_tds::datatypes::column_values::ColumnValues`]).
/// This mirrors how `MsSQLTypeSystem::Intn`/`Floatn` already behave on the
/// Tiberius path today, so no new type-system variants are needed.
#[cfg(feature = "src_mssql_tds")]
impl<'a> From<&'a mssql_tds::query::metadata::ColumnMetadata> for MsSQLTypeSystem {
    fn from(meta: &'a mssql_tds::query::metadata::ColumnMetadata) -> MsSQLTypeSystem {
        use mssql_tds::datatypes::sqldatatypes::TdsDataType;
        use MsSQLTypeSystem::*;

        let nullable = meta.is_nullable();
        match meta.data_type {
            TdsDataType::Int1 => Tinyint(nullable),
            TdsDataType::Int2 => Smallint(nullable),
            TdsDataType::Int4 => Int(nullable),
            TdsDataType::Int8 => Bigint(nullable),
            TdsDataType::IntN => Intn(true),
            TdsDataType::Flt4 => Float24(nullable),
            TdsDataType::Flt8 => Float53(nullable),
            TdsDataType::FltN => Floatn(true),
            TdsDataType::Bit | TdsDataType::BitN => Bit(nullable),
            TdsDataType::NVarChar => Nvarchar(true),
            TdsDataType::BigVarChar => Varchar(true),
            TdsDataType::NChar => Nchar(true),
            TdsDataType::BigChar => Char(true),
            TdsDataType::NText => Ntext(true),
            TdsDataType::Text => Text(true),
            TdsDataType::BigBinary => Binary(true),
            TdsDataType::BigVarBinary => Varbinary(true),
            TdsDataType::Image => Image(true),
            TdsDataType::Guid => Uniqueidentifier(true),
            TdsDataType::DecimalN | TdsDataType::Decimal => Decimal(true),
            TdsDataType::NumericN | TdsDataType::Numeric => Numeric(true),
            TdsDataType::DateTime | TdsDataType::DateTim4 => Datetime(nullable),
            TdsDataType::DateTimeN => Datetime(true),
            TdsDataType::DateTime2N => Datetime2(true),
            TdsDataType::DateN => Date(true),
            TdsDataType::TimeN => Time(true),
            TdsDataType::DateTimeOffsetN => Datetimeoffset(true),
            // Only a fixed-width `Money4` wire type (non-nullable smallmoney)
            // is treated as SmallMoney/f32. A nullable smallmoney column is
            // sent as the generic `MoneyN` wire type with a per-row length
            // (4 or 8 bytes) that isn't known statically from metadata alone
            // - matching Tiberius, which always maps its nullable `MONEYN`
            // wire type to F64 regardless of the per-row byte length (see
            // tiberius's `money::decode`), so parity requires treating
            // `MoneyN` as `Money` (f64) here too, never `SmallMoney`.
            TdsDataType::Money => Money(nullable),
            TdsDataType::MoneyN => Money(true),
            TdsDataType::Money4 => SmallMoney(nullable),
            ref ty => unimplemented!("{}", format!("{:?}", ty)),
        }
    }
}

pub struct IntN(pub i64);
#[cfg(feature = "src_mssql_tiberius")]
impl<'a> FromSql<'a> for IntN {
    fn from_sql(value: &'a ColumnData<'static>) -> Result<Option<Self>, tiberius::error::Error> {
        match value {
            ColumnData::U8(None)
            | ColumnData::I16(None)
            | ColumnData::I32(None)
            | ColumnData::I64(None) => Ok(None),
            ColumnData::U8(Some(d)) => Ok(Some(IntN(*d as i64))),
            ColumnData::I16(Some(d)) => Ok(Some(IntN(*d as i64))),
            ColumnData::I32(Some(d)) => Ok(Some(IntN(*d as i64))),
            ColumnData::I64(Some(d)) => Ok(Some(IntN(*d))),
            v => Err(tiberius::error::Error::Conversion(
                format!("cannot interpret {:?} as a intn value", v).into(),
            )),
        }
    }
}

pub struct FloatN(pub f64);
#[cfg(feature = "src_mssql_tiberius")]
impl<'a> FromSql<'a> for FloatN {
    fn from_sql(value: &'a ColumnData<'static>) -> Result<Option<Self>, tiberius::error::Error> {
        match value {
            ColumnData::F32(None) | ColumnData::F64(None) => Ok(None),
            ColumnData::F32(Some(d)) => Ok(Some(FloatN(*d as f64))),
            ColumnData::F64(Some(d)) => Ok(Some(FloatN(*d))),
            v => Err(tiberius::error::Error::Conversion(
                format!("cannot interpret {:?} as a floatn value", v).into(),
            )),
        }
    }
}
