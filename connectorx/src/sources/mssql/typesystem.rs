use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use rust_decimal::Decimal;
use tiberius::ColumnType;
use uuid::Uuid;

#[derive(Copy, Clone, Debug)]
pub enum MsSQLTypeSystem {
    Tinyint(bool),
    Smallint(bool),
    Int(bool),
    Bigint(bool),
    Float24(bool),
    Float53(bool),
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
}

impl_typesystem! {
    system = MsSQLTypeSystem,
    mappings = {
        { Tinyint  => u8 }
        { Smallint => i16 }
        { Int => i32 }
        { Bigint => i64 }
        { Float24 => f32 }
        { Float53 => f64 }
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
            ColumnType::Int1 => Tinyint(true),
            ColumnType::Int2 => Smallint(true),
            ColumnType::Int4 => Int(true),
            ColumnType::Int8 => Bigint(true),
            ColumnType::Intn => Int(true),
            ColumnType::Float4 => Float24(true),
            ColumnType::Float8 => Float53(true),
            ColumnType::Floatn => Float53(true),
            ColumnType::Bit => Bit(false),
            ColumnType::Bitn => Bit(true),
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
            ColumnType::Datetime => Smalldatetime(true),
            ColumnType::Datetime2 => Datetime2(true),
            ColumnType::Datetimen => Datetime(true),
            ColumnType::Daten => Date(true),
            ColumnType::Timen => Time(true),
            ColumnType::DatetimeOffsetn => Datetimeoffset(true),
            _ => unimplemented!("{}", format!("{:?}", ty)),
        }
    }
}

// Link MsSQLDTypes back to the one defined by the mysql crate.
impl<'a> From<MsSQLTypeSystem> for ColumnType {
    fn from(ty: MsSQLTypeSystem) -> ColumnType {
        use MsSQLTypeSystem::*;
        match ty {
            Tinyint(_) => ColumnType::Int1,
            Smallint(_) => ColumnType::Int2,
            Int(_) => ColumnType::Int4,
            Bigint(_) => ColumnType::Int8,
            Float24(_) => ColumnType::Float4,
            Float53(_) => ColumnType::Float8,
            Bit(_) => ColumnType::Bit,
            Nvarchar(_) => ColumnType::NVarchar,
            Varchar(_) => ColumnType::BigVarChar,
            Nchar(_) => ColumnType::NChar,
            Char(_) => ColumnType::BigChar,
            Ntext(_) => ColumnType::NText,
            Text(_) => ColumnType::Text,
            Binary(_) => ColumnType::BigBinary,
            Varbinary(_) => ColumnType::BigVarBin,
            Image(_) => ColumnType::Image,
            Uniqueidentifier(_) => ColumnType::Guid,
            Decimal(_) => ColumnType::Decimaln,
            Numeric(_) => ColumnType::Numericn,
            Smalldatetime(_) => ColumnType::Datetime,
            Datetime2(_) => ColumnType::Datetime2,
            Datetime(_) => ColumnType::Datetime,
            Date(_) => ColumnType::Daten,
            Time(_) => ColumnType::Timen,
            Datetimeoffset(_) => ColumnType::DatetimeOffsetn,
        }
    }
}
