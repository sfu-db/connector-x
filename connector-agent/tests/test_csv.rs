use connector_agent::data_sources::csv::CSVSource;
use connector_agent::writers::{dummy::U64Writer, Writer};
use connector_agent::{DataType, Worker};
use ndarray::array;
use rayon::prelude::*;

#[test]
fn load_uint_csv() {
    let files = vec!["./tests/data/uint_0.csv", "./tests/data/uint_1.csv"];
    let mut dw = U64Writer::allocate(11, vec![DataType::U64; 5]).unwrap();
    let schema = dw.schema().to_vec();
    let writers = dw.partition_writer(&[4, 7]);

    writers
        .into_par_iter()
        .zip_eq(files)
        .for_each(|(writer, file)| Worker::new(CSVSource::new(file), writer, schema.clone())
            .run_safe()
            .expect("Worker failed"));

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