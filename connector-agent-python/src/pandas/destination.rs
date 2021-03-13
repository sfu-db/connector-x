use super::pandas_columns::{
    BooleanBlock, DateTimeBlock, Float64Block, HasPandasColumn, Int64Block, PandasColumn,
    PandasColumnObject, StringBlock,
};
use super::types::{PandasDType, PandasTypeSystem};
use anyhow::anyhow;
use connector_agent::{
    ConnectorAgentError, Consume, DataOrder, Destination, DestinationPartition, Result, TypeAssoc,
    TypeSystem,
};
use fehler::{throw, throws};
use itertools::Itertools;
use log::debug;
use pyo3::{
    types::{PyDict, PyList},
    FromPyObject, PyAny, Python,
};
use std::any::TypeId;
use std::collections::HashMap;
use std::mem::transmute;
pub struct PandasDestination<'py> {
    py: Python<'py>,
    nrows: Option<usize>,
    schema: Option<Vec<PandasTypeSystem>>,
    buffers: Option<&'py PyList>,
    buffer_column_index: Option<Vec<Vec<usize>>>,
    dataframe: Option<&'py PyAny>, // Using this field other than the return purpose should be careful: this refers to the same data as buffers
}

impl<'a> PandasDestination<'a> {
    pub fn new(py: Python<'a>) -> Self {
        PandasDestination {
            py,
            nrows: None,
            schema: None,
            buffers: None,
            buffer_column_index: None,
            dataframe: None,
        }
    }

    pub fn result(self) -> Option<&'a PyAny> {
        self.dataframe
    }
}

impl<'a> Destination for PandasDestination<'a> {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type TypeSystem = PandasTypeSystem;
    type Partition<'b> = PandasPartitionDestination<'b>;

    #[throws(ConnectorAgentError)]
    fn allocate<S: AsRef<str>>(
        &mut self,
        nrows: usize,
        names: &[S],
        schema: &[PandasTypeSystem],
        data_order: DataOrder,
    ) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order))
        }

        if matches!(self.nrows, Some(_)) {
            throw!(ConnectorAgentError::DuplicatedAllocation);
        }

        let (df, buffers, index) = create_dataframe(self.py, names, schema, nrows);
        debug!("DataFrame created");

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

        self.nrows = Some(nrows);
        self.schema = Some(schema.to_vec());
        self.buffers = Some(buffers);
        self.buffer_column_index = Some(buffer_column_index);
        self.dataframe = Some(df);
    }

    #[throws(ConnectorAgentError)]
    fn partition(&mut self, counts: &[usize]) -> Vec<Self::Partition<'_>> {
        if matches!(self.nrows, None) {
            panic!("{}", ConnectorAgentError::DestinationNotAllocated);
        }

        assert_eq!(counts.iter().sum::<usize>(), self.nrows.unwrap());

        let buffers = self
            .buffers
            .ok_or(ConnectorAgentError::DestinationNotAllocated)
            .unwrap();

        let schema = self.schema.as_ref().unwrap();
        let buffer_column_index = self.buffer_column_index.as_ref().unwrap();

        let mut partitioned_columns: Vec<Vec<Box<dyn PandasColumnObject>>> =
            (0..schema.len()).map(|_| vec![]).collect();

        for (buf, cids) in buffers.iter().zip_eq(buffer_column_index) {
            for &cid in cids {
                match schema[cid] {
                    PandasTypeSystem::F64(_) => {
                        let fblock = Float64Block::extract(buf).map_err(|e| anyhow!(e))?;
                        let fcols = fblock.split();
                        for (&cid, fcol) in cids.iter().zip_eq(fcols) {
                            partitioned_columns[cid] = fcol
                                .partition(&counts)
                                .into_iter()
                                .map(|c| Box::new(c) as _)
                                .collect()
                        }
                    }
                    PandasTypeSystem::I64(_) => {
                        let ublock = Int64Block::extract(buf).map_err(|e| anyhow!(e))?;
                        let ucols = ublock.split();
                        for (&cid, ucol) in cids.iter().zip_eq(ucols) {
                            partitioned_columns[cid] = ucol
                                .partition(&counts)
                                .into_iter()
                                .map(|c| Box::new(c) as _)
                                .collect()
                        }
                    }
                    PandasTypeSystem::Bool(_) => {
                        let bblock = BooleanBlock::extract(buf).map_err(|e| anyhow!(e))?;
                        let bcols = bblock.split();
                        for (&cid, bcol) in cids.iter().zip_eq(bcols) {
                            partitioned_columns[cid] = bcol
                                .partition(&counts)
                                .into_iter()
                                .map(|c| Box::new(c) as _)
                                .collect()
                        }
                    }
                    PandasTypeSystem::String(_) => {
                        let block = StringBlock::extract(buf).map_err(|e| anyhow!(e))?;
                        let cols = block.split();
                        for (&cid, col) in cids.iter().zip_eq(cols) {
                            partitioned_columns[cid] = col
                                .partition(&counts)
                                .into_iter()
                                .map(|c| Box::new(c) as _)
                                .collect()
                        }
                    }
                    PandasTypeSystem::DateTime(_) => {
                        let block = DateTimeBlock::extract(buf).map_err(|e| anyhow!(e))?;
                        let cols = block.split();
                        for (&cid, col) in cids.iter().zip_eq(cols) {
                            partitioned_columns[cid] = col
                                .partition(&counts)
                                .into_iter()
                                .map(|c| Box::new(c) as _)
                                .collect()
                        }
                    }
                }
            }
        }

        let mut par_destinations = vec![];
        for &c in counts.into_iter().rev() {
            let columns: Vec<_> = partitioned_columns
                .iter_mut()
                .map(|partitions| partitions.pop().unwrap())
                .collect();

            par_destinations.push(PandasPartitionDestination::new(
                c,
                columns,
                self.schema.as_ref().unwrap(),
            ));
        }

        // We need to reverse the par_destinations because partitions are poped reversely
        par_destinations.into_iter().rev().collect()
    }

    fn schema(&self) -> &[PandasTypeSystem] {
        self.schema.as_ref().unwrap()
    }
}

