use arrow::array::{BooleanArray, Float64Array, StringArray, UInt64Array};
use arrow::record_batch::RecordBatch;
use connector_agent::{
    data_sources::dummy::MixedSourceBuilder, writers::arrow::ArrowWriter, DataType, Dispatcher,
};

#[test]
fn test_arrow() {
    let schema = [
        DataType::U64(true),
        DataType::F64(true),
        DataType::Bool(false),
        DataType::String(true),
        DataType::F64(false),
    ];
    let nrows = vec![4, 7];
    let ncols = schema.len();
    let mut headers = vec![];
    for c in 0..ncols {
        headers.push(format!("c{}", c));
    }
    let queries: Vec<String> = nrows.iter().map(|v| format!("{},{}", v, ncols)).collect();
    let mut writer = ArrowWriter::new();
    let dispatcher = Dispatcher::new(MixedSourceBuilder::new(), &mut writer, &queries, &schema);
    dispatcher.run_checked().expect("run dispatcher");

    let records: Vec<RecordBatch> = writer.finish(headers);
    assert_eq!(2, records.len());

    for col in 0..ncols {
        match col {
            0 => {
                assert!(records[0]
                    .column(col)
                    .as_any()
                    .downcast_ref::<UInt64Array>()
                    .unwrap()
                    .eq(&UInt64Array::from(vec![0, 1, 2, 3])));
                assert!(records[1]
                    .column(col)
                    .as_any()
                    .downcast_ref::<UInt64Array>()
                    .unwrap()
                    .eq(&UInt64Array::from(vec![0, 1, 2, 3, 4, 5, 6])));
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
