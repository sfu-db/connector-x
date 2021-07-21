use crate::errors::ConnectorAgentPythonError;
use arrow::record_batch::RecordBatch;
use connectorx::{
    destinations::arrow::ArrowDestination,
    source_router::{SourceConn, SourceType},
    sources::{
        mysql::{BinaryProtocol as MySQLBinaryProtocol, MysqlSource, TextProtocol},
        postgres::{
            BinaryProtocol as PgBinaryProtocol, CSVProtocol, CursorProtocol, PostgresSource,
        },
        sqlite::SqliteSource,
    },
    sql::CXQuery,
    transports::{MysqlArrowTransport, PostgresArrowTransport, SqliteArrowTransport},
    Dispatcher,
};
use fehler::throws;
use libc::uintptr_t;
use log::debug;
use pyo3::prelude::*;
use pyo3::{PyAny, Python};

#[throws(ConnectorAgentPythonError)]
pub fn write_arrow<'a>(
    py: Python<'a>,
    source_conn: &SourceConn,
    queries: &[CXQuery<String>],
    protocol: &str,
) -> &'a PyAny {
    let mut destination = ArrowDestination::new();

    // TODO: unlock gil if possible
    match source_conn.ty {
        SourceType::Postgres => {
            debug!("Protocol: {}", protocol);
            match protocol {
                "csv" => {
                    let sb =
                        PostgresSource::<CSVProtocol>::new(&source_conn.conn[..], queries.len())?;
                    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<CSVProtocol>>::new(
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
                        Dispatcher::<_, _, PostgresArrowTransport<PgBinaryProtocol>>::new(
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
                        Dispatcher::<_, _, PostgresArrowTransport<CursorProtocol>>::new(
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
                Dispatcher::<_, _, SqliteArrowTransport>::new(source, &mut destination, queries);
            debug!("Running dispatcher");
            dispatcher.run()?;
        }
        SourceType::Mysql => {
            debug!("Protocol: {}", protocol);
            match protocol {
                "binary" => {
                    let source = MysqlSource::<MySQLBinaryProtocol>::new(
                        &source_conn.conn[..],
                        queries.len(),
                    )?;
                    let dispatcher =
                        Dispatcher::<_, _, MysqlArrowTransport<MySQLBinaryProtocol>>::new(
                            source,
                            &mut destination,
                            queries,
                        );
                    debug!("Running dispatcher");
                    dispatcher.run()?;
                }
                "text" => {
                    let source =
                        MysqlSource::<TextProtocol>::new(&source_conn.conn[..], queries.len())?;
                    let dispatcher = Dispatcher::<_, _, MysqlArrowTransport<TextProtocol>>::new(
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

    let rbs = destination.finish()?;
    let ptrs = to_ptrs(rbs);
    let obj: PyObject = ptrs.into_py(py);
    obj.into_ref(py)
}

fn to_ptrs(rbs: Vec<RecordBatch>) -> (Vec<String>, Vec<Vec<(uintptr_t, uintptr_t)>>) {
    if rbs.is_empty() {
        return (vec![], vec![]);
    }

    let mut result = vec![];
    let names = rbs[0]
        .schema()
        .fields()
        .iter()
        .map(|f| f.name().clone())
        .collect();

    for rb in rbs {
        let mut cols = vec![];

        for array in rb.columns() {
            let (array_ptr, schema_ptr) = array.to_raw().expect("c ptr");
            cols.push((array_ptr as uintptr_t, schema_ptr as uintptr_t));
        }

        result.push(cols);
    }
    (names, result)
}
