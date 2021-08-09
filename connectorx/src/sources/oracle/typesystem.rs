use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use r2d2_oracle::oracle::sql_type::OracleType;
use rust_decimal::Decimal;

#[derive(Copy, Clone, Debug)]
pub enum OracleTypeSystem {
    Number(bool),
    VarChar(bool),
}
