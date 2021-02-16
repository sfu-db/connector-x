use connector_agent::{
    AnyArrayViewMut, ConnectorAgentError, Consume, DataOrder, DataType, PartitionWriter, Result,
    TypeAssoc, TypeSystem, Writer,
};
use fehler::{throw, throws};
use ndarray::{Axis, Ix2};
use std::any::type_name;

pub mod funcs;
pub mod pandas_assoc;

pub struct PandasWriter<'a> {
    nrows: usize,
    schema: Vec<DataType>,
    buffers: Option<Vec<AnyArrayViewMut<'a, Ix2>>>,
    column_buffer_index: Vec<(usize, usize)>,
}

impl<'a> PandasWriter<'a> {
    pub fn new(
        nrows: usize,
        schema: Vec<DataType>,
        buffers: Vec<AnyArrayViewMut<'a, Ix2>>,
        column_buffer_index: Vec<(usize, usize)>,
    ) -> Self {
        PandasWriter {
            nrows,
            schema,
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
        // real memory allocation happened before construction
    }

    fn partition_writers(&mut self, counts: &[usize]) -> Vec<Self::PartitionWriter<'_>> {
        assert_eq!(counts.iter().sum::<usize>(), self.nrows);
        let buffers = self.buffers.take().unwrap();
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

pub struct PandasPartitionWriter<'a> {
    nrows: usize,
    buffers: Vec<AnyArrayViewMut<'a, Ix2>>,
    schema: Vec<DataType>,
    column_buffer_index: Vec<(usize, usize)>,
}

impl<'a> PandasPartitionWriter<'a> {
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
