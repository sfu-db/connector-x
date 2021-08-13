use chrono::{NaiveDate};
use r2d2_oracle::oracle::sql_type::OracleType;

#[derive(Copy, Clone, Debug)]
pub enum OracleTypeSystem {
    Int(bool),
    Float(bool),
    VarChar(bool),
    Char(bool),
    Date(bool),
}

impl_typesystem! {
    system = OracleTypeSystem,
    mappings = {
        { Int => i64 }
        { Float => f64 }
        { VarChar | Char => String }
        { Date => NaiveDate }
    }
}

impl<'a> From<&'a OracleType> for OracleTypeSystem {
    fn from(ty: &'a OracleType) -> OracleTypeSystem {
        use OracleTypeSystem::*;
        match ty {
            OracleType::Number(_, 0) => Int(true),
            OracleType::Number(_, _) => Float(true),
            OracleType::Varchar2(_) => VarChar(true),
            OracleType::Char(_) => Char(true),
            OracleType::Date => Date(true),
            _ => unimplemented!("{}", format!("{:?}", ty)),
        }
    }
}

