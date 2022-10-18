use crate::errors::{ConnectorXPythonError, Result};
use anyhow::anyhow;
use connectorx::source_router::{SourceConn, SourceType};
use connectorx::{
    sources::{
        bigquery::BigQueryDialect,
        mssql::{mssql_config, FloatN, IntN, MsSQLTypeSystem},
        mysql::{MySQLSourceError, MySQLTypeSystem},
        oracle::{connect_oracle, OracleDialect},
        postgres::{rewrite_tls_args, PostgresTypeSystem},
    },
    sql::{
        get_partition_range_query, get_partition_range_query_sep, single_col_partition_query,
        CXQuery,
    },
};
use fehler::{throw, throws};
use gcp_bigquery_client;
use r2d2_mysql::mysql::{prelude::Queryable, Opts, Pool, Row};
use rusqlite::{types::Type, Connection};
use rust_decimal::{prelude::ToPrimitive, Decimal};
use rust_decimal_macros::dec;
use sqlparser::dialect::{MsSqlDialect, MySqlDialect, PostgreSqlDialect, SQLiteDialect};
use std::convert::TryFrom;
use tiberius::Client;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use url::Url;

#[throws(ConnectorXPythonError)]
pub fn parse_source(conn: &str, protocol: Option<&str>) -> SourceConn {
    let mut source_conn = SourceConn::try_from(conn)?;
    match protocol {
        Some(p) => source_conn.set_protocol(p),
        None => {}
    }
    source_conn
}

pub fn get_col_range(source_conn: &SourceConn, query: &str, col: &str) -> Result<(i64, i64)> {
    match source_conn.ty {
        SourceType::Postgres => pg_get_partition_range(&source_conn.conn, query, col),
        SourceType::SQLite => sqlite_get_partition_range(&source_conn.conn, query, col),
        SourceType::MySQL => mysql_get_partition_range(&source_conn.conn, query, col),
        SourceType::MsSQL => mssql_get_partition_range(&source_conn.conn, query, col),
        SourceType::Oracle => oracle_get_partition_range(&source_conn.conn, query, col),
        SourceType::BigQuery => bigquery_get_partition_range(&source_conn.conn, query, col),
        _ => unimplemented!("{:?} not implemented!", source_conn.ty),
    }
}

#[throws(ConnectorXPythonError)]
pub fn get_part_query(
    source_conn: &SourceConn,
    query: &str,
    col: &str,
    lower: i64,
    upper: i64,
) -> CXQuery<String> {
    let query = match source_conn.ty {
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
        SourceType::BigQuery => {
            single_col_partition_query(query, col, lower, upper, &BigQueryDialect {})?
        }
        _ => unimplemented!("{:?} not implemented!", source_conn.ty),
    };
    CXQuery::Wrapped(query)
}

