use crate::DataType;
use postgres::types::Type;

impl<'a> From<&'a Type> for DataType {
    fn from(ty: &'a Type) -> DataType {
        match ty.name() {
            "int8" | "int4" => DataType::I64(true),
            "float4" | "float8" => DataType::F64(true),
            "text" | "bpchar" | "varchar" => DataType::String(true),
            "numeric" => DataType::F64(true),
            "bool" => DataType::Bool(true),
            "timestamp" | "timestamptz" | "date" => DataType::DateTime(true),
            ty => unimplemented!("{}", ty),
        }
    }
}
