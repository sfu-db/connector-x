use super::{
    pandas_columns::{
        ArrayBlock, BooleanBlock, BytesBlock, DateTimeBlock, Float64Block, HasPandasColumn,
        Int64Block, PandasColumn, PandasColumnObject, PyBytes, StringBlock,
    },
    pystring::PyString,
    typesystem::{PandasArrayType, PandasBlockType, PandasTypeSystem},
};
use crate::errors::{ConnectorXPythonError, Result};
use anyhow::anyhow;
use connectorx::prelude::*;
use fehler::{throw, throws};
use itertools::Itertools;
use numpy::{PyArray1, PyArray2};
use pyo3::{
    prelude::{pyclass, pymethods, PyResult},
    types::{IntoPyDict, PyList, PyTuple},
    FromPyObject, IntoPy, PyAny, PyObject, Python,
};
use std::{
    collections::HashMap,
    mem::transmute,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

#[pyclass]
pub struct PandasBlockInfo {
    dt: PandasBlockType,
    #[pyo3(get, set)]
    cids: Vec<usize>, // column ids
}

#[pymethods]
impl PandasBlockInfo {
    #[getter]
    fn dt(&self) -> PyResult<u32> {
        Ok(PandasArrayType::from(self.dt) as u32)
    }
}

pub struct PandasDestination<'py> {
    py: Python<'py>,
    nrow: usize,
    schema: Vec<PandasTypeSystem>,
    names: Vec<String>,
    block_datas: Vec<&'py PyAny>, // either 2d array for normal blocks, or two 1d arrays for extension blocks
    block_infos: Vec<PandasBlockInfo>,
}

impl<'a> PandasDestination<'a> {
    pub fn new(py: Python<'a>) -> Self {
        PandasDestination {
            py,
            nrow: 0,
            schema: vec![],
            names: vec![],
            block_datas: vec![],
            block_infos: vec![],
        }
    }

    pub fn result(self) -> Result<&'a PyAny> {
        #[throws(ConnectorXPythonError)]
        fn to_list<T: IntoPy<PyObject>>(py: Python<'_>, arr: Vec<T>) -> &'_ PyList {
            let list = PyList::empty(py);
            for e in arr {
                list.append(e.into_py(py))?;
            }
            list
        }
        let block_infos = to_list(self.py, self.block_infos)?;
        let names = to_list(self.py, self.names)?;
        let block_datas = to_list(self.py, self.block_datas)?;
        let result = [
            ("data", block_datas),
            ("headers", names),
            ("block_infos", block_infos),
        ]
        .into_py_dict(self.py);
        Ok(result)
    }

    #[throws(ConnectorXPythonError)]
    fn allocate_array<T: numpy::Element + 'a>(
        &mut self,
        dt: PandasBlockType,
        placement: Vec<usize>,
    ) {
        // has to use `zeros` instead of `new` for String type initialization
        let data = PyArray2::<T>::zeros(self.py, [placement.len(), self.nrow], false);
        let block_info = PandasBlockInfo {
            dt,
            cids: placement,
        };

        self.block_datas.push(data.into());
        self.block_infos.push(block_info);
    }

    #[throws(ConnectorXPythonError)]
    fn allocate_masked_array<T: numpy::Element + 'a>(
        &mut self,
        dt: PandasBlockType,
        placement: Vec<usize>,
    ) {
        for pos in placement {
            let block_info = PandasBlockInfo {
                dt,
                cids: vec![pos],
            };
            let data = PyArray1::<T>::zeros(self.py, self.nrow, false);
            let mask = PyArray1::<bool>::zeros(self.py, self.nrow, false);
            let obj = PyTuple::new(self.py, vec![data.as_ref(), mask.as_ref()]);
            self.block_datas.push(obj.into());
            self.block_infos.push(block_info);
        }
    }
}

impl<'a> Destination for PandasDestination<'a> {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type TypeSystem = PandasTypeSystem;
    type Partition<'b> = PandasPartitionDestination<'b> where 'a: 'b;
    type Error = ConnectorXPythonError;

    fn needs_count(&self) -> bool {
        true
    }

