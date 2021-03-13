use connector_agent::{
    data_sources::dummy::MixedSource, transport::DummyMemoryTransport,
    writers::memory::MemoryWriter, DataOrder, DataType, Dispatcher, PartitionWriter, Result,
    Source, Writer,
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
            &["a", "b", "c"],
            &[
                DataType::I64(false),
                DataType::F64(true),
                DataType::String(true),
            ],
            DataOrder::ColumnMajor,
        )
        .unwrap();
}

#[test]
#[should_panic]
fn mixed_source_col_major() {
    let mut source = MixedSource::new(&["a"], &[DataType::F64(false)]);
    source.set_data_order(DataOrder::ColumnMajor).unwrap();
}

#[test]
fn write_mixed_array() -> Result<()> {
    let mut dw = MemoryWriter::new();
    dw.allocate(
        11,
        &["a", "b", "c", "d", "e"],
        &[
            DataType::I64(false),
            DataType::F64(false),
            DataType::I64(false),
            DataType::String(false),
            DataType::F64(false),
            DataType::String(false),
        ],
        DataOrder::RowMajor,
    )
    .unwrap();
    let writers = dw.partition_writers(&[4, 7])?;

    writers.into_par_iter().for_each(|mut writer| {
        for row in 0..writer.nrows() {
            writer.write(row as i64).unwrap();
            writer.write(row as f64).unwrap();
            writer.write(row as i64 + 1000).unwrap();
            writer.write(row.to_string()).unwrap();
            writer.write(row as f64 + 1000.).unwrap();
            writer.write((row + 1000).to_string()).unwrap();
        }
    });
    for (col, _) in dw.schema().into_iter().enumerate() {
        match col {
            0 => {
                assert_eq!(
                    dw.column_view::<i64>(col).unwrap(),
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
                    dw.column_view::<i64>(col).unwrap(),
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
    Ok(())
}

#[test]
fn test_mixed() {
    let schema = [
        DataType::I64(false),
        DataType::F64(false),
        DataType::String(false),
        DataType::F64(false),
        DataType::Bool(false),
        DataType::String(false),
        DataType::F64(false),
    ];
    let nrows = vec![4, 7];
    let ncols = schema.len();
    let queries: Vec<String> = nrows.iter().map(|v| format!("{},{}", v, ncols)).collect();

    let mut writer = MemoryWriter::new();
    let dispatcher = Dispatcher::<_, _, DummyMemoryTransport>::new(
        MixedSource::new(&["a", "b", "c", "d", "e", "f", "g"], &schema),
        &mut writer,
        &queries,
    );
    dispatcher.run().expect("run dispatcher");

    for (col, _) in writer.schema().into_iter().enumerate() {
        match col {
            0 => {
                assert_eq!(
                    writer.column_view::<i64>(col).unwrap(),
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
