#[cfg(feature = "dst_arrow")]
use arrow::datatypes::DataType as ArrowDataType;

#[cfg(feature = "dst_arrow")]
pub const DEFAULT_ARROW_DECIMAL_PRECISION: u8 = 38;

#[cfg(feature = "dst_arrow")]
pub const DEFAULT_ARROW_DECIMAL_SCALE: i8 = 10;

#[cfg(feature = "dst_arrow")]
pub const DEFAULT_ARROW_DECIMAL: ArrowDataType =
    ArrowDataType::Decimal128(DEFAULT_ARROW_DECIMAL_PRECISION, DEFAULT_ARROW_DECIMAL_SCALE);

#[cfg(feature = "dst_arrow")]
pub(crate) const SECONDS_IN_DAY: i64 = 86_400;

#[allow(dead_code)]
const KILO: usize = 1 << 10;

#[cfg(feature = "dst_arrow")]
pub const RECORD_BATCH_SIZE: usize = 64 * KILO;

#[cfg(any(
    feature = "src_postgres",
    feature = "src_mysql",
    feature = "src_oracle",
    feature = "src_mssql"
))]
pub const DB_BUFFER_SIZE: usize = 32;

#[cfg(any(feature = "src_oracle"))]
pub const ORACLE_ARRAY_SIZE: u32 = KILO as u32;

#[cfg(all(not(debug_assertions), feature = "federation"))]
pub const J4RS_BASE_PATH: &str = "../target/release";

#[cfg(all(debug_assertions, feature = "federation"))]
pub const J4RS_BASE_PATH: &str = "../target/debug";

#[cfg(feature = "federation")]
pub const CX_REWRITER_PATH: &str =
    "../connectorx-python/connectorx/dependencies/federated-rewriter.jar";

#[cfg(feature = "federation")]
pub const POSTGRES_JDBC_DRIVER: &str = "org.postgresql.Driver";

#[cfg(feature = "federation")]
pub const MYSQL_JDBC_DRIVER: &str = "com.mysql.cj.jdbc.Driver";

#[cfg(feature = "federation")]
pub const DUCKDB_JDBC_DRIVER: &str = "org.duckdb.DuckDBDriver";

pub const CONNECTORX_PROTOCOL: &str = "cxprotocol";
