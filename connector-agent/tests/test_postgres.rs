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
    let mut source_builder = PostgresDataSourceBuilder::new("connection_code");
    let mut source: PostgresDataSource = source_builder.build();
    source
        .run_query("select * from test_table_1")
        .expect("run query");
}

#[test]
#[should_panic]
fn wrong_table_name() {
    let mut source_builder = PostgresDataSourceBuilder::new("connection_code");
    let mut source: PostgresDataSource = source_builder.build();
    source
        .run_query("select * from test_table_wrong")
        .expect("run query");
}

#[test]
fn load_and_parse() {
    let dburl = env::var("POSTGRES_URL").unwrap();

    #[derive(Debug, PartialEq)]
    enum Value {
        Id(u64),
        Name(String),
        Email(String),
        Age(u64),
    }

    let mut source_builder = PostgresDataSourceBuilder::new(&dburl);
    let mut source: PostgresDataSource = source_builder.build();
    source.run_query("select * from person").expect("run query");

    assert_eq!(3, source.nrows);
    assert_eq!(4, source.ncols);

    let mut results: Vec<Value> = Vec::new();
    for _i in 0..source.nrows {
        results.push(Value::Id(source.produce().expect("parse id")));
        results.push(Value::Name(source.produce().expect("parse name")));
        results.push(Value::Email(source.produce().expect("parse email")));
        results.push(Value::Age(source.produce().expect("parse age")));
    }

    assert_eq!(
        vec![
            Value::Id(1),
            Value::Name(String::from("Raj")),
            Value::Email(String::from("raj@gmail.com")),
            Value::Age(22),
            Value::Id(2),
            Value::Name(String::from("Abhishek")),
            Value::Email(String::from("ab@gmail.com")),
            Value::Age(32),
            Value::Id(3),
            Value::Name(String::from("Ashish")),
            Value::Email(String::from("ashish@gmail.com")),
            Value::Age(25),
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
        DataType::String,
        DataType::U64,
    ];
    let queries = vec![
        "select * from person where id < 2".to_string(),
        "select * from person where id >= 2".to_string(),
    ];
    let builder = PostgresDataSourceBuilder::new(&dburl);
    let dispatcher = Dispatcher::new(builder, MemoryWriter::new(), &schema, queries);

    let dw = dispatcher.run_checked().expect("run dispatcher");
    assert_eq!(array![1, 2, 3], dw.column_view::<u64>(0).unwrap());
    assert_eq!(
        array!["Raj", "Abhishek", "Ashish"],
        dw.column_view::<String>(1).unwrap()
    );
    assert_eq!(
        array!["raj@gmail.com", "ab@gmail.com", "ashish@gmail.com"],
        dw.column_view::<String>(2).unwrap()
    );
    assert_eq!(array![22, 32, 25], dw.column_view::<u64>(3).unwrap());
}
