use connectorx::prelude::*;
use connectorx::sources::dummy::{DummySource, DummyTypeSystem};

#[test]
#[should_panic]
fn dummy_source_col_major() {
    let mut source = DummySource::new(&["a"], &[DummyTypeSystem::F64(false)]);
    source.set_data_order(DataOrder::ColumnMajor).unwrap();
}
