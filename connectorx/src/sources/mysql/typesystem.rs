use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use r2d2_mysql::mysql::consts::ColumnType;
use rust_decimal::Decimal;

#[derive(Copy, Clone, Debug)]
pub enum MySQLTypeSystem {
    Float(bool),
    Double(bool),
    Tiny(bool),
    Short(bool),
    Long(bool),
    LongLong(bool),
    Datetime(bool),
    Date(bool),
    Time(bool),
    Decimal(bool),
    Char(bool),
    VarChar(bool),
    Timestamp(bool),
    Year(bool),
    Enum(bool),
    TinyBlob(bool),
    Blob(bool),
    MediumBlob(bool),
    LongBlob(bool),
}

impl_typesystem! {
    system = MySQLTypeSystem,
    mappings = {
        { Tiny => i8 }
        { Short | Year => i16 }
        { Long | LongLong => i64 }
        { Float => f32 }
        { Double => f64 }
        { Datetime | Timestamp => NaiveDateTime }
        { Date => NaiveDate }
        { Time => NaiveTime }
        { Decimal => Decimal }
        { Char | VarChar => String }
        { Enum => &'r str }
        { TinyBlob | Blob | MediumBlob | LongBlob => Vec<u8>}
    }
}

impl<'a> From<&'a ColumnType> for MySQLTypeSystem {
    fn from(ty: &'a ColumnType) -> MySQLTypeSystem {
        use MySQLTypeSystem::*;
        match ty {
            ColumnType::MYSQL_TYPE_TINY => Tiny(true),
            ColumnType::MYSQL_TYPE_SHORT => Short(true),
            ColumnType::MYSQL_TYPE_LONG => Long(true),
            ColumnType::MYSQL_TYPE_LONGLONG => LongLong(true),
            ColumnType::MYSQL_TYPE_FLOAT => Float(true),
            ColumnType::MYSQL_TYPE_DOUBLE => Double(true),
            ColumnType::MYSQL_TYPE_DATETIME => Datetime(true),
            ColumnType::MYSQL_TYPE_DATE => Date(true),
            ColumnType::MYSQL_TYPE_TIME => Time(true),
            ColumnType::MYSQL_TYPE_NEWDECIMAL => Decimal(true),
            ColumnType::MYSQL_TYPE_STRING => Char(true),
            ColumnType::MYSQL_TYPE_VAR_STRING => VarChar(true),
            ColumnType::MYSQL_TYPE_TIMESTAMP => Timestamp(true),
            ColumnType::MYSQL_TYPE_YEAR => Year(true),
            ColumnType::MYSQL_TYPE_ENUM => Enum(true),
            ColumnType::MYSQL_TYPE_TINY_BLOB => TinyBlob(true),
            ColumnType::MYSQL_TYPE_BLOB => Blob(true),
            ColumnType::MYSQL_TYPE_MEDIUM_BLOB => MediumBlob(true),
            ColumnType::MYSQL_TYPE_LONG_BLOB => LongBlob(true),
            ColumnType::MYSQL_TYPE_JSON
            _ => unimplemented!("{}", format!("{:?}", ty)),
        }
    }
}

// Link MysqlDTypes back to the one defined by the mysql crate.
impl<'a> From<MySQLTypeSystem> for ColumnType {
    fn from(ty: MySQLTypeSystem) -> ColumnType {
        use MySQLTypeSystem::*;
        match ty {
            Tiny(_) => ColumnType::MYSQL_TYPE_TINY,
            Short(_) => ColumnType::MYSQL_TYPE_SHORT
            Long(_) => ColumnType::MYSQL_TYPE_LONG,
            LongLong(_) => ColumnType::MYSQL_TYPE_LONGLONG,
            Float(_) => ColumnType::MYSQL_TYPE_FLOAT,
            Double(_) => ColumnType::MYSQL_TYPE_DOUBLE,
            Datetime(_) => ColumnType::MYSQL_TYPE_DATETIME,
            Date(_) => ColumnType::MYSQL_TYPE_DATE,
            Time(_) => ColumnType::MYSQL_TYPE_TIME,
            Decimal(_) => ColumnType::MYSQL_TYPE_NEWDECIMAL,
            Char(_) => ColumnType::MYSQL_TYPE_STRING,
            VarChar(_) => ColumnType::MYSQL_TYPE_VAR_STRING,
            Timestamp(_) => ColumnType::MYSQL_TYPE_TIMESTAMP,
            Year(_) => ColumnType::MYSQL_TYPE_YEAR,
            Enum(_) => ColumnType::MYSQL_TYPE_ENUM,
            TinyBlob(_) => ColumnType::MYSQL_TYPE_TINY_BLOB
            Blob(_) => ColumnType::MYSQL_TYPE_BLOB,
            MediumBlob(_) => ColumnType::MYSQL_TYPE_MEDIUM_BLOB,
            LongBlob(_) => ColumnType::MYSQL_TYPE_LONG_BLOB,
         }
    }
}
