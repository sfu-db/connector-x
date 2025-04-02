use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use rust_decimal::Decimal;
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

pub struct IntN(pub i64);
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
