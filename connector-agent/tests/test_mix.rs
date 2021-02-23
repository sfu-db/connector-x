use connector_agent::{
    data_sources::mixed::MixedSourceBuilder, writers::mixed::MemoryWriter, DataOrder, DataType,
    Dispatcher, PartitionWriter, SourceBuilder, Writer,
};
use ndarray::array;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[test]
#[should_panic]
fn mixed_writer_col_major() {
    let mut dw = MemoryWriter::new();
    let _ = dw
        .allocate(
            11,
            vec![DataType::U64, DataType::F64, DataType::String],
            DataOrder::ColumnMajor,
        )
        .unwrap();
}

#[test]
#[should_panic]
fn mixed_source_col_major() {
    let mut source = MixedSourceBuilder::new();
    source.set_data_order(DataOrder::ColumnMajor).unwrap();
}

#[test]
fn write_mixed_array() {
    let mut dw = MemoryWriter::new();
    dw.allocate(
        11,
        vec![
            DataType::U64,
            DataType::F64,
            DataType::U64,
            DataType::String,
            DataType::F64,
            DataType::String,
        ],
        DataOrder::RowMajor,
    )
    .unwrap();
    let writers = dw.partition_writers(&[4, 7]);

    writers.into_par_iter().for_each(|mut writer| {
        for row in 0..writer.nrows() {
            writer.write_checked(row, 0, row as u64).unwrap();
            writer.write_checked(row, 1, row as f64).unwrap();
            writer.write_checked(row, 2, row as u64 + 1000).unwrap();
            writer.write_checked(row, 3, row.to_string()).unwrap();
            writer.write_checked(row, 4, row as f64 + 1000.).unwrap();
            writer
                .write_checked(row, 5, (row + 1000).to_string())
                .unwrap();
        }
    });
    for (col, _) in dw.schema().into_iter().enumerate() {
        match col {
            0 => {
                assert_eq!(
                    dw.column_view::<u64>(col).unwrap(),
                    array![0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 6]
                )
            }
            1 => {
                assert_eq!(
                    dw.column_view::<f64>(col).unwrap(),
                    array![0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
                )
            }
            2 => {
                assert_eq!(
                    dw.column_view::<u64>(col).unwrap(),
                    array![1000, 1001, 1002, 1003, 1000, 1001, 1002, 1003, 1004, 1005, 1006]
                )
            }
            3 => {
                assert_eq!(
                    dw.column_view::<String>(col).unwrap(),
                    array![
                        "0".to_string(),
                        "1".to_string(),
                        "2".to_string(),
                        "3".to_string(),
                        "0".to_string(),
                        "1".to_string(),
                        "2".to_string(),
                        "3".to_string(),
                        "4".to_string(),
                        "5".to_string(),
                        "6".to_string()
                    ]
                )
            }
            4 => {
                assert_eq!(
                    dw.column_view::<f64>(col).unwrap(),
                    array![
                        1000.0, 1001.0, 1002.0, 1003.0, 1000.0, 1001.0, 1002.0, 1003.0, 1004.0,
                        1005.0, 1006.0
                    ]
                )
            }
            5 => {
                assert_eq!(
                    dw.column_view::<String>(col).unwrap(),
                    array![
                        "1000".to_string(),
                        "1001".to_string(),
                        "1002".to_string(),
                        "1003".to_string(),
                        "1000".to_string(),
                        "1001".to_string(),
                        "1002".to_string(),
                        "1003".to_string(),
                        "1004".to_string(),
                        "1005".to_string(),
                        "1006".to_string()
                    ]
                )
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_mixed() {
    let schema = vec![
        DataType::U64,
        DataType::F64,
        DataType::String,
        DataType::F64,
        DataType::Bool,
        DataType::String,
        DataType::F64,
    ];
    let nrows = vec![4, 7];
    let ncols = schema.len();
    let queries: Vec<String> = nrows.iter().map(|v| format!("{},{}", v, ncols)).collect();

    let mut writer = MemoryWriter::new();
    let dispatcher = Dispatcher::new(MixedSourceBuilder::new(), &mut writer, &queries, &schema);
    dispatcher.run_checked().expect("run dispatcher");

    for (col, _) in writer.schema().into_iter().enumerate() {
        match col {
            0 => {
                assert_eq!(
                    writer.column_view::<u64>(col).unwrap(),
                    array![0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 6]
                )
            }
            1 => {
                assert_eq!(
                    writer.column_view::<f64>(col).unwrap(),
                    array![0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
                )
            }
            2 => {
                assert_eq!(
                    writer.column_view::<String>(col).unwrap(),
                    array![
                        "0".to_string(),
                        "1".to_string(),
                        "2".to_string(),
                        "3".to_string(),
                        "0".to_string(),
                        "1".to_string(),
                        "2".to_string(),
                        "3".to_string(),
                        "4".to_string(),
                        "5".to_string(),
                        "6".to_string()
                    ]
                )
            }
            3 => {
                assert_eq!(
                    writer.column_view::<f64>(col).unwrap(),
                    array![0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
                )
            }
            4 => {
                assert_eq!(
                    writer.column_view::<bool>(col).unwrap(),
                    array![true, false, true, false, true, false, true, false, true, false, true]
                )
            }
            5 => {
                assert_eq!(
                    writer.column_view::<String>(col).unwrap(),
                    array![
                        "0".to_string(),
                        "1".to_string(),
                        "2".to_string(),
                        "3".to_string(),
                        "0".to_string(),
                        "1".to_string(),
                        "2".to_string(),
                        "3".to_string(),
                        "4".to_string(),
                        "5".to_string(),
                        "6".to_string()
                    ]
                )
            }
            6 => {
                assert_eq!(
                    writer.column_view::<f64>(col).unwrap(),
                    array![0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
                )
            }
            _ => unreachable!(),
        }
    }
}
