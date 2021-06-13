use std::env;
use connectorx::sources::mysql::MysqlSource;
use connectorx::{Source, SourcePartition};
use connectorx::sources::Produce;

#[test]
fn load_and_parse() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("MYSQL_URL").unwrap();
    #[derive(Debug, PartialEq)]
    struct Row(i32, f32);

    let mut source = MysqlSource::new(&dburl, 1).unwrap();
    source.set_queries(&["select * from test_table"]);
    source.fetch_metadata().unwrap();

    let mut partitions = source.partition().unwrap();
    assert!(partitions.len() == 1);
    let mut partition = partitions.remove(0);
    partition.prepare().expect("run query");

    assert_eq!(6, partition.nrows());
    assert_eq!(5, partition.ncols());

    let mut parser = partition.parser().unwrap();

    let mut rows: Vec<Row> = Vec::new();
    for _i in 0..1 {
        rows.push(Row(
            parser.produce().unwrap(),
            parser.produce().unwrap(),
        ));
    }

    assert_eq!(
        vec![
            Row(1, 1.1),
            Row(2, 2.2)
        ],
        rows
    );
}