pub struct PandasPartitionDestination<'a> {
    nrows: usize,
    columns: Vec<Box<dyn PandasColumnObject + 'a>>,
    schema: &'a [PandasTypeSystem],
    seq: usize,
}

impl<'a> PandasPartitionDestination<'a> {
    fn new(
        nrows: usize,
        columns: Vec<Box<dyn PandasColumnObject + 'a>>,
        schema: &'a [PandasTypeSystem],
    ) -> Self {
        Self {
            nrows,
            columns,
            schema,
            seq: 0,
        }
    }

    fn loc(&mut self) -> (usize, usize) {
        let (row, col) = (self.seq / self.ncols(), self.seq % self.ncols());
        self.seq += 1;
        (row, col)
    }
}

impl<'a> DestinationPartition<'a> for PandasPartitionDestination<'a> {
    type TypeSystem = PandasTypeSystem;

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.schema.len()
    }

    fn finalize(&mut self) -> Result<()> {
        for col in &mut self.columns {
            col.finalize();
        }
        Ok(())
    }
}

impl<'a, T> Consume<T> for PandasPartitionDestination<'a>
where
    T: HasPandasColumn + TypeAssoc<PandasTypeSystem> + std::fmt::Debug + 'static,
{
    fn consume(&mut self, value: T) -> Result<()> {
        let (_, col) = self.loc();

        self.schema[col].check::<T>()?;
        assert!(self.columns[col].typecheck(TypeId::of::<T>()));

        let (column, _): (&mut T::PandasColumn<'a>, *const ()) =
            unsafe { transmute(&*self.columns[col]) };
        column.write(value);

        Ok(())
    }
}

/// call python code to construct the dataframe and expose its buffers
fn create_dataframe<'a, S: AsRef<str>>(
    py: Python<'a>,
    names: &[S],
    schema: &[PandasTypeSystem],
    nrows: usize,
) -> (&'a PyAny, &'a PyList, &'a PyList) {
    let names: Vec<_> = names.into_iter().map(|s| s.as_ref()).collect();
    debug!("names: {:?}", names);
    debug!("schema: {:?}", schema);

    let mut schema_dict: HashMap<PandasTypeSystem, Vec<usize>> = HashMap::new();
    schema.iter().enumerate().for_each(|(idx, &dt)| {
        let indices = schema_dict.entry(dt).or_insert(vec![]);
        indices.push(idx);
    });
    debug!("schema_dict: {:?}", schema_dict);

    let mut blocks_code = vec![];
    schema_dict
        .iter()
        .for_each(|(&dt, indices)| {
            if dt.is_extension() {
                // each extension block only contains one column
                for idx in indices {
                blocks_code.push(format!(
                    "pd.core.internals.ExtensionBlock(pd.array(np.empty([{}], dtype='{}'), dtype='{}'), placement={}, ndim=2)",
                    nrows,
                    dt.npdtype(),
                    dt.dtype(),
                    idx,
                ));
                }
            } else {
                blocks_code.push(format!(
                    "pd.core.internals.{}(np.empty([{}, {}], dtype='{}'), placement={:?}, ndim=2)",
                    dt.block_name(),
                    indices.len(),
                    nrows,
                    dt.npdtype(),
                    indices,
                ));
            }
        });

    // https://github.com/pandas-dev/pandas/blob/master/pandas/core/internals/managers.py
    // Suppose we want to find the array corresponding to our i'th column.
    // blknos[i] identifies the block from self.blocks that contains this column.
    // blklocs[i] identifies the column of interest within
    // self.blocks[self.blknos[i]]

    let code = format!(
        r#"import pandas as pd
import numpy as np
blocks = [{}]
block_manager = pd.core.internals.BlockManager(
    blocks, [pd.Index(['{}']), pd.RangeIndex(start=0, stop={}, step=1)])
df = pd.DataFrame(block_manager)
blocks = [b.values for b in df._mgr.blocks]
index = [(i, j) for i, j in zip(df._mgr.blknos, df._mgr.blklocs)]"#,
        blocks_code.join(","),
        format!("{}", names.join("\',\'")),
        nrows,
    );
    debug!("create dataframe code: {}", code);

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
