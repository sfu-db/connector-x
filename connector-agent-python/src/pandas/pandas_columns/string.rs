use super::{HasPandasColumn, PandasColumn, PandasColumnObject};
use ndarray::{ArrayViewMut1, Axis};
use std::any::TypeId;

// Defer string writing to the end: We are not able to allocate string objects
// in this stage because python requires a GIL to be hold.
pub struct StringColumn<'a> {
    data: ArrayViewMut1<'a, Option<String>>,
}

impl<'a> StringColumn<'a> {
    pub fn new(buf: &'a mut [Option<String>]) -> Self {
        StringColumn {
            data: ArrayViewMut1::from(buf),
        }
    }
}

impl<'a> PandasColumnObject for StringColumn<'a> {
    fn typecheck(&self, id: TypeId) -> bool {
        id == TypeId::of::<String>() || id == TypeId::of::<Option<String>>()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn typename(&self) -> &'static str {
        std::any::type_name::<String>()
    }
}

impl<'a> PandasColumn<String> for StringColumn<'a> {
    fn write(&mut self, i: usize, val: String) {
        self.data[i] = Some(val);
    }
}

impl<'a> PandasColumn<Option<String>> for StringColumn<'a> {
    fn write(&mut self, i: usize, val: Option<String>) {
        self.data[i] = val;
    }
}

impl HasPandasColumn for String {
    type PandasColumn<'a> = StringColumn<'a>;
}

impl HasPandasColumn for Option<String> {
    type PandasColumn<'a> = StringColumn<'a>;
}

impl<'a> StringColumn<'a> {
    pub fn partition(self, counts: &[usize]) -> Vec<StringColumn<'a>> {
        let mut partitions = vec![];
        let mut data = self.data;

        for &c in counts {
            let (splitted_data, rest) = data.split_at(Axis(0), c);
            data = rest;

            partitions.push(StringColumn {
                data: splitted_data,
            });
        }

        partitions
    }
}
