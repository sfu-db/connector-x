use arrow::array::Int64Array;
use connectorx::prelude::*;
use connectorx::{
    destinations::arrow::{ArrowDestination, ArrowTypeSystem},
    sources::{
        csv::{CSVSource, CSVTypeSystem},
        PartitionParser,
    },
    sql::CXQuery,
    transports::CSVArrowTransport,
};

#[test]
#[should_panic]
#[ignore]
fn no_file() {
    let mut source = CSVSource::new(&[]);
    source.set_queries(&[CXQuery::naked("./a_fake_file.csv")]);
    let partitions = source.partition().unwrap();
    for mut p in partitions {
        p.result_rows().expect("run query");
    }
}

#[test]
#[should_panic]
#[ignore]
fn empty_file() {
    let mut source = CSVSource::new(&[]);
    source.set_queries(&[CXQuery::naked("./tests/data/empty.csv")]);
    let mut partitions = source.partition().unwrap();
    for p in &mut partitions {
        p.result_rows().expect("run query");
    }
    assert_eq!(0, partitions[0].nrows());
    assert_eq!(0, partitions[0].ncols());
    let mut parser = partitions[0].parser().unwrap();

    parser.fetch_next().unwrap();

    let _v: i64 = parser.produce().expect("produce from emtpy");
}

#[test]
#[ignore]
fn load_and_parse() {
    #[derive(Debug, PartialEq)]
    enum Value {
        City(String),
        State(String),
        Population(i64),
        Longitude(f64),
        Latitude(f64),
    }

    let mut source = CSVSource::new(&[
        CSVTypeSystem::String(false),
        CSVTypeSystem::String(false),
        CSVTypeSystem::I64(false),
        CSVTypeSystem::F64(false),
        CSVTypeSystem::F64(false),
    ]);
    source.set_queries(&[CXQuery::naked("./tests/data/uspop_0.csv")]);

    let mut partitions = source.partition().unwrap();

    let mut partition = partitions.remove(0);
    partition.result_rows().expect("run query");

    assert_eq!(3, partition.nrows());
    assert_eq!(5, partition.ncols());

    let mut results: Vec<Value> = Vec::new();
    let mut parser = partition.parser().unwrap();
    loop {
        let (n, is_last) = parser.fetch_next().unwrap();
        for _i in 0..n {
            results.push(Value::City(parser.produce().expect("parse city")));
            results.push(Value::State(parser.produce().expect("parse state")));
            results.push(Value::Population(
                parser.produce().expect("parse population"),
            ));
            results.push(Value::Longitude(parser.produce().expect("parse longitude")));
            results.push(Value::Latitude(parser.produce().expect("parse latitude")));
        }
        if is_last {
            break;
        }
    }
    assert_eq!(
        vec![
            Value::City(String::from("Kenai")),
            Value::State(String::from("AK")),
            Value::Population(7610),
            Value::Longitude(60.5544444),
            Value::Latitude(-151.2583333),
            Value::City(String::from("Selma")),
            Value::State(String::from("AL")),
            Value::Population(18980),
            Value::Longitude(32.4072222),
            Value::Latitude(-87.0211111),
            Value::City(String::from("El Mirage")),
            Value::State(String::from("AZ")),
            Value::Population(32308),
            Value::Longitude(33.6130556),
            Value::Latitude(-112.3238889)
        ],
        results
    );
}

#[test]
#[ignore]
fn test_csv() {
    let schema = [CSVTypeSystem::I64(false); 5];
    let files = [
        CXQuery::naked("./tests/data/uint_0.csv"),
        CXQuery::naked("./tests/data/uint_1.csv"),
    ];
    let source = CSVSource::new(&schema);

    let mut destination = ArrowDestination::new();
    let dispatcher =
        Dispatcher::<_, _, CSVArrowTransport>::new(source, &mut destination, &files, None);

    dispatcher.run().expect("run dispatcher");

    let result = destination.arrow().unwrap();

    println!("result len: {}", result.len());
    assert!(result.len() == 2);

    for rb in result {
        for i in 0..5 {
            let col = rb.column(i).as_any().downcast_ref::<Int64Array>().unwrap();
            assert!(
                col.eq(&Int64Array::from_iter_values(
                    (4i64..=10).map(|v| v * 5 + i as i64),
                )) || col.eq(&Int64Array::from_iter_values(
                    (0i64..4).map(|v| v * 5 + i as i64),
                ))
            );
        }
    }
}

#[test]
#[ignore]
fn test_csv_infer_schema() {
    let files = [CXQuery::naked("./tests/data/infer_0.csv")];
    let source = CSVSource::new(&[]);

    let mut writer = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, CSVArrowTransport>::new(source, &mut writer, &files, None);

    dispatcher.run().expect("run dispatcher");

    let expected_schema = vec![
        ArrowTypeSystem::Int64(false),
        ArrowTypeSystem::Float64(false),
        ArrowTypeSystem::Boolean(true),
        ArrowTypeSystem::LargeUtf8(true),
        ArrowTypeSystem::Float64(false),
        ArrowTypeSystem::LargeUtf8(true),
        ArrowTypeSystem::LargeUtf8(false),
        ArrowTypeSystem::LargeUtf8(true),
    ];

    assert_eq!(expected_schema, writer.schema());
}
