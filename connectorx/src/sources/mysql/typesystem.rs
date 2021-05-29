use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use mysql::consts::ColumnType;

use rust_decimal::Decimal;
use serde_json::Value;
use uuid::Uuid;

#[derive(Copy, Clone, Debug)]
pub enum MysqlTypeSystem {
    Double(bool),
    Long(bool),
    Text(bool),
}

impl_typesystem! {
    system = MysqlTypeSystem,
    mappings = {
        { Long => i64 }
        { Double => f64 }
        { Text => &'r str }
    }
}

impl<'a> From<&'a ColumnType> for MysqlTypeSystem {
    fn from(ty: &'a ColumnType) -> MysqlTypeSystem {
        use MysqlTypeSystem::*;
        match ty.name() {
            "MYSQL_TYPE_LONG" => Long(true),
            "MYSQL_TYPE_DOUBLE" => Double(true),
            "MYSQL_TYPE_" => Text(true),
            _ => match ty.kind() {
                postgres::types::Kind::Enum(_) => Enum(true),
                _ => unimplemented!("{}", ty.name()),
            },
        }
    }
}

// Link MysqlDTypes back to the one defiend by the postgres crate.
impl<'a> From<MysqlTypeSystem> for ColumnType {
    fn from(ty: MysqlTypeSystem) -> ColumnType {
        use MysqlTypeSystem::*;
        match ty {
            Int4(_) => Type::INT4,
            Float4(_) => Type::FLOAT4,
            Text(_) => Type::TEXT,
        }
    }
}
