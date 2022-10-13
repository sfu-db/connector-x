use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use r2d2_oracle::oracle::sql_type::OracleType;

#[derive(Copy, Clone, Debug)]
pub enum OracleTypeSystem {
    NumInt(bool),
    Float(bool),
    NumFloat(bool),
    BinaryFloat(bool),
    BinaryDouble(bool),
    Blob(bool),
    Clob(bool),
    VarChar(bool),
    Char(bool),
    NVarChar(bool),
    NChar(bool),
    Date(bool),
    Timestamp(bool),
    TimestampTz(bool),
}

impl_typesystem! {
    system = OracleTypeSystem,
    mappings = {
        { NumInt => i64 }
        { Float | NumFloat | BinaryFloat | BinaryDouble => f64 }
        { Blob => Vec<u8>}
        { Clob | VarChar | Char | NVarChar | NChar => String }
        { Date => NaiveDate }
        { Timestamp => NaiveDateTime }
        { TimestampTz => DateTime<Utc> }
    }
}

impl<'a> From<&'a OracleType> for OracleTypeSystem {
    fn from(ty: &'a OracleType) -> OracleTypeSystem {
        use OracleTypeSystem::*;
        match ty {
            OracleType::Number(0, 0) => NumFloat(true),
            OracleType::Number(_, 0) => NumInt(true),
            OracleType::Number(_, _) => NumFloat(true),
            OracleType::Float(_) => Float(true),
            OracleType::BinaryFloat => BinaryFloat(true),
            OracleType::BinaryDouble => BinaryDouble(true),
            OracleType::BLOB => Blob(true),
            OracleType::CLOB => Clob(true),
            OracleType::Char(_) | OracleType::Long => Char(true),
            OracleType::NChar(_) => NChar(true),
            OracleType::Varchar2(_) => VarChar(true),
            OracleType::NVarchar2(_) => NVarChar(true),
            OracleType::Date => Date(true),
            OracleType::Timestamp(_) => Timestamp(true),
            OracleType::TimestampTZ(_) => TimestampTz(true),
            _ => unimplemented!("{}", format!("Type {:?} not implemented for oracle!", ty)),
        }
    }
}
