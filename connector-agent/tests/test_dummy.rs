use connector_agent::data_sources::dummy::{U64CounterSource, StringSource, BoolCounterSource, F64CounterSource};
use connector_agent::writers::{dummy::{U64Writer, StringWriter, BoolWriter, F64Writer}, Writer};
use connector_agent::{DataType, Worker};
use ndarray::array;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[test]
#[should_panic]
fn wrong_data_type() {
    let _ = U64Writer::allocate(
        11,
        vec![
            DataType::U64,
            DataType::U64,
            DataType::U64,
            DataType::F64,
            DataType::U64,
        ],
    )
    .unwrap();
}

#[test]
#[should_panic]
fn wrong_string_data_type() {
    let _ = StringWriter::allocate(
        11,
        vec![
            DataType::String,
            DataType::String,
            DataType::U64,
            DataType::String,
            DataType::String,
        ],
    )
    .unwrap();
}

#[test]
fn write_array() {
    let mut dw = U64Writer::allocate(11, vec![DataType::U64; 5]).unwrap();
    let schema = dw.schema().to_vec();
    let writers = dw.partition_writers(&[4, 7]);

    writers.into_par_iter().for_each(|writer| {
        Worker::new(U64CounterSource::new(), writer, schema.clone(), "")
            .run_checked()
            .expect("Worker failed")
    });

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
        dw.buffer()
    );
}

#[test]
fn write_string_array() {
    let mut dw = StringWriter::allocate(11, vec![DataType::String; 5]).unwrap();
    let schema = dw.schema().to_vec();

    let writers = dw.partition_writers(&[4, 7]);

    writers.into_par_iter().for_each(|writer| {
        Worker::new(StringSource::new(), writer, schema.clone(), "")
            .run_checked()
            .expect("Worker failed")
    });

    assert_eq!(
        array![
            ["0", "1", "2", "3", "4"],
            ["5", "6", "7", "8", "9"],
            ["10", "11", "12", "13", "14"],
            ["15", "16", "17", "18", "19"],
            ["0", "1", "2", "3", "4"],
            ["5", "6", "7", "8", "9"],
            ["10", "11", "12", "13", "14"],
            ["15", "16", "17", "18", "19"],
            ["20", "21", "22", "23", "24"],
            ["25", "26", "27", "28", "29"],
            ["30", "31", "32", "33", "34"]
        ],
        dw.buffer()
    );
}

#[test]
fn write_array_bool() {
    let mut dw = BoolWriter::allocate(11, vec![DataType::Bool; 5]).unwrap();
    let schema = dw.schema().to_vec();
    let writers = dw.partition_writers(&[4, 7]);

    writers.into_par_iter().for_each(|writer| {
        Worker::new(BoolCounterSource::new(), writer, schema.clone(), "")
            .run_checked()
            .expect("Worker failed")
    });

    assert_eq!(
        array![
            [false, true, false, true, false],
            [true, false, true, false, true],
            [false, true, false, true, false],
            [true, false, true, false, true],
            [false, true, false, true, false],
            [true, false, true, false, true],
            [false, true, false, true, false],
            [true, false, true, false, true],
            [false, true, false, true, false],
            [true, false, true, false, true],
            [false, true, false, true, false],
        ],
        dw.buffer()
    );
}

#[test]
fn write_array_f64() {
    let mut dw = F64Writer::allocate(11, vec![DataType::F64; 5]).unwrap();
    let schema = dw.schema().to_vec();
    let writers = dw.partition_writers(&[4, 7]);

    writers
        .into_par_iter()
        .for_each(|writer| Worker::new(F64CounterSource::new(), writer, schema.clone(), "").run_checked().expect("Worker failed"));


    assert_eq!(
        array![[0.0, 0.5, 1.0, 1.5, 2.0],
               [2.5, 3.0, 3.5, 4.0, 4.5],
               [5.0, 5.5, 6.0, 6.5, 7.0],
               [7.5, 8.0, 8.5, 9.0, 9.5],
               [0.0, 0.5, 1.0, 1.5, 2.0],
               [2.5, 3.0, 3.5, 4.0, 4.5],
               [5.0, 5.5, 6.0, 6.5, 7.0],
               [7.5, 8.0, 8.5, 9.0, 9.5],
               [10.0, 10.5, 11.0, 11.5, 12.0],
               [12.5, 13.0, 13.5, 14.0, 14.5],
               [15.0, 15.5, 16.0, 16.5, 17.0]
            ],
        dw.buffer()
    );
}


