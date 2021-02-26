#![feature(custom_test_frameworks)]
#![test_runner(criterion::runner)]

use connector_agent::pg::read_pg;
use criterion::{black_box, Criterion};
use criterion_macro::criterion;
use ndarray::Array1;
use pprof::criterion::{Output, PProfProfiler};
use std::env;
use tokio::runtime::Runtime;

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

    let schema = r#"{"fields": [{"name": "l_orderkey", "nullable": false, "type": {"name": "int", "bitWidth": 64, "isSigned": false}, "children": []}, {"name": "l_partkey", "nullable": false, "type": {"name": "int", "bitWidth": 64, "isSigned": false}, "children": []}, {"name": "l_suppkey", "nullable": false, "type": {"name": "int", "bitWidth": 64, "isSigned": false}, "children": []}, {"name": "l_linenumber", "nullable": false, "type": {"name": "int", "bitWidth": 64, "isSigned": false}, "children": []}, {"name": "l_quantity", "nullable": false, "type": {"name": "floatingpoint", "precision": "DOUBLE"}, "children": []}, {"name": "l_extendedprice", "nullable": false, "type": {"name": "floatingpoint", "precision": "DOUBLE"}, "children": []}, {"name": "l_discount", "nullable": false, "type": {"name": "floatingpoint", "precision": "DOUBLE"}, "children": []}, {"name": "l_tax", "nullable": false, "type": {"name": "floatingpoint", "precision": "DOUBLE"}, "children": []}, {"name": "l_returnflag", "nullable": false, "type": {"name": "utf8"}, "children": []}, {"name": "l_linestatus", "nullable": false, "type": {"name": "utf8"}, "children": []}, {"name": "l_shipdate", "nullable": false, "type": {"name": "utf8"}, "children": []}, {"name": "l_commitdate", "nullable": false, "type": {"name": "utf8"}, "children": []}, {"name": "l_receiptdate", "nullable": false, "type": {"name": "utf8"}, "children": []}, {"name": "l_shipinstruct", "nullable": false, "type": {"name": "utf8"}, "children": []}, {"name": "l_shipmode", "nullable": false, "type": {"name": "utf8"}, "children": []}, {"name": "l_comment", "nullable": false, "type": {"name": "utf8"}, "children": []}], "metadata": {}}"#;

    let runtime = Runtime::new().unwrap();
    c.bench_function("tpch-old 6000", |b| {
        b.iter(|| {
            runtime
                .block_on(read_pg(&conn, &queries, black_box(&schema)))
                .unwrap();
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

// import pyarrow as pa
// import json

// def field_to_json(field):
//     json = {
//         "name": field.name,
//         "nullable": field.nullable,
//     }
//     if isinstance(field.type, pa.ListType):
//         json = {
//             **json,
//             "type": {"name": "list"},
//             "children": [field_to_json(field.type.value_field)],
//         }
//     elif field.type == pa.float64():
//         json = {
//             **json,
//             "type": {"name": "floatingpoint", "precision": "DOUBLE"},
//             "children": [],
//         }
//     elif field.type == pa.uint64():
//         json = {
//             **json,
//             "type": {"name": "int", "bitWidth": 64, "isSigned": False},
//             "children": [],
//         }
//     elif field.type == pa.string():
//         json = {
//             **json,
//             "type": {"name": "utf8"},
//             "children": [],
//         }
//     elif field.type == pa.date32():
//         json = {
//             **json,
//             "type": {"name": "date", "unit": "DAY"},
//             "children": [],
//         }
//     elif isinstance(field.type, pa.StructType):
//         json = {
//             **json,
//             "type": {"name": "struct"},
//             "children": [
//                 field_to_json(field.type[i]) for i in range(field.type.num_fields)
//             ],
//         }
//     else:
//         raise NotImplementedError(field.type)

//     return json

// def schema_to_json(schema):
//     return {
//         "fields": [field_to_json(schema.field(name)) for name in schema.names],
//         "metadata": {},
//     }

// SCHEMA = pa.schema(
//     [
//         pa.field("l_orderkey", pa.uint64(), False),
//         pa.field("l_partkey", pa.uint64(), False),
//         pa.field("l_suppkey", pa.uint64(), False),
//         pa.field("l_linenumber", pa.uint64(), False),
//         pa.field("l_quantity", pa.float64(), False),
//         pa.field("l_extendedprice", pa.float64(), False),
//         pa.field("l_discount", pa.float64(), False),
//         pa.field("l_tax", pa.float64(), False),
//         pa.field("l_returnflag", pa.string(), False),
//         pa.field("l_linestatus", pa.string(), False),
//         # pa.field("l_shipdate", pa.date32(), False),
//         # pa.field("l_commitdate", pa.date32(), False),
//         # pa.field("l_receiptdate", pa.date32(), False),
//         pa.field("l_shipdate", pa.string(), False),
//         pa.field("l_commitdate", pa.string(), False),
//         pa.field("l_receiptdate", pa.string(), False),
//         pa.field("l_shipinstruct", pa.string(), False),
//         pa.field("l_shipmode", pa.string(), False),
//         pa.field("l_comment", pa.string(), False),
//     ]
// )
// json.dumps(schema_to_json(SCHEMA))
