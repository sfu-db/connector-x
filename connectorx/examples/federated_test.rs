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
use j4rs::{ClasspathEntry, InvocationArg, JavaOpt, Jvm, JvmBuilder};
use postgres::NoTls;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::iter::Iterator;
use std::sync::Arc;
use url::Url;

fn main() {
    let db_map = HashMap::from([("db1", "POSTGRES"), ("db2", "MYSQL"), ("LOCAL", "LOCAL")]);

    let path = fs::canonicalize("./federated-rewriter.jar").unwrap();
    println!("path: {:?}", path);
    let entry = ClasspathEntry::new(path.to_str().unwrap());
    let jvm: Jvm = JvmBuilder::new().classpath_entry(entry).build().unwrap();

    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let sql = fs::read_to_string(file).unwrap();
    println!("input sql: {}", sql);
    let sql = InvocationArg::try_from(sql).unwrap();

    let rewriter = jvm
        .create_instance("ai.dataprep.federated.FederatedQueryRewriter", &[])
        .unwrap();

    let plan = jvm.invoke(&rewriter, "rewrite", &[sql]).unwrap();

    let count = jvm.invoke(&plan, "getCount", &[]).unwrap();
    let count: i32 = jvm.to_rust(count).unwrap();

    let mut ctx = ExecutionContext::new();
    let mut local_sql = String::new();
    for i in 0..count {
        println!("query {i}:");

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
        let mut rewrite_sql: String = jvm.to_rust(rewrite_sql).unwrap();
        rewrite_sql = rewrite_sql.replace("$f", "f");
        println!("db: {}, rewrite sql: {}", db, rewrite_sql);

        if db == "LOCAL" {
            local_sql = rewrite_sql;
        } else {
            let mut destination = ArrowDestination::new();
            let queries = [CXQuery::naked(rewrite_sql)];

            match db_map[db.as_str()] {
                "POSTGRES" => {
                    let conn = env::var("POSTGRES_URL").unwrap();
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
                    println!("run dispatcher");
                    dispatcher.run().unwrap();
                }
                "MYSQL" => {
                    let conn = env::var("MYSQL_URL").unwrap();
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
            println!("schema: {}", rbs[0].schema());
            let provider = MemTable::try_new(rbs[0].schema(), vec![rbs]).unwrap();
            ctx.register_table(db.as_str(), Arc::new(provider)).unwrap();
        }
    }

    // until datafusion fix the bug
    local_sql = local_sql.replace("\"", "");

    let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
    let df = rt.block_on(ctx.sql(local_sql.as_str())).unwrap();

    rt.block_on(df.limit(5).unwrap().show()).unwrap();
    let num_rows = rt
        .block_on(df.collect())
        .unwrap()
        .into_iter()
        .map(|rb| rb.num_rows())
        .sum::<usize>();
    println!("Final # rows: {}", num_rows);

    // let counts = result
    //     .iter()
    //     .map(|rb| rb.num_rows())
    //     .collect::<Vec<usize>>();

    // println!("result rows: {}", counts.iter().sum::<usize>());
    // println!("result columns: {}", result[0].schema())
}
