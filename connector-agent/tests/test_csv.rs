use connector_agent::destinations::memory::MemoryDestination;
use connector_agent::sources::{csv::CSVSource, Produce, Source, SourcePartition};
use connector_agent::{transport::CSVMemoryTransport, Dispatcher, DummyTypeSystem};
use ndarray::array;

#[test]
#[should_panic]
fn no_file() {
    let mut source = CSVSource::new(&[]);
    source.set_queries(&["./a_fake_file.csv"]);
    let partitions = source.partition().unwrap();
    for mut p in partitions {
        p.prepare().expect("run query");
    }
}

#[test]
#[should_panic]
fn empty_file() {
    let mut source = CSVSource::new(&[]);
    source.set_queries(&["./tests/data/empty.csv"]);
    let mut partitions = source.partition().unwrap();
    for p in &mut partitions {
        p.prepare().expect("run query");
    }
    assert_eq!(0, partitions[0].nrows());
    assert_eq!(0, partitions[0].ncols());
    let parser = partitions[0].parser();

    let _v: i64 = parser.unwrap().produce().expect("produce from emtpy");
}

#[test]
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
        DummyTypeSystem::String(false),
        DummyTypeSystem::String(false),
        DummyTypeSystem::I64(false),
        DummyTypeSystem::F64(false),
        DummyTypeSystem::F64(false),
    ]);
    source.set_queries(&["./tests/data/uspop_0.csv"]);

    let mut partitions = source.partition().unwrap();

    let mut partition = partitions.remove(0);
    partition.prepare().expect("run query");

    assert_eq!(3, partition.nrows());
    assert_eq!(5, partition.ncols());

    let mut results: Vec<Value> = Vec::new();
    let mut parser = partition.parser().unwrap();
    for _i in 0..3 {
        results.push(Value::City(parser.produce().expect("parse city")));
        results.push(Value::State(parser.produce().expect("parse state")));
        results.push(Value::Population(
            parser.produce().expect("parse population"),
        ));
        results.push(Value::Longitude(parser.produce().expect("parse longitude")));
        results.push(Value::Latitude(parser.produce().expect("parse latitude")));
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
    let schema = [DummyTypeSystem::I64(false); 5];
    let files = ["./tests/data/uint_0.csv", "./tests/data/uint_1.csv"];
    let source = CSVSource::new(&schema);

    let mut destination = MemoryDestination::new();
    let dispatcher = Dispatcher::<_, _, CSVMemoryTransport>::new(source, &mut destination, &files);

    dispatcher.run().expect("run dispatcher");

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
        destination.buffer_view::<i64>(0).unwrap()
    );
}
