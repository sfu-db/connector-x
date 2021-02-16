use connector_agent::{
    AnyArrayViewMut, ConnectorAgentError, Consume, DataOrder, DataType, PartitionWriter, Result,
    TypeAssoc, TypeSystem, Writer,
};
use fehler::{throw, throws};
use itertools::Itertools;
use ndarray::{Axis, Ix2};
use numpy::PyArray;
use pyo3::types::PyList;
use std::any::type_name;

pub struct PandasWriter<'a> {
    nrows: usize,
    schema: Vec<DataType>,
    buffers: Option<Vec<AnyArrayViewMut<'a, Ix2>>>,
    column_buffer_index: Vec<(usize, usize)>,
}

impl<'a> PandasWriter<'a> {
    pub fn new(nrows: usize, schema: &[DataType], buffers: &'a PyList, index: &'a PyList) -> Self {
        let nbuffers = buffers.len();

        // get index for each column: (index of block, index of column within the block)
        let column_buffer_index: Vec<(usize, usize)> =
            index.iter().map(|tuple| tuple.extract().unwrap()).collect();

        // get schema for each block, init by U64
        let mut block_schema_index = vec![DataType::U64; nbuffers];
        column_buffer_index
            .iter()
            .zip_eq(schema)
            .for_each(|((b, _), &s)| block_schema_index[*b] = s);

        // get array view of each block so we can write data into using rust
        // TODO: cannot support multi-type using FArrayViewMut2 since PyArray does not support String and Option type
        let buffers = buffers
            .iter()
            .enumerate()
            // .map(|(i, array)| Realize::<FArrayViewMut2>::realize(block_schema_index[i])(array))
            .map(|(_i, array)| {
                let pyarray = array
                    .downcast::<PyArray<u64, Ix2>>()
                    .expect("other types do not supported yet."); // TODO: add support for other dtypes.
                let mut_view = unsafe { pyarray.as_array_mut() };
                AnyArrayViewMut::<Ix2>::new(mut_view)
            })
            .collect();

        PandasWriter {
            nrows,
            schema: schema.to_vec(),
            buffers: Some(buffers),
            column_buffer_index,
        }
    }
}

impl<'a> Writer for PandasWriter<'a> {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type TypeSystem = DataType;
    type PartitionWriter<'b> = PandasPartitionWriter<'b>;

    #[throws(ConnectorAgentError)]
    fn allocate(&mut self, _nrows: usize, _schema: Vec<DataType>, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }
        // PandasWriter does not own the data buffers, they are allocated by python.
        // Nothing we can do with nrows and schema.
    }

    fn partition_writers(&mut self, counts: &[usize]) -> Vec<Self::PartitionWriter<'_>> {
        assert_eq!(counts.iter().sum::<usize>(), self.nrows);

        let buffers = self
            .buffers
            .take()
            .expect("buffer is empty, partition writer must be called twice");
        let nbuffers = buffers.len();
        let mut ret = vec![];

        let mut views: Vec<_> = buffers.into_iter().map(|v| Some(v)).collect();
        for &c in counts {
            let mut sub_buffers = vec![];
            for bid in 0..nbuffers {
                let view = views[bid].take();
                let (splitted, rest) = view.unwrap().split_at(Axis(1), c);
                views[bid] = Some(rest);
                sub_buffers.push(splitted);
            }
            ret.push(PandasPartitionWriter::new(
                c,
                sub_buffers,
                &self.schema,
                &self.column_buffer_index,
            ));
        }
        ret
    }

    fn schema(&self) -> &[DataType] {
        self.schema.as_slice()
    }
}

pub struct PandasPartitionWriter<'a> {
    nrows: usize,
    buffers: Vec<AnyArrayViewMut<'a, Ix2>>,
    schema: &'a [DataType],
    column_buffer_index: &'a [(usize, usize)],
}

impl<'a> PandasPartitionWriter<'a> {
    fn new(
        nrows: usize,
        buffers: Vec<AnyArrayViewMut<'a, Ix2>>,
        schema: &'a [DataType],
        column_buffer_index: &'a [(usize, usize)],
    ) -> Self {
        Self {
            nrows,
            buffers,
            schema,
            column_buffer_index,
        }
    }
}

impl<'a> PartitionWriter<'a> for PandasPartitionWriter<'a> {
    type TypeSystem = DataType;

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.schema.len()
    }
}

impl<'a, T> Consume<T> for PandasPartitionWriter<'a>
where
    T: TypeAssoc<<Self as PartitionWriter<'a>>::TypeSystem> + 'static,
{
    unsafe fn consume(&mut self, row: usize, col: usize, value: T) {
        let &(bid, col) = &self.column_buffer_index[col];
        let mut_view = self.buffers[bid].udowncast::<T>();
        // row and column in numpy and dataframe are inverse
        *mut_view.get_mut((col, row)).unwrap() = value;
    }

    fn consume_checked(&mut self, row: usize, col: usize, value: T) -> Result<()> {
        self.schema[col].check::<T>()?;
        let &(bid, col) = &self.column_buffer_index[col];

        let mut_view =
            self.buffers[bid]
                .downcast::<T>()
                .ok_or(ConnectorAgentError::UnexpectedType(
                    self.schema[col],
                    type_name::<T>(),
                ))?;
        // row and column in numpy and dataframe are inverse
        *mut_view
            .get_mut((col, row))
            .ok_or(ConnectorAgentError::OutOfBound)? = value;
        Ok(())
    }
}
