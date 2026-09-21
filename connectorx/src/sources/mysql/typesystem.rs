use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use r2d2_mysql::mysql::consts::{ColumnFlags, ColumnType};
use rust_decimal::Decimal;
use serde_json::Value;

#[derive(Copy, Clone, Debug)]
pub enum MySQLTypeSystem {
    Float(bool),
    Double(bool),
    Tiny(bool),
    Short(bool),
    Long(bool),
    Int24(bool),
    LongLong(bool),
    UTiny(bool),
    UShort(bool),
    ULong(bool),
    UInt24(bool),
    ULongLong(bool),
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
    Json(bool),
    Bit(bool),
}

impl_typesystem! {
    system = MySQLTypeSystem,
    mappings = {
        { Tiny => i8 }
        { Short | Year => i16 }
        { Long | Int24 => i32}
        { LongLong => i64 }
        { Float => f32 }
        { Double => f64 }
        { UTiny => u8 }
        { UShort => u16 }
        { ULong | UInt24 => u32}
        { ULongLong => u64 }
        { Datetime | Timestamp => NaiveDateTime }
        { Date => NaiveDate }
        { Time => NaiveTime }
        { Decimal => Decimal }
        { Char | VarChar | Enum => String }
        { TinyBlob | Blob | MediumBlob | LongBlob | Bit => Vec<u8>}
        { Json => Value }
    }
}