    #[throws(ConnectorXPythonError)]
    fn allocate<S: AsRef<str>>(
        &mut self,
        nrows: usize,
        names: &[S],
        schema: &[PandasTypeSystem],
        data_order: DataOrder,
    ) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorXError::UnsupportedDataOrder(data_order))
        }
        self.nrow = nrows;
        self.schema = schema.to_vec();
        self.names
            .extend(names.iter().map(AsRef::as_ref).map(ToString::to_string));

        let mut block_indices = HashMap::<PandasBlockType, Vec<usize>>::new();
        schema
            .iter()
            .enumerate()
            .for_each(|(i, dt)| block_indices.entry((*dt).into()).or_default().push(i));

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
                PandasBlockType::Float64Array => {
                    self.allocate_array::<super::pandas_columns::PyList>(dt, placement)?;
                }
                PandasBlockType::Int64Array => {
                    self.allocate_array::<super::pandas_columns::PyList>(dt, placement)?;
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

    #[throws(ConnectorXPythonError)]
    fn partition(&mut self, counts: usize) -> Vec<Self::Partition<'_>> {
        let mut partitioned_columns: Vec<Vec<Box<dyn PandasColumnObject>>> =
            (0..self.schema.len()).map(|_| Vec::new()).collect();

        for (idx, block) in self.block_infos.iter().enumerate() {
            let buf = self.block_datas[idx];
            match block.dt {
                PandasBlockType::Boolean(_) => {
                    let bblock = BooleanBlock::extract(buf)?;

                    let bcols = bblock.split()?;
                    for (&cid, bcol) in block.cids.iter().zip_eq(bcols) {
                        partitioned_columns[cid] = bcol
                            .partition(counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
                PandasBlockType::Float64 => {
                    let fblock = Float64Block::extract(buf)?;
                    let fcols = fblock.split()?;
                    for (&cid, fcol) in block.cids.iter().zip_eq(fcols) {
                        partitioned_columns[cid] = fcol
                            .partition(counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
                PandasBlockType::Float64Array => {
                    let fblock = ArrayBlock::<f64>::extract(buf)?;
                    let fcols = fblock.split()?;
                    for (&cid, fcol) in block.cids.iter().zip_eq(fcols) {
                        partitioned_columns[cid] = fcol
                            .partition(counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
                PandasBlockType::Int64Array => {
                    let fblock = ArrayBlock::<i64>::extract(buf)?;
                    let fcols = fblock.split()?;
                    for (&cid, fcol) in block.cids.iter().zip_eq(fcols) {
                        partitioned_columns[cid] = fcol
                            .partition(counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
                PandasBlockType::Int64(_) => {
                    let ublock = Int64Block::extract(buf)?;
                    let ucols = ublock.split()?;
                    for (&cid, ucol) in block.cids.iter().zip_eq(ucols) {
                        partitioned_columns[cid] = ucol
                            .partition(counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
                PandasBlockType::String => {
                    let sblock = StringBlock::extract(buf)?;
                    let scols = sblock.split()?;
                    for (&cid, scol) in block.cids.iter().zip_eq(scols) {
                        partitioned_columns[cid] = scol
                            .partition(counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
                PandasBlockType::Bytes => {
                    let bblock = BytesBlock::extract(buf)?;
                    let bcols = bblock.split()?;
                    for (&cid, bcol) in block.cids.iter().zip_eq(bcols) {
                        partitioned_columns[cid] = bcol
                            .partition(counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
                PandasBlockType::DateTime => {
                    let dblock = DateTimeBlock::extract(buf)?;
                    let dcols = dblock.split()?;
                    for (&cid, dcol) in block.cids.iter().zip_eq(dcols) {
                        partitioned_columns[cid] = dcol
                            .partition(counts)
                            .into_iter()
                            .map(|c| Box::new(c) as _)
                            .collect()
                    }
                }
            }
        }

        let mut par_destinations = vec![];
        let glob_row = Arc::new(AtomicUsize::new(0));
        for _ in 0..counts {
            let mut columns = Vec::with_capacity(partitioned_columns.len());
            for (i, partitions) in partitioned_columns.iter_mut().enumerate() {
                columns.push(
                    partitions
                        .pop()
                        .ok_or_else(|| anyhow!("empty partition for {}th column", i))?,
                );
            }

            par_destinations.push(PandasPartitionDestination::new(
                columns,
                &self.schema[..],
                Arc::clone(&glob_row),
            ));
        }

        par_destinations
    }

    fn schema(&self) -> &[Self::TypeSystem] {
        self.schema.as_ref()
    }
}
pub struct PandasPartitionDestination<'a> {
    columns: Vec<Box<dyn PandasColumnObject + 'a>>,
    schema: &'a [PandasTypeSystem],
    seq: usize,
    glob_row: Arc<AtomicUsize>,
    cur_row: usize,
}

impl<'a> PandasPartitionDestination<'a> {
    fn new(
        columns: Vec<Box<dyn PandasColumnObject + 'a>>,
        schema: &'a [PandasTypeSystem],
        glob_row: Arc<AtomicUsize>,
    ) -> Self {
        Self {
            columns,
            schema,
            seq: 0,
            glob_row,
            cur_row: 0,
        }
    }

    fn loc(&mut self) -> (usize, usize) {
        let (row, col) = (
            self.cur_row + self.seq / self.ncols(),
            self.seq % self.ncols(),
        );
        self.seq += 1;
        (row, col)
    }
}

impl<'a> DestinationPartition<'a> for PandasPartitionDestination<'a> {
    type TypeSystem = PandasTypeSystem;
    type Error = ConnectorXPythonError;

    fn ncols(&self) -> usize {
        self.schema.len()
    }

    fn finalize(&mut self) -> Result<()> {
        for col in &mut self.columns {
            col.finalize()?;
        }
        Ok(())
    }

    #[throws(ConnectorXPythonError)]
    fn aquire_row(&mut self, n: usize) -> usize {
        if n == 0 {
            return self.cur_row;
        }
        self.cur_row = self.glob_row.fetch_add(n, Ordering::Relaxed);
        self.seq = 0;
        self.cur_row
    }
}

impl<'a, T> Consume<T> for PandasPartitionDestination<'a>
where
    T: HasPandasColumn + TypeAssoc<PandasTypeSystem> + std::fmt::Debug,
{
    type Error = ConnectorXPythonError;

    fn consume(&mut self, value: T) -> Result<()> {
        let (row, col) = self.loc();

        self.schema[col].check::<T>()?;
        // How do we check type id for borrowed types?
        // assert!(self.columns[col].typecheck(TypeId::of::<T>()));

        let (column, _): (&mut T::PandasColumn<'a>, *const ()) =
            unsafe { transmute(&*self.columns[col]) };
        column.write(value, row)
    }
}
