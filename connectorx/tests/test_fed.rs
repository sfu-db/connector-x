use connectorx::fed_dispatcher::run;
use std::collections::HashMap;
use std::env;

#[test]
fn test_fed() {
    let _ = env_logger::builder().is_test(true).try_init();

    let sql = "select n_name, r_name from db1.nation, db2.region where n_regionkey = r_regionkey and n_nationkey > 3";
    let db_map = HashMap::from([
        (String::from("db1"), env::var("DB1").unwrap()),
        (String::from("db2"), env::var("DB2").unwrap()),
    ]);

    let rbs = run(sql.to_string(), db_map, None).unwrap();
    arrow::util::pretty::print_batches(&rbs).unwrap();
}
