mod destination;
pub mod get_meta;
mod pandas_columns;
mod pystring;
mod transports;
mod typesystem;

pub use self::destination::{PandasBlockInfo, PandasDestination, PandasPartitionDestination};
pub use self::transports::{
    BigQueryPandasTransport, MsSQLPandasTransport, MysqlPandasTransport, OraclePandasTransport,
    PostgresPandasTransport, SqlitePandasTransport,
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
use pyo3::{PyAny, Python};
use std::sync::Arc;

#[throws(ConnectorXPythonError)]
pub fn write_pandas<'a>(
    py: Python<'a>,
    source_conn: &SourceConn,
    origin_query: Option<String>,
    queries: &[CXQuery<String>],
) -> &'a PyAny {
    let mut destination = PandasDestination::new(py);
    let protocol = source_conn.proto.as_str();
    debug!("Protocol: {}", protocol);

    // TODO: unlock gil if possible
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
                    let dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresPandasTransport<CSVProtocol, MakeTlsConnector>,
                    >::new(
                        sb, &mut destination, queries, origin_query
                    );
                    dispatcher.run()?;
                }
                ("csv", None) => {
                    let sb =
                        PostgresSource::<CSVProtocol, NoTls>::new(config, NoTls, queries.len())?;
                    let dispatcher =
                        Dispatcher::<_, _, PostgresPandasTransport<CSVProtocol, NoTls>>::new(
                            sb,
                            &mut destination,
                            queries,
                            origin_query,
                        );
                    dispatcher.run()?;
                }
                ("binary", Some(tls_conn)) => {
                    let sb = PostgresSource::<PgBinaryProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let dispatcher =
                        Dispatcher::<
                            _,
                            _,
                            PostgresPandasTransport<PgBinaryProtocol, MakeTlsConnector>,
                        >::new(sb, &mut destination, queries, origin_query);
                    dispatcher.run()?;
                }
                ("binary", None) => {
                    let sb = PostgresSource::<PgBinaryProtocol, NoTls>::new(
                        config,
                        NoTls,
                        queries.len(),
                    )?;
                    let dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresPandasTransport<PgBinaryProtocol, NoTls>,
                    >::new(
                        sb, &mut destination, queries, origin_query
                    );
                    dispatcher.run()?;
                }
                ("cursor", Some(tls_conn)) => {
                    let sb = PostgresSource::<CursorProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresPandasTransport<CursorProtocol, MakeTlsConnector>,
                    >::new(
                        sb, &mut destination, queries, origin_query
                    );
                    dispatcher.run()?;
                }
                ("cursor", None) => {
                    let sb =
                        PostgresSource::<CursorProtocol, NoTls>::new(config, NoTls, queries.len())?;
                    let dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresPandasTransport<CursorProtocol, NoTls>,
                    >::new(
                        sb, &mut destination, queries, origin_query
                    );
                    dispatcher.run()?;
                }
                ("simple", Some(tls_conn)) => {
                    let sb = PostgresSource::<SimpleProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresPandasTransport<SimpleProtocol, MakeTlsConnector>,
                    >::new(
                        sb, &mut destination, queries, origin_query
                    );
                    dispatcher.run()?;
                }
                ("simple", None) => {
                    let sb =
                        PostgresSource::<SimpleProtocol, NoTls>::new(config, NoTls, queries.len())?;
                    let dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresPandasTransport<SimpleProtocol, NoTls>,
                    >::new(
                        sb, &mut destination, queries, origin_query
                    );
                    dispatcher.run()?;
                }
                _ => unimplemented!("{} protocol not supported", protocol),
            }
        }
        SourceType::SQLite => {
            // remove the first "sqlite://" manually since url.path is not correct for windows
            let path = &source_conn.conn.as_str()[9..];
            let source = SQLiteSource::new(path, queries.len())?;
            let dispatcher = Dispatcher::<_, _, SqlitePandasTransport>::new(
                source,
                &mut destination,
                queries,
                origin_query,
            );
            dispatcher.run()?;
        }
        SourceType::MySQL => match protocol {
            "binary" => {
                let source =
                    MySQLSource::<MySQLBinaryProtocol>::new(&source_conn.conn[..], queries.len())?;
                let dispatcher = Dispatcher::<_, _, MysqlPandasTransport<MySQLBinaryProtocol>>::new(
                    source,
                    &mut destination,
                    queries,
                    origin_query,
                );
                dispatcher.run()?;
            }
            "text" => {
                let source =
                    MySQLSource::<TextProtocol>::new(&source_conn.conn[..], queries.len())?;
                let dispatcher = Dispatcher::<_, _, MysqlPandasTransport<TextProtocol>>::new(
                    source,
                    &mut destination,
                    queries,
                    origin_query,
                );
                dispatcher.run()?;
            }
            _ => unimplemented!("{} protocol not supported", protocol),
        },
        SourceType::MsSQL => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = MsSQLSource::new(rt, &source_conn.conn[..], queries.len())?;
            let dispatcher = Dispatcher::<_, _, MsSQLPandasTransport>::new(
                source,
                &mut destination,
                queries,
                origin_query,
            );
            dispatcher.run()?;
        }
        SourceType::Oracle => {
            let source = OracleSource::new(&source_conn.conn[..], queries.len())?;
            let dispatcher = Dispatcher::<_, _, OraclePandasTransport>::new(
                source,
                &mut destination,
                queries,
                origin_query,
            );
            dispatcher.run()?;
        }
        SourceType::BigQuery => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = BigQuerySource::new(rt, &source_conn.conn[..])?;
            let dispatcher = Dispatcher::<_, _, BigQueryPandasTransport>::new(
                source,
                &mut destination,
                queries,
                origin_query,
            );
            dispatcher.run()?;
        }
        _ => unimplemented!("{:?} not implemented!", source_conn.ty),
    }

    destination.result()?
}
