mod destination;
mod pandas_columns;
mod pystring;
mod transports;
mod types;

pub use self::destination::{PandasBlockInfo, PandasDestination, PandasPartitionDestination};
pub use self::transports::{MysqlPandasTransport, PostgresPandasTransport, SqlitePandasTransport};
pub use self::types::{PandasDType, PandasTypeSystem};
use crate::errors::ConnectorXPythonError;
use crate::source_router::{SourceConn, SourceType};
use connectorx::{
    prelude::*,
    sources::{
        mysql::{BinaryProtocol as MySQLBinaryProtocol, MySQLSource, TextProtocol},
        postgres::{
            BinaryProtocol as PgBinaryProtocol, CSVProtocol, CursorProtocol, PostgresSource,
        },
        sqlite::SQLiteSource,
    },
    sql::CXQuery,
};
use fehler::throws;
use log::debug;
use pyo3::{PyAny, Python};

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
            match protocol {
                "csv" => {
                    let sb =
                        PostgresSource::<CSVProtocol>::new(&source_conn.conn[..], queries.len())?;
                    let dispatcher = Dispatcher::<_, _, PostgresPandasTransport<CSVProtocol>>::new(
                        sb,
                        &mut destination,
                        queries,
                    );

                    debug!("Running dispatcher");
                    dispatcher.run()?;
                }
                "binary" => {
                    let sb = PostgresSource::<PgBinaryProtocol>::new(
                        &source_conn.conn[..],
                        queries.len(),
                    )?;
                    let dispatcher =
                        Dispatcher::<_, _, PostgresPandasTransport<PgBinaryProtocol>>::new(
                            sb,
                            &mut destination,
                            queries,
                        );

                    debug!("Running dispatcher");
                    dispatcher.run()?;
                }
                "cursor" => {
                    let sb = PostgresSource::<CursorProtocol>::new(
                        &source_conn.conn[..],
                        queries.len(),
                    )?;
                    let dispatcher =
                        Dispatcher::<_, _, PostgresPandasTransport<CursorProtocol>>::new(
                            sb,
                            &mut destination,
                            queries,
                        );

                    debug!("Running dispatcher");
                    dispatcher.run()?;
                }
                _ => unimplemented!("{} protocol not supported", protocol),
            }
        }
        SourceType::Sqlite => {
            let source = SQLiteSource::new(&source_conn.conn[..], queries.len())?;
            let dispatcher =
                Dispatcher::<_, _, SqlitePandasTransport>::new(source, &mut destination, queries);
            debug!("Running dispatcher");
            dispatcher.run()?;
        }
        SourceType::Mysql => {
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
    }

    destination.result()?
}
