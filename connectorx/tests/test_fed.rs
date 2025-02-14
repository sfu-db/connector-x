use connectorx::fed_dispatcher::run;
use std::collections::HashMap;
use std::env;

#[test]
#[ignore]
fn test_fed() {
    let _ = env_logger::builder().is_test(true).try_init();

    let sql = "select test_bool, AVG(test_float) as avg_float, SUM(test_int) as sum_int from db1.test_table as a, db2.test_str as b where a.test_int = b.id AND test_nullint is not NULL GROUP BY test_bool ORDER BY sum_int";
    let db_map = HashMap::from([
        (String::from("db1"), env::var("DB1").unwrap()),
        (String::from("db2"), env::var("DB2").unwrap()),
    ]);

    println!("db_map: {:?}", db_map);

    // make sure no error here
    let rbs = run(sql.to_string(), db_map, None, "pushdown").unwrap();
    arrow::util::pretty::print_batches(&rbs).unwrap();
}
