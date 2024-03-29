//! This module contains transport definitions for the sources and destinations implemented in ConnectorX.

#[cfg(all(feature = "src_bigquery", feature = "dst_arrow"))]
mod bigquery_arrow;
#[cfg(all(feature = "src_bigquery", feature = "dst_arrow2"))]
mod bigquery_arrow2;
#[cfg(all(feature = "src_bigquery", feature = "dst_arrow"))]
mod bigquery_arrowstream;
#[cfg(all(feature = "src_csv", feature = "dst_arrow"))]
mod csv_arrow;
#[cfg(all(feature = "src_dummy", feature = "dst_arrow"))]
mod dummy_arrow;
#[cfg(all(feature = "src_dummy", feature = "dst_arrow2"))]
mod dummy_arrow2;
#[cfg(all(feature = "src_dummy", feature = "dst_arrow"))]
mod dummy_arrowstream;
#[cfg(all(feature = "src_mssql", feature = "dst_arrow"))]
mod mssql_arrow;
#[cfg(all(feature = "src_mssql", feature = "dst_arrow2"))]
mod mssql_arrow2;
#[cfg(all(feature = "src_mssql", feature = "dst_arrow"))]
mod mssql_arrowstream;
#[cfg(all(feature = "src_mysql", feature = "dst_arrow"))]
mod mysql_arrow;
#[cfg(all(feature = "src_mysql", feature = "dst_arrow2"))]
mod mysql_arrow2;
#[cfg(all(feature = "src_mysql", feature = "dst_arrow"))]
mod mysql_arrowstream;
#[cfg(all(feature = "src_oracle", feature = "dst_arrow"))]
mod oracle_arrow;
#[cfg(all(feature = "src_oracle", feature = "dst_arrow2"))]
mod oracle_arrow2;
#[cfg(all(feature = "src_oracle", feature = "dst_arrow"))]
mod oracle_arrowstream;
#[cfg(all(feature = "src_postgres", feature = "dst_arrow"))]
mod postgres_arrow;
#[cfg(all(feature = "src_postgres", feature = "dst_arrow2"))]
mod postgres_arrow2;
#[cfg(all(feature = "src_postgres", feature = "dst_arrow"))]
mod postgres_arrowstream;
#[cfg(all(feature = "src_sqlite", feature = "dst_arrow"))]
mod sqlite_arrow;
#[cfg(all(feature = "src_sqlite", feature = "dst_arrow2"))]
mod sqlite_arrow2;
#[cfg(all(feature = "src_sqlite", feature = "dst_arrow"))]
mod sqlite_arrowstream;

#[cfg(all(feature = "src_bigquery", feature = "dst_arrow"))]
pub use bigquery_arrow::{BigQueryArrowTransport, BigQueryArrowTransportError};
#[cfg(all(feature = "src_bigquery", feature = "dst_arrow2"))]
pub use bigquery_arrow2::{BigQueryArrow2Transport, BigQueryArrow2TransportError};
#[cfg(all(feature = "src_bigquery", feature = "dst_arrow"))]
pub use bigquery_arrowstream::{
    BigQueryArrowTransport as BigQueryArrowStreamTransport,
    BigQueryArrowTransportError as BigQueryArrowStreamTransportError,
};
#[cfg(all(feature = "src_csv", feature = "dst_arrow"))]
pub use csv_arrow::CSVArrowTransport;
#[cfg(all(feature = "src_dummy", feature = "dst_arrow"))]
pub use dummy_arrow::DummyArrowTransport;
#[cfg(all(feature = "src_dummy", feature = "dst_arrow2"))]
pub use dummy_arrow2::DummyArrow2Transport;
#[cfg(all(feature = "src_mssql", feature = "dst_arrow"))]
pub use mssql_arrow::{MsSQLArrowTransport, MsSQLArrowTransportError};
#[cfg(all(feature = "src_mssql", feature = "dst_arrow2"))]
pub use mssql_arrow2::{MsSQLArrow2Transport, MsSQLArrow2TransportError};
#[cfg(all(feature = "src_mssql", feature = "dst_arrow"))]
pub use mssql_arrowstream::{
    MsSQLArrowTransport as MsSQLArrowStreamTransport,
    MsSQLArrowTransportError as MsSQLArrowStreamTransportError,
};
#[cfg(all(feature = "src_mysql", feature = "dst_arrow"))]
pub use mysql_arrow::{MySQLArrowTransport, MySQLArrowTransportError};
#[cfg(all(feature = "src_mysql", feature = "dst_arrow2"))]
pub use mysql_arrow2::{MySQLArrow2Transport, MySQLArrow2TransportError};
#[cfg(all(feature = "src_mysql", feature = "dst_arrow"))]
pub use mysql_arrowstream::{
    MySQLArrowTransport as MySQLArrowStreamTransport,
    MySQLArrowTransportError as MySQLArrowStreamTransportError,
};
#[cfg(all(feature = "src_oracle", feature = "dst_arrow"))]
pub use oracle_arrow::{OracleArrowTransport, OracleArrowTransportError};
#[cfg(all(feature = "src_oracle", feature = "dst_arrow2"))]
pub use oracle_arrow2::{OracleArrow2Transport, OracleArrow2TransportError};
#[cfg(all(feature = "src_oracle", feature = "dst_arrow"))]
pub use oracle_arrowstream::{
    OracleArrowTransport as OracleArrowStreamTransport,
    OracleArrowTransportError as OracleArrowStreamTransportError,
};
#[cfg(all(feature = "src_postgres", feature = "dst_arrow"))]
pub use postgres_arrow::{PostgresArrowTransport, PostgresArrowTransportError};
#[cfg(all(feature = "src_postgres", feature = "dst_arrow2"))]
pub use postgres_arrow2::{PostgresArrow2Transport, PostgresArrow2TransportError};
#[cfg(all(feature = "src_postgres", feature = "dst_arrow"))]
pub use postgres_arrowstream::{
    PostgresArrowTransport as PostgresArrowStreamTransport,
    PostgresArrowTransportError as PostgresArrowStreamTransportError,
};
#[cfg(all(feature = "src_sqlite", feature = "dst_arrow"))]
pub use sqlite_arrow::{SQLiteArrowTransport, SQLiteArrowTransportError};
#[cfg(all(feature = "src_sqlite", feature = "dst_arrow2"))]
pub use sqlite_arrow2::{SQLiteArrow2Transport, SQLiteArrow2TransportError};
#[cfg(all(feature = "src_sqlite", feature = "dst_arrow"))]
pub use sqlite_arrowstream::{
    SQLiteArrowTransport as SQLiteArrowStreamTransport,
    SQLiteArrowTransportError as SQLiteArrowStreamTransportError,
};
