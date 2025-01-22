use connectorx::{
    partition::{partition, PartitionQuery},
    source_router::parse_source,
    sql::CXQuery,
};
use fehler::throw;
use pyo3::prelude::*;
use pyo3::{exceptions::PyValueError, PyResult};

use crate::errors::ConnectorXPythonError;

#[derive(FromPyObject)]
#[pyo3(from_item_all)]
pub struct PyPartitionQuery {
    pub query: String,
    pub column: String,
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub num: usize,
}

impl Into<PartitionQuery> for PyPartitionQuery {
    fn into(self) -> PartitionQuery {
        PartitionQuery::new(
            self.query.as_str(),
            self.column.as_str(),
            self.min,
            self.max,
            self.num,
        )
    }
}

pub fn read_sql<'py>(
    py: Python<'py>,
    conn: &str,
    return_type: &str,
    protocol: Option<&str>,
    queries: Option<Vec<String>>,
    partition_query: Option<PyPartitionQuery>,
) -> PyResult<Bound<'py, PyAny>> {
    let source_conn = parse_source(conn, protocol).map_err(|e| ConnectorXPythonError::from(e))?;
    let (queries, origin_query) = match (queries, partition_query) {
        (Some(queries), None) => (queries.into_iter().map(CXQuery::Naked).collect(), None),
        (None, Some(part)) => {
            let origin_query = Some(part.query.clone());
            let queries = partition(&part.into(), &source_conn)
                .map_err(|e| ConnectorXPythonError::from(e))?;
            (queries, origin_query)
        }
        (Some(_), Some(_)) => throw!(PyValueError::new_err(
            "partition_query and queries cannot be both specified",
        )),
        (None, None) => throw!(PyValueError::new_err(
            "partition_query and queries cannot be both None",
        )),
    };

    match return_type {
        "pandas" => Ok(crate::pandas::write_pandas(
            py,
            &source_conn,
            origin_query,
            &queries,
        )?),
        "arrow" => Ok(crate::arrow::write_arrow(
            py,
            &source_conn,
            origin_query,
            &queries,
        )?),
        _ => Err(PyValueError::new_err(format!(
            "return type should be 'pandas' or 'arrow', got '{}'",
            return_type
        ))),
    }
}
