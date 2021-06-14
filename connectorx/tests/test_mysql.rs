use connectorx::sources::mysql::MysqlSource;
use connectorx::sources::Produce;
use connectorx::{Source, SourcePartition};
use std::env;
// use connectorx::destinations::memory::MemoryDestination;

#[test]
fn load_and_parse() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("MYSQL_URL").unwrap();
    #[derive(Debug, PartialEq)]
    struct Row(i64, f64);

    let mut source = MysqlSource::new(&dburl, 1).unwrap();
    source.set_queries(&["select * from test_table"]);
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

    assert_eq!(vec![Row(1, 1.1), Row(2, 2.2), Row(3, 3.3)], rows);
}
