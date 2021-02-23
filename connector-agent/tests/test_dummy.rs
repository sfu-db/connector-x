use connector_agent::data_sources::dummy::{
    BoolSourceBuilder, F64SourceBuilder, StringSourceBuilder, U64SourceBuilder,
};
use connector_agent::writers::{
    dummy::{BoolWriter, F64Writer, StringWriter, U64Writer},
    Writer,
};
use connector_agent::{DataOrder, DataType, Dispatcher};
use ndarray::array;

#[test]
#[should_panic]
fn wrong_data_type() {
    let mut dw = U64Writer::new();
    let _ = dw
        .allocate(
            11,
            vec![
                DataType::U64,
                DataType::U64,
                DataType::U64,
                DataType::F64,
                DataType::U64,
            ],
            DataOrder::RowMajor,
        )
        .unwrap();
}

#[test]
#[should_panic]
fn wrong_string_data_type() {
    let mut dw = StringWriter::new();
    let _ = dw
        .allocate(
            11,
            vec![
                DataType::String,
                DataType::String,
                DataType::U64,
                DataType::String,
                DataType::String,
            ],
            DataOrder::RowMajor,
        )
        .unwrap();
}

#[test]
fn write_array() {
    let schema = [DataType::U64; 5];
    let queries = ["4", "7"];

    let mut writer = U64Writer::new();
    let dispatcher = Dispatcher::new(U64SourceBuilder {}, &mut writer, &queries, &schema);
    dispatcher.run_checked().expect("run dispatcher");

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
        writer.buffer()
    );
}

#[test]
fn write_string_array() {
    let schema = [DataType::String; 5];
    let queries = ["4", "7"];

    let mut writer = StringWriter::new();
    let dispatcher = Dispatcher::new(StringSourceBuilder {}, &mut writer, &queries, &schema);
    dispatcher.run_checked().expect("run dispatcher");

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
        writer.buffer()
    );
}

#[test]
fn write_array_bool() {
    let schema = [DataType::Bool; 5];
    let queries = ["4", "7"];

    let mut writer = BoolWriter::new();
    let dispatcher = Dispatcher::new(BoolSourceBuilder {}, &mut writer, &queries, &schema);
    dispatcher.run_checked().expect("run dispatcher");
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
        writer.buffer()
    );
}

#[test]
fn write_array_f64() {
    let schema = [DataType::F64; 5];
    let queries = ["4", "7"];

    let mut writer = F64Writer::new();
    let dispatcher = Dispatcher::new(F64SourceBuilder {}, &mut writer, &queries, &schema);
    dispatcher.run_checked().expect("run dispatcher");

    assert_eq!(
        array![
            [0.0, 0.5, 1.0, 1.5, 2.0],
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
        writer.buffer()
    );
}
