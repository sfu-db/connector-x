use crate::{
    constants::{
        CX_REWRITER_PATH, DUCKDB_JDBC_DRIVER, J4RS_BASE_PATH, MYSQL_JDBC_DRIVER,
        POSTGRES_JDBC_DRIVER,
    },
    prelude::*,
};
use fehler::throws;
use j4rs::{ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder, Null};
use log::debug;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::{env, fs};

pub struct Plan {
    pub db_name: String,
    pub db_alias: String,
    pub sql: String,
    pub cardinality: usize,
}

pub struct FederatedDataSourceInfo {
    pub conn_str_info: Option<SourceConn>,
    pub manual_info: Option<HashMap<String, Vec<String>>>,
    pub is_local: bool,
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
            let ds = match source_conn.ty {
                SourceType::Postgres => jvm.invoke_static(
                    "ai.dataprep.federated.FederatedQueryRewriter",
                    "createDataSource",
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
                    "ai.dataprep.federated.FederatedQueryRewriter",
                    "createDataSource",
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
                    "ai.dataprep.federated.FederatedQueryRewriter",
                    "createDataSource",
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
                    InvocationArg::try_from(url.scheme()).unwrap(),
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
                let arr_instance = jvm.java_list("java.lang.String", columns.to_vec())?;
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
                &[
                    InvocationArg::try_from("local").unwrap(),
                    InvocationArg::try_from(schema_info).unwrap(),
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

    let sql = InvocationArg::try_from(sql).unwrap();
    let data_sources = create_sources(&jvm, db_map)?;
    let rewriter = jvm.create_instance("ai.dataprep.federated.FederatedQueryRewriter", &[])?;
    let data_sources = InvocationArg::try_from(data_sources).unwrap();
    let plan = jvm.invoke(&rewriter, "rewrite", &[data_sources, sql])?;

    let count = jvm.invoke(&plan, "getCount", &[])?;
    let count: i32 = jvm.to_rust(count)?;
    debug!("rewrite finished, got {} queries", count);

    let mut fed_plan = vec![];
    for i in 0..count {
        let idx = [InvocationArg::try_from(i).unwrap().into_primitive()?];

        let db = jvm.invoke(&plan, "getDBName", &idx)?;
        let db: String = jvm.to_rust(db)?;

        let alias_db = jvm.invoke(&plan, "getAliasDBName", &idx)?;
        let alias_db: String = jvm.to_rust(alias_db)?;

        let rewrite_sql = jvm.invoke(&plan, "getSql", &idx)?;
        let rewrite_sql: String = jvm.to_rust(rewrite_sql)?;

        let cardinality = jvm.invoke(&plan, "getCardinality", &idx)?;
        let cardinality: usize = jvm.to_rust(cardinality)?;

        debug!(
            "{} - db: {}, alias: {}, cardinality: {}, rewrite sql: {}",
            i, db, alias_db, cardinality, rewrite_sql
        );
        fed_plan.push(Plan {
            db_name: db,
            db_alias: alias_db,
            sql: rewrite_sql,
            cardinality,
        });
    }
    fed_plan
}
