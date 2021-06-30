use r2d2_mysql::mysql::consts::ColumnType;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rust_decimal::Decimal;
#[derive(Copy, Clone, Debug)]
pub enum MysqlTypeSystem {
    Double(bool),
    Long(bool),
    Datetime(bool),
    Date(bool),
    Time(bool),
    Decimal(bool),
    Char(bool),
    VarChar(bool)
}

impl_typesystem! {
    system = MysqlTypeSystem,
    mappings = {
        { Long => i64 }
        { Double => f64 }
        { Datetime => NaiveDateTime}
        { Date => NaiveDate}
        { Time => NaiveTime}
        { Decimal => Decimal}
        { Char | VarChar => String}
    }
}

impl<'a> From<&'a ColumnType> for MysqlTypeSystem {
    fn from(ty: &'a ColumnType) -> MysqlTypeSystem {
        use MysqlTypeSystem::*;
        match ty {
            ColumnType::MYSQL_TYPE_LONG => Long(true),
            ColumnType::MYSQL_TYPE_DOUBLE => Double(true),
            ColumnType::MYSQL_TYPE_DATETIME => Datetime(true),
            ColumnType::MYSQL_TYPE_DATE => Date(true),
            ColumnType::MYSQL_TYPE_TIME => Time(true),
            ColumnType::MYSQL_TYPE_NEWDECIMAL => Decimal(true),
            ColumnType::MYSQL_TYPE_STRING => Char(true),
            ColumnType::MYSQL_TYPE_VAR_STRING => VarChar(true),
            _ => unimplemented!("{}", format!("{:?}", ty)),
        }
    }
}

// Link MysqlDTypes back to the one defined by the mysql crate.
impl<'a> From<MysqlTypeSystem> for ColumnType {
    fn from(ty: MysqlTypeSystem) -> ColumnType {
        use MysqlTypeSystem::*;
        match ty {
            Long(_) => ColumnType::MYSQL_TYPE_LONG,
            Double(_) => ColumnType::MYSQL_TYPE_DOUBLE,
            Datetime(_) => ColumnType::MYSQL_TYPE_DATETIME,
            Date(_) => ColumnType::MYSQL_TYPE_DATE,
            Time(_) => ColumnType::MYSQL_TYPE_TIME,
            Decimal(_) => ColumnType::MYSQL_TYPE_NEWDECIMAL,
            Char(_) => ColumnType::MYSQL_TYPE_STRING,
            VarChar(_) => ColumnType::MYSQL_TYPE_VAR_STRING,
        }
    }
}
