use crate::DataType;
use postgres::types::Type;

pub trait PostgresDType {
    fn dtype(&self) -> Type;
}

impl<'a> From<&'a Type> for DataType {
    fn from(ty: &'a Type) -> DataType {
        match ty.name() {
            "int8" => DataType::I64(false),
            "int4" => DataType::I64(false),
            ty => unimplemented!("{}", ty),
        }
    }
}

impl PostgresDType for DataType {
    fn dtype(&self) -> Type {
        match *self {
            DataType::I64(false) => Type::INT8,
            ty => unimplemented!("{:?}", ty),
        }
    }
}
