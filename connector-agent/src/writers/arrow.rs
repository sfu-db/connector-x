use super::{Consume, PartitionWriter, Writer};
use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::types::DataType;
use crate::typesystem::{ParameterizedFunc, ParameterizedOn, Realize, TypeAssoc, TypeSystem};
use arrow::array::{
    ArrayBuilder, ArrayRef, BooleanBuilder, Float64Builder, StringBuilder, UInt64Builder,
};
use arrow::datatypes::DataType as ArrowDataType;
use arrow::datatypes::{Field, Schema};
use arrow::record_batch::RecordBatch;
use fehler::throws;
use itertools::Itertools;
use std::any::Any;
use std::sync::{Arc, Mutex};

/// Associate arrow with native type
pub trait ArrowAssoc {
    type Builder: ArrayBuilder + Send + Sync;

    fn new_builder(nrows: usize) -> Self::Builder;
    fn append(builder: &mut Self::Builder, value: Self);
    fn new_field(header: &str) -> Field;
}
/// For unsupported datatypes
pub struct FakeBuilder {}

impl ArrayBuilder for FakeBuilder {
    fn len(&self) -> usize {
        unimplemented!();
    }
    fn is_empty(&self) -> bool {
        unimplemented!();
    }
    fn finish(&mut self) -> ArrayRef {
        unimplemented!();
    }
    fn as_any(&self) -> &dyn Any {
        unimplemented!();
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        unimplemented!();
    }
    fn into_box_any(self: Box<Self>) -> Box<dyn Any> {
        unimplemented!();
    }
}

impl ArrowAssoc for u64 {
    type Builder = UInt64Builder;

    fn new_builder(nrows: usize) -> UInt64Builder {
        UInt64Builder::new(nrows)
    }

    fn append(builder: &mut UInt64Builder, value: u64) {
        builder.append_value(value).unwrap();
    }

    fn new_field(header: &str) -> Field {
        Field::new(header, ArrowDataType::UInt64, false)
    }
}

impl ArrowAssoc for Option<u64> {
    type Builder = UInt64Builder;

    fn new_builder(nrows: usize) -> UInt64Builder {
        UInt64Builder::new(nrows)
    }

    fn append(builder: &mut UInt64Builder, value: Option<u64>) {
        builder.append_option(value).unwrap();
    }

    fn new_field(header: &str) -> Field {
        Field::new(header, ArrowDataType::UInt64, true)
    }
}

impl ArrowAssoc for f64 {
    type Builder = Float64Builder;

    fn new_builder(nrows: usize) -> Float64Builder {
        Float64Builder::new(nrows)
    }

    fn append(builder: &mut Self::Builder, value: f64) {
        builder.append_value(value).unwrap();
    }

    fn new_field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Float64, false)
    }
}

impl ArrowAssoc for bool {
    type Builder = BooleanBuilder;

    fn new_builder(nrows: usize) -> BooleanBuilder {
        BooleanBuilder::new(nrows)
    }

    fn append(builder: &mut Self::Builder, value: bool) {
        builder.append_value(value).unwrap();
    }

    fn new_field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Boolean, false)
    }
}

impl ArrowAssoc for String {
    type Builder = StringBuilder;

    fn new_builder(nrows: usize) -> StringBuilder {
        StringBuilder::new(nrows)
    }

    fn append(builder: &mut Self::Builder, value: String) {
        builder.append_value(value.as_str()).unwrap();
    }

    fn new_field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Utf8, false)
    }
}

type GBuilder = Box<dyn Any + Send + Sync>;
type VecGBuilder = Arc<Mutex<Vec<GBuilder>>>;

struct FNewBuilder;

impl ParameterizedFunc for FNewBuilder {
    type Function = fn(nrows: usize) -> GBuilder;
}

impl<T> ParameterizedOn<T> for FNewBuilder
where
    T: ArrowAssoc,
{
    fn parameterize() -> Self::Function {
        fn new_builder<T>(nrows: usize) -> GBuilder
        where
            T: ArrowAssoc,
        {
            Box::new(T::new_builder(nrows)) as GBuilder
        }
        new_builder::<T>
    }
}

struct FFinishBuilder;

impl ParameterizedFunc for FFinishBuilder {
    type Function = fn(&mut GBuilder) -> ArrayRef;
}

