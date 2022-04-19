use connectorx::fed_dispatcher::run;
use std::collections::HashMap;

#[test]
fn test_fed() {
    let _ = env_logger::builder().is_test(true).try_init();

    let sql = "select n_name, r_name from db1.nation, db2.region where n_regionkey = r_regionkey and n_nationkey > 3";
    let db_map = HashMap::from([
        (
            String::from("db1"),
            String::from("postgresql://postgres:postgres@127.0.0.1:5432/tpchsf1"),
        ),
        (
            String::from("db2"),
            String::from("postgresql://postgres:postgres@127.0.0.1:5433/tpchsf1"),
        ),
    ]);

    let rbs = run(sql.to_string(), db_map).unwrap();
    arrow::util::pretty::print_batches(&rbs).unwrap();
}
