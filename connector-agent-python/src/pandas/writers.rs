use super::pandas_column::{
    Float64Block, Float64Column, HasPandasColumn, PandasColumn, PandasColumnObject, UInt64Block,
    UInt64Column,
};
use super::PandasDType;
use connector_agent::{
    ConnectorAgentError, Consume, DataOrder, DataType, PartitionWriter, Result, TypeAssoc,
    TypeSystem, Writer,
};
use fehler::{throw, throws};
use itertools::Itertools;
use pyo3::{
    types::{PyDict, PyList},
    PyAny, Python,
};
use std::any::TypeId;
use std::mem::transmute;

pub struct PandasWriter<'py> {
    py: Python<'py>,
    nrows: Option<usize>,
    schema: Option<Vec<DataType>>,
    buffers: Option<&'py PyList>,
    buffer_types: Option<Vec<DataType>>,
    buffer_column_index: Option<Vec<Vec<usize>>>,
    dataframe: Option<&'py PyAny>, // Using this field other than the return purpose should be careful: this refers to the same data as buffers
}

impl<'a> PandasWriter<'a> {
    pub fn new(py: Python<'a>) -> Self {
        PandasWriter {
            py,
            nrows: None,
            schema: None,
            buffers: None,
            buffer_types: None,
            buffer_column_index: None,
            dataframe: None,
        }
    }

    pub fn result(self) -> Option<&'a PyAny> {
        self.dataframe
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

        let nbuffers = buffers.len();
        // buffer_column_index[i][j] = the column id of the j-th row (pandas buffer stores columns row-wise) in the i-th buffer.
        let mut buffer_column_index = vec![vec![]; nbuffers];
        let mut column_buffer_index_cid: Vec<_> = column_buffer_index.iter().enumerate().collect();
        column_buffer_index_cid.sort_by_key(|(_, blk)| *blk);

        for (cid, &(blkno, _)) in column_buffer_index_cid {
            buffer_column_index[blkno].push(cid);
        }

        // get types for each block
        let mut buffer_types = buffer_column_index.iter().map(|v| schema[v[0]]).collect();

