use super::arrow_assoc::ArrowAssoc;
use super::Builder;
use crate::typesystem::{ParameterizedFunc, ParameterizedOn};
use arrow::array::{ArrayBuilder, ArrayRef};
use arrow::datatypes::Field;

pub struct FNewBuilder;

impl ParameterizedFunc for FNewBuilder {
    type Function = fn(nrows: usize) -> Builder;
}

impl<T> ParameterizedOn<T> for FNewBuilder
where
    T: ArrowAssoc,
{
    fn parameterize() -> Self::Function {
        fn imp<T>(nrows: usize) -> Builder
        where
            T: ArrowAssoc,
        {
            Box::new(T::builder(nrows)) as Builder
        }
        imp::<T>
    }
}

pub struct FFinishBuilder;

impl ParameterizedFunc for FFinishBuilder {
    type Function = fn(Builder) -> ArrayRef;
}

impl<T> ParameterizedOn<T> for FFinishBuilder
where
    T: ArrowAssoc,
{
    fn parameterize() -> Self::Function {
        fn imp<T>(mut builder: Builder) -> ArrayRef
        where
            T: ArrowAssoc,
        {
            ArrayBuilder::finish(builder.downcast_mut::<T::Builder>().unwrap())
        }
        imp::<T>
    }
}

pub struct FNewField;

impl ParameterizedFunc for FNewField {
    type Function = fn(header: &str) -> Field;
}

impl<T> ParameterizedOn<T> for FNewField
where
    T: ArrowAssoc,
{
    fn parameterize() -> Self::Function {
        fn imp<T>(header: &str) -> Field
        where
            T: ArrowAssoc,
        {
            T::field(header)
        }
        imp::<T>
    }
}
