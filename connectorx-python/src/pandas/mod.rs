mod destination;
mod dispatcher;
pub mod get_meta;
mod pandas_columns;
mod pystring;
mod transports;
mod typesystem;

pub use self::destination::{PandasBlockInfo, PandasDestination, PandasPartitionDestination};
use self::dispatcher::PandasDispatcher;
pub use self::transports::{
    BigQueryPandasTransport, MsSQLPandasTransport, MysqlPandasTransport, OraclePandasTransport,
    PostgresPandasTransport, SqlitePandasTransport, TrinoPandasTransport,
};
pub use self::typesystem::{PandasDType, PandasTypeSystem};
use crate::errors::ConnectorXPythonError;
use connectorx::source_router::{SourceConn, SourceType};
use connectorx::sources::oracle::OracleSource;
use connectorx::{
    prelude::*,
    sources::{
        mysql::{BinaryProtocol as MySQLBinaryProtocol, TextProtocol},
        postgres::{
            rewrite_tls_args, BinaryProtocol as PgBinaryProtocol, CSVProtocol, CursorProtocol,
            SimpleProtocol,
        },
    },
    sql::CXQuery,
};
use fehler::throws;
use log::debug;
use postgres::NoTls;
use postgres_openssl::MakeTlsConnector;
use pyo3::prelude::*;
use std::sync::Arc;

