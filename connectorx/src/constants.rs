#[cfg(any(feature = "dst_arrow", feature = "dst_arrow2"))]
pub(crate) const SECONDS_IN_DAY: i64 = 86_400;

#[cfg(any(feature = "dst_arrow", feature = "dst_arrow2"))]
pub const RECORD_BATCH_SIZE: usize = 64000;

#[cfg(any(
    feature = "src_postgres",
    feature = "dst_mysql",
    feature = "dst_oracle",
    feature = "dst_mssql"
))]
pub const DB_BUFFER_SIZE: usize = 32;