#[throws(ConnectorXPythonError)]
fn pg_get_partition_range(conn: &Url, query: &str, col: &str) -> (i64, i64) {
    let (config, tls) = rewrite_tls_args(conn)?;
    let mut client = match tls {
        None => config.connect(postgres::NoTls)?,
        Some(tls_conn) => config.connect(tls_conn)?,
    };
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
        PostgresTypeSystem::Numeric(_) => {
            let min_v: Option<Decimal> = row.get(0);
            let max_v: Option<Decimal> = row.get(1);
            (
                min_v.unwrap_or(dec!(0.0)).to_i64().unwrap_or(0),
                max_v.unwrap_or(dec!(0.0)).to_i64().unwrap_or(0),
            )
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
            Type::Real => {
                let v: f64 = row.get(0)?;
                Ok(v as i64)
            }
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
            Type::Real => {
                let v: f64 = row.get(0)?;
                Ok(v as i64)
            }
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

    let col_type =
        MySQLTypeSystem::from((&row.columns()[0].column_type(), &row.columns()[0].flags()));

    let (min_v, max_v) = match col_type {
        MySQLTypeSystem::Tiny(_) => {
            let min_v: Option<i8> = row
                .get(0)
                .ok_or_else(|| anyhow!("mysql range: cannot get min value"))?;
            let max_v: Option<i8> = row
                .get(1)
                .ok_or_else(|| anyhow!("mysql range: cannot get max value"))?;
            (min_v.unwrap_or(0) as i64, max_v.unwrap_or(0) as i64)
        }
        MySQLTypeSystem::Short(_) => {
            let min_v: Option<i16> = row
                .get(0)
                .ok_or_else(|| anyhow!("mysql range: cannot get min value"))?;
            let max_v: Option<i16> = row
                .get(1)
                .ok_or_else(|| anyhow!("mysql range: cannot get max value"))?;
            (min_v.unwrap_or(0) as i64, max_v.unwrap_or(0) as i64)
        }
        MySQLTypeSystem::Int24(_) => {
            let min_v: Option<i32> = row
                .get(0)
                .ok_or_else(|| anyhow!("mysql range: cannot get min value"))?;
            let max_v: Option<i32> = row
                .get(1)
                .ok_or_else(|| anyhow!("mysql range: cannot get max value"))?;
            (min_v.unwrap_or(0) as i64, max_v.unwrap_or(0) as i64)
        }
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
        MySQLTypeSystem::UTiny(_) => {
            let min_v: Option<u8> = row
                .get(0)
                .ok_or_else(|| anyhow!("mysql range: cannot get min value"))?;
            let max_v: Option<u8> = row
                .get(1)
                .ok_or_else(|| anyhow!("mysql range: cannot get max value"))?;
            (min_v.unwrap_or(0) as i64, max_v.unwrap_or(0) as i64)
        }
        MySQLTypeSystem::UShort(_) => {
            let min_v: Option<u16> = row
                .get(0)
                .ok_or_else(|| anyhow!("mysql range: cannot get min value"))?;
            let max_v: Option<u16> = row
                .get(1)
                .ok_or_else(|| anyhow!("mysql range: cannot get max value"))?;
            (min_v.unwrap_or(0) as i64, max_v.unwrap_or(0) as i64)
        }
        MySQLTypeSystem::UInt24(_) => {
            let min_v: Option<u32> = row
                .get(0)
                .ok_or_else(|| anyhow!("mysql range: cannot get min value"))?;
            let max_v: Option<u32> = row
                .get(1)
                .ok_or_else(|| anyhow!("mysql range: cannot get max value"))?;
            (min_v.unwrap_or(0) as i64, max_v.unwrap_or(0) as i64)
        }
        MySQLTypeSystem::ULong(_) => {
            let min_v: Option<u32> = row
                .get(0)
                .ok_or_else(|| anyhow!("mysql range: cannot get min value"))?;
            let max_v: Option<u32> = row
                .get(1)
                .ok_or_else(|| anyhow!("mysql range: cannot get max value"))?;
            (min_v.unwrap_or(0) as i64, max_v.unwrap_or(0) as i64)
        }
        MySQLTypeSystem::ULongLong(_) => {
            let min_v: Option<u64> = row
                .get(0)
                .ok_or_else(|| anyhow!("mysql range: cannot get min value"))?;
            let max_v: Option<u64> = row
                .get(1)
                .ok_or_else(|| anyhow!("mysql range: cannot get max value"))?;
            (min_v.unwrap_or(0) as i64, max_v.unwrap_or(0) as i64)
        }
        MySQLTypeSystem::Float(_) => {
            let min_v: Option<f32> = row
                .get(0)
                .ok_or_else(|| anyhow!("mysql range: cannot get min value"))?;
            let max_v: Option<f32> = row
                .get(1)
                .ok_or_else(|| anyhow!("mysql range: cannot get max value"))?;
            (min_v.unwrap_or(0.0) as i64, max_v.unwrap_or(0.0) as i64)
        }
        MySQLTypeSystem::Double(_) => {
            let min_v: Option<f64> = row
                .get(0)
                .ok_or_else(|| anyhow!("mysql range: cannot get min value"))?;
            let max_v: Option<f64> = row
                .get(1)
                .ok_or_else(|| anyhow!("mysql range: cannot get max value"))?;
            (min_v.unwrap_or(0.0) as i64, max_v.unwrap_or(0.0) as i64)
        }
        _ => throw!(anyhow!("Partition can only be done on int columns")),
    };

    (min_v, max_v)
}

#[throws(ConnectorXPythonError)]
fn mssql_get_partition_range(conn: &Url, query: &str, col: &str) -> (i64, i64) {
    let rt = Runtime::new().expect("Failed to create runtime");
    let config = mssql_config(conn)?;
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
    let connector = connect_oracle(conn)?;
    let conn = connector.connect()?;
    let range_query = get_partition_range_query(query, col, &OracleDialect {})?;
    let row = conn.query_row(range_query.as_str(), &[])?;
    let min_v: i64 = row.get(0).unwrap_or(0);
    let max_v: i64 = row.get(1).unwrap_or(0);
    (min_v, max_v)
}

#[throws(ConnectorXPythonError)] // TODO
fn bigquery_get_partition_range(conn: &Url, query: &str, col: &str) -> (i64, i64) {
    let rt = Runtime::new().expect("Failed to create runtime");
    let url = Url::parse(conn.as_str())?;
    let sa_key_path = url.path();
    let client = rt.block_on(gcp_bigquery_client::Client::from_service_account_key_file(
        sa_key_path,
    ));

    let auth_data = std::fs::read_to_string(sa_key_path)?;
    let auth_json: serde_json::Value = serde_json::from_str(&auth_data)?;
    let project_id = auth_json
        .get("project_id")
        .ok_or_else(|| anyhow!("Cannot get project_id from auth file"))?
        .as_str()
        .ok_or_else(|| anyhow!("Cannot get project_id as string from auth file"))?;
    let range_query = get_partition_range_query(query, col, &BigQueryDialect {})?;

    let mut query_result = rt.block_on(client.job().query(
        project_id,
        gcp_bigquery_client::model::query_request::QueryRequest::new(range_query.as_str()),
    ))?;
    query_result.next_row();
    let min_v = query_result.get_i64(0)?.unwrap_or(0);
    let max_v = query_result.get_i64(1)?.unwrap_or(0);

    (min_v, max_v)
}
