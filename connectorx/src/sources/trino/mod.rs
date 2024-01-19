use crate::prelude::CXQuery;

use self::typesystem::TrinoTypeSystem;

mod errors;
mod typesystem;

pub struct TrinoSource {
    origin_query: Option<String>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<TrinoTypeSystem>,
}
