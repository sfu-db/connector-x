use super::funcs::FSeriesStr;
use connector_agent::{
    AnyArrayViewMut, ConnectorAgentError, Consume, DataOrder, DataType, PartitionWriter, Realize,
    Result, TypeAssoc, TypeSystem, Writer,
};
use fehler::{throw, throws};
use ndarray::{Axis, Ix2};
use numpy::PyArray;
use pyo3::{
    types::{PyDict, PyList},
    PyAny, Python,
};
use std::any::type_name;

pub struct PandasWriter<'a> {
    py: Python<'a>,
    nrows: Option<usize>,
    schema: Option<Vec<DataType>>,
    buffers: Option<&'a PyList>,
    column_buffer_index: Option<Vec<(usize, usize)>>,
    dataframe: Option<&'a PyAny>, // Using this field other than the return purpose should be careful: this refers to the same data as buffers
}

impl<'a> PandasWriter<'a> {
    pub fn new(py: Python<'a>) -> Self {
        PandasWriter {
            py,
            nrows: None,
            schema: None,
            buffers: None,
            column_buffer_index: None,
            dataframe: None,
        }
    }

    pub fn result(self) -> &'a PyAny {
        self.dataframe.unwrap()
    }
}

impl<'a> Writer for PandasWriter<'a> {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type TypeSystem = DataType;
    type PartitionWriter<'b> = PandasPartitionWriter<'b>;

    #[throws(ConnectorAgentError)]
    fn allocate(&mut self, nrows: usize, schema: Vec<DataType>, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }

        if matches!(self.nrows, Some(_)) {
            throw!(ConnectorAgentError::DuplicatedAllocation);
        }

        let (df, buffers, index) = create_dataframe(self.py, &schema, nrows);

        // get index for each column: (index of block, index of column within the block)
        let column_buffer_index: Vec<(usize, usize)> =
            index.iter().map(|tuple| tuple.extract().unwrap()).collect();

        self.nrows = Some(nrows);
        self.schema = Some(schema);
        self.buffers = Some(buffers);
        self.column_buffer_index = Some(column_buffer_index);
        self.dataframe = Some(df)
    }

    fn partition_writers(&mut self, counts: &[usize]) -> Vec<Self::PartitionWriter<'_>> {
        if matches!(self.nrows, None) {
            panic!("{}", ConnectorAgentError::WriterNotAllocated);
        }

        assert_eq!(counts.iter().sum::<usize>(), self.nrows.unwrap());

        // get schema for each block, init by U64
        // let mut block_schema_index = vec![DataType::U64; nbuffers];
        // column_buffer_index
        //     .iter()
        //     .zip_eq(schema)
        //     .for_each(|((b, _), &s)| block_schema_index[*b] = s);

        // get array view of each block so we can write data into using rust
        // TODO: cannot support multi-type using FArrayViewMut2 since PyArray does not support String and Option type
        let buffers = self
            .buffers
            .take()
            .ok_or(ConnectorAgentError::WriterNotAllocated)
            .unwrap();

        let buffer_views: Vec<AnyArrayViewMut<'a, Ix2>> = buffers
            .iter()
            .enumerate()
            // .map(|(i, array)| Realize::<FArrayViewMut2>::realize(block_schema_index[i])(array))
            .map(|(_i, array)| {
                let pyarray = array
                    .downcast::<PyArray<u64, Ix2>>()
                    .expect("other types are not supported yet."); // TODO: add support for other dtypes.
                let mut_view = unsafe { pyarray.as_array_mut() };
                AnyArrayViewMut::<Ix2>::new(mut_view)
            })
            .collect();

        let nbuffers = buffer_views.len();
        let mut par_writers = vec![];

        let mut views: Vec<_> = buffer_views.into_iter().map(|v| Some(v)).collect();
        for &c in counts {
            let mut sub_buffers = vec![];
            for bid in 0..nbuffers {
                let view = views[bid].take();
                let (splitted, rest) = view.unwrap().split_at(Axis(1), c);
                views[bid] = Some(rest);
                sub_buffers.push(splitted);
            }
            par_writers.push(PandasPartitionWriter::new(
                c,
                sub_buffers,
                self.schema.as_ref().unwrap(),
                self.column_buffer_index.as_ref().unwrap(),
            ));
        }
        par_writers
    }

    fn schema(&self) -> &[DataType] {
        self.schema.as_ref().unwrap()
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

/// call python code to construct the dataframe and expose its buffers
fn create_dataframe<'a>(
    py: Python<'a>,
    schema: &[DataType],
    nrows: usize,
) -> (&'a PyAny, &'a PyList, &'a PyList) {
    let series: Vec<String> = schema
        .iter()
        .enumerate()
        .map(|(i, &dt)| Realize::<FSeriesStr>::realize(dt)(i, nrows))
        .collect();

    // https://github.com/pandas-dev/pandas/blob/master/pandas/core/internals/managers.py
    // Suppose we want to find the array corresponding to our i'th column.
    // blknos[i] identifies the block from self.blocks that contains this column.
    // blklocs[i] identifies the column of interest within
    // self.blocks[self.blknos[i]]

    let code = format!(
        r#"import pandas as pd
df = pd.DataFrame({{{}}})
blocks = [b.values for b in df._mgr.blocks]
index = [(i, j) for i, j in zip(df._mgr.blknos, df._mgr.blklocs)]"#,
        series.join(",")
    );

    // run python code
    let locals = PyDict::new(py);
    py.run(code.as_str(), None, Some(locals)).unwrap();

    // get # of blocks in dataframe
    let buffers: &PyList = locals
        .get_item("blocks")
        .expect("cannot get `blocks` from locals")
        .downcast::<PyList>()
        .expect("cannot downcast `blocks` to PyList");

    let index = locals
        .get_item("index")
        .expect("cannot get `index` from locals")
        .downcast::<PyList>()
        .expect("cannot downcast `index` to PyList");

    let df = locals.get_item("df").expect("cannot get `df` from locals");

    (df, buffers, index)
}
