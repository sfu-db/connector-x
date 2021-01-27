use super::{PartitionWriter, Writer};
use crate::any_array::{AnyArray, AnyArrayViewMut};
use crate::errors::Result;
use crate::types::DataType;
use crate::typesystem::TypeSystem;
use itertools::Itertools;
use ndarray::{Array2, ArrayView1, ArrayView2, Axis, Ix2};
use std::collections::HashMap;
use std::mem::transmute;

fn test<T: Send>() {}
/// This `Writer` can only write u64 into it.
pub struct MemoryWriter {
    nrows: usize,
    schema: Vec<DataType>,
    buffers: Vec<Box<dyn AnyArray<Ix2>>>,
    column_buffer_index: Vec<(usize, usize)>,
}

impl<'a> Writer<'a> for MemoryWriter {
    type TypeSystem = DataType;
    type PartitionWriter = MemoryPartitionWriter<'a>;

    fn allocate(nrows: usize, schema: Vec<DataType>) -> Result<Self> {
        test::<MemoryWriter>();
        // The schema needs to be sorted due to the group by only works on consecutive identity keys.
        let mut sorted_schema = schema.clone();
        sorted_schema.sort();

        let mut block_indices = HashMap::new();
        let mut buffers = vec![];
        for (bid, (dt, grp)) in sorted_schema.iter().group_by(|&&v| v).into_iter().enumerate() {
            block_indices.insert(dt, bid);
            let count = grp.count();
            let buffer = match dt {
                DataType::F64 => Box::new(Array2::<f64>::zeros((nrows, count))) as Box<dyn AnyArray<Ix2>>,
                DataType::U64 => Box::new(Array2::<u64>::zeros((nrows, count))) as Box<dyn AnyArray<Ix2>>,
                DataType::Bool => Box::new(Array2::<bool>::from_elem((nrows, count), false)) as Box<dyn AnyArray<Ix2>>,
                DataType::String => Box::new(Array2::<String>::from_elem((nrows, count), "".to_string())) as Box<dyn AnyArray<Ix2>>,
            };
            buffers.push(buffer);
        }

        let mut per_buffer_counter = HashMap::new();

        let mut col_buffer_index = vec![];
        for dt in &schema {
            let count = per_buffer_counter.entry(*dt).or_insert(0);
            col_buffer_index.push((block_indices[dt], *count));
            *count += 1;
        }

        Ok(MemoryWriter {
            nrows,
            schema,
            buffers,
            column_buffer_index: col_buffer_index,
        })
    }

    fn partition_writers(&'a mut self, counts: &[usize]) -> Vec<Self::PartitionWriter> {
        assert_eq!(counts.iter().sum::<usize>(), self.nrows);

        let nbuffers = self.buffers.len();
        let mut views: Vec<_> = self.buffers.iter_mut().map(|buf| Some(buf.view_mut())).collect();
        let mut ret = vec![];
        for &c in counts {
            let mut sub_buffers = vec![];

            for bid in 0..nbuffers {
                let view = views[bid].take();
                let (splitted, rest) = view.unwrap().split_at(Axis(0), c);
                views[bid] = Some(rest);
                sub_buffers.push(splitted);
            }
            ret.push(MemoryPartitionWriter::new(c, sub_buffers, self.schema.clone(), self.column_buffer_index.clone()));
        }
        ret
    }

    fn schema(&self) -> &[DataType] {
        self.schema.as_slice()
    }
}

impl MemoryWriter {
    pub fn buffer_view<'a, T: 'static>(&'a self, bid: usize) -> Option<ArrayView2<T>> {
        self.buffers[bid].as_any().downcast_ref::<Array2<T>>().map(|arr| arr.view())
    }

    pub fn column_view<'a, T: 'static>(&'a self, col: usize) -> Option<ArrayView1<T>> {
        let (bid, sid) = self.column_buffer_index(col);

        self.buffers[bid].as_any().downcast_ref::<Array2<T>>().map(|arr| arr.column(sid))
    }

    pub fn column_buffer_index(&self, col: usize) -> (usize, usize) {
        self.column_buffer_index[col]
    }
}
/// The `PartitionedWriter` of `MemoryWriter`.
pub struct MemoryPartitionWriter<'a> {
    nrows: usize,
    buffers: Vec<Box<dyn AnyArrayViewMut<'a, Ix2> + 'a>>,
    schema: Vec<DataType>,
    col_buffer_index: Vec<(usize, usize)>,
}

impl<'a> PartitionWriter<'a> for MemoryPartitionWriter<'a> {
    type TypeSystem = DataType;

    unsafe fn write<T>(&mut self, row: usize, col: usize, value: T) {
        let buffer_index = &self.col_buffer_index[col];

        let target: &mut T = transmute(self.buffers[buffer_index.0].uget_mut((row, buffer_index.1)));
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
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.schema.len()
    }
}

impl<'a> MemoryPartitionWriter<'a> {
    fn new(nrows: usize, buffers: Vec<Box<dyn AnyArrayViewMut<'a, Ix2> + 'a>>, schema: Vec<DataType>, col_buffer_index: Vec<(usize, usize)>) -> Self {
        Self {
            nrows,
            buffers,
            schema,
            col_buffer_index,
        }
    }
}
