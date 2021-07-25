use connectorx::prelude::*;
use connectorx::{
    sources::mysql::{BinaryProtocol, MySQLSource},
    sql::CXQuery,
};
use std::env;

#[test]
fn load_and_parse() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("MYSQL_URL").unwrap();
    #[derive(Debug, PartialEq)]
    struct Row(i64, f64);

    let mut source = MySQLSource::<BinaryProtocol>::new(&dburl, 1).unwrap();
    source.set_queries(&[CXQuery::naked("select * from test_table")]);
    source.fetch_metadata().unwrap();

    let mut partitions = source.partition().unwrap();
    assert!(partitions.len() == 1);
    let mut partition = partitions.remove(0);
    partition.prepare().expect("run query");

    assert_eq!(6, partition.nrows());
    assert_eq!(2, partition.ncols());

    let mut parser = partition.parser().unwrap();

    let mut rows: Vec<Row> = Vec::new();
    for _i in 0..6 {
        rows.push(Row(parser.produce().unwrap(), parser.produce().unwrap()));
    }

    assert_eq!(
        vec![
            Row(1, 1.1),
            Row(2, 2.2),
            Row(3, 3.3),
            Row(4, 4.4),
            Row(5, 5.5),
            Row(6, 6.6)
        ],
        rows
    );
}

#[test]
fn test_types() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("MYSQL_URL").unwrap();
    #[derive(Debug, PartialEq)]
    struct Row(f64, String);

    let mut source = MySQLSource::<BinaryProtocol>::new(&dburl, 1).unwrap();
    source.set_queries(&[CXQuery::naked(
        "select test_decimal, test_char from test_types",
    )]);
    source.fetch_metadata().unwrap();

    let mut partitions = source.partition().unwrap();
    assert!(partitions.len() == 1);
    let mut partition = partitions.remove(0);
    partition.prepare().expect("run query");

    assert_eq!(3, partition.nrows());
    assert_eq!(2, partition.ncols());

    let mut parser = partition.parser().unwrap();

    let mut rows: Vec<Row> = Vec::new();
    for _i in 0..3 {
        rows.push(Row(parser.produce().unwrap(), parser.produce().unwrap()));
    }

    assert_eq!(
        vec![
            Row(1.0, "char1".to_string()),
            Row(2.0, "char2".to_string()),
            Row(3.0, "char3".to_string())
        ],
        rows
    );
}
