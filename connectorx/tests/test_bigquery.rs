// use arrow::{
//     array::{BooleanArray, Float64Array, Int32Array, LargeStringArray},
//     record_batch::RecordBatch,
// };
use connectorx::{
    destinations::arrow::ArrowDestination, prelude::*, sources::bigquery::BigQuerySource,
    sources::PartitionParser, sql::CXQuery, transports::BigQueryArrowTransport,
};
use std::env;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[test]
fn test_bigquery() {
    let _ = env_logger::builder().is_test(true).try_init();
    let dburl = "bigquery:///home/jinze/dataprep-bigquery-d6514e01c1db.json"; // TODO: hard-code

    let queries = [
        CXQuery::naked("select * from test_table where test_int < 2"),
        CXQuery::naked("select * from test_table where test_int >= 2"),
    ];
    let rt = Arc::new(Runtime::new().unwrap());

    let mut source = BigQuerySource::new(&dburl).unwrap();

    source.set_queries(&queries);
    source.fetch_metadata().unwrap();
    assert!(source.result_rows().unwrap() == 3);

    // let mut destination = ArrowDestination::new();

    // let dispatcher =
    //     Dispatcher::<_, _, BigQueryArrowTransport>::new(source, &mut destination, &queries, None);
    // dispatcher.run.unwrap();

    // let result = destination.arrow().unwrap();
    // TODO: verify result
}
