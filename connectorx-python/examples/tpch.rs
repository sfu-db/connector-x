use connectorx_python::cx_read_sql::{read_sql, PyPartitionQuery};
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
            Some(PyPartitionQuery {
                query: QUERY.to_string(),
                column: "L_ORDERKEY".to_string(),
                min: None,
                max: None,
                num: nq,
            }),
        )
        .unwrap();
    });
}

#[allow(dead_code)]
fn main() {
    run(1, "POSTGRES_URL");
}
