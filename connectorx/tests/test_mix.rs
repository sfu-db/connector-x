use connectorx::{
    destinations::memory::MemoryDestination, sources::dummy::DummySource, sql::CXQuery,
    transports::DummyMemoryTransport, DataOrder, Destination, DestinationPartition, Dispatcher,
    DummyTypeSystem, Result, Source,
};
use ndarray::array;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[test]
#[should_panic]
fn mixed_destination_col_major() {
    let mut dw = MemoryDestination::new();
    let _ = dw
        .allocate(
            11,
            &["a", "b", "c"],
            &[
                DummyTypeSystem::I64(false),
                DummyTypeSystem::F64(true),
                DummyTypeSystem::String(true),
            ],
            DataOrder::ColumnMajor,
        )
        .unwrap();
}

#[test]
#[should_panic]
fn mixed_source_col_major() {
    let mut source = DummySource::new(&["a"], &[DummyTypeSystem::F64(false)]);
    source.set_data_order(DataOrder::ColumnMajor).unwrap();
}

#[test]
fn write_mixed_array() -> Result<()> {
    let mut dw = MemoryDestination::new();
    dw.allocate(
        11,
        &["a", "b", "c", "d", "e"],
        &[
            DummyTypeSystem::I64(false),
            DummyTypeSystem::F64(false),
            DummyTypeSystem::I64(false),
            DummyTypeSystem::String(false),
            DummyTypeSystem::F64(false),
            DummyTypeSystem::String(false),
        ],
        DataOrder::RowMajor,
    )
    .unwrap();
    let destinations = dw.partition(&[4, 7])?;

    destinations.into_par_iter().for_each(|mut destination| {
        for row in 0..destination.nrows() {
            destination.write(row as i64).unwrap();
            destination.write(row as f64).unwrap();
            destination.write(row as i64 + 1000).unwrap();
            destination.write(row.to_string()).unwrap();
            destination.write(row as f64 + 1000.).unwrap();
            destination.write((row + 1000).to_string()).unwrap();
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
        DummyTypeSystem::I64(false),
        DummyTypeSystem::F64(false),
        DummyTypeSystem::String(false),
        DummyTypeSystem::F64(false),
        DummyTypeSystem::Bool(false),
        DummyTypeSystem::String(false),
        DummyTypeSystem::F64(false),
    ];
    let nrows = vec![4, 7];
    let ncols = schema.len();
    let queries: Vec<CXQuery> = nrows
        .iter()
        .map(|v| CXQuery::naked(format!("{},{}", v, ncols)))
        .collect();

    let mut destination = MemoryDestination::new();
    let dispatcher = Dispatcher::<_, _, DummyMemoryTransport>::new(
        DummySource::new(&["a", "b", "c", "d", "e", "f", "g"], &schema),
        &mut destination,
        &queries,
    );
    dispatcher.run().expect("run dispatcher");

    for (col, _) in destination.schema().into_iter().enumerate() {
        match col {
            0 => {
                assert_eq!(
                    destination.column_view::<i64>(col).unwrap(),
                    array![0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 6]
                )
            }
            1 => {
                assert_eq!(
                    destination.column_view::<f64>(col).unwrap(),
                    array![0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
                )
            }
            2 => {
                assert_eq!(
                    destination.column_view::<String>(col).unwrap(),
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
                    destination.column_view::<f64>(col).unwrap(),
                    array![0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
                )
            }
            4 => {
                assert_eq!(
                    destination.column_view::<bool>(col).unwrap(),
                    array![true, false, true, false, true, false, true, false, true, false, true]
                )
            }
            5 => {
                assert_eq!(
                    destination.column_view::<String>(col).unwrap(),
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
                    destination.column_view::<f64>(col).unwrap(),
                    array![0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
                )
            }
            _ => unreachable!(),
        }
    }
}
