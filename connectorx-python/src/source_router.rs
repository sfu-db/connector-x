use crate::errors::{ConnectorXPythonError, Result};
use anyhow::anyhow;
use connectorx::{
    sources::{
        mysql::MySQLTypeSystem,
        postgres::{rewrite_tls_args, PostgresTypeSystem},
    },
    sql::{
        get_partition_range_query, get_partition_range_query_sep, single_col_partition_query,
        CXQuery,
    },
};
use fehler::{throw, throws};
use r2d2_mysql::mysql::{prelude::Queryable, Pool, Row};
use rusqlite::{types::Type, Connection};
use sqlparser::dialect::MySqlDialect;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::dialect::SQLiteDialect;
use std::convert::TryFrom;
use url::Url;

pub enum SourceType {
    Postgres,
    Sqlite,
    Mysql,
}

pub struct SourceConn {
    pub ty: SourceType,
    pub conn: String,
}

impl TryFrom<&str> for SourceConn {
    type Error = ConnectorXPythonError;

    fn try_from(conn: &str) -> Result<SourceConn> {
        let url = Url::parse(conn).map_err(|e| anyhow!("parse error: {}", e))?;
        match url.scheme() {
            "postgres" | "postgresql" => Ok(SourceConn {
                ty: SourceType::Postgres,
                conn: conn.into(),
            }),
            "sqlite" => Ok(SourceConn {
                ty: SourceType::Sqlite,
                conn: conn[9..].into(),
            }),
            "mysql" => Ok(SourceConn {
                ty: SourceType::Mysql,
                conn: conn.into(),
            }),
            _ => unimplemented!("Connection: {} not supported!", conn),
        }
    }
}

impl SourceType {
    pub fn get_col_range(&self, conn: &str, query: &str, col: &str) -> Result<(i64, i64)> {
        match self {
            SourceType::Postgres => pg_get_partition_range(conn, query, col),
            SourceType::Sqlite => sqlite_get_partition_range(conn, query, col),
            SourceType::Mysql => mysql_get_partition_range(conn, query, col),
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
        let query = match self {
            SourceType::Postgres => {
                single_col_partition_query(query, col, lower, upper, &PostgreSqlDialect {})?
            }
            SourceType::Sqlite => {
                single_col_partition_query(query, col, lower, upper, &SQLiteDialect {})?
            }
            SourceType::Mysql => {
                single_col_partition_query(query, col, lower, upper, &MySqlDialect {})?
            }
        };

        CXQuery::Wrapped(query)
    }
}

#[throws(ConnectorXPythonError)]
fn pg_get_partition_range(conn: &str, query: &str, col: &str) -> (i64, i64) {
    let (config, tls) = rewrite_tls_args(conn)?;
    let mut client = config.connect(tls.unwrap())?;
    let range_query = get_partition_range_query(query, col, &PostgreSqlDialect {})?;
    let row = client.query_one(range_query.as_str(), &[])?;

    let col_type = PostgresTypeSystem::from(row.columns()[0].type_());
    let (min_v, max_v) = match col_type {
        PostgresTypeSystem::Int4(_) => {
            let min_v: i32 = row.get(0);
            let max_v: i32 = row.get(1);
            (min_v as i64, max_v as i64)
        }
        PostgresTypeSystem::Int8(_) => {
            let min_v: i64 = row.get(0);
            let max_v: i64 = row.get(1);
            (min_v, max_v)
        }
        PostgresTypeSystem::Float4(_) => {
            let min_v: f32 = row.get(0);
            let max_v: f32 = row.get(1);
            (min_v as i64, max_v as i64)
        }
        PostgresTypeSystem::Float8(_) => {
            let min_v: f64 = row.get(0);
            let max_v: f64 = row.get(1);
            (min_v as i64, max_v as i64)
        }
        _ => throw!(anyhow!(
            "Partition can only be done on int or float columns"
        )),
    };

    (min_v, max_v)
}

#[throws(ConnectorXPythonError)]
fn sqlite_get_partition_range(conn: &str, query: &str, col: &str) -> (i64, i64) {
    let conn = Connection::open(&conn[9..])?;
    // SQLite only optimize min max queries when there is only one aggregation
    // https://www.sqlite.org/optoverview.html#minmax
    let (min_query, max_query) = get_partition_range_query_sep(query, col, &SQLiteDialect {})?;
    let mut error = None;
    let min_v = conn.query_row(min_query.as_str(), [], |row| {
        // declare type for count query will be None, only need to check the returned value type
        let col_type = row.get_ref(0)?.data_type();
        match col_type {
            Type::Integer => row.get(0),
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
fn mysql_get_partition_range(conn: &str, query: &str, col: &str) -> (i64, i64) {
    let pool = Pool::new(conn)?;
    let mut conn = pool.get_conn()?;
    let range_query = get_partition_range_query(query, col, &MySqlDialect {})?;
    let row: Row = conn
        .query_first(range_query)?
        .ok_or_else(|| anyhow!("mysql range: no row returns"))?;

    let col_type = MySQLTypeSystem::from(&row.columns()[0].column_type());
    let (min_v, max_v) = match col_type {
        MySQLTypeSystem::Long(_) => {
            let min_v: i64 = row
                .get(0)
                .ok_or_else(|| anyhow!("mysql range: cannot get min value"))?;
            let max_v: i64 = row
                .get(1)
                .ok_or_else(|| anyhow!("mysql range: cannot get max value"))?;
            (min_v, max_v)
        }
        MySQLTypeSystem::LongLong(_) => {
            let min_v: i64 = row
                .get(0)
                .ok_or_else(|| anyhow!("mysql range: cannot get min value"))?;
            let max_v: i64 = row
                .get(1)
                .ok_or_else(|| anyhow!("mysql range: cannot get max value"))?;
            (min_v, max_v)
        }
        _ => throw!(anyhow!("Partition can only be done on int columns")),
    };

    (min_v, max_v)
}
