use connector_agent::data_sources::{
    csv::{CSVSource, CSVSourceBuilder},
    DataSource, Parse,
};
use connector_agent::writers::dummy::U64Writer;
use connector_agent::{DataType, Dispatcher};
use ndarray::array;

#[test]
#[should_panic]
fn no_file() {
    let mut source = CSVSource::new();
    source.run_query("./a_fake_file.csv").expect("run query");
}

#[test]
#[should_panic]
fn empty_file() {
    let mut source = CSVSource::new();
    source
        .run_query("./tests/data/empty.csv")
        .expect("run query");

    assert_eq!(0, source.nrows);
    assert_eq!(0, source.ncols);
    let _v: u64 = source.parse().expect("produce from emtpy");
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
        .run_query("./tests/data/uspop_0.csv")
        .expect("run query");

    assert_eq!(3, source.nrows);
    assert_eq!(5, source.ncols);

    let mut results: Vec<Value> = Vec::new();
    for _i in 0..source.nrows {
        results.push(Value::City(source.parse().expect("parse city")));
        results.push(Value::State(source.parse().expect("parse state")));
        results.push(Value::Population(source.parse().expect("parse population")));
        results.push(Value::Longitude(source.parse().expect("parse longitude")));
        results.push(Value::Latitude(source.parse().expect("parse latitude")));
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
    let schema = vec![DataType::U64; 5];
    let files = vec![
        "./tests/data/uint_0.csv".to_string(),
        "./tests/data/uint_1.csv".to_string(),
    ];
    let mut dispatcher = Dispatcher::new(CSVSourceBuilder::new(), schema, files);

    let dw = dispatcher
        .run_checked::<U64Writer>()
        .expect("run dispatcher");

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
        dw.buffer()
    );
}
