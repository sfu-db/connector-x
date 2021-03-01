use connector_agent_python::pandas::write_pandas;
use ndarray::Array1;
use pprof::protos::Message;
use pyo3::Python;
use std::env;
use std::fs::File;
use std::io::Write;

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

fn main() {
    let conn = env::var("POSTGRES_URL").unwrap();
    let queries = get_sqls(10);
    let queries: Vec<_> = queries.iter().map(AsRef::as_ref).collect();

    let guard = pprof::ProfilerGuard::new(100).unwrap();

    Python::with_gil(|py| {
        write_pandas(py, &conn, &queries, false).unwrap();
    });

    if let Ok(report) = guard.report().build() {
        let file = File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();

        let mut file = File::create("profile.pb").unwrap();
        let profile = report.pprof().unwrap();

        let mut content = Vec::new();
        profile.encode(&mut content).unwrap();
        file.write_all(&content).unwrap();
    };
}
