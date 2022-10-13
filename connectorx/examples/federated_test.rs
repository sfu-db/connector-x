use connectorx::{
    prelude::*,
    sources::{
        mysql::BinaryProtocol as MYSQLBinaryProtocol,
        postgres::{rewrite_tls_args, BinaryProtocol, PostgresSource},
    },
    sql::CXQuery,
    transports::PostgresArrowTransport,
};
use datafusion::datasource::MemTable;
use datafusion::prelude::*;
use j4rs::{ClasspathEntry, InvocationArg, Jvm, JvmBuilder};
use postgres::NoTls;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::iter::Iterator;
use std::sync::Arc;
use url::Url;

fn main() {
    let db_map = HashMap::from([("db1", "POSTGRES"), ("db2", "POSTGRES"), ("LOCAL", "LOCAL")]);

    let path = fs::canonicalize("../federated-rewriter.jar").unwrap();
    let entry = ClasspathEntry::new(path.to_str().unwrap());
    let jvm: Jvm = JvmBuilder::new().classpath_entry(entry).build().unwrap();

    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let sql = fs::read_to_string(file).unwrap();
    println!("input sql: {}", sql);
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

    let ctx = SessionContext::new();
    let mut local_sql = String::new();
    let mut alias_names = vec![];
    for i in 0..count {
        println!("\nquery {i}:");

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
        println!("db: {}, rewrite sql: {}", db, rewrite_sql);

        if db == "LOCAL" {
            local_sql = rewrite_sql;
        } else {
            let mut destination = ArrowDestination::new();
            let queries = [CXQuery::naked(rewrite_sql)];

            let conn = match db.as_str() {
                "db1" => env::var("DB1").unwrap(),
                "db2" => env::var("DB2").unwrap(),
                _ => unimplemented!(),
            };

            match db_map[db.as_str()] {
                "POSTGRES" => {
                    let url = Url::parse(&conn).unwrap();
                    let (config, _) = rewrite_tls_args(&url).unwrap();

                    let sb =
                        PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 1).unwrap();
                    let dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresArrowTransport<BinaryProtocol, NoTls>,
                    >::new(
                        sb, &mut destination, &queries, None
                    );
                    // println!("run dispatcher");
                    dispatcher.run().unwrap();
                }
                "MYSQL" => {
                    let source = MySQLSource::<MYSQLBinaryProtocol>::new(conn.as_str(), 1).unwrap();
                    let dispatcher =
                        Dispatcher::<_, _, MySQLArrowTransport<MYSQLBinaryProtocol>>::new(
                            source,
                            &mut destination,
                            &queries,
                            None,
                        );
                    dispatcher.run().unwrap();
                }
                _ => {}
            };
            let rbs = destination.arrow().unwrap();
            // println!("schema: {}", rbs[0].schema());
            // arrow::util::pretty::print_batches(&rbs).unwrap();
            let provider = MemTable::try_new(rbs[0].schema(), vec![rbs]).unwrap();
            ctx.register_table(alias_db.as_str(), Arc::new(provider))
                .unwrap();
            alias_names.push(alias_db);
        }
    }

    println!("\nquery final:");
    let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
    // until datafusion fix the bug
    for alias in alias_names {
        local_sql = local_sql.replace(format!("\"{}\"", alias).as_str(), alias.as_str());
    }
    println!("{}", local_sql);

    // let sql1 = "SELECT * FROM db2 INNER JOIN db1 ON db2.\"p_partkey\" = db1.\"l_partkey\" AND db2.\"EXPR$0\" AND (db2.\"EXPR$1\" AND db1.\"EXPR$1\") AND (db1.\"EXPR$2\" AND db2.\"EXPR$2\" AND (db1.\"EXPR$3\" AND db1.\"EXPR$4\"))";
    // println!("==== run sql 1 ====");
    // let t = rt.block_on(ctx.sql(sql1)).unwrap();
    // // rt.block_on(t.limit(5).unwrap().show()).unwrap();
    // let rbs1 = rt.block_on(t.collect()).unwrap();
    // arrow::util::pretty::print_batches(&rbs1).unwrap();
    // println!("==== run sql 2 ====");

    let df = rt.block_on(ctx.sql(local_sql.as_str())).unwrap();
    rt.block_on(df.explain(false, false).unwrap().show())
        .unwrap();
    rt.block_on(df.limit(5, None).unwrap().show()).unwrap();
    let num_rows = rt
        .block_on(df.collect())
        .unwrap()
        .into_iter()
        .map(|rb| rb.num_rows())
        .sum::<usize>();
    println!("Final # rows: {}", num_rows);
}
