use connectorx::arrow_batch_iter::ArrowBatchIter;
use connectorx::prelude::*;
use connectorx::sources::postgres::{rewrite_tls_args, BinaryProtocol as PgBinaryProtocol};
use postgres::NoTls;
use std::convert::TryFrom;
use std::time::Instant;

fn main() {
    // let queries = &[CXQuery::naked("select * from test_table")];
    // let queries = &[
    //     CXQuery::naked("select * from test_table where test_int < 3"),
    //     CXQuery::naked("select * from test_table where test_int >= 3"),
    // ];

    let start = Instant::now();

    let queries = &[
        CXQuery::naked("select * from lineitem where l_orderkey < 1000000"),
        CXQuery::naked(
            "select * from lineitem where l_orderkey >= 1000000 AND l_orderkey < 2000000",
        ),
        CXQuery::naked(
            "select * from lineitem where l_orderkey >= 2000000 AND l_orderkey < 3000000",
        ),
        CXQuery::naked(
            "select * from lineitem where l_orderkey >= 3000000 AND l_orderkey < 4000000",
        ),
        CXQuery::naked(
            "select * from lineitem where l_orderkey >= 4000000 AND l_orderkey < 5000000",
        ),
        CXQuery::naked("select * from lineitem where l_orderkey >= 5000000"),
    ];

    let origin_query = None;

    let conn = "postgresql://postgres:postgres@localhost:5432/tpch";
    let source = SourceConn::try_from(conn).unwrap();
    let (config, _) = rewrite_tls_args(&source.conn).unwrap();
    let source =
        PostgresSource::<PgBinaryProtocol, NoTls>::new(config, NoTls, queries.len()).unwrap();

    let destination = ArrowStreamDestination::new_with_batch_size(2048);

    let mut batch_iter: ArrowBatchIter<_, PostgresArrowStreamTransport<PgBinaryProtocol, NoTls>> =
        ArrowBatchIter::new(source, destination, origin_query, queries).unwrap();

    batch_iter.prepare();

    let mut num_rows = 0;
    let mut num_batches = 0;
    for record_batch in batch_iter {
        let record_batch = record_batch;
        println!("got 1 batch, with {} rows", record_batch.num_rows());
        num_rows += record_batch.num_rows();
        num_batches += 1;
        // arrow::util::pretty::print_batches(&[record_batch]).unwrap();
    }
    println!(
        "got {} batches, {} rows in total, took {:?}",
        num_batches,
        num_rows,
        start.elapsed()
    );
}
