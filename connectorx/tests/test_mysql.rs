use arrow::{
    array::{Float64Array, Int64Array, StringArray},
    record_batch::RecordBatch,
};
use connectorx::{
    destinations::arrow::ArrowDestination,
    prelude::*,
    sources::mysql::{BinaryProtocol, MySQLSource, TextProtocol},
    sql::CXQuery,
    transports::MySQLArrowTransport,
};
use std::env;

#[test]
fn test_mysql() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("MYSQL_URL").unwrap();

    let queries = [
        CXQuery::naked("select * from test_table where test_int <= 2"),
        CXQuery::naked("select * from test_table where test_int > 2"),
    ];

    let builder = MySQLSource::<BinaryProtocol>::new(&dburl, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, MySQLArrowTransport<BinaryProtocol>>::new(
        builder,
        &mut destination,
        &queries,
        Some(String::from("select * from test_table")),
    );
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();
    verify_arrow_results(result);
}

#[test]
fn test_mysql_text() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("MYSQL_URL").unwrap();

    let queries = [
        CXQuery::naked("select * from test_table where test_int <= 2"),
        CXQuery::naked("select * from test_table where test_int > 2"),
    ];

    let builder = MySQLSource::<TextProtocol>::new(&dburl, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, MySQLArrowTransport<TextProtocol>>::new(
        builder,
        &mut destination,
        &queries,
        None,
    );
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();
    verify_arrow_results(result);
}

#[test]
fn test_mysql_pre_execution_queries() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("MYSQL_URL").unwrap();

    let queries = [CXQuery::naked(
        "SELECT @@SESSION.max_execution_time, @@SESSION.wait_timeout",
    )];

    let pre_execution_queries = [
        String::from("SET SESSION max_execution_time = 2151"),
        String::from("SET SESSION wait_timeout = 2252"),
    ];

    let builder = MySQLSource::<BinaryProtocol>::new(&dburl, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let mut dispatcher = Dispatcher::<_, _, MySQLArrowTransport<BinaryProtocol>>::new(
        builder,
        &mut destination,
        &queries,
        None,
    );
    dispatcher.set_pre_execution_queries(Some(&pre_execution_queries));
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();

    assert!(result.len() == 1);

    assert!(result[0]
        .column(0)
        .as_any()
        .downcast_ref::<Float64Array>()
        .unwrap()
        .eq(&Float64Array::from(vec![2151.0])));

    assert!(result[0]
        .column(1)
        .as_any()
        .downcast_ref::<Float64Array>()
        .unwrap()
        .eq(&Float64Array::from(vec![2252.0])));
}

#[test]
fn test_mysql_partitioned_pre_execution_queries() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("MYSQL_URL").unwrap();

    let queries = [
        CXQuery::naked(
            "SELECT 'max_execution_time' AS name, @@SESSION.max_execution_time AS setting",
        ),
        CXQuery::naked("SELECT 'wait_timeout' AS name, @@SESSION.wait_timeout AS setting"),
    ];

    let pre_execution_queries = [
        String::from("SET SESSION max_execution_time = 2151"),
        String::from("SET SESSION wait_timeout = 2252"),
    ];

    let builder = MySQLSource::<BinaryProtocol>::new(&dburl, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let mut dispatcher = Dispatcher::<_, _, MySQLArrowTransport<BinaryProtocol>>::new(
        builder,
        &mut destination,
        &queries,
        None,
    );
    dispatcher.set_pre_execution_queries(Some(&pre_execution_queries));
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();

    assert!(result.len() == 2);

    let mut result_map = std::collections::HashMap::new();
    for record_batch in result {
        let name = record_batch
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap()
            .value(0)
            .to_string();
        let setting = record_batch
            .column(1)
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap()
            .value(0);
        result_map.insert(name, setting);
    }

    assert_eq!(result_map.get("max_execution_time"), Some(&2151.0));
    assert_eq!(result_map.get("wait_timeout"), Some(&2252.0));
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
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .eq(&StringArray::from(vec!["odd", "even"])));
                assert!(r
                    .column(3)
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
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .eq(&StringArray::from(vec!["odd", "even", "odd", "even"])));
                assert!(r
                    .column(3)
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
