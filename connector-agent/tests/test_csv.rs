use connector_agent::data_sources::{
    csv::{CSVSource, CSVSourceBuilder},
    PartitionedSource, Produce,
};
use connector_agent::writers::memory::MemoryWriter;
use connector_agent::{DataType, Dispatcher};
use ndarray::array;

#[test]
#[should_panic]
fn no_file() {
    let mut source = CSVSource::new();
    source.prepare("./a_fake_file.csv").expect("run query");
}

#[test]
#[should_panic]
fn empty_file() {
    let mut source = CSVSource::new();
    source.prepare("./tests/data/empty.csv").expect("run query");

    assert_eq!(0, source.nrows);
    assert_eq!(0, source.ncols);
    let _v: u64 = source.produce().expect("produce from emtpy");
}

#[test]
fn load_and_parse() {
    #[derive(Debug, PartialEq)]
    enum Value {
        City(String),
        State(String),
        Population(u64),
        Longitude(f64),
        Latitude(f64),
    }

    let mut source = CSVSource::new();
    source
        .prepare("./tests/data/uspop_0.csv")
        .expect("run query");

    assert_eq!(3, source.nrows);
    assert_eq!(5, source.ncols);

    let mut results: Vec<Value> = Vec::new();
    for _i in 0..source.nrows {
        results.push(Value::City(source.produce().expect("parse city")));
        results.push(Value::State(source.produce().expect("parse state")));
        results.push(Value::Population(
            source.produce().expect("parse population"),
        ));
        results.push(Value::Longitude(source.produce().expect("parse longitude")));
        results.push(Value::Latitude(source.produce().expect("parse latitude")));
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
fn test_csv() {
    let schema = [DataType::U64(false); 5];
    let files = ["./tests/data/uint_0.csv", "./tests/data/uint_1.csv"];
    let mut writer = MemoryWriter::new();
    let dispatcher = Dispatcher::new(CSVSourceBuilder::new(), &mut writer, &files, &schema);

    dispatcher.run_checked().expect("run dispatcher");

    assert_eq!(
        array![
            [0, 1, 2, 3, 4],
            [5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14],
            [15, 16, 17, 18, 19],
            [20, 21, 22, 23, 24],
            [25, 26, 27, 28, 29],
            [30, 31, 32, 33, 34],
            [35, 36, 37, 38, 39],
            [40, 41, 42, 43, 44],
            [45, 46, 47, 48, 49],
            [50, 51, 52, 53, 54],
        ],
        writer.buffer_view::<u64>(0).unwrap()
    );
}
