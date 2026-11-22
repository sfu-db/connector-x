use connectorx::{
    partition::{partition, PartitionQuery},
    source_router::parse_source,
    sql::CXQuery,
};
use fehler::throw;
use pyo3::prelude::*;
use pyo3::{exceptions::PyValueError, PyResult};

use crate::errors::ConnectorXPythonError;
use pyo3::types::PyDict;

#[derive(FromPyObject)]
#[pyo3(from_item_all)]
pub struct PyPartitionQuery {
    pub query: String,
    pub column: String,
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub num: usize,
}

impl From<PyPartitionQuery> for PartitionQuery {
    fn from(val: PyPartitionQuery) -> Self {
        PartitionQuery::new(
            val.query.as_str(),
            val.column.as_str(),
            val.min,
            val.max,
            val.num,
        )
    }
}

#[allow(clippy::too_many_arguments)]
pub fn read_sql<'py>(
    py: Python<'py>,
    conn: &str,
    return_type: &str,
    protocol: Option<&str>,
    queries: Option<Vec<String>>,
    partition_query: Option<PyPartitionQuery>,
    pre_execution_queries: Option<Vec<String>>,
    kwargs: Option<&Bound<PyDict>>,
) -> PyResult<Bound<'py, PyAny>> {
    let source_conn = parse_source(conn, protocol).map_err(ConnectorXPythonError::from)?;
    let (queries, origin_query) = match (queries, partition_query) {
        (Some(queries), None) => (queries.into_iter().map(CXQuery::Naked).collect(), None),
        (None, Some(part)) => {
            let origin_query = Some(part.query.clone());
            let queries =
                partition(&part.into(), &source_conn).map_err(ConnectorXPythonError::from)?;
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
            pre_execution_queries.as_deref(),
        )?),
        "arrow" => Ok(crate::arrow::write_arrow(
            py,
            &source_conn,
            origin_query,
            &queries,
            pre_execution_queries.as_deref(),
        )?),
        "arrow_stream" => {
            let batch_size = kwargs
                .and_then(|dict| dict.get_item("batch_size").ok().flatten())
                .and_then(|obj| obj.extract::<usize>().ok())
                .unwrap_or(10000);

            Ok(crate::arrow::get_arrow_rb_iter(
                py,
                &source_conn,
                origin_query,
                &queries,
                pre_execution_queries.as_deref(),
                batch_size,
            )?)
        }

        _ => Err(PyValueError::new_err(format!(
            "return type should be 'pandas' or 'arrow', got '{}'",
            return_type
        ))),
    }
}
