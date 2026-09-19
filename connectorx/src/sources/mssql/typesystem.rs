use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use rust_decimal::Decimal;
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

/// Nullable int/float columns are reported as a single `IntN`/`FltN`
/// metadata variant regardless of byte width. The actual width is available
/// when each row is decoded into a
/// [`mssql_tds::datatypes::column_values::ColumnValues`].
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
            // Treat the generic nullable `MoneyN` as `Money` (f64); its
            // metadata does not reveal whether a row uses 4 or 8 bytes.
            TdsDataType::Money => Money(nullable),
            TdsDataType::MoneyN => Money(true),
            TdsDataType::Money4 => SmallMoney(nullable),
            ref ty => unimplemented!("{}", format!("{:?}", ty)),
        }
    }
}

pub struct IntN(pub i64);

pub struct FloatN(pub f64);
