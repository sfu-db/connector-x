use connector_agent::data_sources::{
    postgres::{PostgresDataSource, PostgresDataSourceBuilder},
    PartitionedSource, Produce,
};
use connector_agent::writers::memory::MemoryWriter;
use connector_agent::Source;
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
    struct Row(u64, Option<u64>, String, Option<f64>, Option<bool>);

    let mut source_builder = PostgresDataSourceBuilder::new(&dburl);
    let mut source: PostgresDataSource = source_builder.build();
    source
        .prepare("select * from test_table")
        .expect("run query");

    assert_eq!(6, source.nrows());
    assert_eq!(5, source.ncols());

    let mut rows: Vec<Row> = Vec::new();
    for _i in 0..source.nrows() {
        rows.push(Row(
            source.produce().unwrap(),
            source.produce().unwrap(),
            source.produce().unwrap(),
            source.produce().unwrap(),
            source.produce().unwrap(),
        ));
    }

    assert_eq!(
        vec![
            Row(1, Some(3), "str1".into(), None, Some(true)),
            Row(2, None, "str2".into(), Some(2.2), Some(false)),
            Row(0, Some(5), "a".into(), Some(3.1), None),
            Row(3, Some(7), "b".into(), Some(3.), Some(false)),
            Row(4, Some(9), "c".into(), Some(7.8), None),
            Row(1314, Some(2), "".into(), Some(-10.), Some(true)),
        ],
        rows
    );
}

#[test]
fn test_postgres() {
    let dburl = env::var("POSTGRES_URL").unwrap();
    let schema = [
        DataType::U64(false),
        DataType::U64(true),
        DataType::String(false),
        DataType::F64(true),
        DataType::Bool(true),
    ];
    let queries = [
        "select * from test_table where test_int < 2",
        "select * from test_table where test_int >= 2",
    ];
    let builder = PostgresDataSourceBuilder::new(&dburl);
    let mut writer = MemoryWriter::new();
    let dispatcher = Dispatcher::new(builder, &mut writer, &queries, &schema);

    dispatcher.run_checked().expect("run dispatcher");
    assert_eq!(
        array![1, 0, 2, 3, 4, 1314],
        writer.column_view::<u64>(0).unwrap()
    );
    assert_eq!(
        array![Some(3), Some(5), None, Some(7), Some(9), Some(2)],
        writer.column_view::<Option<u64>>(1).unwrap()
    );
    assert_eq!(
        array![
            "str1".to_string(),
            "a".to_string(),
            "str2".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string()
        ],
        writer.column_view::<String>(2).unwrap()
    );

    assert_eq!(
        array![None, Some(3.1), Some(2.2), Some(3.), Some(7.8), Some(-10.)],
        writer.column_view::<Option<f64>>(3).unwrap()
    );

    assert_eq!(
        array![Some(true), None, Some(false), Some(false), None, Some(true)],
        writer.column_view::<Option<bool>>(4).unwrap()
    );
}
