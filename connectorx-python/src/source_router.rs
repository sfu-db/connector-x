use crate::errors::{ConnectorXPythonError, Result};
use anyhow::anyhow;
use connectorx::{
    sources::{
        mssql::{FloatN, IntN, MsSQLTypeSystem},
        mysql::{MySQLSourceError, MySQLTypeSystem},
        oracle::OracleDialect,
        postgres::{rewrite_tls_args, PostgresTypeSystem},
    },
    sql::{
        get_partition_range_query, get_partition_range_query_sep, single_col_partition_query,
        CXQuery,
    },
};
use fehler::{throw, throws};
use r2d2_mysql::mysql::{prelude::Queryable, Opts, Pool, Row};
use r2d2_oracle::oracle::Connection as oracle_conn;
use rusqlite::{types::Type, Connection};
use sqlparser::dialect::{MsSqlDialect, MySqlDialect, PostgreSqlDialect, SQLiteDialect};
use std::convert::TryFrom;
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use url::Url;
use urlencoding::decode;

pub enum SourceType {
    Postgres,
    SQLite,
    MySQL,
    MsSQL,
    Oracle,
}

pub struct SourceConn {
    pub ty: SourceType,
    pub conn: Url,
}

impl TryFrom<&str> for SourceConn {
    type Error = ConnectorXPythonError;

    fn try_from(conn: &str) -> Result<SourceConn> {
        let url = Url::parse(conn).map_err(|e| anyhow!("parse error: {}", e))?;
        match url.scheme() {
            "postgres" | "postgresql" => Ok(SourceConn {
                ty: SourceType::Postgres,
                conn: url,
            }),
            "sqlite" => Ok(SourceConn {
                ty: SourceType::SQLite,
                conn: url,
            }),
            "mysql" => Ok(SourceConn {
                ty: SourceType::MySQL,
                conn: url,
            }),
            "mssql" => Ok(SourceConn {
                ty: SourceType::MsSQL,
                conn: url,
            }),
            "oracle" => Ok(SourceConn {
                ty: SourceType::Oracle,
                conn: url,
            }),

            _ => unimplemented!("Connection: {} not supported!", conn),
        }
    }
}

impl SourceConn {
    pub fn get_col_range(&self, query: &str, col: &str) -> Result<(i64, i64)> {
        match self.ty {
            SourceType::Postgres => pg_get_partition_range(&self.conn, query, col),
            SourceType::SQLite => sqlite_get_partition_range(&self.conn, query, col),
            SourceType::MySQL => mysql_get_partition_range(&self.conn, query, col),
            SourceType::MsSQL => mssql_get_partition_range(&self.conn, query, col),
            SourceType::Oracle => oracle_get_partition_range(&self.conn, query, col),
        }
    }

    #[throws(ConnectorXPythonError)]
    pub fn get_part_query(
        &self,
        query: &str,
        col: &str,
        lower: i64,
        upper: i64,
    ) -> CXQuery<String> {
        let query = match self.ty {
            SourceType::Postgres => {
                single_col_partition_query(query, col, lower, upper, &PostgreSqlDialect {})?
            }
            SourceType::SQLite => {
                single_col_partition_query(query, col, lower, upper, &SQLiteDialect {})?
            }
            SourceType::MySQL => {
                single_col_partition_query(query, col, lower, upper, &MySqlDialect {})?
            }
            SourceType::MsSQL => {
                single_col_partition_query(query, col, lower, upper, &MsSqlDialect {})?
            }
            SourceType::Oracle => {
                single_col_partition_query(query, col, lower, upper, &OracleDialect {})?
            }
        };
        println!("get partition query: {:?}", query);
        CXQuery::Wrapped(query)
    }
}

