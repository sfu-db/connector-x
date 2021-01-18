use connector_agent::data_sources::dummy::DummySource;
use connector_agent::writers::{dummy::DummyWriter, Writer};
use connector_agent::{DataType, Worker};
use ndarray::array;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[test]
#[should_panic]
fn wrong_data_type() {
    let mut dw = DummyWriter::allocate(11, vec![DataType::U64, DataType::U64, DataType::U64, DataType::F64, DataType::U64]);
    let schema = dw.schema().to_vec();
    let writers = dw.partition_writer(&[4, 7]);

    writers
        .into_par_iter()
        .for_each(|writer| Worker::new(DummySource::new(), writer, schema.clone()).run_safe().expect("Worker failed"));
}

#[test]
fn write_array() {
    let mut dw = DummyWriter::allocate(11, vec![DataType::U64; 5]);
    let schema = dw.schema().to_vec();
    let writers = dw.partition_writer(&[4, 7]);

    writers
        .into_par_iter()
        .for_each(|writer| Worker::new(DummySource::new(), writer, schema.clone()).run_safe().expect("Worker failed"));

    assert_eq!(
        array![
            [0, 1, 2, 3, 4],
            [5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14],
            [15, 16, 17, 18, 19],
            [0, 1, 2, 3, 4],
            [5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14],
            [15, 16, 17, 18, 19],
            [20, 21, 22, 23, 24],
            [25, 26, 27, 28, 29],
            [30, 31, 32, 33, 34]
        ],
        dw.buffer
    );
}
