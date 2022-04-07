use arrow::{
    array::{BooleanArray, Float64Array, Int32Array, LargeStringArray},
    record_batch::RecordBatch,
};
use connectorx::{
    destinations::arrow::ArrowDestination, prelude::*, sources::bigquery::{BigQuerySource}, sql::CXQuery,
    transports::BigQueryArrowTransport,
};
use std::env;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[test]

fn test_source() {
    let dburl = "bigquery:///home/jinze/dataprep-bigquery-d6514e01c1db.json";
    let rt = Arc::new(Runtime::new().unwrap());
    let mut source = BigQuerySource::new(rt, &dburl).unwrap();
    source.set_queries(&[
        CXQuery::naked("SELECT * FROM (SELECT * FROM `dataprep-bigquery.dataprep.lineitem` LIMIT 1000) AS CXTMPTAB_PART WHERE 1281 <= CXTMPTAB_PART.L_ORDERKEY AND CXTMPTAB_PART.L_ORDERKEY < 19419500"),
        CXQuery::naked("SELECT * FROM (SELECT * FROM `dataprep-bigquery.dataprep.lineitem` LIMIT 1000) AS CXTMPTAB_PART WHERE 19419500 <= CXTMPTAB_PART.L_ORDERKEY AND CXTMPTAB_PART.L_ORDERKEY < 38837719"),
        CXQuery::naked("SELECT * FROM (SELECT * FROM `dataprep-bigquery.dataprep.lineitem` LIMIT 1000) AS CXTMPTAB_PART WHERE 38837719 <= CXTMPTAB_PART.L_ORDERKEY AND CXTMPTAB_PART.L_ORDERKEY < 58255940"),]);
    source.fetch_metadata().unwrap();
}

#[test]
fn test_bigquery_partition() {
    let dburl = "bigquery:///home/jinze/dataprep-bigquery-d6514e01c1db.json";
    let rt = Arc::new(Runtime::new().unwrap());
    let mut source = BigQuerySource::new(rt, &dburl).unwrap();
    let queries = [
        CXQuery::naked("SELECT * FROM (SELECT * FROM `dataprep-bigquery.dataprep.lineitem` LIMIT 1000) AS CXTMPTAB_PART WHERE 1281 <= CXTMPTAB_PART.L_ORDERKEY AND CXTMPTAB_PART.L_ORDERKEY < 29128610"),
        CXQuery::naked("SELECT * FROM (SELECT * FROM `dataprep-bigquery.dataprep.lineitem` LIMIT 1000) AS CXTMPTAB_PART WHERE 29128610 <= CXTMPTAB_PART.L_ORDERKEY AND CXTMPTAB_PART.L_ORDERKEY < 58255940"),
    ];
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, BigQueryArrowTransport>::new(
        source,
        &mut destination,
        &queries,
        None,
    );
    dispatcher.run().unwrap();
    let result = destination.arrow().unwrap();
}