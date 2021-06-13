#[derive(Copy, Clone, Debug)]
pub enum SqliteTypeSystem {
    Integer(bool),
    Real(bool),
    Text(bool),
}

impl_typesystem! {
    system = SqliteTypeSystem,
    mappings = {
        { Integer => i64 }
        { Real => f64 }
        { Text => Box<str> }
    }
}
