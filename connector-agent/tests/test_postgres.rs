use connector_agent::data_sources::{
    postgres::{PostgresDataSource, PostgresDataSourceBuilder},
    DataSource, Produce,
};
use connector_agent::{SourceBuilder, DataType, Dispatcher};
use connector_agent::writers::mixed::MemoryWriter;

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
    #[derive(Debug, PartialEq)]
    enum Value {
        Name(String),
        Id(String),
        Value(u64),
        Sex(bool)
    }

    let mut source_builder = PostgresDataSourceBuilder::new("host=localhost user=postgres dbname=TestDataprep port=5432 password=111");
    let mut source: PostgresDataSource = source_builder.build();
    source
        .run_query("select * from test_table_1")
        .expect("run query");

    // assert_eq!(3, source.nrows);
    // assert_eq!(4, source.ncols);

    let mut results: Vec<Value> = Vec::new();
    for _i in 0..source.nrows {
        results.push(Value::Name(source.produce().expect("parse name")));
        results.push(Value::Id(source.produce().expect("parse id")));
        results.push(Value::Value(source.produce().expect("parse value")));
        results.push(Value::Sex(source.produce().expect("parse sex")));
    }

    assert_eq!(
        vec![
            Value::Name("nick".to_string()),
            Value::Id(String::from("1")),
            Value::Value(100),
            Value::Sex(false),
            Value::Name("charlie".to_string()),
            Value::Id(String::from("2")),
            Value::Value(100),
            Value::Sex(false),
        ],
        results
    );
}

#[test]
fn test_postgres() {
    #[derive(Debug, PartialEq)]
    enum Value {
        Name(String),
        Id(String),
        Value(u64),
        Sex(bool)
    }
    let queries = vec![
        "select * from test_table_1 asc limit 1".to_string(),
        "select * from test_table_1 desc limit 1".to_string()
    ];
    let schema = vec![(DataType::String,DataType::String, DataType::U64, DataType::Bool)];
    let mut source_builder = PostgresDataSourceBuilder::new("host=localhost user=postgres dbname=TestDataprep port=5432 password=wjz283200");
    let dispatcher = Dispatcher::new(source_builder, schema, queries);
    let dw = dispatcher
        .run_checked::<MemoryWriter>()
        .expect("run dispatcher");
    print!("{}", dw.buffer_view(1));
    // assert_eq!(
    //     array![
    //         [0, 1, 2, 3, 4],
    //         [5, 6, 7, 8, 9],
    //         [10, 11, 12, 13, 14],
    //         [15, 16, 17, 18, 19],
    //         [20, 21, 22, 23, 24],
    //         [25, 26, 27, 28, 29],
    //         [30, 31, 32, 33, 34],
    //         [35, 36, 37, 38, 39],
    //         [40, 41, 42, 43, 44],
    //         [45, 46, 47, 48, 49],
    //         [50, 51, 52, 53, 54],
    //     ],
    //     dw.buffer()
    // );
}
