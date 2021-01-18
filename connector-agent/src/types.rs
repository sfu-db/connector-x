#[derive(Debug, Clone)]
pub enum DataType {
    F64,
    U64,
}

pub trait TypeInfo {}

impl TypeInfo for f64 {}
impl TypeInfo for u64 {}
