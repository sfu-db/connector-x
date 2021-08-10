mod destination;
mod pandas_columns;
mod pystring;
mod transports;
mod typesystem;

pub use self::destination::{PandasBlockInfo, PandasDestination, PandasPartitionDestination};
pub use self::transports::{
    MsSQLPandasTransport, MysqlPandasTransport, PostgresPandasTransport, SqlitePandasTransport,
};
pub use self::typesystem::{PandasDType, PandasTypeSystem};
use crate::errors::ConnectorXPythonError;
use crate::source_router::{SourceConn, SourceType};
use connectorx::{
    prelude::*,
    sources::{
        mssql::MsSQLSource,
        mysql::{BinaryProtocol as MySQLBinaryProtocol, MySQLSource, TextProtocol},
        postgres::{
            rewrite_tls_args, BinaryProtocol as PgBinaryProtocol, CSVProtocol, CursorProtocol,
            PostgresSource,
        },
        sqlite::SQLiteSource,
    },
    sql::CXQuery,
};
use fehler::throws;
use log::debug;
use postgres::NoTls;
use postgres_native_tls::MakeTlsConnector;
use pyo3::{PyAny, Python};
use std::sync::Arc;
use connectorx::sources::oracle::OracleSource;

#[throws(ConnectorXPythonError)]
pub fn write_pandas<'a>(
    py: Python<'a>,
    source_conn: &SourceConn,
    queries: &[CXQuery<String>],
    protocol: &str,
) -> &'a PyAny {
    let mut destination = PandasDestination::new(py);

    // TODO: unlock gil if possible
    match source_conn.ty {
        SourceType::Postgres => {
            debug!("Protocol: {}", protocol);
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
                    >::new(sb, &mut destination, queries);
                    debug!("Running dispatcher");
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
                        );
                    debug!("Running dispatcher");
                    dispatcher.run()?;
                }
                ("binary", Some(tls_conn)) => {
                    let sb = PostgresSource::<PgBinaryProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresPandasTransport<PgBinaryProtocol, MakeTlsConnector>,
                    >::new(sb, &mut destination, queries);
                    debug!("Running dispatcher");
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
                    >::new(sb, &mut destination, queries);
                    debug!("Running dispatcher");
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
                    >::new(sb, &mut destination, queries);
                    debug!("Running dispatcher");
                    dispatcher.run()?;
                }
                ("cursor", None) => {
                    let sb =
                        PostgresSource::<CursorProtocol, NoTls>::new(config, NoTls, queries.len())?;
                    let dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresPandasTransport<CursorProtocol, NoTls>,
                    >::new(sb, &mut destination, queries);
                    debug!("Running dispatcher");
                    dispatcher.run()?;
                }
                _ => unimplemented!("{} protocol not supported", protocol),
            }
        }
        SourceType::SQLite => {
            let source = SQLiteSource::new(source_conn.conn.path(), queries.len())?;
            let dispatcher =
                Dispatcher::<_, _, SqlitePandasTransport>::new(source, &mut destination, queries);
            debug!("Running dispatcher");
            dispatcher.run()?;
        }
        SourceType::MySQL => {
            debug!("Protocol: {}", protocol);
            match protocol {
                "binary" => {
                    let source = MySQLSource::<MySQLBinaryProtocol>::new(
                        &source_conn.conn[..],
                        queries.len(),
                    )?;
                    let dispatcher =
                        Dispatcher::<_, _, MysqlPandasTransport<MySQLBinaryProtocol>>::new(
                            source,
                            &mut destination,
                            queries,
                        );
                    debug!("Running dispatcher");
                    dispatcher.run()?;
                }
                "text" => {
                    let source =
                        MySQLSource::<TextProtocol>::new(&source_conn.conn[..], queries.len())?;
                    let dispatcher = Dispatcher::<_, _, MysqlPandasTransport<TextProtocol>>::new(
                        source,
                        &mut destination,
                        queries,
                    );
                    debug!("Running dispatcher");
                    dispatcher.run()?;
                }
                _ => unimplemented!("{} protocol not supported", protocol),
            }
        }
        SourceType::MsSQL => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = MsSQLSource::new(rt, &source_conn.conn[..], queries.len())?;
            let dispatcher =
                Dispatcher::<_, _, MsSQLPandasTransport>::new(source, &mut destination, queries);
            debug!("Running dispatcher");
            dispatcher.run()?;
        }
        SourceType::Oracle => {
            debug!("Protocol: {}", protocol);
            match protocol {
                "text" => {
                    let source =
                        OracleSource::<TextProtocol>::new(&source_conn.conn[..], queries.len())?;
                    let dispatcher = Dispatcher::<_, _, OraclePandasTransport<TextProtocol>>::new(
                        source,
                        &mut destination,
                        queries,
                    );
                    debug!("Running dispatcher");
                    dispatcher.run()?;
                }
                _ => unimplemented!("{} protocol not supported", protocol),
            }
        }
    }

    destination.result()?
}
