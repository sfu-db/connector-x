use connectorx_python::read_sql::{read_sql, PartitionQuery};
use pyo3::Python;
use std::env;

const QUERY: &'static str = r#"
SELECT 
    *
FROM LINEITEM"#;

pub fn run(nq: usize, conn: &str) {
    let conn = env::var(conn).unwrap();

    Python::with_gil(|py| {
        read_sql(
            py,
            &conn,
            "pandas",
            None,
            None,
            Some(PartitionQuery::new(QUERY, "L_ORDERKEY", None, None, nq)),
        )
        .unwrap();
    });
}

fn main() {
    run(1, "POSTGRES_URL");
}
