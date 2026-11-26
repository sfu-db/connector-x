use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use rust_decimal::Decimal;
use uuid_old::Uuid;

// https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-tds/ce3183a6-9d89-47e8-a02f-de5a1a1303de
/// Column types of the SQL Server source. The `bool` is `true` when the column is nullable.
#[derive(Copy, Clone, Debug)]
pub enum MsSQLTypeSystem {
    /// SQL Server `tinyint` (unsigned 8-bit).
    Tinyint(bool),
    /// SQL Server `smallint`.
    Smallint(bool),
    /// SQL Server `int`.
    Int(bool),
    /// SQL Server `bigint`.
    Bigint(bool),
    /// Nullable SQL Server integer of any width, read as `i64`.
    Intn(bool),
    /// SQL Server `real` (`float(24)`).
    Float24(bool),
    /// SQL Server `float` (`float(53)`).
    Float53(bool),
    /// Nullable SQL Server float of either width, read as `f64`.
    Floatn(bool),
    /// SQL Server `bit`.
    Bit(bool),
    /// SQL Server `nvarchar`.
    Nvarchar(bool),
    /// SQL Server `varchar`.
    Varchar(bool),
    /// SQL Server `nchar`.
    Nchar(bool),
    /// SQL Server `char`.
    Char(bool),
    /// SQL Server `ntext`.
    Ntext(bool),
    /// SQL Server `text`.
    Text(bool),
    /// SQL Server `binary`.
    Binary(bool),
    /// SQL Server `varbinary`.
    Varbinary(bool),
    /// SQL Server `image`.
    Image(bool),
    /// SQL Server `uniqueidentifier`.
    Uniqueidentifier(bool),
    /// SQL Server `numeric`.
    Numeric(bool),
    /// SQL Server `decimal`.
    Decimal(bool),
    /// SQL Server `datetime`.
    Datetime(bool),
    /// SQL Server `datetime2`.
    Datetime2(bool),
    /// SQL Server `smalldatetime`.
    Smalldatetime(bool),
    /// SQL Server `date`.
    Date(bool),
    /// SQL Server `time`.
    Time(bool),
    /// SQL Server `datetimeoffset`, read as UTC.
    Datetimeoffset(bool),
    /// SQL Server `money`, read as `f64`.
    Money(bool),
    /// SQL Server `smallmoney`, read as `f32`.
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

/// Value of a nullable integer column of any width, widened to `i64`.
pub struct IntN(pub i64);

/// Value of a nullable float column of either width, widened to `f64`.
pub struct FloatN(pub f64);