#[throws(ConnectorXPythonError)]
fn pg_get_partition_range(conn: &Url, query: &str, col: &str) -> (i64, i64) {
    let (config, tls) = rewrite_tls_args(conn)?;
    let mut client = match tls {
        None => config.connect(postgres::NoTls)?,
        Some(tls_conn) => config.connect(tls_conn)?,
    };
    // let mut client = config.connect(postgres::NoTls)?;
    // let mut client = config.connect(tls.unwrap_or(postgres::NoTls))?;
    let range_query = get_partition_range_query(query, col, &PostgreSqlDialect {})?;
    let row = client.query_one(range_query.as_str(), &[])?;

    let col_type = PostgresTypeSystem::from(row.columns()[0].type_());
    let (min_v, max_v) = match col_type {
        PostgresTypeSystem::Int2(_) => {
            let min_v: Option<i16> = row.get(0);
            let max_v: Option<i16> = row.get(1);
            (min_v.unwrap_or(0) as i64, max_v.unwrap_or(0) as i64)
        }
        PostgresTypeSystem::Int4(_) => {
            let min_v: Option<i32> = row.get(0);
            let max_v: Option<i32> = row.get(1);
            (min_v.unwrap_or(0) as i64, max_v.unwrap_or(0) as i64)
        }
        PostgresTypeSystem::Int8(_) => {
            let min_v: Option<i64> = row.get(0);
            let max_v: Option<i64> = row.get(1);
            (min_v.unwrap_or(0), max_v.unwrap_or(0))
        }
        PostgresTypeSystem::Float4(_) => {
            let min_v: Option<f32> = row.get(0);
            let max_v: Option<f32> = row.get(1);
            (min_v.unwrap_or(0.0) as i64, max_v.unwrap_or(0.0) as i64)
        }
        PostgresTypeSystem::Float8(_) => {
            let min_v: Option<f64> = row.get(0);
            let max_v: Option<f64> = row.get(1);
            (min_v.unwrap_or(0.0) as i64, max_v.unwrap_or(0.0) as i64)
        }
        _ => throw!(anyhow!(
            "Partition can only be done on int or float columns"
        )),
    };

    (min_v, max_v)
}

#[throws(ConnectorXPythonError)]
fn sqlite_get_partition_range(conn: &Url, query: &str, col: &str) -> (i64, i64) {
    let conn = Connection::open(conn.path())?;
    // SQLite only optimize min max queries when there is only one aggregation
    // https://www.sqlite.org/optoverview.html#minmax
    let (min_query, max_query) = get_partition_range_query_sep(query, col, &SQLiteDialect {})?;
    let mut error = None;
    let min_v = conn.query_row(min_query.as_str(), [], |row| {
        // declare type for count query will be None, only need to check the returned value type
        let col_type = row.get_ref(0)?.data_type();
        match col_type {
            Type::Integer => row.get(0),
            Type::Null => Ok(0),
            _ => {
                error = Some(anyhow!("Partition can only be done on integer columns"));
                Ok(0)
            }
        }
    })?;
    match error {
        None => {}
        Some(e) => throw!(e),
    }
    let max_v = conn.query_row(max_query.as_str(), [], |row| {
        let col_type = row.get_ref(0)?.data_type();
        match col_type {
            Type::Integer => row.get(0),
            Type::Null => Ok(0),
            _ => {
                error = Some(anyhow!("Partition can only be done on integer columns"));
                Ok(0)
            }
        }
    })?;
    match error {
        None => {}
        Some(e) => throw!(e),
    }

    (min_v, max_v)
}

#[throws(ConnectorXPythonError)]
fn mysql_get_partition_range(conn: &Url, query: &str, col: &str) -> (i64, i64) {
    let pool = Pool::new(Opts::from_url(conn.as_str()).map_err(MySQLSourceError::MySQLUrlError)?)?;
    let mut conn = pool.get_conn()?;
    let range_query = get_partition_range_query(query, col, &MySqlDialect {})?;
    let row: Row = conn
        .query_first(range_query)?
        .ok_or_else(|| anyhow!("mysql range: no row returns"))?;

    let col_type = MySQLTypeSystem::from(&row.columns()[0].column_type());
    let (min_v, max_v) = match col_type {
        MySQLTypeSystem::Long(_) => {
            let min_v: Option<i64> = row
                .get(0)
                .ok_or_else(|| anyhow!("mysql range: cannot get min value"))?;
            let max_v: Option<i64> = row
                .get(1)
                .ok_or_else(|| anyhow!("mysql range: cannot get max value"))?;
            (min_v.unwrap_or(0), max_v.unwrap_or(0))
        }
        MySQLTypeSystem::LongLong(_) => {
            let min_v: Option<i64> = row
                .get(0)
                .ok_or_else(|| anyhow!("mysql range: cannot get min value"))?;
            let max_v: Option<i64> = row
                .get(1)
                .ok_or_else(|| anyhow!("mysql range: cannot get max value"))?;
            (min_v.unwrap_or(0), max_v.unwrap_or(0))
        }
        _ => throw!(anyhow!("Partition can only be done on int columns")),
    };

    (min_v, max_v)
}

