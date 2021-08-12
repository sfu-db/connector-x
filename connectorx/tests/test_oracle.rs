use connectorx::prelude::*;
use connectorx::sources::oracle::{OracleSource, TextProtocol};
use connectorx::sql::CXQuery;
use std::env;

#[test]
fn test_types() {
    let _ = env_logger::builder().is_test(true).try_init();
    let dburl = env::var("ORACLE_URL").unwrap();
    let mut source = OracleSource::<TextProtocol>::new(&dburl, 1).unwrap();
    #[derive(Debug, PartialEq)]
    struct Row(i64, f64, String);

    source.set_queries(&[CXQuery::naked("select * from admin.test_types")]);
    source.fetch_metadata().unwrap();
    let mut partitions = source.partition().unwrap();
    assert!(partitions.len() == 1);
    let mut partition = partitions.remove(0);
    partition.prepare().expect("run query");
    assert_eq!(3, partition.nrows());
    assert_eq!(3, partition.ncols());

    let mut parser = partition.parser().unwrap();

    let mut rows: Vec<Row> = Vec::new();
    for _i in 0..3 {
        rows.push(
            Row(
                parser.produce().unwrap(), 
                parser.produce().unwrap(), 
                parser.produce().unwrap()
            )
        );
    }
    // rows[0].0
    // println!("{:?}", rows[0]);
    assert_eq!(
        vec![
            Row(1, 1.1, "a1".to_string()),
            Row(2, 2.2, "b2".to_string()),
            Row(3, 3.3, "c3".to_string())
        ],
        rows
    );
}
