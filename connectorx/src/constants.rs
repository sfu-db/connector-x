#[cfg(feature = "dst_arrow")]
use arrow::datatypes::DataType as ArrowDataType;

/// Precision used for Arrow `Decimal128` columns when the source does not give one.
#[cfg(feature = "dst_arrow")]
pub const DEFAULT_ARROW_DECIMAL_PRECISION: u8 = 38;

/// Scale used for Arrow `Decimal128` columns when the source does not give one.
#[cfg(feature = "dst_arrow")]
pub const DEFAULT_ARROW_DECIMAL_SCALE: i8 = 10;

/// Default Arrow decimal type, `Decimal128(38, 10)`.
#[cfg(feature = "dst_arrow")]
pub const DEFAULT_ARROW_DECIMAL: ArrowDataType =
    ArrowDataType::Decimal128(DEFAULT_ARROW_DECIMAL_PRECISION, DEFAULT_ARROW_DECIMAL_SCALE);

#[cfg(feature = "dst_arrow")]
pub(crate) const SECONDS_IN_DAY: i64 = 86_400;

#[allow(dead_code)]
const KILO: usize = 1 << 10;

/// Default number of rows per Arrow record batch.
#[cfg(feature = "dst_arrow")]
pub const RECORD_BATCH_SIZE: usize = 64 * KILO;

/// Number of rows buffered per fetch by the row-based database sources.
#[cfg(any(
    feature = "src_postgres",
    feature = "src_mysql",
    feature = "src_oracle",
    feature = "src_mssql"
))]
pub const DB_BUFFER_SIZE: usize = 32;

/// Number of rows Oracle prefetches per round trip.
#[cfg(feature = "src_oracle")]
pub const ORACLE_ARRAY_SIZE: u32 = KILO as u32;

/// Directory holding the j4rs jars used by the federated query rewriter.
#[cfg(all(not(debug_assertions), feature = "federation"))]
pub const J4RS_BASE_PATH: &str = "../target/release";

/// Directory holding the j4rs jars used by the federated query rewriter.
#[cfg(all(debug_assertions, feature = "federation"))]
pub const J4RS_BASE_PATH: &str = "../target/debug";

/// Default path of the federated query rewriter jar.
#[cfg(feature = "federation")]
pub const CX_REWRITER_PATH: &str =
    "../connectorx-python/connectorx/dependencies/federated-rewriter.jar";

/// JDBC driver class for Postgres, used by the federated rewriter.
#[cfg(feature = "federation")]
pub const POSTGRES_JDBC_DRIVER: &str = "org.postgresql.Driver";

/// JDBC driver class for MySQL, used by the federated rewriter.
#[cfg(feature = "federation")]
pub const MYSQL_JDBC_DRIVER: &str = "com.mysql.cj.jdbc.Driver";

/// JDBC driver class for DuckDB, used by the federated rewriter.
#[cfg(feature = "federation")]
pub const DUCKDB_JDBC_DRIVER: &str = "org.duckdb.DuckDBDriver";

/// Connection string query parameter that selects the ConnectorX protocol (e.g. `binary`, `csv`).
pub const CONNECTORX_PROTOCOL: &str = "cxprotocol";
