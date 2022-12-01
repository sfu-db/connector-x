use super::{
    destination::PandasDestination,
    transports::{
        BigQueryPandasTransport, MsSQLPandasTransport, MysqlPandasTransport, OraclePandasTransport,
        PostgresPandasTransport, SqlitePandasTransport,
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
pub fn get_meta<'a>(py: Python<'a>, conn: &str, protocol: &str, query: String) -> &'a PyAny {
    let source_conn = SourceConn::try_from(conn)?;
    let mut destination = PandasDestination::new(py);
    let queries = &[CXQuery::Naked(query)];

    match source_conn.ty {
        SourceType::Postgres => {
            debug!("Protocol: {}", protocol);
            let (config, tls) = rewrite_tls_args(&source_conn.conn)?;
            match (protocol, tls) {
                ("csv", Some(tls_conn)) => {
                    let sb =
                        PostgresSource::<CSVProtocol, MakeTlsConnector>::new(config, tls_conn, 1)?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresPandasTransport<CSVProtocol, MakeTlsConnector>,
                    >::new(
                        sb, &mut destination, queries, None
                    );
                    debug!("Running dispatcher");
                    dispatcher.get_meta()?;
                }
                ("csv", None) => {
                    let sb = PostgresSource::<CSVProtocol, NoTls>::new(config, NoTls, 1)?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresPandasTransport<CSVProtocol, NoTls>,
                    >::new(
                        sb, &mut destination, queries, None
                    );
                    debug!("Running dispatcher");
                    dispatcher.get_meta()?;
                }
                ("binary", Some(tls_conn)) => {
                    let sb = PostgresSource::<PgBinaryProtocol, MakeTlsConnector>::new(
                        config, tls_conn, 1,
                    )?;
                    let mut dispatcher =
                        Dispatcher::<
                            _,
                            _,
                            PostgresPandasTransport<PgBinaryProtocol, MakeTlsConnector>,
                        >::new(sb, &mut destination, queries, None);
                    debug!("Running dispatcher");
                    dispatcher.get_meta()?;
                }
                ("binary", None) => {
                    let sb = PostgresSource::<PgBinaryProtocol, NoTls>::new(config, NoTls, 1)?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresPandasTransport<PgBinaryProtocol, NoTls>,
                    >::new(
                        sb, &mut destination, queries, None
                    );
                    debug!("Running dispatcher");
                    dispatcher.get_meta()?;
                }
                ("cursor", Some(tls_conn)) => {
                    let sb = PostgresSource::<CursorProtocol, MakeTlsConnector>::new(
                        config, tls_conn, 1,
                    )?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresPandasTransport<CursorProtocol, MakeTlsConnector>,
                    >::new(
                        sb, &mut destination, queries, None
                    );
                    debug!("Running dispatcher");
                    dispatcher.get_meta()?;
                }
                ("cursor", None) => {
                    let sb = PostgresSource::<CursorProtocol, NoTls>::new(config, NoTls, 1)?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresPandasTransport<CursorProtocol, NoTls>,
                    >::new(
                        sb, &mut destination, queries, None
                    );
                    debug!("Running dispatcher");
                    dispatcher.get_meta()?;
                }
                ("simple", Some(tls_conn)) => {
                    let sb = PostgresSource::<SimpleProtocol, MakeTlsConnector>::new(
                        config, tls_conn, 1,
                    )?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresPandasTransport<SimpleProtocol, MakeTlsConnector>,
                    >::new(
                        sb, &mut destination, queries, None
                    );
                    debug!("Running dispatcher");
                    dispatcher.get_meta()?;
                }
                ("simple", None) => {
                    let sb = PostgresSource::<SimpleProtocol, NoTls>::new(config, NoTls, 1)?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresPandasTransport<SimpleProtocol, NoTls>,
                    >::new(
                        sb, &mut destination, queries, None
                    );
                    debug!("Running dispatcher");
                    dispatcher.get_meta()?;
                }
                _ => unimplemented!("{} protocol not supported", protocol),
            }
        }
        SourceType::SQLite => {
            // remove the first "sqlite://" manually since url.path is not correct for windows
            let path = &source_conn.conn.as_str()[9..];
            let source = SQLiteSource::new(path, 1)?;
            let mut dispatcher = Dispatcher::<_, _, SqlitePandasTransport>::new(
                source,
                &mut destination,
                queries,
                None,
            );
            debug!("Running dispatcher");
            dispatcher.get_meta()?;
        }
        SourceType::MySQL => {
            debug!("Protocol: {}", protocol);
            match protocol {
                "binary" => {
                    let source = MySQLSource::<MySQLBinaryProtocol>::new(&source_conn.conn[..], 1)?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        MysqlPandasTransport<MySQLBinaryProtocol>,
                    >::new(
                        source, &mut destination, queries, None
                    );
                    debug!("Running dispatcher");
                    dispatcher.get_meta()?;
                }
                "text" => {
                    let source = MySQLSource::<TextProtocol>::new(&source_conn.conn[..], 1)?;
                    let mut dispatcher =
                        Dispatcher::<_, _, MysqlPandasTransport<TextProtocol>>::new(
                            source,
                            &mut destination,
                            queries,
                            None,
                        );
                    debug!("Running dispatcher");
                    dispatcher.get_meta()?;
                }
                _ => unimplemented!("{} protocol not supported", protocol),
            }
        }
        SourceType::MsSQL => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = MsSQLSource::new(rt, &source_conn.conn[..], 1)?;
            let mut dispatcher = Dispatcher::<_, _, MsSQLPandasTransport>::new(
                source,
                &mut destination,
                queries,
                None,
            );
            debug!("Running dispatcher");
            dispatcher.get_meta()?;
        }
        SourceType::Oracle => {
            let source = OracleSource::new(&source_conn.conn[..], 1)?;
            let mut dispatcher = Dispatcher::<_, _, OraclePandasTransport>::new(
                source,
                &mut destination,
                queries,
                None,
            );
            debug!("Running dispatcher");
            dispatcher.get_meta()?;
        }
        SourceType::BigQuery => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = BigQuerySource::new(rt, &source_conn.conn[..])?;
            let mut dispatcher = Dispatcher::<_, _, BigQueryPandasTransport>::new(
                source,
                &mut destination,
                queries,
                None,
            );
            debug!("Running dispatcher");
            dispatcher.get_meta()?;
        }
        _ => unimplemented!("{:?} not implemented!", source_conn.ty),
    }

    destination.result()?
}
