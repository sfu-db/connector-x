use chrono::{NaiveDate, NaiveDateTime};
use r2d2_oracle::oracle::sql_type::OracleType;

#[derive(Copy, Clone, Debug)]
pub enum OracleTypeSystem {
    NumInt(bool),
    Float(bool),
    NumFloat(bool),
    VarChar(bool),
    Char(bool),
    NVarChar(bool),
    NChar(bool),
    Date(bool),
    Timestamp(bool)
}

impl_typesystem! {
    system = OracleTypeSystem,
    mappings = {
        { NumInt => i64 }
        { Float | NumFloat => f64 }
        { VarChar | Char | NVarChar | NChar => String }
        { Date => NaiveDate }
        { Timestamp => NaiveDateTime }
    }
}

impl<'a> From<&'a OracleType> for OracleTypeSystem {
    fn from(ty: &'a OracleType) -> OracleTypeSystem {
        use OracleTypeSystem::*;
        match ty {
            OracleType::Number(_, 0) => NumInt(true),
            OracleType::Number(_, _) => NumFloat(true),
            OracleType::Float(_) => Float(true),
            OracleType::Varchar2(_) => VarChar(true),
            OracleType::Char(_) => Char(true),
            OracleType::NChar(_) => NChar(true),
            OracleType::NVarchar2(_) => NVarChar(true),
            OracleType::Date => Date(true),
            OracleType::Timestamp(_) => Timestamp(true),
            _ => unimplemented!("{}", format!("{:?}", ty)),
        }
    }
}

