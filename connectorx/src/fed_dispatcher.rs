use crate::{
    constants::{
        CX_REWRITER_PATH, DUCKDB_JDBC_DRIVER, J4RS_BASE_PATH, MYSQL_JDBC_DRIVER,
        POSTGRES_JDBC_DRIVER,
    },
    prelude::*,
    sql::CXQuery,
    CXFederatedPlan,
};
use arrow::record_batch::RecordBatch;
use datafusion::datasource::MemTable;
use datafusion::prelude::*;
use fehler::throws;
use j4rs::{ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder, Null};
use libc::c_char;
use log::debug;
use rayon::prelude::*;
use std::collections::HashMap;
use std::convert::{Into, TryFrom};
use std::ffi::CString;
use std::sync::{mpsc::channel, Arc};
use std::{env, fs};

pub struct FederatedDataSourceInfo {
    conn_str_info: Option<SourceConn>,
    manual_info: Option<HashMap<String, Vec<String>>>,
    is_local: bool,
}

impl FederatedDataSourceInfo {
    pub fn new_from_conn_str(source_conn: SourceConn, is_local: bool) -> Self {
        Self {
            conn_str_info: Some(source_conn),
            manual_info: None,
            is_local,
        }
    }
    pub fn new_from_manual_schema(
        manual_schema: HashMap<String, Vec<String>>,
        is_local: bool,
    ) -> Self {
        Self {
            conn_str_info: None,
            manual_info: Some(manual_schema),
            is_local,
        }
    }
}

pub struct Plan {
    db_name: String,
    db_alias: String,
    sql: String,
}

impl Into<CXFederatedPlan> for Plan {
    fn into(self) -> CXFederatedPlan {
        CXFederatedPlan {
            db_name: CString::new(self.db_name.as_str())
                .expect("new CString error")
                .into_raw() as *const c_char,
            db_alias: CString::new(self.db_alias.as_str())
                .expect("new CString error")
                .into_raw() as *const c_char,
            sql: CString::new(self.sql.as_str())
                .expect("new CString error")
                .into_raw() as *const c_char,
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
fn create_sources(jvm: &Jvm, db_map: &HashMap<String, FederatedDataSourceInfo>) -> Instance {
    let data_sources = jvm.create_instance("java.util.HashMap", &[])?;

    for (db_name, db_info) in db_map.iter() {
        if db_info.conn_str_info.is_some() {
            let source_conn = db_info.conn_str_info.as_ref().unwrap();
            let url = &source_conn.conn;
            debug!("url: {:?}", url);
            println!("url: {:?}", url);
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
                    InvocationArg::try_from(db_info.is_local).unwrap(),
                ],
            )?;
            jvm.invoke(
                &data_sources,
                "put",
                &[
                    InvocationArg::try_from(db_name).unwrap(),
                    InvocationArg::try_from(fed_ds).unwrap(),
                ],
            )?;
        } else {
            assert!(db_info.manual_info.is_some());
            let manual_info = db_info.manual_info.as_ref().unwrap();
            let schema_info = jvm.create_instance("java.util.HashMap", &[])?;
            for (name, columns) in manual_info {
                let col_names: Vec<InvocationArg> = columns
                    .iter()
                    .map(|c| InvocationArg::try_from(c).unwrap())
                    .collect();
                let arr_instance = jvm.create_java_list("java.lang.String", &col_names)?;
                jvm.invoke(
                    &schema_info,
                    "put",
                    &[
                        InvocationArg::try_from(name).unwrap(),
                        InvocationArg::try_from(arr_instance).unwrap(),
                    ],
                )?;
            }
            let fed_ds = jvm.create_instance(
                "ai.dataprep.federated.FederatedDataSource",
                &[InvocationArg::try_from(schema_info).unwrap()],
            )?;
            jvm.invoke(
                &data_sources,
                "put",
                &[
                    InvocationArg::try_from(db_name).unwrap(),
                    InvocationArg::try_from(fed_ds).unwrap(),
                ],
            )?;
        }
    }
    data_sources
}

#[throws(ConnectorXOutError)]
pub fn rewrite_sql(
    sql: &str,
    db_map: &HashMap<String, FederatedDataSourceInfo>,
    j4rs_base: Option<&str>,
) -> Vec<Plan> {
    let jvm = init_jvm(j4rs_base)?;
    debug!("init jvm successfully!");
    println!("init jvm successfully!");

    let sql = InvocationArg::try_from(sql).unwrap();
    let data_sources = create_sources(&jvm, db_map)?;
    let rewriter = jvm.create_instance("ai.dataprep.federated.FederatedQueryRewriter", &[])?;
    let data_sources = InvocationArg::try_from(data_sources).unwrap();
    let plan = jvm.invoke(&rewriter, "rewrite", &[data_sources, sql])?;

    let count = jvm.invoke(&plan, "getCount", &[])?;
    let count: i32 = jvm.to_rust(count)?;
    debug!("rewrite finished, got {} queries", count);
    println!("rewrite finished, got {} queries", count);

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
    let mut db_conn_map: HashMap<String, FederatedDataSourceInfo> = HashMap::new();
    for (k, v) in db_map.into_iter() {
        db_conn_map.insert(
            k,
            FederatedDataSourceInfo::new_from_conn_str(SourceConn::try_from(v.as_str())?, false),
        );
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
                    let source_conn = &db_conn_map[p.db_name.as_str()]
                        .conn_str_info
                        .as_ref()
                        .unwrap();

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
