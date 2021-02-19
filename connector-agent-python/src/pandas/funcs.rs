use super::pandas_assoc::PandasAssoc;
use connector_agent::{AnyArrayViewMut, ParameterizedFunc, ParameterizedOn};
use ndarray::Ix2;
use numpy::{Element, PyArray};
use pyo3::types::PyAny;

pub struct FSeriesStr;

impl ParameterizedFunc for FSeriesStr {
    type Function = fn(cid: usize, nrows: usize) -> String;
}

impl<T> ParameterizedOn<T> for FSeriesStr
where
    T: PandasAssoc,
{
    fn parameterize() -> Self::Function {
        fn imp<T>(cid: usize, nrows: usize) -> String
        where
            T: PandasAssoc,
        {
            T::new_series_str(cid, nrows)
        }
        imp::<T>
    }
}

pub struct FArrayViewMut2;

impl ParameterizedFunc for FArrayViewMut2 {
    type Function = fn(&PyAny) -> AnyArrayViewMut<Ix2>;
}

// PyArray cannot support String and Option type here
impl<T> ParameterizedOn<T> for FArrayViewMut2
where
    T: Element + Send + 'static,
{
    fn parameterize() -> Self::Function {
        fn imp<T>(array: &PyAny) -> AnyArrayViewMut<Ix2>
        where
            T: Element + Send + 'static,
        {
            let pyarray = array.downcast::<PyArray<T, Ix2>>().unwrap();
            let mut_view = unsafe { pyarray.as_array_mut() };
            AnyArrayViewMut::<Ix2>::new(mut_view)
        }
        imp::<T>
    }
}
