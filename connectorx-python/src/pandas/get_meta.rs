use super::{
    destination::PandasDestination,
    dispatcher::PandasDispatcher,
    transports::{
        BigQueryPandasTransport, MsSQLPandasTransport, MysqlPandasTransport, OraclePandasTransport,
        PostgresPandasTransport, SqlitePandasTransport, TrinoPandasTransport,
    },
};
use crate::errors::ConnectorXPythonError;
use connectorx::source_router::{SourceConn, SourceType};
use connectorx::{
    prelude::*,
    sources::{
        bigquery::BigQuerySource,
        mssql::MsSQLSource,
        mysql::{BinaryProtocol as MySQLBinaryProtocol, MySQLSource, TextProtocol},
        postgres::{
            rewrite_tls_args, BinaryProtocol as PgBinaryProtocol, CSVProtocol, CursorProtocol,
            PostgresSource, SimpleProtocol,
        },
        sqlite::SQLiteSource,
        trino::TrinoSource,
    },
    sql::CXQuery,
};
use fehler::throws;
use log::debug;
use postgres::NoTls;
use postgres_openssl::MakeTlsConnector;
use pyo3::prelude::*;
use std::convert::TryFrom;
use std::sync::Arc;

#[throws(ConnectorXPythonError)]
pub fn get_meta<'py>(
    py: Python<'py>,
    conn: &str,
    protocol: &str,
    query: String,
) -> Bound<'py, PyAny> {
    let source_conn = SourceConn::try_from(conn)?;
    let destination = PandasDestination::new();
    let queries = &[CXQuery::Naked(query)];

    match source_conn.ty {
        SourceType::Postgres => {
            debug!("Protocol: {}", protocol);
            let (config, tls) = rewrite_tls_args(&source_conn.conn)?;
            match (protocol, tls) {
                ("csv", Some(tls_conn)) => {
                    let sb =
                        PostgresSource::<CSVProtocol, MakeTlsConnector>::new(config, tls_conn, 1)?;
                    let dispatcher = PandasDispatcher::<
                        _,
                        PostgresPandasTransport<CSVProtocol, MakeTlsConnector>,
                    >::new(sb, destination, queries, None, None);
                    debug!("Running dispatcher");
                    dispatcher.get_meta(py)?
                }
                ("csv", None) => {
                    let sb = PostgresSource::<CSVProtocol, NoTls>::new(config, NoTls, 1)?;
                    let dispatcher = PandasDispatcher::<
                        _,
                        PostgresPandasTransport<CSVProtocol, NoTls>,
                    >::new(sb, destination, queries, None, None);
                    debug!("Running dispatcher");
                    dispatcher.get_meta(py)?
                }
                ("binary", Some(tls_conn)) => {
                    let sb = PostgresSource::<PgBinaryProtocol, MakeTlsConnector>::new(
                        config, tls_conn, 1,
                    )?;
                    let dispatcher = PandasDispatcher::<
                        _,
                        PostgresPandasTransport<PgBinaryProtocol, MakeTlsConnector>,
                    >::new(sb, destination, queries, None, None);
                    debug!("Running dispatcher");
                    dispatcher.get_meta(py)?
                }
                ("binary", None) => {
                    let sb = PostgresSource::<PgBinaryProtocol, NoTls>::new(config, NoTls, 1)?;
                    let dispatcher = PandasDispatcher::<
                        _,
                        PostgresPandasTransport<PgBinaryProtocol, NoTls>,
                    >::new(sb, destination, queries, None, None);
                    debug!("Running dispatcher");
                    dispatcher.get_meta(py)?
                }
                ("cursor", Some(tls_conn)) => {
                    let sb = PostgresSource::<CursorProtocol, MakeTlsConnector>::new(
                        config, tls_conn, 1,
                    )?;
                    let dispatcher = PandasDispatcher::<
                        _,
                        PostgresPandasTransport<CursorProtocol, MakeTlsConnector>,
                    >::new(sb, destination, queries, None, None);
                    debug!("Running dispatcher");
                    dispatcher.get_meta(py)?
                }
                ("cursor", None) => {
                    let sb = PostgresSource::<CursorProtocol, NoTls>::new(config, NoTls, 1)?;
                    let dispatcher = PandasDispatcher::<
                        _,
                        PostgresPandasTransport<CursorProtocol, NoTls>,
                    >::new(sb, destination, queries, None, None);
                    debug!("Running dispatcher");
                    dispatcher.get_meta(py)?
                }
                ("simple", Some(tls_conn)) => {
                    let sb = PostgresSource::<SimpleProtocol, MakeTlsConnector>::new(
                        config, tls_conn, 1,
                    )?;
                    let dispatcher = PandasDispatcher::<
                        _,
                        PostgresPandasTransport<SimpleProtocol, MakeTlsConnector>,
                    >::new(sb, destination, queries, None, None);
                    debug!("Running dispatcher");
                    dispatcher.get_meta(py)?
                }
                ("simple", None) => {
                    let sb = PostgresSource::<SimpleProtocol, NoTls>::new(config, NoTls, 1)?;
                    let dispatcher = PandasDispatcher::<
                        _,
                        PostgresPandasTransport<SimpleProtocol, NoTls>,
                    >::new(sb, destination, queries, None, None);
                    debug!("Running dispatcher");
                    dispatcher.get_meta(py)?
                }
                _ => unimplemented!("{} protocol not supported", protocol),
            }
        }
        SourceType::SQLite => {
            // remove the first "sqlite://" manually since url.path is not correct for windows
            let path = &source_conn.conn.as_str()[9..];
            let source = SQLiteSource::new(path, 1)?;
            let dispatcher = PandasDispatcher::<_, SqlitePandasTransport>::new(
                source,
                destination,
                queries,
                None,
                None,
            );
            debug!("Running dispatcher");
            dispatcher.get_meta(py)?
        }
        SourceType::MySQL => {
            debug!("Protocol: {}", protocol);
            match protocol {
                "binary" => {
                    let source = MySQLSource::<MySQLBinaryProtocol>::new(&source_conn.conn[..], 1)?;
                    let dispatcher =
                        PandasDispatcher::<_, MysqlPandasTransport<MySQLBinaryProtocol>>::new(
                            source,
                            destination,
                            queries,
                            None,
                            None,
                        );
                    debug!("Running dispatcher");
                    dispatcher.get_meta(py)?
                }
                "text" => {
                    let source = MySQLSource::<TextProtocol>::new(&source_conn.conn[..], 1)?;
                    let dispatcher = PandasDispatcher::<_, MysqlPandasTransport<TextProtocol>>::new(
                        source,
                        destination,
                        queries,
                        None,
                        None,
                    );
                    debug!("Running dispatcher");
                    dispatcher.get_meta(py)?
                }
                _ => unimplemented!("{} protocol not supported", protocol),
            }
        }
        SourceType::MsSQL => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = MsSQLSource::new(rt, &source_conn.conn[..], 1)?;
            let dispatcher = PandasDispatcher::<_, MsSQLPandasTransport>::new(
                source,
                destination,
                queries,
                None,
                None,
            );
            debug!("Running dispatcher");
            dispatcher.get_meta(py)?
        }
        SourceType::Oracle => {
            let source = OracleSource::new(&source_conn.conn[..], 1)?;
            let dispatcher = PandasDispatcher::<_, OraclePandasTransport>::new(
                source,
                destination,
                queries,
                None,
                None,
            );
            debug!("Running dispatcher");
            dispatcher.get_meta(py)?
        }
        SourceType::BigQuery => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = BigQuerySource::new(rt, &source_conn.conn[..])?;
            let dispatcher = PandasDispatcher::<_, BigQueryPandasTransport>::new(
                source,
                destination,
                queries,
                None,
                None,
            );
            debug!("Running dispatcher");
            dispatcher.get_meta(py)?
        }
        SourceType::Trino => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = TrinoSource::new(rt, &source_conn.conn[..])?;
            let dispatcher = PandasDispatcher::<_, TrinoPandasTransport>::new(
                source,
                destination,
                queries,
                None,
                None,
            );
            dispatcher.get_meta(py)?
        }
        _ => unimplemented!("{:?} not implemented!", source_conn.ty),
    }
}
