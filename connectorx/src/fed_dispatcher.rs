use crate::{
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
use std::env;
use std::fs;
use std::sync::Arc;
use url::Url;

#[throws(ConnectorXError)]
pub fn run(sql: String, db_map: HashMap<String, String>) -> Vec<RecordBatch> {
    debug!("federated input sql: {}", sql);

    let base = fs::canonicalize("./target/release").unwrap();
    let path = fs::canonicalize("./federated-rewriter.jar").unwrap();
    let entry = ClasspathEntry::new(path.to_str().unwrap());
    // let jvm: Jvm = JvmBuilder::new().classpath_entry(entry).build().unwrap();
    let jvm: Jvm = JvmBuilder::new()
        .classpath_entry(entry)
        .with_base_path(base.to_str().unwrap())
        .build()
        .unwrap();

    let sql = InvocationArg::try_from(sql).unwrap();
    let ds1 = jvm
        .invoke_static(
            "org.apache.calcite.adapter.jdbc.JdbcSchema",
            "dataSource",
            &[
                InvocationArg::try_from(env::var("DB1_JDBC_URL").unwrap()).unwrap(),
                InvocationArg::try_from(env::var("DB1_JDBC_DRIVER").unwrap()).unwrap(),
                InvocationArg::try_from(env::var("DB1_USER").unwrap()).unwrap(),
                InvocationArg::try_from(env::var("DB1_PASSWORD").unwrap()).unwrap(),
            ],
        )
        .unwrap();

    let ds2 = jvm
        .invoke_static(
            "org.apache.calcite.adapter.jdbc.JdbcSchema",
            "dataSource",
            &[
                InvocationArg::try_from(env::var("DB2_JDBC_URL").unwrap()).unwrap(),
                InvocationArg::try_from(env::var("DB2_JDBC_DRIVER").unwrap()).unwrap(),
                InvocationArg::try_from(env::var("DB2_USER").unwrap()).unwrap(),
                InvocationArg::try_from(env::var("DB2_PASSWORD").unwrap()).unwrap(),
            ],
        )
        .unwrap();

    let db_conns = jvm.create_instance("java.util.HashMap", &[]).unwrap();
    jvm.invoke(
        &db_conns,
        "put",
        &[
            InvocationArg::try_from("db1").unwrap(),
            InvocationArg::try_from(ds1).unwrap(),
        ],
    )
    .unwrap();
    jvm.invoke(
        &db_conns,
        "put",
        &[
            InvocationArg::try_from("db2").unwrap(),
            InvocationArg::try_from(ds2).unwrap(),
        ],
    )
    .unwrap();

    let db_conns = InvocationArg::try_from(db_conns).unwrap();

    let rewriter = jvm
        .create_instance("ai.dataprep.federated.FederatedQueryRewriter", &[])
        .unwrap();

    let plan = jvm.invoke(&rewriter, "rewrite", &[db_conns, sql]).unwrap();

    let count = jvm.invoke(&plan, "getCount", &[]).unwrap();
    let count: i32 = jvm.to_rust(count).unwrap();
    debug!("rewrite finished, got {} queries", count);

    // let ctx = SessionContext::new();
    let mut ctx = ExecutionContext::new();
    let mut local_sql = String::new();
    let mut alias_names = vec![];
    for i in 0..count {
        debug!("\nquery {i}:");

        let db = jvm
            .invoke(
                &plan,
                "getDBName",
                &[InvocationArg::try_from(i)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        let db: String = jvm.to_rust(db).unwrap();

        let alias_db = jvm
            .invoke(
                &plan,
                "getAliasDBName",
                &[InvocationArg::try_from(i)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        let alias_db: String = jvm.to_rust(alias_db).unwrap();

        let rewrite_sql = jvm
            .invoke(
                &plan,
                "getSql",
                &[InvocationArg::try_from(i)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        let rewrite_sql: String = jvm.to_rust(rewrite_sql).unwrap();
        debug!("db: {}, rewrite sql: {}", db, rewrite_sql);

        if db == "LOCAL" {
            local_sql = rewrite_sql;
        } else {
            let mut destination = ArrowDestination::new();
            let queries = [CXQuery::naked(rewrite_sql)];

            let conn = &db_map[db.as_str()];

            let url = Url::parse(conn).unwrap();
            let (config, _) = rewrite_tls_args(&url).unwrap();

            let sb = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 1).unwrap();
            let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
                sb,
                &mut destination,
                &queries,
                None,
            );

            dispatcher.run().unwrap();
            let rbs = destination.arrow().unwrap();
            let provider = MemTable::try_new(rbs[0].schema(), vec![rbs]).unwrap();
            ctx.register_table(alias_db.as_str(), Arc::new(provider))
                .unwrap();
            alias_names.push(alias_db);
        }
    }

    debug!("\nexecute query final:");
    let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
    // until datafusion fix the bug
    for alias in alias_names {
        local_sql = local_sql.replace(format!("\"{}\"", alias).as_str(), alias.as_str());
    }

    let df = rt.block_on(ctx.sql(local_sql.as_str())).unwrap();
    rt.block_on(df.collect()).unwrap()
}
