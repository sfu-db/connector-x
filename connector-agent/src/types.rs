#[derive(Debug, Clone, Copy)]
pub enum DataType {
    F64,
    U64,
}

pub trait TypeInfo {}

impl TypeInfo for f64 {}
impl TypeInfo for u64 {}

pub trait DataTypeCheck<T> {
    fn verify(self) -> bool;
    fn found() -> String;
}

impl DataTypeCheck<f64> for DataType {
    fn verify(self) -> bool {
        match self {
            DataType::F64 => true,
            _ => false,
        }
    }

    fn found() -> String {
        "f64".to_string()
    }
}

impl DataTypeCheck<u64> for DataType {
    fn verify(self) -> bool {
        match self {
            DataType::U64 => true,
            _ => false,
        }
    }

    fn found() -> String {
        "u64".to_string()
    }
}

impl<T> DataTypeCheck<T> for DataType {
    default fn verify(self) -> bool {
        match self {
            _ => false,
        }
    }

    default fn found() -> String {
        "T".to_string()
    }
}
