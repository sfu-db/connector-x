mod any_array;
mod errors;

pub use self::errors::MemoryDestinationError;
use super::{Consume, Destination, DestinationPartition};
use crate::{
    data_order::DataOrder,
    dummy_typesystem::DummyTypeSystem,
    typesystem::{ParameterizedFunc, ParameterizedOn, Realize, TypeAssoc, TypeSystem},
    ConnectorAgentError,
};
use any_array::{AnyArray, AnyArrayViewMut};
use anyhow::anyhow;
use chrono::{DateTime, NaiveDateTime, Utc};
use fehler::{throw, throws};
use itertools::Itertools;
use ndarray::{Array2, ArrayView1, ArrayView2, Axis, Ix2};
use std::any::type_name;
use std::collections::HashMap;

/// A destination which stores data in memory.
/// For testing purpose.
pub struct MemoryDestination {
    nrows: usize,
    schema: Vec<DummyTypeSystem>,
    buffers: Vec<AnyArray<Ix2>>,
    column_buffer_index: Vec<(usize, usize)>,
}

impl Default for MemoryDestination {
    fn default() -> Self {
        MemoryDestination {
            nrows: 0,
            schema: vec![],
            buffers: vec![],
            column_buffer_index: vec![],
        }
    }
}

impl MemoryDestination {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Destination for MemoryDestination {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type TypeSystem = DummyTypeSystem;
    type Partition<'a> = MemoryPartitionDestination<'a>;
    type Error = MemoryDestinationError;

    #[throws(MemoryDestinationError)]
    fn allocate<S: AsRef<str>>(
        &mut self,
        nrows: usize,
        _names: &[S],
        schema: &[DummyTypeSystem],
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
            let buffer = Realize::<FArray2>::realize(dt)?(nrows, count);
            self.buffers.push(buffer);
        }

        let mut per_buffer_counter = HashMap::new();

        for dt in &self.schema {
            let count = per_buffer_counter.entry(*dt).or_insert(0);
            self.column_buffer_index.push((block_indices[dt], *count));
            *count += 1;
        }
    }

    #[throws(MemoryDestinationError)]
    fn partition(&mut self, counts: &[usize]) -> Vec<Self::Partition<'_>> {
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

            for (bid, view) in views.iter_mut().enumerate().take(nbuffers) {
                let (splitted, rest) = view
                    .take()
                    .ok_or_else(|| anyhow!("got None for {}th view", bid))?
                    .split_at(Axis(0), c);
                *view = Some(rest);
                sub_buffers.push(splitted);
            }
            ret.push(MemoryPartitionDestination::new(
                c,
                sub_buffers,
                self.schema.clone(),
                self.column_buffer_index.clone(),
            ));
        }
        ret
    }

    fn schema(&self) -> &[DummyTypeSystem] {
        self.schema.as_slice()
    }
}

impl MemoryDestination {
    pub fn buffer_view<T>(&self, bid: usize) -> Option<ArrayView2<T>>
    where
        T: 'static + Send,
    {
        self.buffers[bid].downcast_ref::<T>().map(|arr| arr.view())
    }

    pub fn column_view<T>(&self, col: usize) -> Option<ArrayView1<T>>
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
/// The `PartitionedDestination` of `MemoryDestination`.
pub struct MemoryPartitionDestination<'a> {
    nrows: usize,
    schema: Vec<DummyTypeSystem>,
    buffers: Vec<AnyArrayViewMut<'a, Ix2>>,
    column_buffer_index: Vec<(usize, usize)>,
    current: usize,
}

impl<'a> MemoryPartitionDestination<'a> {
    fn new(
        nrows: usize,
        buffers: Vec<AnyArrayViewMut<'a, Ix2>>,
        schema: Vec<DummyTypeSystem>,
        column_buffer_index: Vec<(usize, usize)>,
    ) -> Self {
        Self {
            nrows,
            buffers,
            schema,
            column_buffer_index,
            current: 0,
        }
    }

    fn loc(&mut self) -> (usize, usize) {
        let (row, col) = (self.current / self.ncols(), self.current % self.ncols());
        self.current += 1;
        (row, col)
    }
}

impl<'a> DestinationPartition<'a> for MemoryPartitionDestination<'a> {
    type TypeSystem = DummyTypeSystem;
    type Error = MemoryDestinationError;

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.schema.len()
    }
}

impl<'a, T> Consume<T> for MemoryPartitionDestination<'a>
where
    T: TypeAssoc<<Self as DestinationPartition<'a>>::TypeSystem> + 'static,
{
    type Error = MemoryDestinationError;

    #[throws(MemoryDestinationError)]
    fn consume(&mut self, value: T) {
        let (row, col) = self.loc();
        let col_schema = self.schema[col];
        col_schema.check::<T>()?;

        let &(bid, col) = &self.column_buffer_index[col];

        let mut_view = self.buffers[bid].downcast::<T>().ok_or_else(|| {
            ConnectorAgentError::TypeCheckFailed(format!("{:?}", col_schema), type_name::<T>())
        })?;
        *mut_view
            .get_mut((row, col))
            .ok_or(MemoryDestinationError::OutOfBound)? = value;
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
    i32,
    i64,
    f64,
    String,
    bool,
    Option<i32>,
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
