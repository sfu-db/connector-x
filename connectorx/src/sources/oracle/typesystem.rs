use chrono::{DateTime, NaiveDateTime, Utc};
use r2d2_oracle::oracle::sql_type::OracleType;
use rust_decimal::Decimal;

/// Column types of the Oracle source. The `bool` is `true` when the column is nullable.
#[derive(Copy, Clone, Debug)]
pub enum OracleTypeSystem {
    /// Oracle `NUMBER(p, 0)` with a declared precision, read as `i64`.
    NumInt(bool),
    /// Oracle `NUMBER` with a nonzero scale or no declared precision, read as `Decimal`.
    NumDecimal(bool),
    /// Oracle `FLOAT`, read as `f64`.
    Float(bool),
    /// Floating-point Oracle `NUMBER`, read as `f64`. Type detection does not currently produce it.
    NumFloat(bool),
    /// Oracle `BINARY_FLOAT`, read as `f64`.
    BinaryFloat(bool),
    /// Oracle `BINARY_DOUBLE`, read as `f64`.
    BinaryDouble(bool),
    /// Oracle `BLOB`, read as `Vec<u8>`.
    Blob(bool),
    /// Oracle `CLOB`, read as `String`.
    Clob(bool),
    /// Oracle `VARCHAR2`, read as `String`.
    VarChar(bool),
    /// Oracle `CHAR` or `LONG`, read as `String`.
    Char(bool),
    /// Oracle `NVARCHAR2`, read as `String`.
    NVarChar(bool),
    /// Oracle `NCHAR`, read as `String`.
    NChar(bool),
    /// Oracle `DATE`, read as `NaiveDateTime`.
    Date(bool),
    /// Oracle `TIMESTAMP` with up to 6 fractional digits, read as `NaiveDateTime`.
    Timestamp(bool),
    /// Oracle `TIMESTAMP WITH TIME ZONE` with up to 6 fractional digits, read as `DateTime<Utc>`.
    TimestampTz(bool),
    /// Oracle `TIMESTAMP(7)` to `TIMESTAMP(9)`, read as `NaiveDateTime` with nanosecond precision.
    TimestampNano(bool),
    /// Oracle `TIMESTAMP(7..9) WITH TIME ZONE`, read as `DateTime<Utc>` with nanosecond precision.
    TimestampTzNano(bool),
    /// Oracle `BOOLEAN`, read as `bool`.
    Boolean(bool),
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
        { Boolean => bool }
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
            OracleType::Boolean => Boolean(true),
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
        assert!(matches!(
            OracleTypeSystem::from(&OracleType::Boolean),
            OracleTypeSystem::Boolean(true)
        ));
    }
}
