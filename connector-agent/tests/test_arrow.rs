use arrow::array::{BooleanArray, Float64Array, StringArray, UInt64Array};
use arrow::record_batch::RecordBatch;
use connector_agent::{
    data_sources::{dummy::OptU64SourceBuilder, mixed::MixedSourceBuilder},
    writers::arrow::ArrowWriter,
    DataType, Dispatcher,
};
use itertools::Itertools;
use rand::Rng;

#[test]
fn test_arrow() {
    let schema = vec![
        DataType::U64,
        DataType::F64,
        DataType::Bool,
        DataType::String,
        DataType::F64,
    ];
    let nrows = vec![4, 7];
    let ncols = schema.len();
    let mut headers = vec![];
    for c in 0..ncols {
        headers.push(format!("c{}", c));
    }
    let queries: Vec<String> = nrows.iter().map(|v| format!("{},{}", v, ncols)).collect();

    let dispatcher = Dispatcher::new(MixedSourceBuilder::new(), schema, queries);
    let dw = dispatcher
        .run_checked::<ArrowWriter>()
        .expect("run dispatcher");

    let records: Vec<RecordBatch> = dw.finish(headers);
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

#[test]
fn test_option_arrow() {
    let ncols = 3;
    let mut headers = vec![];
    for c in 0..ncols {
        headers.push(format!("c{}", c));
    }
    let schema = vec![DataType::OptU64; ncols];
    let nrows = vec![12, 8];

    let mut rng = rand::thread_rng();
    let mut data = vec![];

    nrows.iter().for_each(|n| {
        let mut val = vec![];
        for _i in 0..(n * ncols) {
            let v: u64 = rng.gen();
            if v % 2 == 0 {
                val.push(Some(v));
            } else {
                val.push(None);
            }
        }
        data.push(val);
    });

    // println!("{:?}", data);

    let dispatcher = Dispatcher::new(
        OptU64SourceBuilder::new(data.clone(), ncols),
        schema,
        nrows.iter().map(|_n| String::new()).collect(),
    );

    let dw = dispatcher
        .run_checked::<ArrowWriter>()
        .expect("run dispatcher");

    let records: Vec<RecordBatch> = dw.finish(headers);
    for (i, (rb, odata)) in records.iter().zip_eq(data).enumerate() {
        // println!("{:?}", rb);
        assert_eq!(ncols, rb.num_columns());
        assert_eq!(nrows[i], rb.num_rows());

        let mut cdata = vec![vec![]; ncols];
        for (j, &d) in odata.iter().enumerate() {
            cdata[j % ncols].push(d);
        }
        for c in 0..ncols {
            let a: &UInt64Array = rb
                .column(ncols - c - 1)
                .as_any()
                .downcast_ref::<UInt64Array>()
                .unwrap();
            let b = UInt64Array::from(cdata.pop().unwrap());
            assert!(b.eq(a));
        }
    }
}
