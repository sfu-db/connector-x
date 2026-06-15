use crate::impl_typesystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy)]
pub struct DateTimeWrapperMicro(pub DateTime<Utc>);

#[derive(Debug, Clone, Copy)]
pub struct NaiveTimeWrapperMicro(pub NaiveTime);

#[derive(Debug, Clone, Copy)]
pub struct NaiveDateTimeWrapperMicro(pub NaiveDateTime);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArrowTypeSystem {
    Int16(bool),
    Int32(bool),
    Int64(bool),
    UInt16(bool),
    UInt32(bool),
    UInt64(bool),
    Float32(bool),
    Float64(bool),
    Decimal(bool),
    Boolean(bool),
    LargeUtf8(bool),
    LargeBinary(bool),
    Date32(bool),
    Date64(bool),
    Date64Micro(bool),
    Time64(bool),
    Time64Micro(bool),
    DateTimeTz(bool),
    DateTimeTzMicro(bool),
    BoolArray(bool),
    Utf8Array(bool),
    Int16Array(bool),
    Int32Array(bool),
    Int64Array(bool),
    UInt16Array(bool),
    UInt32Array(bool),
    UInt64Array(bool),
    Float32Array(bool),
    Float64Array(bool),
    DecimalArray(bool),
}

impl_typesystem! {
    system = ArrowTypeSystem,
    mappings = {
        { Int16           => i16                       }
        { Int32           => i32                       }
        { Int64           => i64                       }
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
