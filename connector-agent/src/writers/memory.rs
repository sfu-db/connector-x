use super::{Consume, PartitionWriter, Writer};
use crate::any_array::{AnyArray, AnyArrayViewMut};
use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::types::DataType;
use crate::typesystem::{ParameterizedFunc, ParameterizedOn, Realize, TypeAssoc, TypeSystem};
use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, Utc};
use fehler::{throw, throws};
use itertools::Itertools;
use ndarray::{Array2, ArrayView1, ArrayView2, Axis, Ix2};
use std::any::type_name;
use std::collections::HashMap;
/// This `Writer` can support mixed data type.
pub struct MemoryWriter {
    nrows: usize,
    schema: Vec<DataType>,
    buffers: Vec<AnyArray<Ix2>>,
    column_buffer_index: Vec<(usize, usize)>,
}

impl MemoryWriter {
    pub fn new() -> Self {
        MemoryWriter {
            nrows: 0,
            schema: vec![],
            buffers: vec![],
            column_buffer_index: vec![],
        }
    }
}

impl Writer for MemoryWriter {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type TypeSystem = DataType;
    type PartitionWriter<'a> = MemoryPartitionWriter<'a>;

    #[throws(ConnectorAgentError)]
    fn allocate<S: AsRef<str>>(
        &mut self,
        nrows: usize,
        _names: &[S],
        schema: &[DataType],
        data_order: DataOrder,
    ) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }

        self.nrows = nrows;
        self.schema = schema.to_vec();

        // The schema needs to be sorted due to the group by only works on consecutive identity keys.
        let mut sorted_schema = self.schema.clone();
        sorted_schema.sort();

        let mut block_indices = HashMap::new();
        for (bid, (dt, grp)) in sorted_schema
            .iter()
            .group_by(|&&v| v)
            .into_iter()
            .enumerate()
        {
            block_indices.insert(dt, bid);
            let count = grp.count();
            let buffer = Realize::<FArray2>::realize(dt)(nrows, count);
            self.buffers.push(buffer);
        }

        let mut per_buffer_counter = HashMap::new();

        for dt in &self.schema {
            let count = per_buffer_counter.entry(*dt).or_insert(0);
            self.column_buffer_index.push((block_indices[dt], *count));
            *count += 1;
        }
    }

    #[throws(ConnectorAgentError)]
    fn partition_writers(&mut self, counts: &[usize]) -> Vec<Self::PartitionWriter<'_>> {
        assert_eq!(counts.iter().sum::<usize>(), self.nrows);

        let nbuffers = self.buffers.len();
        let mut views: Vec<_> = self
            .buffers
            .iter_mut()
            .map(|buf| Some(buf.view_mut()))
            .collect();
        let mut ret = vec![];
        for &c in counts {
            let mut sub_buffers = vec![];

            for bid in 0..nbuffers {
                let view = views[bid].take();
                let (splitted, rest) = view.unwrap().split_at(Axis(0), c);
                views[bid] = Some(rest);
                sub_buffers.push(splitted);
            }
            ret.push(MemoryPartitionWriter::new(
                c,
                sub_buffers,
                self.schema.clone(),
                self.column_buffer_index.clone(),
            ));
        }
        ret
    }

    fn schema(&self) -> &[DataType] {
        self.schema.as_slice()
    }
}

impl MemoryWriter {
    pub fn buffer_view<'a, T>(&'a self, bid: usize) -> Option<ArrayView2<T>>
    where
        T: 'static + Send,
    {
        self.buffers[bid].downcast_ref::<T>().map(|arr| arr.view())
    }

    pub fn column_view<'a, T>(&'a self, col: usize) -> Option<ArrayView1<T>>
    where
        T: 'static + Send,
    {
        let (bid, sid) = self.column_buffer_index(col);

        self.buffers[bid]
            .downcast_ref::<T>()
            .map(|arr| arr.column(sid))
    }

    pub fn column_buffer_index(&self, col: usize) -> (usize, usize) {
        self.column_buffer_index[col]
    }
}
/// The `PartitionedWriter` of `MemoryWriter`.
pub struct MemoryPartitionWriter<'a> {
    nrows: usize,
    buffers: Vec<AnyArrayViewMut<'a, Ix2>>,
    schema: Vec<DataType>,
    column_buffer_index: Vec<(usize, usize)>,
}

