use crate::{
    constants::{
        CX_REWRITER_PATH, DUCKDB_JDBC_DRIVER, J4RS_BASE_PATH, MYSQL_JDBC_DRIVER,
        POSTGRES_JDBC_DRIVER,
    },
    prelude::*,
    sql::CXQuery,
    CXFederatedPlan, CXSlice,
};
use arrow::record_batch::RecordBatch;
use datafusion::datasource::MemTable;
use datafusion::prelude::*;
use fehler::throws;
use j4rs::{ClasspathEntry, InvocationArg, Jvm, JvmBuilder, Null};
use log::debug;
use rayon::prelude::*;
use std::collections::HashMap;
use std::convert::{Into, TryFrom};
use std::sync::{mpsc::channel, Arc};
use std::{env, fs};

pub struct Plan {
    db_name: String,
    db_alias: String,
    sql: String,
}

impl Into<CXFederatedPlan> for Plan {
    fn into(mut self) -> CXFederatedPlan {
        self.db_name.shrink_to_fit();
        self.db_alias.shrink_to_fit();
        self.sql.shrink_to_fit();
        let (data1, len1, _) = self.db_name.into_raw_parts();
        let (data2, len2, _) = self.db_alias.into_raw_parts();
        let (data3, len3, _) = self.sql.into_raw_parts();
        CXFederatedPlan {
            db_name: CXSlice::<u8> {
                data: data1,
                len: len1,
            },
            db_alias: CXSlice::<u8> {
                data: data2,
                len: len2,
            },
            sql: CXSlice::<u8> {
                data: data3,
                len: len3,
            },
        }
    }
}

#[throws(ConnectorXOutError)]
fn init_jvm(j4rs_base: Option<&str>) -> Jvm {
    let base = match j4rs_base {
        Some(path) => fs::canonicalize(path)
            .map_err(|_| ConnectorXOutError::FileNotFoundError(path.to_string()))?,
        None => fs::canonicalize(J4RS_BASE_PATH)
            .map_err(|_| ConnectorXOutError::FileNotFoundError(J4RS_BASE_PATH.to_string()))?,
    };
    debug!("j4rs base path: {:?}", base);

    let rewriter_path = env::var("CX_REWRITER_PATH").unwrap_or(CX_REWRITER_PATH.to_string());
    let path = fs::canonicalize(rewriter_path.as_str())
        .map_err(|_| ConnectorXOutError::FileNotFoundError(rewriter_path))?;

    debug!("rewriter path: {:?}", path);

    let entry = ClasspathEntry::new(path.to_str().unwrap());
    JvmBuilder::new()
        .skip_setting_native_lib()
        .classpath_entry(entry)
        .with_base_path(base.to_str().unwrap())
        .build()?
}

