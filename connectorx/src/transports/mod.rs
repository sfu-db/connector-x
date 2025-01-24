//! This module contains transport definitions for the sources and destinations implemented in ConnectorX.

#[cfg(all(feature = "src_bigquery", feature = "dst_arrow"))]
mod bigquery_arrow;
#[cfg(all(feature = "src_bigquery", feature = "dst_arrow"))]
mod bigquery_arrowstream;
#[cfg(all(feature = "src_csv", feature = "dst_arrow"))]
mod csv_arrow;
#[cfg(all(feature = "src_dummy", feature = "dst_arrow"))]
mod dummy_arrow;
#[cfg(all(feature = "src_dummy", feature = "dst_arrow"))]
mod dummy_arrowstream;
#[cfg(all(feature = "src_mssql", feature = "dst_arrow"))]
mod mssql_arrow;
#[cfg(all(feature = "src_mssql", feature = "dst_arrow"))]
mod mssql_arrowstream;
#[cfg(all(feature = "src_mysql", feature = "dst_arrow"))]
mod mysql_arrow;
#[cfg(all(feature = "src_mysql", feature = "dst_arrow"))]
mod mysql_arrowstream;
#[cfg(all(feature = "src_oracle", feature = "dst_arrow"))]
mod oracle_arrow;
#[cfg(all(feature = "src_oracle", feature = "dst_arrow"))]
mod oracle_arrowstream;
#[cfg(all(feature = "src_postgres", feature = "dst_arrow"))]
mod postgres_arrow;
#[cfg(all(feature = "src_postgres", feature = "dst_arrow"))]
mod postgres_arrowstream;
#[cfg(all(feature = "src_sqlite", feature = "dst_arrow"))]
mod sqlite_arrow;
#[cfg(all(feature = "src_sqlite", feature = "dst_arrow"))]
mod sqlite_arrowstream;
#[cfg(all(feature = "src_trino", feature = "dst_arrow"))]
mod trino_arrow;
#[cfg(all(feature = "src_trino", feature = "dst_arrow"))]
mod trino_arrowstream;
#[cfg(all(feature = "src_bigquery", feature = "dst_arrow"))]
pub use bigquery_arrow::{BigQueryArrowTransport, BigQueryArrowTransportError};
#[cfg(all(feature = "src_bigquery", feature = "dst_arrow"))]
pub use bigquery_arrowstream::{
    BigQueryArrowTransport as BigQueryArrowStreamTransport,
    BigQueryArrowTransportError as BigQueryArrowStreamTransportError,
};
#[cfg(all(feature = "src_csv", feature = "dst_arrow"))]
pub use csv_arrow::CSVArrowTransport;
#[cfg(all(feature = "src_dummy", feature = "dst_arrow"))]
pub use dummy_arrow::DummyArrowTransport;
#[cfg(all(feature = "src_mssql", feature = "dst_arrow"))]
pub use mssql_arrow::{MsSQLArrowTransport, MsSQLArrowTransportError};
#[cfg(all(feature = "src_mssql", feature = "dst_arrow"))]
pub use mssql_arrowstream::{
    MsSQLArrowTransport as MsSQLArrowStreamTransport,
    MsSQLArrowTransportError as MsSQLArrowStreamTransportError,
};
#[cfg(all(feature = "src_mysql", feature = "dst_arrow"))]
pub use mysql_arrow::{MySQLArrowTransport, MySQLArrowTransportError};
#[cfg(all(feature = "src_mysql", feature = "dst_arrow"))]
pub use mysql_arrowstream::{
    MySQLArrowTransport as MySQLArrowStreamTransport,
    MySQLArrowTransportError as MySQLArrowStreamTransportError,
};
#[cfg(all(feature = "src_oracle", feature = "dst_arrow"))]
pub use oracle_arrow::{OracleArrowTransport, OracleArrowTransportError};
#[cfg(all(feature = "src_oracle", feature = "dst_arrow"))]
pub use oracle_arrowstream::{
    OracleArrowTransport as OracleArrowStreamTransport,
    OracleArrowTransportError as OracleArrowStreamTransportError,
};
#[cfg(all(feature = "src_postgres", feature = "dst_arrow"))]
pub use postgres_arrow::{PostgresArrowTransport, PostgresArrowTransportError};
#[cfg(all(feature = "src_postgres", feature = "dst_arrow"))]
pub use postgres_arrowstream::{
    PostgresArrowTransport as PostgresArrowStreamTransport,
    PostgresArrowTransportError as PostgresArrowStreamTransportError,
};
#[cfg(all(feature = "src_sqlite", feature = "dst_arrow"))]
pub use sqlite_arrow::{SQLiteArrowTransport, SQLiteArrowTransportError};
#[cfg(all(feature = "src_sqlite", feature = "dst_arrow"))]
pub use sqlite_arrowstream::{
    SQLiteArrowTransport as SQLiteArrowStreamTransport,
    SQLiteArrowTransportError as SQLiteArrowStreamTransportError,
};
#[cfg(all(feature = "src_trino", feature = "dst_arrow"))]
pub use trino_arrow::{TrinoArrowTransport, TrinoArrowTransportError};
#[cfg(all(feature = "src_trino", feature = "dst_arrow"))]
pub use trino_arrowstream::{
    TrinoArrowTransport as TrinoArrowStreamTransport,
    TrinoArrowTransportError as TrinoArrowStreamTransportError,
};
