use connectorx::{
    prelude::*,
    sources::postgres::{rewrite_tls_args, BinaryProtocol, PostgresSource},
    sql::CXQuery,
    transports::PostgresArrowTransport,
};
use j4rs::{ClasspathEntry, InvocationArg, Jvm, JvmBuilder};
use postgres::NoTls;
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::iter::Iterator;
use url::Url;

fn main() {
    let path = fs::canonicalize("./federated-rewriter.jar").unwrap();
    println!("path: {:?}", path);
    let entry = ClasspathEntry::new(path.to_str().unwrap());
    let jvm: Jvm = JvmBuilder::new().classpath_entry(entry).build().unwrap();

    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let sql = fs::read_to_string(file).unwrap();
    println!("input sql: {}", sql);
    let sql = InvocationArg::try_from(sql).unwrap();
    let rewrite_sql = jvm
        .invoke_static("ai.dataprep.federated.QueryRewriter", "rewrite", &[sql])
        .unwrap();

    let rewrite_sql: String = jvm.to_rust(rewrite_sql).unwrap();

    println!("rewrite sql: {}", rewrite_sql);

    let conn = env::var("POSTGRES_URL").unwrap();
    let url = Url::parse(&conn).unwrap();
    let (config, _) = rewrite_tls_args(&url).unwrap();

    let sb = PostgresSource::<BinaryProtocol, NoTls>::new(config, NoTls, 1).unwrap();
    let mut destination = ArrowDestination::new();
    let queries = [CXQuery::naked(rewrite_sql)];
    let dispatcher = Dispatcher::<_, _, PostgresArrowTransport<BinaryProtocol, NoTls>>::new(
        sb,
        &mut destination,
        &queries,
        None,
    );
    println!("run dispatcher");
    dispatcher.run().unwrap();
    let result = destination.arrow().unwrap();
    let counts = result
        .iter()
        .map(|rb| rb.num_rows())
        .collect::<Vec<usize>>();

    println!("result rows: {}", counts.iter().sum::<usize>());
    println!("result columns: {}", result[0].schema())
}