#[throws(ConnectorXOutError)]
pub fn rewrite_sql(
    sql: &str,
    db_map: &HashMap<String, SourceConn>,
    j4rs_base: Option<&str>,
) -> Vec<Plan> {
    let jvm = init_jvm(j4rs_base)?;
    debug!("init jvm successfully!");

    let sql = InvocationArg::try_from(sql).unwrap();
    let db_conns = jvm.create_instance("java.util.HashMap", &[])?;
    for (db_name, source_conn) in db_map.iter() {
        let url = &source_conn.conn;
        debug!("url: {:?}", url);
        let ds = match source_conn.ty {
            SourceType::Postgres => jvm.invoke_static(
                "org.apache.calcite.adapter.jdbc.JdbcSchema",
                "dataSource",
                &[
                    InvocationArg::try_from(format!(
                        "jdbc:postgresql://{}:{}{}",
                        url.host_str().unwrap_or("localhost"),
                        url.port().unwrap_or(5432),
                        url.path()
                    ))
                    .unwrap(),
                    InvocationArg::try_from(POSTGRES_JDBC_DRIVER).unwrap(),
                    InvocationArg::try_from(url.username()).unwrap(),
                    InvocationArg::try_from(url.password().unwrap_or("")).unwrap(),
                ],
            )?,
            SourceType::MySQL => jvm.invoke_static(
                "org.apache.calcite.adapter.jdbc.JdbcSchema",
                "dataSource",
                &[
                    InvocationArg::try_from(format!(
                        "jdbc:mysql://{}:{}{}",
                        url.host_str().unwrap_or("localhost"),
                        url.port().unwrap_or(3306),
                        url.path()
                    ))
                    .unwrap(),
                    InvocationArg::try_from(MYSQL_JDBC_DRIVER).unwrap(),
                    InvocationArg::try_from(url.username()).unwrap(),
                    InvocationArg::try_from(url.password().unwrap_or("")).unwrap(),
                ],
            )?,
            SourceType::DuckDB => jvm.invoke_static(
                "org.apache.calcite.adapter.jdbc.JdbcSchema",
                "dataSource",
                &[
                    InvocationArg::try_from(format!("jdbc:duckdb:{}", url.path())).unwrap(),
                    InvocationArg::try_from(DUCKDB_JDBC_DRIVER).unwrap(),
                    InvocationArg::try_from(Null::String).unwrap(),
                    InvocationArg::try_from(Null::String).unwrap(),
                ],
            )?,
            _ => unimplemented!("Connection: {:?} not supported!", url),
        };
        let fed_ds = jvm.create_instance(
            "ai.dataprep.federated.FederatedDataSource",
            &[
                InvocationArg::try_from(ds).unwrap(),
                InvocationArg::try_from(source_conn.local).unwrap(),
            ],
        )?;
        jvm.invoke(
            &db_conns,
            "put",
            &[
                InvocationArg::try_from(db_name).unwrap(),
                InvocationArg::try_from(fed_ds).unwrap(),
            ],
        )?;
    }

    let rewriter = jvm.create_instance("ai.dataprep.federated.FederatedQueryRewriter", &[])?;
    let db_conns = InvocationArg::try_from(db_conns).unwrap();
    let plan = jvm.invoke(&rewriter, "rewrite", &[db_conns, sql])?;

    let count = jvm.invoke(&plan, "getCount", &[])?;
    let count: i32 = jvm.to_rust(count)?;
    debug!("rewrite finished, got {} queries", count);

    let mut fed_plan = vec![];
    for i in 0..count {
        let db = jvm.invoke(
            &plan,
            "getDBName",
            &[InvocationArg::try_from(i).unwrap().into_primitive()?],
        )?;
        let db: String = jvm.to_rust(db)?;

        let alias_db = jvm.invoke(
            &plan,
            "getAliasDBName",
            &[InvocationArg::try_from(i).unwrap().into_primitive()?],
        )?;
        let alias_db: String = jvm.to_rust(alias_db)?;

        let rewrite_sql = jvm.invoke(
            &plan,
            "getSql",
            &[InvocationArg::try_from(i).unwrap().into_primitive()?],
        )?;
        let rewrite_sql: String = jvm.to_rust(rewrite_sql)?;
        debug!(
            "{} - db: {}, alias: {} rewrite sql: {}",
            i, db, alias_db, rewrite_sql
        );
        fed_plan.push(Plan {
            db_name: db,
            db_alias: alias_db,
            sql: rewrite_sql,
        });
    }
    fed_plan
}

#[throws(ConnectorXOutError)]
pub fn run(
    sql: String,
    db_map: HashMap<String, String>,
    j4rs_base: Option<&str>,
) -> Vec<RecordBatch> {
    debug!("federated input sql: {}", sql);
    let mut db_conn_map: HashMap<String, SourceConn> = HashMap::new();
    for (k, v) in db_map.into_iter() {
        db_conn_map.insert(k, SourceConn::try_from(v.as_str())?);
    }
    let fed_plan = rewrite_sql(sql.as_str(), &db_conn_map, j4rs_base)?;

    debug!("fetch queries from remote");
    let (sender, receiver) = channel();
    fed_plan.into_par_iter().enumerate().try_for_each_with(
        sender,
        |s, (i, p)| -> Result<(), ConnectorXOutError> {
            match p.db_name.as_str() {
                "LOCAL" => {
                    s.send((p.sql, None)).expect("send error local");
                }
                _ => {
                    debug!("start query {}: {}", i, p.sql);
                    let queries = [CXQuery::naked(p.sql)];
                    let source_conn = &db_conn_map[p.db_name.as_str()];

                    let destination = get_arrow(source_conn, None, &queries)?;
                    let rbs = destination.arrow()?;

                    let provider = MemTable::try_new(rbs[0].schema(), vec![rbs])?;
                    s.send((p.db_alias, Some(Arc::new(provider))))
                        .expect(&format!("send error {}", i));
                    debug!("query {} finished", i);
                }
            }
            Ok(())
        },
    )?;

    let ctx = SessionContext::new();
    let mut alias_names: Vec<String> = vec![];
    let mut local_sql = String::new();
    receiver
        .iter()
        .try_for_each(|(alias, provider)| -> Result<(), ConnectorXOutError> {
            match provider {
                Some(p) => {
                    ctx.register_table(alias.as_str(), p)?;
                    alias_names.push(alias);
                }
                None => local_sql = alias,
            }

            Ok(())
        })?;

    debug!("\nexecute query final...");
    let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
    // until datafusion fix the bug: https://github.com/apache/arrow-datafusion/issues/2147
    for alias in alias_names {
        local_sql = local_sql.replace(format!("\"{}\"", alias).as_str(), alias.as_str());
    }

    let df = rt.block_on(ctx.sql(local_sql.as_str()))?;
    rt.block_on(df.collect())?
}