impl<T> ParameterizedOn<T> for FFinishBuilder
where
    T: ArrowAssoc,
{
    fn parameterize() -> Self::Function {
        fn build<T>(builder: &mut GBuilder) -> ArrayRef
        where
            T: ArrowAssoc,
        {
            ArrayBuilder::finish(builder.downcast_mut::<T::Builder>().unwrap())
        }
        build::<T>
    }
}

struct FNewField;

impl ParameterizedFunc for FNewField {
    type Function = fn(header: &str) -> Field;
}

impl<T> ParameterizedOn<T> for FNewField
where
    T: ArrowAssoc,
{
    fn parameterize() -> Self::Function {
        fn new_field<T>(header: &str) -> Field
        where
            T: ArrowAssoc,
        {
            T::new_field(header)
        }
        new_field::<T>
    }
}

pub struct ArrowPartitionWriter {
    nrows: usize,
    schema: Vec<DataType>,
    builders: VecGBuilder,
}

impl ArrowPartitionWriter {
    fn new(schema: Vec<DataType>, nrows: usize) -> Self {
        let mut builders = vec![];
        schema
            .iter()
            .for_each(|&dt| builders.push(Realize::<FNewBuilder>::realize(dt)(nrows)));
        ArrowPartitionWriter {
            nrows,
            schema,
            builders: Arc::new(Mutex::new(builders)),
        }
    }

    fn builders(&self) -> VecGBuilder {
        Arc::clone(&self.builders)
    }
}

impl<'a> PartitionWriter<'a> for ArrowPartitionWriter {
    type TypeSystem = DataType;

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.schema.len()
    }
}

impl<'a, T> Consume<T> for ArrowPartitionWriter
where
    T: TypeAssoc<<Self as PartitionWriter<'a>>::TypeSystem> + ArrowAssoc + 'static,
{
    unsafe fn consume(&mut self, _row: usize, col: usize, value: T) {
        // NOTE: can use `get_mut_unchecked` instead of Mutex in the future to speed up
        <T as ArrowAssoc>::append(
            self.builders.lock().unwrap()[col]
                .downcast_mut::<T::Builder>()
                .unwrap(),
            value,
        );
    }

    fn consume_checked(&mut self, row: usize, col: usize, value: T) -> Result<()> {
        self.schema[col].check::<T>()?;
        unsafe { self.write(row, col, value) };
        Ok(())
    }
}

pub struct ArrowWriter {
    nrows: usize,
    schema: Vec<DataType>,
    pbuilders: Vec<VecGBuilder>,
}

impl ArrowWriter {
    pub fn record_batches<'a>(&'a mut self, headers: Vec<String>) -> Vec<RecordBatch> {
        let fields: Vec<Field> = self
            .schema
            .iter()
            .zip_eq(headers)
            .map(|(&dt, h)| Realize::<FNewField>::realize(dt)(h.as_str()))
            .collect();
        let schema = Arc::new(Schema::new(fields));
        let self_schema = self.schema.clone();
        self.pbuilders
            .iter_mut()
            .map(|pbuilder| {
                let mut columns = vec![];
                pbuilder
                    .lock()
                    .unwrap()
                    .iter_mut()
                    .zip(self_schema.iter())
                    .for_each(|(builder, dt)| {
                        columns.push(Realize::<FFinishBuilder>::realize(*dt)(builder));
                    });
                RecordBatch::try_new(Arc::clone(&schema), columns).unwrap()
            })
            .collect()
    }
}

impl<'a> Writer<'a> for ArrowWriter {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::ColumnMajor, DataOrder::RowMajor];
    type TypeSystem = DataType;
    type PartitionWriter = ArrowPartitionWriter;

    #[throws(ConnectorAgentError)]
    fn allocate(nrows: usize, schema: Vec<DataType>, _data_order: DataOrder) -> Self {
        // cannot really allocate memory since do not know each partition size here
        ArrowWriter {
            nrows,
            schema,
            pbuilders: vec![],
        }
    }

    fn partition_writers(&'a mut self, counts: &[usize]) -> Vec<Self::PartitionWriter> {
        assert_eq!(counts.iter().sum::<usize>(), self.nrows);
        let mut ret = vec![];
        for &c in counts {
            let pwriter = ArrowPartitionWriter::new(self.schema.clone(), c);
            self.pbuilders.push(pwriter.builders());
            ret.push(pwriter);
        }
        ret
    }

    fn schema(&self) -> &[DataType] {
        self.schema.as_slice()
    }
}
