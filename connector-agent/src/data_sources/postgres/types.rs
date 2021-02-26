use crate::types::DataType;
use postgres::types::Type;

trait PostgresDType {
    fn dtype(self) -> Type;
}

// impl PostgresDType for  DataType {
//     fn dtype(self) -> Type {
//         match
//     }
// }
