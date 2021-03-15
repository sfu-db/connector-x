use connector_agent_python::read_sql::{read_sql, PartitionQuery};
use pyo3::Python;
use std::env;

const QUERY: &'static str = r#"
SELECT 
    l_orderkey,
    l_partkey,
    l_suppkey,
    l_linenumber,
    l_quantity::float8,
    l_extendedprice::float8,
    l_discount::float8,
    l_tax::float8,
    l_returnflag,
    l_linestatus,
    l_shipdate,
    l_commitdate,
    l_receiptdate,
    l_shipinstruct,
    l_shipmode,
    l_comment 
FROM lineitem"#;

pub fn run(nq: usize) {
    let conn = env::var("POSTGRES_URL").unwrap();

    Python::with_gil(|py| {
        read_sql(
            py,
            &conn,
            "pandas",
            None,
            None,
            Some(PartitionQuery::new(QUERY, "l_orderkey", 0, 6000000, nq)),
        )
        .unwrap();
    });
}

fn main() {
    run(1);
}
