use arrow::array::{BooleanArray, Float64Array, Int64Array, StringArray};
use arrow::record_batch::RecordBatch;
use connector_agent::{
    destinations::arrow::ArrowDestination, sources::dummy::DummySource,
    transports::DummyArrowTransport, Dispatcher, DummyTypeSystem,
};

#[test]
fn test_arrow() {
    let schema = [
        DummyTypeSystem::I64(true),
        DummyTypeSystem::F64(true),
        DummyTypeSystem::Bool(false),
        DummyTypeSystem::String(true),
        DummyTypeSystem::F64(false),
    ];
    let nrows = vec![4, 7];
    let ncols = schema.len();
    let mut headers = vec![];
    for c in 0..ncols {
        headers.push(format!("c{}", c));
    }
    let queries: Vec<String> = nrows.iter().map(|v| format!("{},{}", v, ncols)).collect();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, DummyArrowTransport>::new(
        DummySource::new(&["a", "b", "c", "d", "e"], &schema),
        &mut destination,
        &queries,
    );
    dispatcher.run().expect("run dispatcher");

    let records: Vec<RecordBatch> = destination.finish(headers).unwrap();
    assert_eq!(2, records.len());

    for col in 0..ncols {
        match col {
            0 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![0, 1, 2, 3])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .unwrap()
                    .eq(&Int64Array::from(vec![0, 1, 2, 3, 4, 5, 6])));
            }
            1 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![0.0, 1.0, 2.0, 3.0])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0])));
            }
            2 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .unwrap()
                    .eq(&BooleanArray::from(vec![true, false, true, false])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .unwrap()
                    .eq(&BooleanArray::from(vec![
                        true, false, true, false, true, false, true
                    ])));
            }
            3 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .eq(&StringArray::from(vec!["0", "1", "2", "3"])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .eq(&StringArray::from(vec!["0", "1", "2", "3", "4", "5", "6"])));
            }
            4 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![0.0, 1.0, 2.0, 3.0])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0])));
            }
            _ => unreachable!(),
        }
    }
}
