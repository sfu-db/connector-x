use chrono::{DateTime, NaiveDateTime, Utc};
use r2d2_oracle::oracle::sql_type::OracleType;
use rust_decimal::Decimal;

#[derive(Copy, Clone, Debug)]
pub enum OracleTypeSystem {
    NumInt(bool),
    NumDecimal(bool),
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
    TimestampNano(bool),
    TimestampTzNano(bool),
}

impl_typesystem! {
    system = OracleTypeSystem,
    mappings = {
        { NumInt => i64 }
        { NumDecimal => Decimal }
        { Float | NumFloat | BinaryFloat | BinaryDouble => f64 }
        { Blob => Vec<u8>}
        { Clob | VarChar | Char | NVarChar | NChar => String }
        { Date | Timestamp | TimestampNano => NaiveDateTime }
        { TimestampTz | TimestampTzNano => DateTime<Utc> }
    }
}

impl<'a> From<&'a OracleType> for OracleTypeSystem {
    fn from(ty: &'a OracleType) -> OracleTypeSystem {
        use OracleTypeSystem::*;
        match ty {
            OracleType::Number(0, 0) => NumDecimal(true),
            OracleType::Number(_, 0) => NumInt(true),
            OracleType::Number(_, _) => NumDecimal(true),
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
            OracleType::Timestamp(7) | OracleType::Timestamp(8) | OracleType::Timestamp(9) => {
                TimestampNano(true)
            }
            OracleType::Timestamp(_) => Timestamp(true),
            OracleType::TimestampTZ(7)
            | OracleType::TimestampTZ(8)
            | OracleType::TimestampTZ(9) => TimestampTzNano(true),
            OracleType::TimestampTZ(_) => TimestampTz(true),
            _ => unimplemented!("{}", format!("Type {:?} not implemented for oracle!", ty)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OracleTypeSystem;
    use oracle::sql_type::OracleType;

    #[test]
    fn maps_oracle_types() {
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::Number(0, 0)),
            OracleTypeSystem::NumDecimal(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::Number(10, 0)),
            OracleTypeSystem::NumInt(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::Number(10, 2)),
            OracleTypeSystem::NumDecimal(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::Float(10)),
            OracleTypeSystem::Float(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::BinaryFloat),
            OracleTypeSystem::BinaryFloat(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::BinaryDouble),
            OracleTypeSystem::BinaryDouble(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::BLOB),
            OracleTypeSystem::Blob(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::CLOB),
            OracleTypeSystem::Clob(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::Char(10)),
            OracleTypeSystem::Char(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::Long),
            OracleTypeSystem::Char(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::NChar(10)),
            OracleTypeSystem::NChar(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::Varchar2(10)),
            OracleTypeSystem::VarChar(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::NVarchar2(10)),
            OracleTypeSystem::NVarChar(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::Date),
            OracleTypeSystem::Date(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::Timestamp(6)),
            OracleTypeSystem::Timestamp(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::Timestamp(7)),
            OracleTypeSystem::TimestampNano(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::TimestampTZ(6)),
            OracleTypeSystem::TimestampTz(true)
        ));
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::TimestampTZ(9)),
            OracleTypeSystem::TimestampTzNano(true)
        ));
    }
}
