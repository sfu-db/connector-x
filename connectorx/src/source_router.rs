use crate::errors::{ConnectorAgentError, Result};
use crate::sources::postgres::PostgresTypeSystem;
use crate::sql::{
    get_partition_range_query, get_partition_range_query_sep, single_col_partition_query,
};
use anyhow::anyhow;
use fehler::{throw, throws};
use postgres::{Client, NoTls};
use rusqlite::{types::Type, Connection};
use sqlparser::dialect::{PostgreSqlDialect, SQLiteDialect};
use std::convert::TryFrom;
use url::Url;

pub enum SourceType {
    Postgres,
    Sqlite,
}

pub struct SourceConn {
    pub ty: SourceType,
    pub conn: String,
}

impl TryFrom<&str> for SourceConn {
    type Error = ConnectorAgentError;

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
            _ => unimplemented!("Connection: {} not supported!", conn),
        }
    }
}

impl SourceType {
    pub fn get_col_range(&self, conn: &str, query: &str, col: &str) -> Result<(i64, i64)> {
        match *self {
            SourceType::Postgres => pg_get_partition_range(conn, query, col),
            SourceType::Sqlite => sqlite_get_partition_range(conn, query, col),
        }
    }

    pub fn get_part_query(&self, query: &str, col: &str, lower: i64, upper: i64) -> Result<String> {
        match *self {
            SourceType::Postgres => {
                single_col_partition_query(query, col, lower, upper, &PostgreSqlDialect {})
            }
            SourceType::Sqlite => {
                single_col_partition_query(query, col, lower, upper, &SQLiteDialect {})
            }
        }
    }
}

#[throws(ConnectorAgentError)]
fn pg_get_partition_range(conn: &str, query: &str, col: &str) -> (i64, i64) {
    let mut client = Client::connect(conn, NoTls)?;
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

#[throws(ConnectorAgentError)]
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
