use anyhow::anyhow;
use connector_agent::writers::dummy::U64Writer;
use connector_agent::{
    data_sources::{DataSource, Parse},
    DataType, PartitionWriter, Result, TypeSystem, Worker, Writer,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fehler::throw;
use ndarray::{Array2, ArrayView2, ArrayViewMut2, Axis};
use rand::Rng;
use rayon::prelude::*;
use std::mem::transmute;

const NROWS: usize = 100000;
const NCOLS: usize = 100;

fn bench_option(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut data: Vec<Option<u64>> = Vec::new();

    for _i in 0..(NROWS * NCOLS) {
        let v: u64 = rng.gen();
        if v % 2 == 0 {
            data.push(Some(v));
        } else {
            data.push(None);
        }
    }

    let data = data.as_slice();

    let part = vec![NROWS / 2, NROWS / 2];

    c.bench_function("write option", |b| {
        b.iter(|| {
            let data = black_box(data);

            let mut dw = OptU64Writer::allocate(NROWS, vec![DataType::OptU64; NCOLS]).unwrap();
            let schema = dw.schema().to_vec();
            let writers = dw.partition_writers(&part);

            let mut sources: Vec<OptU64TestSource> = vec![];
            let mut start = 0;
            writers.iter().for_each(|writer| {
                let end = start + (writer.nrows() * writer.ncols());
                sources.push(OptU64TestSource::new(data[start..end].to_vec()));
                start = end;
            });

            writers.into_par_iter().zip_eq(sources).for_each(|(writer, source)| {
                Worker::new(source, writer, schema.clone(), "").run_checked().expect("Worker failed");
            });
        })
    });
}

fn bench_non_option(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut data: Vec<u64> = Vec::new();

    for _i in 0..(NROWS * NCOLS) {
        let v: u64 = rng.gen();
        if v % 2 == 0 {
            data.push(v);
        } else {
            data.push(0);
        }
    }

    let data = data.as_slice();

    let part = vec![NROWS / 2, NROWS / 2];

    c.bench_function("write non option", |b| {
        b.iter(|| {
            let data = black_box(data);

            let mut dw = U64Writer::allocate(NROWS, vec![DataType::U64; NCOLS]).unwrap();
            let schema = dw.schema().to_vec();
            let writers = dw.partition_writers(&part);

            let mut sources: Vec<U64TestSource> = vec![];
            let mut start = 0;
            writers.iter().for_each(|writer| {
                let end = start + (writer.nrows() * writer.ncols());
                sources.push(U64TestSource::new(data[start..end].to_vec()));
                start = end;
            });

            writers.into_par_iter().zip_eq(sources).for_each(|(writer, source)| {
                Worker::new(source, writer, schema.clone(), "").run_checked().expect("Worker failed");
            });
        })
    });
}

fn bench_option_v2(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut data: Vec<u64> = Vec::new();

    for _i in 0..(NROWS * NCOLS) {
        let v: u64 = rng.gen();
        if v % 2 == 0 {
            data.push(v);
        } else {
            data.push(0);
        }
    }

    let data = data.as_slice();

    let part = vec![NROWS / 2, NROWS / 2];

    c.bench_function("write option v2", |b| {
        b.iter(|| {
            let data = black_box(data);

            let mut dw = OptU64Writer::allocate(NROWS, vec![DataType::OptU64; NCOLS]).unwrap();
            let schema = dw.schema().to_vec();
            let writers = dw.partition_writers(&part);

            let mut sources: Vec<OptU64TestSourceV2> = vec![];
            let mut start = 0;
            writers.iter().for_each(|writer| {
                let end = start + (writer.nrows() * writer.ncols());
                sources.push(OptU64TestSourceV2::new(data[start..end].to_vec()));
                start = end;
            });

            writers.into_par_iter().zip_eq(sources).for_each(|(writer, source)| {
                Worker::new(source, writer, schema.clone(), "").run_checked().expect("Worker failed");
            });
        })
    });
}

criterion_group!(benches, bench_option, bench_non_option, bench_option_v2);
criterion_main!(benches);

struct OptU64TestSource {
    counter: usize,
    vals: Vec<Option<u64>>,
}

impl OptU64TestSource {
    pub fn new(vals: Vec<Option<u64>>) -> Self {
        OptU64TestSource { counter: 0, vals: vals }
    }
}

impl DataSource for OptU64TestSource {
    type TypeSystem = DataType;
    fn run_query(&mut self, _: &str) -> Result<()> {
        Ok(())
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

struct U64TestSource {
    counter: usize,
    vals: Vec<u64>,
}

impl U64TestSource {
    pub fn new(vals: Vec<u64>) -> Self {
        U64TestSource { counter: 0, vals: vals }
    }
}

impl DataSource for U64TestSource {
    type TypeSystem = DataType;
    fn run_query(&mut self, _: &str) -> Result<()> {
        Ok(())
    }
}

impl Parse<u64> for U64TestSource {
    fn parse(&mut self) -> Result<u64> {
        let v = self.vals[self.counter];
        self.counter += 1;
        Ok(v)
    }
}

impl Parse<Option<u64>> for U64TestSource {
    fn parse(&mut self) -> Result<Option<u64>> {
        throw!(anyhow!("Only u64 is supported"));
    }
}

impl Parse<f64> for U64TestSource {
    fn parse(&mut self) -> Result<f64> {
        throw!(anyhow!("Only u64 is supported"));
    }
}

struct OptU64TestSourceV2 {
    counter: usize,
    vals: Vec<u64>,
}

impl OptU64TestSourceV2 {
    pub fn new(vals: Vec<u64>) -> Self {
        OptU64TestSourceV2 { counter: 0, vals: vals }
    }
}

impl DataSource for OptU64TestSourceV2 {
    type TypeSystem = DataType;
    fn run_query(&mut self, _: &str) -> Result<()> {
        Ok(())
    }
}

impl Parse<u64> for OptU64TestSourceV2 {
    fn parse(&mut self) -> Result<u64> {
        throw!(anyhow!("Only Option<u64> is supported"));
    }
}

impl Parse<Option<u64>> for OptU64TestSourceV2 {
    fn parse(&mut self) -> Result<Option<u64>> {
        let v = self.vals[self.counter];
        self.counter += 1;
        Ok(Some(v))
    }
}

impl Parse<f64> for OptU64TestSourceV2 {
    fn parse(&mut self) -> Result<f64> {
        throw!(anyhow!("Only Option<u64> is supported"));
    }
}

#[derive(Clone)]
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
    type PartitionWriter = OptU64PartitionWriter<'a>;
    type TypeSystem = DataType;

    fn allocate(nrows: usize, schema: Vec<DataType>) -> Result<Self> {
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

    unsafe fn write<T>(&mut self, row: usize, col: usize, value: T) {
        let target: *mut T = transmute(self.buffer.uget_mut((row, col)));
        *target = value;
    }

    fn write_checked<T>(&mut self, row: usize, col: usize, value: T) -> Result<()>
    where
        Self::TypeSystem: TypeSystem<T>,
    {
        self.schema[col].check()?;
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
//         writers.into_par_iter().zip_eq(sources).for_each(|(writer, source)| {
//             Worker::new(source, writer, schema.clone(), "").run_checked().expect("Worker failed");
//         });
//         println!("Write Option<u64> ({}, {}, {:?}) takes {:?}", nrows, ncols, part, start_stmp.elapsed());
//     }

//     // measure u64 time
//     {
//         let mut dw = U64Writer::allocate(nrows, vec![DataType::U64; ncols]).unwrap();
//         let schema = dw.schema().to_vec();
//         let writers = dw.partition_writers(&part);

//         let start_stmp = Instant::now();
//         writers.into_par_iter().for_each(|writer| {
//             Worker::new(U64CounterSource::new(), writer, schema.clone(), "").run_checked().expect("Worker failed");
//         });
//         println!("Write u64 ({}, {}, {:?}) takes {:?}", nrows, ncols, part, start_stmp.elapsed());
//     }
// }
