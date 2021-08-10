// use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use r2d2_oracle::oracle::sql_type::OracleType;
// use rust_decimal::Decimal;

#[derive(Copy, Clone, Debug)]
pub enum OracleTypeSystem {
    Int(bool),
    Float(bool),
    VarChar(bool),
}

impl_typesystem! {
    system = OracleTypeSystem,
    mappings = {
        { Int => i64 }
        { Float => f64 }
        { VarChar => String }
    }
}

impl<'a> From<&'a OracleType> for OracleTypeSystem {
    fn from(ty: &'a OracleType) -> OracleTypeSystem {
        use OracleTypeSystem::*;
        match ty {
            OracleType::Number(_, 0) => Int(true),
            OracleType::Number(_, _) => Float(true),
            OracleType::Varchar2(_) => VarChar(true),
            _ => unimplemented!("{}", format!("{:?}", ty)),
        }
    }
}

impl<'a> From<OracleTypeSystem> for OracleType {
    fn from(ty: OracleTypeSystem) -> OracleType {
        use OracleTypeSystem::*;
        match ty {
            Int(_) => OracleType::Number(38, 0),
            Float(_) => OracleType::Number(38, 127),
            VarChar(_) => OracleType::Varchar2(10),
        }
    }
}
