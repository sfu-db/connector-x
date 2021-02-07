use connector_agent::data_sources::{
    postgres::{PostgresDataSource, PostgresDataSourceBuilder},
    DataSource, Produce,
};
use connector_agent::writers::dummy::U64Writer;
use connector_agent::{DataType, Dispatcher};
use ndarray::array;
#[test]
fn load_and_parse() {
    #[derive(Debug, PartialEq)]
    enum Value {
        Id(u64),
        Name(String),
        Email(String),
        Age(u64)
    }

    // maybe change pg record to byterecord
    let source_builder = PostgresDataSourceBuilder::new("host=localhost user=postgres dbname=Person port=5432 password=postgres");

    let mut source = PostgresDataSource::new();
    source
        .run_query("select * from Person")
        .expect("run query");

    assert_eq!(3, source.nrows);
    assert_eq!(4, source.ncols);

    let mut results: Vec<Value> = Vec::new();
    for _i in 0..source.nrows {
        results.push(Value::Id(source.produce().expect("parse id")));
        results.push(Value::Name(source.produce().expect("parse name")));
        results.push(Value::Email(
            source.produce().expect("parse email"),
        ));
        results.push(Value::Age(source.produce().expect("parse age")));
    }

    assert_eq!(
        vec![
            Value::Id(1),
            Value::Name(String::from("Raj")),
            Value::Email(String::from("raj@gmail.com")),
            Value::Age(22),
            Value::Id(2),
            Value::Name(String::from("Abishek")),
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

