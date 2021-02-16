pub trait PandasAssoc {
    fn new_series_str(cid: usize, nrows: usize) -> String;
}

impl PandasAssoc for u64 {
    fn new_series_str(cid: usize, nrows: usize) -> String {
        format!(
            "'{}': pd.Series(index=range({}), dtype='uint64')",
            cid, nrows
        )
    }
}

impl PandasAssoc for Option<u64> {
    fn new_series_str(cid: usize, nrows: usize) -> String {
        format!(
            "'{}': pd.Series(index=range({}), dtype='UInt64')",
            cid, nrows
        )
    }
}

impl PandasAssoc for f64 {
    fn new_series_str(cid: usize, nrows: usize) -> String {
        format!(
            "'{}': pd.Series(index=range({}), dtype='float64')",
            cid, nrows
        )
    }
}

impl PandasAssoc for bool {
    fn new_series_str(cid: usize, nrows: usize) -> String {
        format!("'{}': pd.Series(index=range({}), dtype='bool')", cid, nrows)
    }
}

impl PandasAssoc for String {
    fn new_series_str(cid: usize, nrows: usize) -> String {
        format!(
            "'{}': pd.Series(index=range({}), dtype='object')",
            cid, nrows
        )
    }
}
