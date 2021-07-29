use crate::impl_typesystem;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArrowTypeSystem {
    Int32(bool),
    Int64(bool),
    UInt32(bool),
    UInt64(bool),
    Float32(bool),
    Float64(bool),
    Boolean(bool),
    LargeUtf8(bool),
    LargeBinary(bool),
    Date32(bool),
    Date64(bool),
    Time64(bool),
    DateTimeTz(bool),
}

impl_typesystem! {
    system = ArrowTypeSystem,
    mappings = {
        { Int32      => i32           }
        { Int64      => i64           }
        { UInt32     => u32           }
        { UInt64     => u64           }
        { Float64    => f64           }
        { Float32    => f32           }
        { Boolean    => bool          }
        { LargeUtf8  => String        }
        { LargeBinary => Vec<u8>      }
        { Date32     => NaiveDate     }
        { Date64     => NaiveDateTime }
        { Time64     => NaiveTime     }
        { DateTimeTz => DateTime<Utc> }
    }
}