        self.nrows = Some(nrows);
        self.schema = Some(schema);
        self.buffers = Some(buffers);
        self.buffer_types = Some(buffer_types);
        self.buffer_column_index = Some(buffer_column_index);
        self.dataframe = Some(df)
    }

    fn partition_writers(&mut self, counts: &[usize]) -> Vec<Self::PartitionWriter<'_>> {
        if matches!(self.nrows, None) {
            panic!("{}", ConnectorAgentError::WriterNotAllocated);
        }

        assert_eq!(counts.iter().sum::<usize>(), self.nrows.unwrap());

        let buffers = self
            .buffers
            .take()
            .ok_or(ConnectorAgentError::WriterNotAllocated)
            .unwrap();

        let schema = self.schema.as_ref().unwrap();
        let buffer_column_index = self.buffer_column_index.as_ref().unwrap();

        let mut partitioned_columns: Vec<Vec<Box<dyn PandasColumnObject>>> =
            (0..schema.len()).map(|_| vec![]).collect();

        for ((buf, cids), &dtype) in buffers.iter().zip_eq(buffer_column_index).zip_eq(schema) {
            match dtype {
                DataType::F64(_) => {
                    let fblock = Float64Block::extract(self.py, buf).unwrap();
                    let fcols = fblock.split();
                    for (&cid, fcol) in cids.iter().zip_eq(fcols) {
                        partitioned_columns[cid] = fcol
                            .partition(&counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
                DataType::U64(_) => {
                    let ublock = UInt64Block::extract(self.py, buf).unwrap();
                    let ucols = ublock.split();
                    for (&cid, fcol) in cids.iter().zip_eq(ucols) {
                        partitioned_columns[cid] = fcol
                            .partition(&counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
                _ => unimplemented!(),
            }
        }

        let mut par_writers = vec![];
        for &c in counts {
            let columns = partitioned_columns
                .iter_mut()
                .map(|partitions| partitions.pop().unwrap())
                .collect();
            par_writers.push(PandasPartitionWriter::new(
                c,
                columns,
                self.schema.as_ref().unwrap(),
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
    columns: Vec<Box<dyn PandasColumnObject + 'a>>,
    schema: &'a [DataType],
}

impl<'a> PandasPartitionWriter<'a> {
    fn new(
        nrows: usize,
        columns: Vec<Box<dyn PandasColumnObject + 'a>>,
        schema: &'a [DataType],
    ) -> Self {
        Self {
            nrows,
            columns,
            schema,
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
    T: HasPandasColumn + TypeAssoc<DataType>,
{
    unsafe fn consume(&mut self, row: usize, col: usize, value: T) {
        let (column, _): (&mut T::PandasColumn<'a>, *const ()) = transmute(&*self.columns[col]);
        column.write(row, value);
    }

    fn consume_checked(&mut self, row: usize, col: usize, value: T) -> Result<()> {
        self.schema[col].check::<T>()?;
        assert_eq!(self.columns[col].elem_type_id(), TypeId::of::<f64>());

        unsafe { self.consume(row, col, value) };

        Ok(())
    }
}

// impl<'a> Consume<f64> for PandasPartitionWriter<'a> {
//     unsafe fn consume(&mut self, row: usize, col: usize, value: f64) {
//         let (column, _): (&mut Float64Column, *const ()) = transmute(&*self.columns[col]);
//         column.write(row, value);
//     }

//     fn consume_checked(&mut self, row: usize, col: usize, value: f64) -> Result<()> {
//         self.schema[col].check::<f64>()?;
//         assert_eq!(self.columns[col].elem_type_id(), TypeId::of::<f64>());

//         unsafe { self.consume(row, col, value) };

//         Ok(())
//     }
// }

// impl<'a> Consume<Option<f64>> for PandasPartitionWriter<'a> {
//     unsafe fn consume(&mut self, row: usize, col: usize, value: Option<f64>) {
//         let (column, _): (&mut Float64Column, *const ()) = transmute(&*self.columns[col]);
//         column.write(row, value);
//     }

//     fn consume_checked(&mut self, row: usize, col: usize, value: Option<f64>) -> Result<()> {
//         self.schema[col].check::<f64>()?;
//         assert_eq!(self.columns[col].elem_type_id(), TypeId::of::<f64>());

//         unsafe { self.consume(row, col, value) };

//         Ok(())
//     }
// }

// impl<'a> Consume<u64> for PandasPartitionWriter<'a> {
//     unsafe fn consume(&mut self, row: usize, col: usize, value: u64) {
//         let (column, _): (UInt64Column, *const ()) = transmute(self.columns[col]);
//         column.write(row, value);
//     }

//     fn consume_checked(&mut self, row: usize, col: usize, value: u64) -> Result<()> {
//         self.schema[col].check::<u64>()?;
//         assert_eq!(self.columns[col].elem_type_id(), TypeId::of::<u64>());

//         unsafe { self.consume(row, col, value) };

//         Ok(())
//     }
// }

// impl<'a> Consume<Option<u64>> for PandasPartitionWriter<'a> {
//     unsafe fn consume(&mut self, row: usize, col: usize, value: Option<u64>) {
//         let (column, _): (UInt64Column, *const ()) = transmute(self.columns[col]);
//         column.write(row, value);
//     }

//     fn consume_checked(&mut self, row: usize, col: usize, value: Option<u64>) -> Result<()> {
//         self.schema[col].check::<u64>()?;
//         assert_eq!(self.columns[col].elem_type_id(), TypeId::of::<u64>());

//         unsafe { self.consume(row, col, value) };

//         Ok(())
//     }
// }

// impl<'a> Consume<String> for PandasPartitionWriter<'a> {
//     unsafe fn consume(&mut self, row: usize, col: usize, value: String) {
//         todo!()
//     }

//     fn consume_checked(&mut self, row: usize, col: usize, value: String) -> Result<()> {
//         todo!()
//     }
// }

// impl<'a> Consume<Option<String>> for PandasPartitionWriter<'a> {
//     unsafe fn consume(&mut self, row: usize, col: usize, value: Option<String>) {
//         todo!()
//     }

//     fn consume_checked(&mut self, row: usize, col: usize, value: Option<String>) -> Result<()> {
//         todo!()
//     }
// }

// impl<'a> Consume<bool> for PandasPartitionWriter<'a> {
//     unsafe fn consume(&mut self, row: usize, col: usize, value: bool) {
//         todo!()
//     }

//     fn consume_checked(&mut self, row: usize, col: usize, value: bool) -> Result<()> {
//         todo!()
//     }
// }

// impl<'a> Consume<Option<bool>> for PandasPartitionWriter<'a> {
//     unsafe fn consume(&mut self, row: usize, col: usize, value: Option<bool>) {
//         todo!()
//     }

//     fn consume_checked(&mut self, row: usize, col: usize, value: Option<bool>) -> Result<()> {
//         todo!()
//     }
// }
// impl<'a, T> Consume<T> for PandasPartitionWriter<'a>
// where
//     T: TypeAssoc<<Self as PartitionWriter<'a>>::TypeSystem> + 'static,
// {
//     unsafe fn consume(&mut self, row: usize, col: usize, value: T) {
//         let &(bid, col) = &self.column_buffer_index[col];
//         let mut_view = self.buffers[bid].udowncast::<T>();
//         // row and column in numpy and dataframe are inverse
//         *mut_view.get_mut((col, row)).unwrap() = value;
//     }

//     fn consume_checked(&mut self, row: usize, col: usize, value: T) -> Result<()> {
//         self.schema[col].check::<T>()?;
//         let &(bid, col) = &self.column_buffer_index[col];

//         let mut_view =
//             self.buffers[bid]
//                 .downcast::<T>()
//                 .ok_or(ConnectorAgentError::UnexpectedType(
//                     self.schema[col],
//                     type_name::<T>(),
//                 ))?;
//         // row and column in numpy and dataframe are inverse
//         *mut_view
//             .get_mut((col, row))
//             .ok_or(ConnectorAgentError::OutOfBound)? = value;
//         Ok(())
//     }
// }

/// call python code to construct the dataframe and expose its buffers
fn create_dataframe<'a>(
    py: Python<'a>,
    schema: &[DataType],
    nrows: usize,
) -> (&'a PyAny, &'a PyList, &'a PyList) {
    let series: Vec<String> = schema
        .iter()
        .enumerate()
        .map(|(i, &dt)| format!("'{}': pd.Series(dtype='{}')", i, dt.dtype()))
        .collect();

    // https://github.com/pandas-dev/pandas/blob/master/pandas/core/internals/managers.py
    // Suppose we want to find the array corresponding to our i'th column.
    // blknos[i] identifies the block from self.blocks that contains this column.
    // blklocs[i] identifies the column of interest within
    // self.blocks[self.blknos[i]]

    let code = format!(
        r#"import pandas as pd
df = pd.DataFrame(index=range({}), {{{}}})
blocks = [b.values for b in df._mgr.blocks]
index = [(i, j) for i, j in zip(df._mgr.blknos, df._mgr.blklocs)]"#,
        nrows,
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
