#[cfg(any(
    feature = "src_postgres",
    feature = "src_mysql",
    feature = "src_sqlite",
    feature = "src_mssql_common",
    feature = "src_oracle",
    feature = "src_bigquery",
    feature = "src_clickhouse",
))]
use crate::arrow_batch_iter::ArrowBatchIter;
#[cfg(feature = "src_mysql")]
use crate::sources::mysql::{BinaryProtocol as MySQLBinaryProtocol, TextProtocol};
#[cfg(feature = "src_postgres")]
use crate::sources::postgres::{
    rewrite_tls_args, BinaryProtocol as PgBinaryProtocol, CSVProtocol, CursorProtocol,
    SimpleProtocol,
};
use crate::{arrow_batch_iter::RecordBatchIterator, prelude::*, sql::CXQuery};
use fehler::{throw, throws};
use log::debug;
#[cfg(feature = "src_postgres")]
use postgres::NoTls;
#[cfg(feature = "src_postgres")]
use postgres_openssl::MakeTlsConnector;
#[allow(unused_imports)]
use std::sync::Arc;

/// Run `queries` against `source_conn` and collect the result in an [`ArrowDestination`].
#[allow(unreachable_code, unreachable_patterns, unused_variables, unused_mut)]
#[throws(ConnectorXOutError)]
pub fn get_arrow(
    source_conn: &SourceConn,
    origin_query: Option<String>,
    queries: &[CXQuery<String>],
    pre_execution_queries: Option<&[String]>,
) -> ArrowDestination {
    let mut destination = ArrowDestination::new();
    let protocol = source_conn.proto.as_str();
    debug!("Protocol: {}", protocol);

    match source_conn.ty {
        #[cfg(feature = "src_postgres")]
        SourceType::Postgres => {
            let (config, tls) = rewrite_tls_args(&source_conn.conn)?;
            match (protocol, tls) {
                ("csv", Some(tls_conn)) => {
                    let source = PostgresSource::<CSVProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresArrowTransport<CSVProtocol, MakeTlsConnector>,
                    >::new(
                        source, &mut destination, queries, origin_query
                    );
                    dispatcher.set_pre_execution_queries(pre_execution_queries);
                    dispatcher.run()?;
                }
                ("csv", None) => {
                    let source =
                        PostgresSource::<CSVProtocol, NoTls>::new(config, NoTls, queries.len())?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresArrowTransport<CSVProtocol, NoTls>,
                    >::new(
                        source, &mut destination, queries, origin_query
                    );
                    dispatcher.set_pre_execution_queries(pre_execution_queries);
                    dispatcher.run()?;
                }
                ("binary", Some(tls_conn)) => {
                    let source = PostgresSource::<PgBinaryProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresArrowTransport<PgBinaryProtocol, MakeTlsConnector>,
                    >::new(
                        source, &mut destination, queries, origin_query
                    );
                    dispatcher.set_pre_execution_queries(pre_execution_queries);
                    dispatcher.run()?;
                }
                ("binary", None) => {
                    let source = PostgresSource::<PgBinaryProtocol, NoTls>::new(
                        config,
                        NoTls,
                        queries.len(),
                    )?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresArrowTransport<PgBinaryProtocol, NoTls>,
                    >::new(
                        source, &mut destination, queries, origin_query
                    );
                    dispatcher.set_pre_execution_queries(pre_execution_queries);
                    dispatcher.run()?;
                }
                ("cursor", Some(tls_conn)) => {
                    let source = PostgresSource::<CursorProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresArrowTransport<CursorProtocol, MakeTlsConnector>,
                    >::new(
                        source, &mut destination, queries, origin_query
                    );
                    dispatcher.set_pre_execution_queries(pre_execution_queries);
                    dispatcher.run()?;
                }
                ("cursor", None) => {
                    let source =
                        PostgresSource::<CursorProtocol, NoTls>::new(config, NoTls, queries.len())?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresArrowTransport<CursorProtocol, NoTls>,
                    >::new(
                        source, &mut destination, queries, origin_query
                    );
                    dispatcher.set_pre_execution_queries(pre_execution_queries);
                    dispatcher.run()?;
                }
                ("simple", Some(tls_conn)) => {
                    let sb = PostgresSource::<SimpleProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresArrowTransport<SimpleProtocol, MakeTlsConnector>,
                    >::new(
                        sb, &mut destination, queries, origin_query
                    );
                    debug!("Running dispatcher");
                    dispatcher.set_pre_execution_queries(pre_execution_queries);
                    dispatcher.run()?;
                }
                ("simple", None) => {
                    let sb =
                        PostgresSource::<SimpleProtocol, NoTls>::new(config, NoTls, queries.len())?;
                    let mut dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresArrowTransport<SimpleProtocol, NoTls>,
                    >::new(
                        sb, &mut destination, queries, origin_query
                    );
                    debug!("Running dispatcher");
                    dispatcher.set_pre_execution_queries(pre_execution_queries);
                    dispatcher.run()?;
                }
                _ => throw!(ConnectorXOutError::protocol_not_supported(
                    protocol,
                    SourceType::Postgres,
                    &["binary", "csv", "cursor", "simple"]
                )),
            }
        }
        #[cfg(feature = "src_mysql")]
        SourceType::MySQL => match protocol {
            "binary" => {
                let source =
                    MySQLSource::<MySQLBinaryProtocol>::new(&source_conn.conn[..], queries.len())?;
                let mut dispatcher =
                    Dispatcher::<_, _, MySQLArrowTransport<MySQLBinaryProtocol>>::new(
                        source,
                        &mut destination,
                        queries,
                        origin_query,
                    );
                dispatcher.set_pre_execution_queries(pre_execution_queries);
                dispatcher.run()?;
            }
            "text" => {
                let source =
                    MySQLSource::<TextProtocol>::new(&source_conn.conn[..], queries.len())?;
                let mut dispatcher = Dispatcher::<_, _, MySQLArrowTransport<TextProtocol>>::new(
                    source,
                    &mut destination,
                    queries,
                    origin_query,
                );
                dispatcher.set_pre_execution_queries(pre_execution_queries);
                dispatcher.run()?;
            }
            _ => throw!(ConnectorXOutError::protocol_not_supported(
                protocol,
                SourceType::MySQL,
                &["binary", "text"]
            )),
        },
        #[cfg(feature = "src_sqlite")]
        SourceType::SQLite => {
            // remove the first "sqlite://" manually since url.path is not correct for windows
            let path = &source_conn.conn.as_str()[9..];
            let source = SQLiteSource::new(path, queries.len())?;
            let dispatcher = Dispatcher::<_, _, SQLiteArrowTransport>::new(
                source,
                &mut destination,
                queries,
                origin_query,
            );
            dispatcher.run()?;
        }
        #[cfg(feature = "src_mssql_common")]
        SourceType::MsSQL => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = MsSQLSource::new(rt, &source_conn.conn[..], queries.len())?;
            let dispatcher = Dispatcher::<_, _, MsSQLArrowTransport>::new(
                source,
                &mut destination,
                queries,
                origin_query,
            );
            dispatcher.run()?;
        }
        #[cfg(feature = "src_oracle")]
        SourceType::Oracle => {
            let source = OracleSource::new(&source_conn.conn[..], queries.len())?;
            let dispatcher = Dispatcher::<_, _, OracleArrowTransport>::new(
                source,
                &mut destination,
                queries,
                origin_query,
            );
            dispatcher.run()?;
        }
        #[cfg(feature = "src_bigquery")]
        SourceType::BigQuery => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = BigQuerySource::new(rt, &source_conn.conn[..])?;
            let dispatcher = Dispatcher::<_, _, BigQueryArrowTransport>::new(
                source,
                &mut destination,
                queries,
                origin_query,
            );
            dispatcher.run()?;
        }
        #[cfg(feature = "src_trino")]
        SourceType::Trino => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = TrinoSource::new(rt, &source_conn.conn[..])?;
            let dispatcher = Dispatcher::<_, _, TrinoArrowTransport>::new(
                source,
                &mut destination,
                queries,
                origin_query,
            );
            dispatcher.run()?;
        }
        #[cfg(feature = "src_clickhouse")]
        SourceType::ClickHouse => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = ClickHouseSource::new(rt, &source_conn.conn[..])?;
            let dispatcher = Dispatcher::<_, _, ClickHouseArrowTransport>::new(
                source,
                &mut destination,
                queries,
                origin_query,
            );
            dispatcher.run()?;
        }
        _ => throw!(ConnectorXOutError::SourceNotSupport(format!(
            "{:?}",
            source_conn.ty
        ))),
    }

    destination
}

