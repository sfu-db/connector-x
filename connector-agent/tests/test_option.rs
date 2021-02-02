use anyhow::anyhow;
use arrow::array::{UInt64Array, UInt64Builder};
use connector_agent::{
    data_sources::{DataSource, Parse, SourceBuilder},
    ConnectorAgentError, DataOrder, DataType, Dispatcher, PartitionWriter, Result, TypeAssoc,
    TypeSystem, Writer,
};
use fehler::{throw, throws};
use ndarray::{Array, Array2, ArrayView2, ArrayViewMut2, Axis};
use rand::Rng;
use std::mem::transmute;
use std::sync::{Arc, Mutex};
// use std::time::Instant;

struct OptU64SourceBuilder {
    fake_values: Vec<Vec<Option<u64>>>,
    ncols: usize,
}

impl OptU64SourceBuilder {
    fn new(fake_values: Vec<Vec<Option<u64>>>, ncols: usize) -> Self {
        OptU64SourceBuilder { fake_values, ncols }
    }
}

impl SourceBuilder for OptU64SourceBuilder {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type DataSource = OptU64TestSource;

    #[throws(ConnectorAgentError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }
    }

    fn build(&mut self) -> Self::DataSource {
        let ret = OptU64TestSource::new(self.fake_values.swap_remove(0), self.ncols);
        ret
    }
}

struct OptU64TestSource {
    counter: usize,
    vals: Vec<Option<u64>>,
    ncols: usize,
}

impl OptU64TestSource {
    pub fn new(vals: Vec<Option<u64>>, ncols: usize) -> Self {
        OptU64TestSource {
            counter: 0,
            vals: vals,
            ncols,
        }
    }
}

impl DataSource for OptU64TestSource {
    type TypeSystem = DataType;
    fn run_query(&mut self, _: &str) -> Result<()> {
        Ok(())
    }

    fn nrows(&self) -> usize {
        self.vals.len() / self.ncols
    }
}

impl Parse<u64> for OptU64TestSource {
    fn parse(&mut self) -> Result<u64> {
        throw!(anyhow!("Only Option<u64> is supported"));
    }
}

impl Parse<Option<u64>> for OptU64TestSource {
    fn parse(&mut self) -> Result<Option<u64>> {
        let v = self.vals[self.counter];
        self.counter += 1;
        Ok(v)
    }
}

impl Parse<f64> for OptU64TestSource {
    fn parse(&mut self) -> Result<f64> {
        throw!(anyhow!("Only Option<u64> is supported"));
    }
}

impl Parse<bool> for OptU64TestSource {
    fn parse(&mut self) -> Result<bool> {
        throw!(anyhow!("Only Option<u64> is supported"));
    }
}

impl Parse<String> for OptU64TestSource {
    fn parse(&mut self) -> Result<String> {
        throw!(anyhow!("Only Option<u64> is supported"));
    }
}

pub struct OptU64Writer {
    nrows: usize,
    schema: Vec<DataType>,
    buffer: Array2<Option<u64>>,
}

impl OptU64Writer {
    pub fn buffer(&self) -> ArrayView2<Option<u64>> {
        self.buffer.view()
    }
}

impl<'a> Writer<'a> for OptU64Writer {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type PartitionWriter = OptU64PartitionWriter<'a>;
    type TypeSystem = DataType;

    fn allocate(nrows: usize, schema: Vec<DataType>, data_order: DataOrder) -> Result<Self> {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }

        let ncols = schema.len();
        for field in &schema {
            if !matches!(field, DataType::OptU64) {
                throw!(anyhow!("OptU64Writer only accepts OptU64 only schema"));
            }
        }

        Ok(OptU64Writer {
            nrows,
            schema,
            buffer: Array2::from_elem((nrows, ncols), None),
        })
    }

    fn partition_writers(&'a mut self, counts: &[usize]) -> Vec<Self::PartitionWriter> {
        assert_eq!(counts.iter().sum::<usize>(), self.nrows);
        let schema = self.schema().to_vec();

        let mut mut_view = self.buffer.view_mut();
        let mut ret = vec![];
        for &c in counts {
            let (splitted, rest) = mut_view.split_at(Axis(0), c);
            mut_view = rest;
            ret.push(OptU64PartitionWriter::new(splitted, schema.clone()));
        }
        ret
    }

    fn schema(&self) -> &[DataType] {
        self.schema.as_slice()
    }
}

pub struct OptU64PartitionWriter<'a> {
    buffer: ArrayViewMut2<'a, Option<u64>>,
    schema: Vec<DataType>,
}

impl<'a> OptU64PartitionWriter<'a> {
    fn new(buffer: ArrayViewMut2<'a, Option<u64>>, schema: Vec<DataType>) -> Self {
        Self { buffer, schema }
    }
}

impl<'a> PartitionWriter<'a> for OptU64PartitionWriter<'a> {
    type TypeSystem = DataType;

    unsafe fn write<T: 'static>(&mut self, row: usize, col: usize, value: T) {
        let target: *mut T = transmute(self.buffer.uget_mut((row, col)));
        *target = value;
    }

    fn write_checked<T: 'static>(&mut self, row: usize, col: usize, value: T) -> Result<()>
    where
        T: TypeAssoc<Self::TypeSystem>,
    {
        self.schema[col].check::<T>()?;
        unsafe { self.write(row, col, value) };
        Ok(())
    }

    fn nrows(&self) -> usize {
        self.buffer.nrows()
    }

    fn ncols(&self) -> usize {
        self.buffer.ncols()
    }
}

