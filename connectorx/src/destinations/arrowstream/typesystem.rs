use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use rust_decimal::Decimal;

/// UTC timestamp that is written with microsecond precision.
#[derive(Debug, Clone, Copy)]
pub struct DateTimeWrapperMicro(pub DateTime<Utc>);

/// Time of day that is written with microsecond precision.
#[derive(Debug, Clone, Copy)]
pub struct NaiveTimeWrapperMicro(pub NaiveTime);

/// Date and time without time zone that is written with microsecond precision.
#[derive(Debug, Clone, Copy)]
pub struct NaiveDateTimeWrapperMicro(pub NaiveDateTime);

/// Column types of the Arrow destination. The `bool` is `true` when the column is nullable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArrowTypeSystem {
    /// Arrow `Int8`.
    Int8(bool),
    /// Arrow `Int16`.
    Int16(bool),
    /// Arrow `Int32`.
    Int32(bool),
    /// Arrow `Int64`.
    Int64(bool),
    /// Arrow `UInt8`.
    UInt8(bool),
    /// Arrow `UInt16`.
    UInt16(bool),
    /// Arrow `UInt32`.
    UInt32(bool),
    /// Arrow `UInt64`.
    UInt64(bool),
    /// Arrow `Float32`.
    Float32(bool),
    /// Arrow `Float64`.
    Float64(bool),
    /// Arrow `Decimal128(38, 10)`.
    Decimal(bool),
    /// Arrow `Boolean`.
    Boolean(bool),
    /// String, written as Arrow `Utf8`.
    LargeUtf8(bool),
    /// Arrow `LargeBinary`.
    LargeBinary(bool),
    /// Arrow `Date32` (days since the epoch).
    Date32(bool),
    /// Date and time without time zone, written as Arrow `Timestamp(Nanosecond, None)`.
    Date64(bool),
    /// Date and time without time zone, written as Arrow `Timestamp(Microsecond, None)`.
    Date64Micro(bool),
    /// Arrow `Time64(Nanosecond)`.
    Time64(bool),
    /// Arrow `Time64(Microsecond)`.
    Time64Micro(bool),
    /// UTC timestamp, written as Arrow `Timestamp(Nanosecond, None)`.
    DateTimeTz(bool),
    /// UTC timestamp, written as Arrow `Timestamp(Microsecond, "+00:00")`.
    DateTimeTzMicro(bool),
    /// Arrow `LargeList<Boolean>`.
    BoolArray(bool),
    /// Arrow `LargeList<Utf8>`.
    Utf8Array(bool),
    /// Arrow `LargeList<Int16>`.
    Int16Array(bool),
    /// Arrow `LargeList<Int32>`.
    Int32Array(bool),
    /// Arrow `LargeList<Int64>`.
    Int64Array(bool),
    /// Arrow `LargeList<UInt16>`.
    UInt16Array(bool),
    /// Arrow `LargeList<UInt32>`.
    UInt32Array(bool),
    /// Arrow `LargeList<UInt64>`.
    UInt64Array(bool),
    /// Arrow `LargeList<Float32>`.
    Float32Array(bool),
    /// Arrow `LargeList<Float64>`.
    Float64Array(bool),
    /// Arrow `LargeList<Decimal128(38, 10)>`.
    DecimalArray(bool),
}

impl_typesystem! {
    system = ArrowTypeSystem,
    mappings = {
        { Int8            => i8                        }
        { Int16           => i16                       }
        { Int32           => i32                       }
        { Int64           => i64                       }
        { UInt8           => u8                        }
        { UInt16          => u16                       }
        { UInt32          => u32                       }
        { UInt64          => u64                       }
        { Float32         => f32                       }
        { Float64         => f64                       }
        { Decimal         => Decimal                   }
        { Boolean         => bool                      }
        { LargeUtf8       => String                    }
        { LargeBinary     => Vec<u8>                   }
        { Date32          => NaiveDate                 }
        { Date64          => NaiveDateTime             }
        { Date64Micro     => NaiveDateTimeWrapperMicro }
        { Time64          => NaiveTime                 }
        { Time64Micro     => NaiveTimeWrapperMicro     }
        { DateTimeTz      => DateTime<Utc>             }
        { DateTimeTzMicro => DateTimeWrapperMicro      }
        { BoolArray       => Vec<Option<bool>>         }
        { Utf8Array       => Vec<Option<String>>       }
        { Int16Array      => Vec<Option<i16>>          }
        { Int32Array      => Vec<Option<i32>>          }
        { Int64Array      => Vec<Option<i64>>          }
        { UInt16Array     => Vec<Option<u16>>          }
        { UInt32Array     => Vec<Option<u32>>          }
        { UInt64Array     => Vec<Option<u64>>          }
        { Float32Array    => Vec<Option<f32>>          }
        { Float64Array    => Vec<Option<f64>>          }
        { DecimalArray    => Vec<Option<Decimal>>      }
    }
}