/// Run `queries` against `source_conn` and return an iterator yielding record batches of at most `batch_size` rows.
#[allow(unreachable_code, unreachable_patterns, unused_variables, unused_mut)]
#[throws(ConnectorXOutError)]
pub fn new_record_batch_iter(
    source_conn: &SourceConn,
    origin_query: Option<String>,
    queries: &[CXQuery<String>],
    batch_size: usize,
    pre_execution_queries: Option<&[String]>,
) -> Box<dyn RecordBatchIterator> {
    let destination = ArrowStreamDestination::new_with_batch_size(batch_size);
    let protocol = source_conn.proto.as_str();
    debug!("Protocol: {}", protocol);

    match source_conn.ty {
        #[cfg(feature = "src_postgres")]
        SourceType::Postgres => {
            let (config, tls) = rewrite_tls_args(&source_conn.conn)?;
            match (protocol, tls) {
                ("csv", Some(tls_conn)) => {
                    let mut source = PostgresSource::<CSVProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;

                    source.set_pre_execution_queries(pre_execution_queries);

                    let batch_iter =
                        ArrowBatchIter::<
                            _,
                            PostgresArrowStreamTransport<CSVProtocol, MakeTlsConnector>,
                        >::new(source, destination, origin_query, queries)?;
                    let iter: Box<dyn RecordBatchIterator> = Box::new(batch_iter);
                    return iter;
                }
                ("csv", None) => {
                    let mut source =
                        PostgresSource::<CSVProtocol, NoTls>::new(config, NoTls, queries.len())?;

                    source.set_pre_execution_queries(pre_execution_queries);

                    let batch_iter = ArrowBatchIter::<
                        _,
                        PostgresArrowStreamTransport<CSVProtocol, NoTls>,
                    >::new(
                        source, destination, origin_query, queries
                    )?;
                    let iter: Box<dyn RecordBatchIterator> = Box::new(batch_iter);
                    return iter;
                }
                ("binary", Some(tls_conn)) => {
                    let mut source = PostgresSource::<PgBinaryProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;

                    source.set_pre_execution_queries(pre_execution_queries);

                    let batch_iter =
                        ArrowBatchIter::<
                            _,
                            PostgresArrowStreamTransport<PgBinaryProtocol, MakeTlsConnector>,
                        >::new(source, destination, origin_query, queries)?;
                    let iter: Box<dyn RecordBatchIterator> = Box::new(batch_iter);
                    return iter;
                }
                ("binary", None) => {
                    let mut source = PostgresSource::<PgBinaryProtocol, NoTls>::new(
                        config,
                        NoTls,
                        queries.len(),
                    )?;

                    source.set_pre_execution_queries(pre_execution_queries);

                    let batch_iter = ArrowBatchIter::<
                        _,
                        PostgresArrowStreamTransport<PgBinaryProtocol, NoTls>,
                    >::new(
                        source, destination, origin_query, queries
                    )?;
                    let iter: Box<dyn RecordBatchIterator> = Box::new(batch_iter);
                    return iter;
                }
                ("cursor", Some(tls_conn)) => {
                    let mut source = PostgresSource::<CursorProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;

                    source.set_pre_execution_queries(pre_execution_queries);

                    let batch_iter =
                        ArrowBatchIter::<
                            _,
                            PostgresArrowStreamTransport<CursorProtocol, MakeTlsConnector>,
                        >::new(source, destination, origin_query, queries)?;
                    let iter: Box<dyn RecordBatchIterator> = Box::new(batch_iter);
                    return iter;
                }
                ("cursor", None) => {
                    let mut source =
                        PostgresSource::<CursorProtocol, NoTls>::new(config, NoTls, queries.len())?;

                    source.set_pre_execution_queries(pre_execution_queries);

                    let batch_iter = ArrowBatchIter::<
                        _,
                        PostgresArrowStreamTransport<CursorProtocol, NoTls>,
                    >::new(
                        source, destination, origin_query, queries
                    )?;
                    let iter: Box<dyn RecordBatchIterator> = Box::new(batch_iter);
                    return iter;
                }
                _ => throw!(ConnectorXOutError::protocol_not_supported(
                    protocol,
                    SourceType::Postgres,
                    &["binary", "csv", "cursor"]
                )),
            }
        }
        #[cfg(feature = "src_mysql")]
        SourceType::MySQL => match protocol {
            "binary" => {
                let mut source =
                    MySQLSource::<MySQLBinaryProtocol>::new(&source_conn.conn[..], queries.len())?;

                source.set_pre_execution_queries(pre_execution_queries);

                let batch_iter =
                    ArrowBatchIter::<_, MySQLArrowStreamTransport<MySQLBinaryProtocol>>::new(
                        source,
                        destination,
                        origin_query,
                        queries,
                    )?;
                let iter: Box<dyn RecordBatchIterator> = Box::new(batch_iter);
                return iter;
            }
            "text" => {
                let mut source =
                    MySQLSource::<TextProtocol>::new(&source_conn.conn[..], queries.len())?;

                source.set_pre_execution_queries(pre_execution_queries);

                let batch_iter = ArrowBatchIter::<_, MySQLArrowStreamTransport<TextProtocol>>::new(
                    source,
                    destination,
                    origin_query,
                    queries,
                )?;
                let iter: Box<dyn RecordBatchIterator> = Box::new(batch_iter);
                return iter;
            }
            _ => throw!(ConnectorXOutError::protocol_not_supported(
                protocol,
                SourceType::MySQL,
                &["binary", "text"]
            )),
        },
        #[cfg(feature = "src_sqlite")]
        SourceType::SQLite => {
            // remove the first "sqlite://" manually since url.path is not correct for windows
            let path = &source_conn.conn.as_str()[9..];
            let source = SQLiteSource::new(path, queries.len())?;
            let batch_iter = ArrowBatchIter::<_, SQLiteArrowStreamTransport>::new(
                source,
                destination,
                origin_query,
                queries,
            )?;
            let iter: Box<dyn RecordBatchIterator> = Box::new(batch_iter);
            return iter;
        }
        #[cfg(feature = "src_mssql_common")]
        SourceType::MsSQL => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = MsSQLSource::new(rt, &source_conn.conn[..], queries.len())?;
            let batch_iter = ArrowBatchIter::<_, MsSQLArrowStreamTransport>::new(
                source,
                destination,
                origin_query,
                queries,
            )?;
            let iter: Box<dyn RecordBatchIterator> = Box::new(batch_iter);
            return iter;
        }
        #[cfg(feature = "src_oracle")]
        SourceType::Oracle => {
            let source = OracleSource::new(&source_conn.conn[..], queries.len())?;
            let batch_iter = ArrowBatchIter::<_, OracleArrowStreamTransport>::new(
                source,
                destination,
                origin_query,
                queries,
            )?;
            let iter: Box<dyn RecordBatchIterator> = Box::new(batch_iter);
            return iter;
        }
        #[cfg(feature = "src_bigquery")]
        SourceType::BigQuery => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = BigQuerySource::new(rt, &source_conn.conn[..])?;
            let batch_iter = ArrowBatchIter::<_, BigQueryArrowStreamTransport>::new(
                source,
                destination,
                origin_query,
                queries,
            )?;
            let iter: Box<dyn RecordBatchIterator> = Box::new(batch_iter);
            return iter;
        }
        #[cfg(feature = "src_clickhouse")]
        SourceType::ClickHouse => {
            let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
            let source = ClickHouseSource::new(rt, &source_conn.conn[..])?;
            let batch_iter = ArrowBatchIter::<_, ClickHouseArrowStreamTransport>::new(
                source,
                destination,
                origin_query,
                queries,
            )?;
            let iter: Box<dyn RecordBatchIterator> = Box::new(batch_iter);
            return iter;
        }
        _ => throw!(ConnectorXOutError::SourceNotSupport(format!(
            "{:?}",
            source_conn.ty
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source_router::SourceConn;

    /// Fails with the error for an unsupported `protocol`, before any
    /// connection is attempted (nothing listens on port 1).
    fn assert_protocol_rejected(url: &str, protocol: &str, db: &str, supported: &str) {
        let mut conn = SourceConn::try_from(url).unwrap();
        conn.set_protocol(protocol);
        let queries = [CXQuery::naked("SELECT 1")];
        let expected = format!(
            "Protocol \"{protocol}\" is not supported for {db}; supported protocols: {supported}."
        );

        let err = get_arrow(&conn, None, &queries, None).err().unwrap();
        assert!(matches!(
            err,
            ConnectorXOutError::ProtocolNotSupported { .. }
        ));
        assert_eq!(err.to_string(), expected);

        let err = new_record_batch_iter(&conn, None, &queries, 1024, None)
            .err()
            .unwrap();
        assert!(matches!(
            err,
            ConnectorXOutError::ProtocolNotSupported { .. }
        ));
        // The record batch iterator has no `simple` protocol for Postgres.
        assert!(err
            .to_string()
            .starts_with(&format!("Protocol \"{protocol}\"")));
    }

    #[cfg(feature = "src_postgres")]
    #[test]
    fn unsupported_postgres_protocol_is_an_error() {
        assert_protocol_rejected(
            "postgresql://u:p@127.0.0.1:1/db",
            "text",
            "Postgres",
            "binary, csv, cursor, simple",
        );
    }

    #[cfg(feature = "src_mysql")]
    #[test]
    fn unsupported_mysql_protocol_is_an_error() {
        assert_protocol_rejected(
            "mysql://u:p@127.0.0.1:1/db",
            "cursor",
            "MySQL",
            "binary, text",
        );
    }

    #[test]
    fn unsupported_source_is_an_error() {
        let conn = SourceConn::try_from("nosuchdb://127.0.0.1:1/db").unwrap();
        let queries = [CXQuery::naked("SELECT 1")];
        let err = get_arrow(&conn, None, &queries, None).err().unwrap();
        assert_eq!(err.to_string(), "Source Unknown not supported.");
        let err = new_record_batch_iter(&conn, None, &queries, 1024, None)
            .err()
            .unwrap();
        assert_eq!(err.to_string(), "Source Unknown not supported.");
    }
}
