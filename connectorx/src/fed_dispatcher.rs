use crate::{
    constants::{J4RS_BASE_PATH, POSTGRES_JDBC_DRIVER},
    prelude::*,
    sources::postgres::{rewrite_tls_args, BinaryProtocol, PostgresSource},
    sql::CXQuery,
    transports::PostgresArrowTransport,
};
use arrow::record_batch::RecordBatch;
use datafusion::datasource::MemTable;
use datafusion::prelude::*;
use fehler::throws;
use j4rs::{ClasspathEntry, InvocationArg, Jvm, JvmBuilder};
use log::debug;
use postgres::NoTls;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::sync::Arc;
use url::Url;

struct Plan {
    db_name: String,
    db_alias: String,
    sql: String,
}

#[throws(ConnectorXError)]
fn init_jvm(j4rs_base: Option<&str>) -> Jvm {
    let base = match j4rs_base {
        Some(path) => fs::canonicalize(path)?,
        None => fs::canonicalize(J4RS_BASE_PATH)?,
    };
    let path = fs::canonicalize("../federated-rewriter.jar")?;
    let entry = ClasspathEntry::new(path.to_str().unwrap());
    JvmBuilder::new()
        .classpath_entry(entry)
        .with_base_path(base.to_str().unwrap())
        .build()?
}

#[throws(ConnectorXError)]
fn rewrite_sql(jvm: &Jvm, sql: &str, db_map: &HashMap<String, Url>) -> Vec<Plan> {
    let sql = InvocationArg::try_from(sql).unwrap();
    let db_conns = jvm.create_instance("java.util.HashMap", &[])?;
    for (db_name, url) in db_map.iter() {
        debug!("url: {:?}", url);
        let ds = jvm.invoke_static(
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
        )?;

        jvm.invoke(
            &db_conns,
            "put",
            &[
                InvocationArg::try_from(db_name).unwrap(),
                InvocationArg::try_from(ds).unwrap(),
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

#[throws(ConnectorXError)]
pub fn run(
    sql: String,
    db_map: HashMap<String, String>,
    j4rs_base: Option<&str>,
) -> Vec<RecordBatch> {
    debug!("federated input sql: {}", sql);

    let jvm = init_jvm(j4rs_base)?;

    let mut db_url_map: HashMap<String, Url> = HashMap::new();
    for (k, v) in db_map.into_iter() {
        db_url_map.insert(k, Url::parse(v.as_str())?);
    }

    let fed_plan = rewrite_sql(&jvm, sql.as_str(), &db_url_map)?;

    // let ctx = SessionContext::new();
    let mut ctx = ExecutionContext::new();
    let mut local_sql = String::new();
    let mut alias_names = vec![];
    for p in fed_plan {
        match p.db_name.as_str() {
            "LOCAL" => local_sql = p.sql,
            _ => {
                let mut destination = ArrowDestination::new();
                let queries = [CXQuery::naked(p.sql)];
                let url = &db_url_map[p.db_name.as_str()];
                let (config, _) = rewrite_tls_args(&url).expect("postgres config error");
                let sb = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 1)
                    .expect("postgres init error");
                let dispatcher =
                    Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
                        sb,
                        &mut destination,
                        &queries,
                        None,
                    );
                dispatcher.run().expect("run dispatcher fails");
                let rbs = destination.arrow().expect("get arrow fails");
                let provider = MemTable::try_new(rbs[0].schema(), vec![rbs])?;
                ctx.register_table(p.db_alias.as_str(), Arc::new(provider))?;
                alias_names.push(p.db_alias);
            }
        }
    }

    debug!("\nexecute query final:");
    let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
    // until datafusion fix the bug: https://github.com/apache/arrow-datafusion/issues/2147
    for alias in alias_names {
        local_sql = local_sql.replace(format!("\"{}\"", alias).as_str(), alias.as_str());
    }

    let df = rt.block_on(ctx.sql(local_sql.as_str()))?;
    rt.block_on(df.collect())?
}
