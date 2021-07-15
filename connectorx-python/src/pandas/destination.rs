use super::pandas_columns::{
    BooleanBlock, BytesBlock, DateTimeBlock, Float64Block, HasPandasColumn, Int64Block,
    PandasColumn, PandasColumnObject, PyBytes, StringBlock,
};
use super::pystring::PyString;
use super::types::{PandasBlockType, PandasDType, PandasTypeSystem};
use anyhow::anyhow;
use connectorx::{
    ConnectorAgentError, Consume, DataOrder, Destination, DestinationPartition, Result, TypeAssoc,
    TypeSystem,
};
use fehler::{throw, throws};
use itertools::Itertools;
use log::debug;
use numpy::{PyArray1, PyArray2};
use pyo3::{
    types::{PyDict, PyList, PyTuple},
    FromPyObject, PyAny, Python,
};
use std::collections::HashMap;
use std::mem::transmute;

struct PandasBlockInfo {
    dt: PandasBlockType,
    cids: Vec<usize>, // column ids
    idx: usize,       // index in numpy array list
}

pub struct PandasDestination<'py> {
    py: Python<'py>,
    nrow: usize,
    schema: Vec<PandasTypeSystem>,
    col_names: String,
    arr_list: &'py PyList,
    blocks: Vec<PandasBlockInfo>,
}

impl<'a> PandasDestination<'a> {
    pub fn new(py: Python<'a>) -> Self {
        PandasDestination {
            py,
            nrow: 0,
            schema: vec![],
            col_names: String::new(),
            arr_list: PyList::empty(py),
            blocks: vec![],
        }
    }

    pub fn result(self) -> Result<&'a PyAny> {
        // generate code converting numpy arrays into pd.DataFrame
        let blocks_code: Vec<String> = self
            .blocks
            .iter()
            .map(|block| match block.dt.is_masked() {
                true => format!("pd.core.internals.make_block(pd.core.arrays.{}(arr_list[{}][0], arr_list[{}][1]), placement={})", block.dt.array_name(), block.idx, block.idx, block.cids[0]),
                false => {
                    match block.dt.array_name() {
                        "" => format!("pd.core.internals.make_block(arr_list[{}], placement={:?})", block.idx, block.cids),
                        name => format!("pd.core.internals.make_block(pd.core.arrays.{}(arr_list[{}]), placement={:?})", name, block.idx, block.cids)
                    }
                }
            })
            .collect();

        let code = format!(
            r#"import pandas as pd
blocks = [{}]
block_manager = pd.core.internals.BlockManager(
    blocks, [pd.Index({}), pd.RangeIndex(start=0, stop={}, step=1)])
df = pd.DataFrame(block_manager)"#,
            blocks_code.join(","),
            self.col_names,
            self.nrow,
        );
        debug!("convert to dataframe:\n {}", code);

        // run python code
        let locals = PyDict::new(self.py);
        locals
            .set_item("arr_list", self.arr_list)
            .map_err(|e| anyhow!(e))?;
        self.py
            .run(code.as_str(), None, Some(locals))
            .map_err(|e| anyhow!(e))?;

