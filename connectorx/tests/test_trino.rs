use arrow::{
    array::{Float64Array, Int64Array},
    record_batch::RecordBatch,
};
use connectorx::{
    destinations::arrow::ArrowDestination, prelude::*, sources::trino::TrinoSource, sql::CXQuery,
    transports::TrinoArrowTransport,
};
use std::{env, sync::Arc};

#[test]
#[ignore]
fn test_trino() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("TRINO_URL").unwrap();

    let queries = [
        CXQuery::naked("select * from test.test_table where test_int <= 2 order by test_int"),
        CXQuery::naked("select * from test.test_table where test_int > 2 order by test_int"),
    ];

    let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
    let builder = TrinoSource::new(rt, &dburl).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, TrinoArrowTransport>::new(
        builder,
        &mut destination,
        &queries,
        Some(String::from(
            "select * from test.test_table order by test_int",
        )),
    );
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();
    verify_arrow_results(result);
}

#[test]
#[ignore]
fn test_trino_text() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("TRINO_URL").unwrap();

    let queries = [
        CXQuery::naked("select * from test.test_table where test_int <= 2 order by test_int"),
        CXQuery::naked("select * from test.test_table where test_int > 2 order by test_int"),
    ];

    let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
    let builder = TrinoSource::new(rt, &dburl).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher =
        Dispatcher::<_, _, TrinoArrowTransport>::new(builder, &mut destination, &queries, None);
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();
    verify_arrow_results(result);
}

pub fn verify_arrow_results(result: Vec<RecordBatch>) {
    assert!(result.len() == 2);

    for r in result {
        match r.num_rows() {
            2 => {
                assert!(r
                    .column(0)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![1, 2])));
                assert!(r
                    .column(1)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![1.1, 2.2])));
                assert!(r
                    .column(2)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![None, None])));
            }
            4 => {
                assert!(r
                    .column(0)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![3, 4, 5, 6])));
                assert!(r
                    .column(1)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![3.3, 4.4, 5.5, 6.6])));
                assert!(r
                    .column(2)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![None, None, None, None])));
            }
            _ => {
                println!("got {} rows in a record batch!", r.num_rows());
                unreachable!()
            }
        }
    }
}