pub struct ArrowU64Writer {
    nrows: usize,
    schema: Vec<DataType>,
    array_builders: Vec<UInt64Array>,
}

pub struct ArrowU64PartitionWriter {
    nrows: usize,
    schema: Vec<DataType>,
    builders: Vec<UInt64Builder>,
}

// impl<'a> Writer<'a> for ArrowU64Writer {
//     const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
//     type TypeSystem = DataType;
//     type PartitionWriter = ArrowU64PartitionWriter;

//     #[throws(ConnectorAgentError)]
//     fn allocate(nrows: usize, schema: Vec<DataType>, data_order: DataOrder) -> Self {
//         for field in &schema {
//             if !(matches!(field, DataType::U64) && matches!(field, DataType::OptU64)) {
//                 throw!(anyhow!("U64Writer only accepts U64/OptU64 only schema"));
//             }
//         }
//         if !matches!(data_order, DataOrder::RowMajor) {
//             throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
//         }

//         ArrowU64Writer {
//             nrows,
//             schema,
//             array_builders: vec![], // cannot really allocate memory since do not know each partition size here
//         }
//     }

//     // fn partition_writers(&'a mut self, counts: &[usize]) -> Vec<Self::PartitionWriter> {
//     //     assert_eq!(counts.iter().sum::<usize>(), self.nrows);
//     //     let ncols = self.schema.len();

//     //     let mut ret = vec![];
//     //     for &c in counts {

//     //         ret.push();
//     //     }
//     //     ret
//     // }

//     fn schema(&self) -> &[DataType] {
//         self.schema.as_slice()
//     }
// }

impl ArrowU64PartitionWriter {
    fn new(schema: Vec<DataType>, nrows: usize) -> Self {
        let ncols = schema.len();
        let mut builders = vec![];
        for _i in 0..ncols {
            builders.push(UInt64Array::builder(nrows));
        }
        ArrowU64PartitionWriter {
            nrows,
            schema,
            builders,
        }
    }
}

impl<'a> PartitionWriter<'a> for ArrowU64PartitionWriter {
    type TypeSystem = DataType;

    unsafe fn write<T>(&mut self, _row: usize, col: usize, value: T) {
        // let v = Box::new(value) as Box<dyn std::any::Any>;
        // let (data, _vtable): (&Option<u64>, usize) =
        //     transmute(Box::new(value) as Box<dyn std::any::Any>);
        // self.builders[col].append_option(*data);
    }

    fn write_checked<T: 'static>(&mut self, row: usize, col: usize, value: T) -> Result<()>
    where
        T: TypeAssoc<Self::TypeSystem>,
    {
        self.schema[col].check::<T>()?;
        unsafe { self.write(row, col, value) };
        Ok(())
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.builders.len()
    }
}

#[test]
fn test_option() {
    let ncols = 3;
    let schema = vec![DataType::OptU64; ncols];
    let nrows = vec![4, 6];

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
        .run_checked::<OptU64Writer>()
        .expect("run dispatcher");

    let mut fake_data = vec![];
    data.iter_mut().for_each(|d| fake_data.append(d));

    // println!("{:?}", dw.buffer());
    assert_eq!(
        Array::from_shape_vec((dw.nrows, dw.schema.len()), fake_data).unwrap(),
        dw.buffer()
    )
}

// #[test]
// #[ignore]
// fn compare_time() {
//     let nrows = 1000000;
//     let ncols = 100;
//     let part = vec![500000, 500000];

//     // measure Option<u64> time
//     {
//         let mut dw = OptU64Writer::allocate(nrows, vec![DataType::OptU64; ncols]).unwrap();
//         let schema = dw.schema().to_vec();
//         let writers = dw.partition_writers(&part);

//         // try to make it unpredictable to cpu
//         let mut rng = rand::thread_rng();
//         let mut data: Vec<Option<u64>> = Vec::new();

//         for _i in 0..(nrows * schema.len()) {
//             let v: u64 = rng.gen();
//             if v % 2 == 0 {
//                 data.push(Some(v));
//             } else {
//                 data.push(None);
//             }
//         }

//         let mut sources: Vec<OptU64TestSource> = vec![];
//         let mut start = 0;
//         writers.iter().for_each(|writer| {
//             let end = start + (writer.nrows() * writer.ncols());
//             sources.push(OptU64TestSource::new(data[start..end].to_vec()));
//             start = end;
//         });

//         let start_stmp = Instant::now();
//         writers
//             .into_par_iter()
//             .zip_eq(sources)
//             .for_each(|(writer, source)| {
//                 Worker::new(source, writer, schema.clone(), "")
//                     .run_checked()
//                     .expect("Worker failed");
//             });
//         println!(
//             "Write Option<u64> ({}, {}, {:?}) takes {:?}",
//             nrows,
//             ncols,
//             part,
//             start_stmp.elapsed()
//         );
//     }

//     // measure u64 time
//     {
//         let mut dw = U64Writer::allocate(nrows, vec![DataType::U64; ncols]).unwrap();
//         let schema = dw.schema().to_vec();
//         let writers = dw.partition_writers(&part);

//         let start_stmp = Instant::now();
//         writers.into_par_iter().for_each(|writer| {
//             Worker::new(U64CounterSource::new(), writer, schema.clone(), "")
//                 .run_checked()
//                 .expect("Worker failed");
//         });
//         println!(
//             "Write u64 ({}, {}, {:?}) takes {:?}",
//             nrows,
//             ncols,
//             part,
//             start_stmp.elapsed()
//         );
//     }
// }
