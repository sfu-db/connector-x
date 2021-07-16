mod destination;
mod pandas_columns;
mod pystring;
mod transports;
mod types;

pub use self::destination::{PandasBlockInfo, PandasDestination, PandasPartitionDestination};
pub use self::transports::{MysqlPandasTransport, PostgresPandasTransport, SqlitePandasTransport};
pub use self::types::{PandasDType, PandasTypeSystem};
use crate::errors::ConnectorAgentPythonError;
use connectorx::{
    source_router::{SourceConn, SourceType},
    sources::{
        mysql::{MysqlBinary, MysqlSource, MysqlText},
        postgres::{Binary, Cursor, PostgresSource, CSV},
        sqlite::SqliteSource,
    },
    sql::CXQuery,
    Dispatcher,
};
use fehler::throws;
use log::debug;
use pyo3::{PyAny, Python};

#[throws(ConnectorAgentPythonError)]
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
                    let sb = PostgresSource::<CSV>::new(&source_conn.conn[..], queries.len())?;
                    let dispatcher = Dispatcher::<_, _, PostgresPandasTransport<CSV>>::new(
                        sb,
                        &mut destination,
                        queries,
                    );

                    debug!("Running dispatcher");
                    dispatcher.run()?;
                }
                "binary" => {
                    let sb = PostgresSource::<Binary>::new(&source_conn.conn[..], queries.len())?;
                    let dispatcher = Dispatcher::<_, _, PostgresPandasTransport<Binary>>::new(
                        sb,
                        &mut destination,
                        queries,
                    );

                    debug!("Running dispatcher");
                    dispatcher.run()?;
                }
                "cursor" => {
                    let sb = PostgresSource::<Cursor>::new(&source_conn.conn[..], queries.len())?;
                    let dispatcher = Dispatcher::<_, _, PostgresPandasTransport<Cursor>>::new(
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
            let source = SqliteSource::new(&source_conn.conn[..], queries.len())?;
            let dispatcher =
                Dispatcher::<_, _, SqlitePandasTransport>::new(source, &mut destination, queries);
            debug!("Running dispatcher");
            dispatcher.run()?;
        }
        SourceType::Mysql => {
            debug!("Protocol: {}", protocol);
            match protocol {
                "binary" => {
                    let source =
                        MysqlSource::<MysqlBinary>::new(&source_conn.conn[..], queries.len())?;
                    let dispatcher = Dispatcher::<_, _, MysqlPandasTransport<MysqlBinary>>::new(
                        source,
                        &mut destination,
                        queries,
                    );
                    debug!("Running dispatcher");
                    dispatcher.run()?;
                }
                "text" => {
                    let source =
                        MysqlSource::<MysqlText>::new(&source_conn.conn[..], queries.len())?;
                    let dispatcher = Dispatcher::<_, _, MysqlPandasTransport<MysqlText>>::new(
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