#[throws(ConnectorXPythonError)]
pub fn write_pandas<'a, 'py: 'a>(
    py: Python<'py>,
    source_conn: &SourceConn,
    origin_query: Option<String>,
    queries: &[CXQuery<String>],
    pre_execution_queries: Option<&[String]>,
) -> Bound<'py, PyAny> {
    let destination = PandasDestination::new();
    let protocol = source_conn.proto.as_str();
    debug!("Protocol: {}", protocol);

    match source_conn.ty {
        SourceType::Postgres => {
            let (config, tls) = rewrite_tls_args(&source_conn.conn)?;
            match (protocol, tls) {
                ("csv", Some(tls_conn)) => {
                    let sb = PostgresSource::<CSVProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let dispatcher = PandasDispatcher::<
                        _,
                        PostgresPandasTransport<CSVProtocol, MakeTlsConnector>,
                    >::new(
                        sb, destination, queries, origin_query, pre_execution_queries
                    );
                    dispatcher.run(py)?
                }
                ("csv", None) => {
                    let sb =
                        PostgresSource::<CSVProtocol, NoTls>::new(config, NoTls, queries.len())?;
                    let dispatcher = PandasDispatcher::<
                        _,
                        PostgresPandasTransport<CSVProtocol, NoTls>,
                    >::new(
                        sb, destination, queries, origin_query, pre_execution_queries
                    );
                    dispatcher.run(py)?
                }
                ("binary", Some(tls_conn)) => {
                    let sb = PostgresSource::<PgBinaryProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let dispatcher =
                        PandasDispatcher::<
                            _,
                            PostgresPandasTransport<PgBinaryProtocol, MakeTlsConnector>,
                        >::new(sb, destination, queries, origin_query, pre_execution_queries);
                    dispatcher.run(py)?
                }
                ("binary", None) => {
                    let sb = PostgresSource::<PgBinaryProtocol, NoTls>::new(
                        config,
                        NoTls,
                        queries.len(),
                    )?;
                    let dispatcher = PandasDispatcher::<
                        _,
                        PostgresPandasTransport<PgBinaryProtocol, NoTls>,
                    >::new(
                        sb, destination, queries, origin_query, pre_execution_queries
                    );
                    dispatcher.run(py)?
                }
                ("cursor", Some(tls_conn)) => {
                    let sb = PostgresSource::<CursorProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let dispatcher =
                        PandasDispatcher::<
                            _,
                            PostgresPandasTransport<CursorProtocol, MakeTlsConnector>,
                        >::new(sb, destination, queries, origin_query, pre_execution_queries);
                    dispatcher.run(py)?
                }
                ("cursor", None) => {
                    let sb =
                        PostgresSource::<CursorProtocol, NoTls>::new(config, NoTls, queries.len())?;
                    let dispatcher = PandasDispatcher::<
                        _,
                        PostgresPandasTransport<CursorProtocol, NoTls>,
                    >::new(
                        sb, destination, queries, origin_query, pre_execution_queries
                    );
                    dispatcher.run(py)?
                }
                ("simple", Some(tls_conn)) => {
                    let sb = PostgresSource::<SimpleProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let dispatcher =
                        PandasDispatcher::<
                            _,
                            PostgresPandasTransport<SimpleProtocol, MakeTlsConnector>,
                        >::new(sb, destination, queries, origin_query, pre_execution_queries);
                    dispatcher.run(py)?
                }
                ("simple", None) => {
                    let sb =
                        PostgresSource::<SimpleProtocol, NoTls>::new(config, NoTls, queries.len())?;
                    let dispatcher = PandasDispatcher::<
                        _,
                        PostgresPandasTransport<SimpleProtocol, NoTls>,
                    >::new(
                        sb, destination, queries, origin_query, pre_execution_queries
                    );
                    dispatcher.run(py)?
                }
                _ => unimplemented!("{} protocol not supported", protocol),
            }
        }
        SourceType::SQLite => {
            // remove the first "sqlite://" manually since url.path is not correct for windows
            let path = &source_conn.conn.as_str()[9..];
            let source = SQLiteSource::new(path, queries.len())?;
            let dispatcher = PandasDispatcher::<_, SqlitePandasTransport>::new(
                source,
                destination,
                queries,
                origin_query,
                pre_execution_queries,
            );
            dispatcher.run(py)?
        }
        SourceType::MySQL => match protocol {
            "binary" => {
                let source =
                    MySQLSource::<MySQLBinaryProtocol>::new(&source_conn.conn[..], queries.len())?;
                let dispatcher =
                    PandasDispatcher::<_, MysqlPandasTransport<MySQLBinaryProtocol>>::new(
                        source,
                        destination,
                        queries,
                        origin_query,
                        pre_execution_queries,
                    );
                dispatcher.run(py)?
            }
            "text" => {
                let source =
                    MySQLSource::<TextProtocol>::new(&source_conn.conn[..], queries.len())?;
                let dispatcher = PandasDispatcher::<_, MysqlPandasTransport<TextProtocol>>::new(
                    source,
                    destination,
                    queries,
                    origin_query,
                    pre_execution_queries,
                );
                dispatcher.run(py)?
            }
            _ => unimplemented!("{} protocol not supported", protocol),
        },
        SourceType::MsSQL => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = MsSQLSource::new(rt, &source_conn.conn[..], queries.len())?;
            let dispatcher = PandasDispatcher::<_, MsSQLPandasTransport>::new(
                source,
                destination,
                queries,
                origin_query,
                pre_execution_queries,
            );
            dispatcher.run(py)?
        }
        SourceType::Oracle => {
            let source = OracleSource::new(&source_conn.conn[..], queries.len())?;
            let dispatcher = PandasDispatcher::<_, OraclePandasTransport>::new(
                source,
                destination,
                queries,
                origin_query,
                pre_execution_queries,
            );
            dispatcher.run(py)?
        }
        SourceType::BigQuery => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = BigQuerySource::new(rt, &source_conn.conn[..])?;
            let dispatcher = PandasDispatcher::<_, BigQueryPandasTransport>::new(
                source,
                destination,
                queries,
                origin_query,
                pre_execution_queries,
            );
            dispatcher.run(py)?
        }
        SourceType::Trino => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = TrinoSource::new(rt, &source_conn.conn[..])?;
            let dispatcher = PandasDispatcher::<TrinoSource, TrinoPandasTransport>::new(
                source,
                destination,
                queries,
                origin_query,
                pre_execution_queries,
            );
            dispatcher.run(py)?
        }
        _ => unimplemented!("{:?} not implemented!", source_conn.ty),
    }
}
