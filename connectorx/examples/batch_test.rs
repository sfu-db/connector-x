use connectorx::arrow_batch_iter::PostgresArrowBatchIter;
use connectorx::prelude::*;
use connectorx::sources::postgres::{rewrite_tls_args, BinaryProtocol as PgBinaryProtocol};
use postgres::NoTls;
use std::convert::TryFrom;

fn main() {
    // let queries = &[CXQuery::naked("select * from test_table")];
    // let queries = &[
    //     CXQuery::naked("select * from test_table where test_int < 3"),
    //     CXQuery::naked("select * from test_table where test_int >= 3"),
    // ];

    // let queries = &[CXQuery::naked("select * from lineitem limit 10")];
    let queries = &[
        CXQuery::naked("select * from lineitem where l_orderkey < 3000000"),
        CXQuery::naked("select * from lineitem where l_orderkey >= 3000000"),
    ];

    let origin_query = None;

    let conn = "postgresql://postgres:postgres@localhost:5432/tpch";
    let source = SourceConn::try_from(conn).unwrap();
    let (config, _) = rewrite_tls_args(&source.conn).unwrap();
    let source =
        PostgresSource::<PgBinaryProtocol, NoTls>::new(config, NoTls, queries.len()).unwrap();

    let destination = ArrowDestination::new();

    let mut batch_iter =
        PostgresArrowBatchIter::new(source, destination, origin_query, queries, 1024);

    let mut num_rows = 0;
    let mut num_batches = 0;
    while let Some(record_batch) = batch_iter.next() {
        println!("got 1 batch, with {} rows", record_batch.num_rows());
        num_rows += record_batch.num_rows();
        num_batches += 1;
        // arrow::util::pretty::print_batches(&[record_batch]).unwrap();
    }
    println!("got {} batches, {} rows in total", num_batches, num_rows);
}