        // get pd.DataFrame
        let df = locals
            .get_item("df")
            .ok_or_else(|| anyhow!("cannot get `df` from locals"))?;
        Ok(df)
    }

    #[throws(ConnectorAgentError)]
    fn allocate_array<T: numpy::Element>(&mut self, dt: PandasBlockType, placement: Vec<usize>) {
        let data = PyArray2::<T>::new(self.py, [placement.len(), self.nrow], false);
        let block_info = PandasBlockInfo {
            dt,
            cids: placement,
            idx: self.arr_list.len(),
        };
        self.arr_list.append(data).map_err(|e| anyhow!(e))?;
        self.blocks.push(block_info);
    }

    #[throws(ConnectorAgentError)]
    fn allocate_masked_array<T: numpy::Element>(
        &mut self,
        dt: PandasBlockType,
        placement: Vec<usize>,
    ) {
        for pos in placement {
            let block_info = PandasBlockInfo {
                dt,
                cids: vec![pos],
                idx: self.arr_list.len(),
            };
            let data = PyArray1::<T>::new(self.py, self.nrow, false);
            let mask = PyArray1::<bool>::new(self.py, self.nrow, false);
            self.arr_list
                .append(PyTuple::new(self.py, vec![data.as_ref(), mask.as_ref()]))
                .map_err(|e| anyhow!(e))?;
            self.blocks.push(block_info);
        }
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
        self.nrow = nrows;
        self.schema = schema.to_vec();
        let names: Vec<_> = names.into_iter().map(|s| s.as_ref()).collect();
        self.col_names = format!("['{}']", names.join("\',\'"));

        let mut block_indices = HashMap::<PandasBlockType, Vec<usize>>::new();
        schema.iter().enumerate().for_each(|(i, dt)| {
            block_indices
                .entry((*dt).into())
                .and_modify(|e| e.push(i))
                .or_insert(vec![i]);
        });

        for (dt, placement) in block_indices {
            match dt {
                PandasBlockType::Boolean(true) => {
                    self.allocate_masked_array::<bool>(dt, placement)?;
                }
                PandasBlockType::Boolean(false) => {
                    self.allocate_array::<bool>(dt, placement)?;
                }
                PandasBlockType::Int64(true) => {
                    self.allocate_masked_array::<i64>(dt, placement)?;
                }
                PandasBlockType::Int64(false) => {
                    self.allocate_array::<i64>(dt, placement)?;
                }
                PandasBlockType::Float64 => {
                    self.allocate_array::<f64>(dt, placement)?;
                }
                PandasBlockType::String => {
                    self.allocate_array::<PyString>(dt, placement)?;
                }
                PandasBlockType::DateTime => {
                    self.allocate_array::<i64>(dt, placement)?;
                }
                PandasBlockType::Bytes => {
                    self.allocate_array::<PyBytes>(dt, placement)?;
                }
            };
        }
    }

    #[throws(ConnectorAgentError)]
    fn partition(&mut self, counts: &[usize]) -> Vec<Self::Partition<'_>> {
        assert_eq!(
            counts.iter().sum::<usize>(),
            self.nrow,
            "counts: {} != nrows: {:?}",
            counts.iter().sum::<usize>(),
            self.nrow
        );

        let mut partitioned_columns: Vec<Vec<Box<dyn PandasColumnObject>>> =
            (0..self.schema.len()).map(|_| vec![]).collect();

        for block in self.blocks.iter() {
            let buf = self.arr_list.get_item(block.idx as isize);
            match block.dt {
                PandasBlockType::Boolean(_) => {
                    let bblock = BooleanBlock::extract(buf).map_err(|e| anyhow!(e))?;

                    let bcols = bblock.split()?;
                    for (&cid, bcol) in block.cids.iter().zip_eq(bcols) {
                        partitioned_columns[cid] = bcol
                            .partition(&counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
                PandasBlockType::Float64 => {
                    let fblock = Float64Block::extract(buf).map_err(|e| anyhow!(e))?;
                    let fcols = fblock.split()?;
                    for (&cid, fcol) in block.cids.iter().zip_eq(fcols) {
                        partitioned_columns[cid] = fcol
                            .partition(&counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
                PandasBlockType::Int64(_) => {
                    let ublock = Int64Block::extract(buf).map_err(|e| anyhow!(e))?;
                    let ucols = ublock.split()?;
                    for (&cid, ucol) in block.cids.iter().zip_eq(ucols) {
                        partitioned_columns[cid] = ucol
                            .partition(&counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
                PandasBlockType::String => {
                    let sblock = StringBlock::extract(buf).map_err(|e| anyhow!(e))?;
                    let scols = sblock.split()?;
                    for (&cid, scol) in block.cids.iter().zip_eq(scols) {
                        partitioned_columns[cid] = scol
                            .partition(&counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
                PandasBlockType::Bytes => {
                    let bblock = BytesBlock::extract(buf).map_err(|e| anyhow!(e))?;
                    let bcols = bblock.split()?;
                    for (&cid, bcol) in block.cids.iter().zip_eq(bcols) {
                        partitioned_columns[cid] = bcol
                            .partition(&counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
                PandasBlockType::DateTime => {
                    let dblock = DateTimeBlock::extract(buf).map_err(|e| anyhow!(e))?;
                    let dcols = dblock.split()?;
                    for (&cid, dcol) in block.cids.iter().zip_eq(dcols) {
                        partitioned_columns[cid] = dcol
                            .partition(&counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
            }
        }

        let mut par_destinations = vec![];
        for &c in counts.into_iter().rev() {
            let mut columns = Vec::with_capacity(partitioned_columns.len());
            for (i, partitions) in partitioned_columns.iter_mut().enumerate() {
                columns.push(
                    partitions
                        .pop()
                        .ok_or_else(|| anyhow!("empty partition for {}th column", i))?,
                );
            }

            par_destinations.push(PandasPartitionDestination::new(
                c,
                columns,
                &self.schema[..],
            ));
        }

        // We need to reverse the par_destinations because partitions are poped reversely
        par_destinations.into_iter().rev().collect()
    }

    fn schema(&self) -> &[Self::TypeSystem] {
        self.schema.as_ref()
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
            col.finalize()?;
        }
        Ok(())
    }
}

impl<'a, T> Consume<T> for PandasPartitionDestination<'a>
where
    T: HasPandasColumn + TypeAssoc<PandasTypeSystem> + std::fmt::Debug,
{
    fn consume(&mut self, value: T) -> Result<()> {
        let (_, col) = self.loc();

        self.schema[col].check::<T>()?;
        // How do we check type id for borrowed types?
        // assert!(self.columns[col].typecheck(TypeId::of::<T>()));

        let (column, _): (&mut T::PandasColumn<'a>, *const ()) =
            unsafe { transmute(&*self.columns[col]) };
        column.write(value)
    }
}