#[throws(ConnectorXPythonError)]
fn mssql_get_partition_range(conn: &Url, query: &str, col: &str) -> (i64, i64) {
    use tiberius::{AuthMethod, Client, Config, EncryptionLevel};
    use tokio::runtime::Runtime;
    let rt = Runtime::new().unwrap();
    let mut config = Config::new();

    config.host(conn.host_str().unwrap_or("localhost"));
    config.port(conn.port().unwrap_or(1433));
    config.authentication(AuthMethod::sql_server(
        conn.username(),
        conn.password().unwrap_or(""),
    ));

    config.database(&conn.path()[1..]); // remove the leading "/"
    config.encryption(EncryptionLevel::NotSupported);

    let tcp = rt.block_on(TcpStream::connect(config.get_addr()))?;
    tcp.set_nodelay(true)?;

    let mut client = rt.block_on(Client::connect(config, tcp.compat_write()))?;

    let range_query = get_partition_range_query(query, col, &MsSqlDialect {})?;
    let query_result = rt.block_on(client.query(range_query.as_str(), &[]))?;
    let row = rt.block_on(query_result.into_row())?.unwrap();

    let col_type = MsSQLTypeSystem::from(&row.columns()[0].column_type());
    let (min_v, max_v) = match col_type {
        MsSQLTypeSystem::Tinyint(_) => {
            let min_v: u8 = row.get(0).unwrap_or(0);
            let max_v: u8 = row.get(1).unwrap_or(0);
            (min_v as i64, max_v as i64)
        }
        MsSQLTypeSystem::Smallint(_) => {
            let min_v: i16 = row.get(0).unwrap_or(0);
            let max_v: i16 = row.get(1).unwrap_or(0);
            (min_v as i64, max_v as i64)
        }
        MsSQLTypeSystem::Int(_) => {
            let min_v: i32 = row.get(0).unwrap_or(0);
            let max_v: i32 = row.get(1).unwrap_or(0);
            (min_v as i64, max_v as i64)
        }
        MsSQLTypeSystem::Bigint(_) => {
            let min_v: i64 = row.get(0).unwrap_or(0);
            let max_v: i64 = row.get(1).unwrap_or(0);
            (min_v, max_v)
        }
        MsSQLTypeSystem::Intn(_) => {
            let min_v: IntN = row.get(0).unwrap_or(IntN(0));
            let max_v: IntN = row.get(1).unwrap_or(IntN(0));
            (min_v.0, max_v.0)
        }
        MsSQLTypeSystem::Float24(_) => {
            let min_v: f32 = row.get(0).unwrap_or(0.0);
            let max_v: f32 = row.get(1).unwrap_or(0.0);
            (min_v as i64, max_v as i64)
        }
        MsSQLTypeSystem::Float53(_) => {
            let min_v: f64 = row.get(0).unwrap_or(0.0);
            let max_v: f64 = row.get(1).unwrap_or(0.0);
            (min_v as i64, max_v as i64)
        }
        MsSQLTypeSystem::Floatn(_) => {
            let min_v: FloatN = row.get(0).unwrap_or(FloatN(0.0));
            let max_v: FloatN = row.get(1).unwrap_or(FloatN(0.0));
            (min_v.0 as i64, max_v.0 as i64)
        }
        _ => throw!(anyhow!(
            "Partition can only be done on int or float columns"
        )),
    };

    (min_v, max_v)
}

#[throws(ConnectorXPythonError)]
fn oracle_get_partition_range(conn: &Url, query: &str, col: &str) -> (i64, i64) {
    let conn = Url::parse(conn.as_str())?;
    let user = decode(conn.username())?.into_owned();
    let password = decode(conn.password().unwrap_or(""))?.into_owned();
    let host = "//".to_owned() + conn.host_str().unwrap_or("localhost") + conn.path();
    let conn = oracle_conn::connect(user.as_str(), password.as_str(), host)?;
    let range_query = get_partition_range_query(query, col, &OracleDialect {})?;
    let row = conn.query_row(range_query.as_str(), &[])?;
    let min_v: i64 = row.get(0).unwrap_or(0);
    let max_v: i64 = row.get(1).unwrap_or(0);
    (min_v, max_v)
}