impl<'a> MemoryPartitionWriter<'a> {
    fn new(
        nrows: usize,
        buffers: Vec<AnyArrayViewMut<'a, Ix2>>,
        schema: Vec<DataType>,
        column_buffer_index: Vec<(usize, usize)>,
    ) -> Self {
        Self {
            nrows,
            buffers,
            schema,
            column_buffer_index,
        }
    }
}

impl<'a> PartitionWriter<'a> for MemoryPartitionWriter<'a> {
    type TypeSystem = DataType;

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.schema.len()
    }
}

impl<'a, T> Consume<T> for MemoryPartitionWriter<'a>
where
    T: TypeAssoc<<Self as PartitionWriter<'a>>::TypeSystem> + 'static,
{
    unsafe fn consume(&mut self, row: usize, col: usize, value: T) {
        let &(bid, col) = &self.column_buffer_index[col];
        let mut_view = self.buffers[bid].udowncast::<T>();
        *mut_view.get_mut((row, col)).unwrap() = value;
    }

    fn consume_checked(&mut self, row: usize, col: usize, value: T) -> Result<()> {
        self.schema[col].check::<T>()?;
        let &(bid, col) = &self.column_buffer_index[col];

        let mut_view =
            self.buffers[bid]
                .downcast::<T>()
                .ok_or(ConnectorAgentError::UnexpectedType(
                    format!("{:?}", self.schema[col]),
                    type_name::<T>(),
                ))?;
        *mut_view
            .get_mut((row, col))
            .ok_or(ConnectorAgentError::OutOfBound)? = value;
        Ok(())
    }
}

struct FArray2;

impl ParameterizedFunc for FArray2 {
    type Function = fn(nrows: usize, ncols: usize) -> AnyArray<Ix2>;
}

macro_rules! FArray2Parameterize {
    ($($t: ty),+) => {
        $(
            impl ParameterizedOn<$t> for FArray2 {
                fn parameterize() -> Self::Function {
                    create_default_array::<$t>
                }
            }
        )+
    };
}

FArray2Parameterize!(
    u64,
    i64,
    f64,
    String,
    bool,
    Option<u64>,
    Option<i64>,
    Option<f64>,
    Option<String>,
    Option<bool>
);

fn create_default_array<T>(nrows: usize, ncols: usize) -> AnyArray<Ix2>
where
    T: Default + Send + 'static,
{
    Array2::<T>::default((nrows, ncols)).into()
}

impl ParameterizedOn<DateTime<Utc>> for FArray2 {
    fn parameterize() -> Self::Function {
        fn imp(nrows: usize, ncols: usize) -> AnyArray<Ix2> {
            let t = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc);
            Array2::from_elem((nrows, ncols), t).into()
        }
        imp
    }
}

impl ParameterizedOn<Option<DateTime<Utc>>> for FArray2 {
    fn parameterize() -> Self::Function {
        fn imp(nrows: usize, ncols: usize) -> AnyArray<Ix2> {
            Array2::<Option<DateTime<Utc>>>::from_elem((nrows, ncols), None).into()
        }
        imp
    }
}

impl ParameterizedOn<Date<Utc>> for FArray2 {
    fn parameterize() -> Self::Function {
        fn imp(nrows: usize, ncols: usize) -> AnyArray<Ix2> {
            let t = Date::<Utc>::from_utc(NaiveDate::from_yo(1970, 0), Utc);
            Array2::from_elem((nrows, ncols), t).into()
        }
        imp
    }
}

impl ParameterizedOn<Option<Date<Utc>>> for FArray2 {
    fn parameterize() -> Self::Function {
        fn imp(nrows: usize, ncols: usize) -> AnyArray<Ix2> {
            Array2::<Option<Date<Utc>>>::from_elem((nrows, ncols), None).into()
        }
        imp
    }
}
