use connector_agent_python::pandas::write_pandas;
use ndarray::Array1;
use pyo3::Python;
use std::env;

fn get_sqls(count: usize) -> Vec<String> {
    let mut sqls = vec![];

    let split = Array1::linspace(0., 6000000., count + 1);

    for i in 0..split.len() - 1 {
        sqls.push(format!(
            "select 
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
                l_comment from lineitem where l_orderkey > {} and l_orderkey <= {}",
            split[[i,]] as usize,
            split[[i + 1,]] as usize
        ));
    }

    sqls
}

pub fn run(nq: usize) {
    let conn = env::var("POSTGRES_URL").unwrap();
    let queries = get_sqls(nq);
    let queries: Vec<_> = queries.iter().map(AsRef::as_ref).collect();

    Python::with_gil(|py| {
        write_pandas(py, &conn, &queries, false).unwrap();
    });
}

fn main() {
    run(1);
}
