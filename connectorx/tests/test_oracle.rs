use connectorx::prelude::*;
use connectorx::sources::oracle::OracleSource;
use connectorx::sql::CXQuery;

mod test_db;

#[test]
#[ignore]
fn test_types() {
    let _ = env_logger::builder().is_test(true).try_init();
    let dburl = test_db::oracle_url();
    let mut source = OracleSource::new(&dburl, 1).unwrap();
    #[derive(Debug, PartialEq)]
    struct Row(i64, Option<String>, Option<f64>);

    source.set_queries(&[CXQuery::naked(
        "select test_int, test_char, test_float from admin.test_table order by test_int",
    )]);
    source.fetch_metadata().unwrap();
    let mut partitions = source.partition().unwrap();
    assert!(partitions.len() == 1);
    let mut partition = partitions.remove(0);
    partition.result_rows().expect("run query");
    assert_eq!(5, partition.nrows());
    assert_eq!(3, partition.ncols());

    let mut parser = partition.parser().unwrap();

    let mut rows: Vec<Row> = Vec::new();
    loop {
        let (n, is_last) = parser.fetch_next().unwrap();
        for _i in 0..n {
            rows.push(Row(
                parser.produce().unwrap(),
                parser.produce().unwrap(),
                parser.produce().unwrap(),
            ));
        }
        if is_last {
            break;
        }
    }

    assert_eq!(
        vec![
            Row(1, Some("str1 ".to_string()), Some(1.1)),
            Row(2, Some("str2 ".to_string()), Some(2.2)),
            Row(4, None, Some(-4.44)),
            Row(5, Some("str05".to_string()), None),
            Row(2333, None, None),
        ],
        rows
    );
}
