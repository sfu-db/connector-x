#[cfg(any(feature = "dst_arrow", feature = "dst_arrow2"))]
pub(crate) const SECONDS_IN_DAY: i64 = 86_400;

#[allow(dead_code)]
const KILO: usize = 1 << 10;

#[cfg(any(feature = "dst_arrow", feature = "dst_arrow2"))]
pub const RECORD_BATCH_SIZE: usize = 64 * KILO;

#[cfg(any(
    feature = "src_postgres",
    feature = "src_mysql",
    feature = "src_oracle",
    feature = "src_mssql"
))]
pub const DB_BUFFER_SIZE: usize = 32;

#[cfg(any(feature = "src_oracle"))]
pub const ORACLE_ARRAY_SIZE: u32 = (1 * KILO) as u32;
