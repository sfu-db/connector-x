use super::pandas_assoc::PandasAssoc;
use connector_agent::{ParameterizedFunc, ParameterizedOn};

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
