use connector_agent::data_sources::{
    postgres::{PostgresDataSource, PostgresDataSourceBuilder},
    DataSource, Produce,
};
use connector_agent::writers::mixed::MemoryWriter;
use connector_agent::SourceBuilder;
use connector_agent::{DataType, Dispatcher};
use ndarray::array;
use std::env;

#[test]
#[should_panic]
fn wrong_connection() {
    let mut source_builder = PostgresDataSourceBuilder::new("wrong_connection_code");
    let mut source: PostgresDataSource = source_builder.build();
    source
        .prepare("select * from test_table_1")
        .expect("run query");
}

#[test]
#[should_panic]
fn wrong_table_name() {
    let dburl = env::var("POSTGRES_URL").unwrap();
    let mut source_builder = PostgresDataSourceBuilder::new(&dburl);
    let mut source: PostgresDataSource = source_builder.build();
    source
        .prepare("select * from test_table_wrong")
        .expect("run query");
}

#[test]
fn load_and_parse() {
    let dburl = env::var("POSTGRES_URL").unwrap();
    #[derive(Debug, PartialEq)]
    enum Value {
        TestInt(u64),
        TestStr(String),
        TestFloat(f64),
        TestBool(bool),
    }

    let mut source_builder = PostgresDataSourceBuilder::new(&dburl);
    let mut source: PostgresDataSource = source_builder.build();
    source
        .prepare("select * from test_postgres_conn")
        .expect("run query");

    assert_eq!(2, source.nrows);
    assert_eq!(4, source.ncols);

    let mut results: Vec<Value> = Vec::new();
    for _i in 0..source.nrows {
        results.push(Value::TestInt(source.produce().expect("parse test_int")));
        results.push(Value::TestStr(source.produce().expect("parse test_str")));
        results.push(Value::TestFloat(
            source.produce().expect("parse test_float"),
        ));
        results.push(Value::TestBool(source.produce().expect("parse test_bool")));
    }

    assert_eq!(
        vec![
            Value::TestInt(1),
            Value::TestStr(String::from("str1")),
            Value::TestFloat(1.1),
            Value::TestBool(true),
            Value::TestInt(2),
            Value::TestStr(String::from("str2")),
            Value::TestFloat(2.2),
            Value::TestBool(false),
        ],
        results
    );
}

#[test]
fn test_postgres() {
    let dburl = env::var("POSTGRES_URL").unwrap();
    let schema = vec![
        DataType::U64,
        DataType::String,
        DataType::F64,
        DataType::Bool,
    ];
    let queries = vec![
        "select * from test_postgres_conn where test_int < 2".to_string(),
        "select * from test_postgres_conn where test_int >= 2".to_string(),
    ];
    let builder = PostgresDataSourceBuilder::new(&dburl);
    let dispatcher = Dispatcher::new(builder, MemoryWriter::new(), &schema, queries);

    let dw = dispatcher.run_checked().expect("run dispatcher");
    assert_eq!(array![1, 2], dw.column_view::<u64>(0).unwrap());
    assert_eq!(array!["str1", "str2"], dw.column_view::<String>(1).unwrap());
    assert_eq!(array![1.1, 2.2], dw.column_view::<f64>(2).unwrap());
    assert_eq!(array![true, false], dw.column_view::<bool>(3).unwrap());
}
