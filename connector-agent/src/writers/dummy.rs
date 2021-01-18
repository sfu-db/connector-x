use super::{PartitionWriter, Writer};
use crate::errors::Result;
use crate::types::{DataType, TypeInfo};
use anyhow::anyhow;
use fehler::throw;
use ndarray::{Array2, ArrayView2, ArrayViewMut2, Axis};
use std::mem::transmute;
use std::ptr::copy_nonoverlapping;

#[derive(Clone)]
pub struct U64Writer {
    nrows: usize,
    schema: Vec<DataType>,
    buffer: Array2<u64>,
}

impl U64Writer {
    pub fn buffer(&self) -> ArrayView2<u64> {
        self.buffer.view()
    }
}

impl Writer for U64Writer {
    type PartitionWriter<'a> = U64PartitionWriter<'a>;

    fn allocate(nrows: usize, schema: Vec<DataType>) -> Result<Self> {
        let ncols = schema.len();
        for field in &schema {
            if !matches!(field, DataType::U64) {
                throw!(anyhow!("U64Writer only accepts U64 only schema"));
            }
        }

        Ok(U64Writer {
            nrows,
            schema,
            buffer: Array2::zeros((nrows, ncols)),
        })
    }

    fn partition_writer<'a>(&'a mut self, counts: &[usize]) -> Vec<Self::PartitionWriter<'a>> {
        assert_eq!(counts.iter().sum::<usize>(), self.nrows);
        let schema = self.schema().to_vec();

        let mut mut_view = self.buffer.view_mut();
        let mut ret = vec![];
        for &c in counts {
            let (splitted, rest) = mut_view.split_at(Axis(0), c);
            mut_view = rest;
            ret.push(U64PartitionWriter::new(splitted, schema.clone()));
        }
        ret
    }

    fn schema(&self) -> &[DataType] {
        self.schema.as_slice()
    }
}

pub struct U64PartitionWriter<'a> {
    buffer: ArrayViewMut2<'a, u64>,
    schema: Vec<DataType>,
}

impl<'a> PartitionWriter<'a> for U64PartitionWriter<'a> {
    unsafe fn write<T>(&mut self, row: usize, col: usize, value: T) {
        let target: *mut T = transmute(self.buffer.uget_mut((row, col)));
        copy_nonoverlapping(&value, target, 1);
    }

    fn write_checked<T>(&mut self, row: usize, col: usize, value: T) -> Result<()>
    where
        T: TypeInfo,
    {
        T::check(self.schema[col])?;
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

impl<'a> U64PartitionWriter<'a> {
    fn new(buffer: ArrayViewMut2<'a, u64>, schema: Vec<DataType>) -> Self {
        Self { buffer, schema }
    }
}