impl<'a> From<(&'a ColumnType, &'a ColumnFlags, u16)> for MySQLTypeSystem {
    fn from(col: (&'a ColumnType, &'a ColumnFlags, u16)) -> MySQLTypeSystem {
        use MySQLTypeSystem::*;
        let (ty, flag, charset) = col;
        let null_ok = !flag.contains(ColumnFlags::NOT_NULL_FLAG);
        let unsigned = flag.contains(ColumnFlags::UNSIGNED_FLAG);

        // The "binary" pseudo-charset (collation id 63) is the only reliable signal
        // that a string/blob column holds raw bytes rather than text. BINARY_FLAG is
        // also set for any *_bin collation (e.g. ascii_bin, utf8mb4_bin), which are
        // still TEXT columns, so it must not be used to decide binary-ness.
        const MYSQL_BINARY_CHARSET: u16 = 63;
        let is_binary = charset == MYSQL_BINARY_CHARSET;

        match ty {
            ColumnType::MYSQL_TYPE_TINY => {
                if unsigned {
                    UTiny(null_ok)
                } else {
                    Tiny(null_ok)
                }
            }

            ColumnType::MYSQL_TYPE_SHORT => {
                if unsigned {
                    UShort(null_ok)
                } else {
                    Short(null_ok)
                }
            }
            ColumnType::MYSQL_TYPE_INT24 => {
                if unsigned {
                    UInt24(null_ok)
                } else {
                    Int24(null_ok)
                }
            }
            ColumnType::MYSQL_TYPE_LONG => {
                if unsigned {
                    ULong(null_ok)
                } else {
                    Long(null_ok)
                }
            }
            ColumnType::MYSQL_TYPE_LONGLONG => {
                if unsigned {
                    ULongLong(null_ok)
                } else {
                    LongLong(null_ok)
                }
            }
            ColumnType::MYSQL_TYPE_FLOAT => Float(null_ok),
            ColumnType::MYSQL_TYPE_DOUBLE => Double(null_ok),
            ColumnType::MYSQL_TYPE_DATETIME => Datetime(null_ok),
            ColumnType::MYSQL_TYPE_DATE => Date(null_ok),
            ColumnType::MYSQL_TYPE_TIME => Time(null_ok),
            ColumnType::MYSQL_TYPE_DECIMAL => Decimal(null_ok),
            ColumnType::MYSQL_TYPE_NEWDECIMAL => Decimal(null_ok),

            // CHAR/BINARY, VARCHAR/VARBINARY, and TEXT/BLOB share identical type codes, with the actual type determined by the charset.
            ColumnType::MYSQL_TYPE_STRING => {
                if is_binary {
                    TinyBlob(null_ok)
                } else {
                    Char(null_ok)
                }
            }
            ColumnType::MYSQL_TYPE_VAR_STRING => {
                if is_binary {
                    Blob(null_ok)
                } else {
                    VarChar(null_ok)
                }
            }

            ColumnType::MYSQL_TYPE_TIMESTAMP => Timestamp(null_ok),
            ColumnType::MYSQL_TYPE_YEAR => Year(null_ok),
            ColumnType::MYSQL_TYPE_ENUM => Enum(null_ok),

            ColumnType::MYSQL_TYPE_TINY_BLOB => {
                if is_binary {
                    TinyBlob(null_ok)
                } else {
                    VarChar(null_ok)
                }
            } // TINYTEXT
            ColumnType::MYSQL_TYPE_BLOB => {
                if is_binary {
                    Blob(null_ok)
                } else {
                    VarChar(null_ok)
                }
            } // TEXT
            ColumnType::MYSQL_TYPE_MEDIUM_BLOB => {
                if is_binary {
                    MediumBlob(null_ok)
                } else {
                    VarChar(null_ok)
                }
            } // MEDIUMTEXT
            ColumnType::MYSQL_TYPE_LONG_BLOB => {
                if is_binary {
                    LongBlob(null_ok)
                } else {
                    VarChar(null_ok)
                }
            } // LONGTEXT

            ColumnType::MYSQL_TYPE_JSON => Json(null_ok),
            ColumnType::MYSQL_TYPE_VARCHAR => VarChar(null_ok),
            ColumnType::MYSQL_TYPE_BIT => Bit(null_ok),
            _ => unimplemented!("{}", format!("{:?}", ty)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MySQLTypeSystem;
    use mysql_common::constants::{ColumnFlags, ColumnType};

    fn map(ty: ColumnType, flags: ColumnFlags, charset: u16) -> MySQLTypeSystem {
        MySQLTypeSystem::from((&ty, &flags, charset))
    }

    #[test]
    fn maps_numeric_and_temporal_columns() {
        let nullable = ColumnFlags::empty();
        let not_null = ColumnFlags::NOT_NULL_FLAG;
        let unsigned = ColumnFlags::UNSIGNED_FLAG | ColumnFlags::NOT_NULL_FLAG;

        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_TINY, nullable, 0),
            MySQLTypeSystem::Tiny(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_TINY, unsigned, 0),
            MySQLTypeSystem::UTiny(false)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_SHORT, unsigned, 0),
            MySQLTypeSystem::UShort(false)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_SHORT, nullable, 0),
            MySQLTypeSystem::Short(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_INT24, unsigned, 0),
            MySQLTypeSystem::UInt24(false)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_INT24, nullable, 0),
            MySQLTypeSystem::Int24(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_LONG, unsigned, 0),
            MySQLTypeSystem::ULong(false)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_LONG, nullable, 0),
            MySQLTypeSystem::Long(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_LONGLONG, unsigned, 0),
            MySQLTypeSystem::ULongLong(false)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_LONGLONG, nullable, 0),
            MySQLTypeSystem::LongLong(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_FLOAT, nullable, 0),
            MySQLTypeSystem::Float(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_DOUBLE, nullable, 0),
            MySQLTypeSystem::Double(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_DATETIME, nullable, 0),
            MySQLTypeSystem::Datetime(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_DATE, nullable, 0),
            MySQLTypeSystem::Date(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_TIME, nullable, 0),
            MySQLTypeSystem::Time(true)
        ));
        for ty in [
            ColumnType::MYSQL_TYPE_DECIMAL,
            ColumnType::MYSQL_TYPE_NEWDECIMAL,
        ] {
            assert!(matches!(
                map(ty, nullable, 0),
                MySQLTypeSystem::Decimal(true)
            ));
        }
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_TIMESTAMP, not_null, 0),
            MySQLTypeSystem::Timestamp(false)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_YEAR, nullable, 0),
            MySQLTypeSystem::Year(true)
        ));
    }

    #[test]
    fn maps_text_binary_and_special_columns() {
        let nullable = ColumnFlags::empty();
        for (ty, expected) in [
            (ColumnType::MYSQL_TYPE_STRING, MySQLTypeSystem::Char(true)),
            (
                ColumnType::MYSQL_TYPE_VAR_STRING,
                MySQLTypeSystem::VarChar(true),
            ),
            (
                ColumnType::MYSQL_TYPE_TINY_BLOB,
                MySQLTypeSystem::VarChar(true),
            ),
            (ColumnType::MYSQL_TYPE_BLOB, MySQLTypeSystem::VarChar(true)),
            (
                ColumnType::MYSQL_TYPE_MEDIUM_BLOB,
                MySQLTypeSystem::VarChar(true),
            ),
            (
                ColumnType::MYSQL_TYPE_LONG_BLOB,
                MySQLTypeSystem::VarChar(true),
            ),
        ] {
            let actual = map(ty, nullable, 0);
            assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
        }
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_STRING, nullable, 63),
            MySQLTypeSystem::TinyBlob(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_VAR_STRING, nullable, 63),
            MySQLTypeSystem::Blob(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_TINY_BLOB, nullable, 63),
            MySQLTypeSystem::TinyBlob(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_BLOB, nullable, 63),
            MySQLTypeSystem::Blob(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_MEDIUM_BLOB, nullable, 63),
            MySQLTypeSystem::MediumBlob(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_LONG_BLOB, nullable, 63),
            MySQLTypeSystem::LongBlob(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_JSON, nullable, 0),
            MySQLTypeSystem::Json(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_VARCHAR, nullable, 0),
            MySQLTypeSystem::VarChar(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_ENUM, nullable, 0),
            MySQLTypeSystem::Enum(true)
        ));
        assert!(matches!(
            map(ColumnType::MYSQL_TYPE_BIT, nullable, 0),
            MySQLTypeSystem::Bit(true)
        ));
    }
}
