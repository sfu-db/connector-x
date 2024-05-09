use arrow::{
    array::{BooleanArray, Float64Array, Int64Array, StringArray},
    record_batch::RecordBatch,
};
use connectorx::{
    destinations::arrow::ArrowDestination, prelude::*, sources::mssql::MsSQLSource, sql::CXQuery,
    transports::MsSQLArrowTransport,
};
use std::env;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[test]
#[ignore]
fn test_mssql() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("MSSQL_URL").unwrap();

    let queries = [
        CXQuery::naked("select * from test_table where test_int < 2"),
        CXQuery::naked("select * from test_table where test_int >= 2"),
    ];
    let rt = Arc::new(Runtime::new().unwrap());

    let builder = MsSQLSource::new(rt, &dburl, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher =
        Dispatcher::<_, _, MsSQLArrowTransport>::new(builder, &mut destination, &queries, None);
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();
    verify_arrow_results(result);
}

#[test]
#[ignore]
fn test_mssql_agg() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("MSSQL_URL").unwrap();

    let queries = [CXQuery::naked(
        "SELECT test_bool, SUM(test_float) AS SUM FROM test_table GROUP BY test_bool",
    )];
    let rt = Arc::new(Runtime::new().unwrap());

    let builder = MsSQLSource::new(rt, &dburl, 1).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, MsSQLArrowTransport>::new(
        builder,
        &mut destination,
        &queries,
        Some(String::from(
            "SELECT test_bool, SUM(test_float) AS SUM FROM test_table GROUP BY test_bool",
        )),
    );
    dispatcher.run().unwrap();

    let mut result = destination.arrow().unwrap();
    assert!(result.len() == 1);
    let rb = result.pop().unwrap();
    assert!(rb.columns().len() == 2);

    assert!(rb
        .column(0)
        .as_any()
        .downcast_ref::<BooleanArray>()
        .unwrap()
        .eq(&BooleanArray::from(vec![None, Some(false), Some(true)])));

    assert!(rb
        .column(1)
        .as_any()
        .downcast_ref::<Float64Array>()
        .unwrap()
        .eq(&Float64Array::from(vec![
            Some(10.9),
            Some(5.2),
            Some(-10.0),
        ])));
}

pub fn verify_arrow_results(result: Vec<RecordBatch>) {
    assert!(result.len() == 2);

    for rb in result {
        assert!(rb.columns().len() == 5);
        match rb.num_rows() {
            2 => {
                assert!(rb
                    .column(0)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![1, 0])));

                assert!(rb
                    .column(1)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![Some(3), Some(5)])));

                assert!(rb
                    .column(2)
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .eq(&StringArray::from(vec![Some("str1"), Some("a"),])));

                assert!(rb
                    .column(3)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![None, Some(3.1_f64)])));

                assert!(rb
                    .column(4)
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .unwrap()
                    .eq(&BooleanArray::from(vec![Some(true), None])));
            }
            4 => {
                assert!(rb
                    .column(0)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![2, 3, 4, 1314])));

                assert!(rb
                    .column(1)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![None, Some(7), Some(9), Some(2)])));

                assert!(rb
                    .column(2)
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .eq(&StringArray::from(vec![
                        Some("str2"),
                        Some("b"),
                        Some("c"),
                        None,
                    ])));

                assert!(rb
                    .column(3)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![
                        Some(2.2_f64),
                        Some(3_f64),
                        Some(7.8_f64),
                        Some(-10_f64),
                    ])));

                assert!(rb
                    .column(4)
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .unwrap()
                    .eq(&BooleanArray::from(vec![
                        Some(false),
                        Some(false),
                        None,
                        Some(true),
                    ])));
            }
            _ => unreachable!(),
        }
    }
}
