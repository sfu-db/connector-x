#![feature(custom_test_frameworks)]
#![test_runner(criterion::runner)]

use connector_agent_python::pandas::write_pandas;
use criterion::{black_box, Criterion};
use criterion_macro::criterion;
use ndarray::Array1;
use pprof::criterion::{Output, PProfProfiler};
use pyo3::Python;
use std::env;

fn config() -> Criterion {
    Criterion::default()
        // .measurement_time(std::time::Duration::from_secs(120))
        // .sample_size(30)
        .with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)))
}

#[criterion(config())]
fn benchmark(c: &mut Criterion) {
    let conn = env::var("POSTGRES_URL").unwrap();
    let queries = get_sqls(1);
    let queries: Vec<_> = queries.iter().map(AsRef::as_ref).collect();
    let schema = [
        "int64", "int64", "int64", "int64", "float64", "float64", "float64", "float64", "string",
        "string", "date", "date", "date", "string", "string", "string",
    ];

    c.bench_function("tpch 6000", |b| {
        b.iter(|| {
            Python::with_gil(|py| {
                write_pandas(py, &conn, &queries, black_box(&schema), false).unwrap();
            })
        })
    });
}

fn get_sqls(count: usize) -> Vec<String> {
    let mut sqls = vec![];

    let split = Array1::linspace(0., 6000., count + 1);

    for i in 0..split.len() - 1 {
        sqls.push(format!(
            "select * from lineitem where l_orderkey > {} and l_orderkey <= {}",
            split[[i,]] as usize,
            split[[i + 1,]] as usize
        ));
    }

    sqls
}
