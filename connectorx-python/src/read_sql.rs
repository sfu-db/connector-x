use crate::source_router::SourceConn;
use connectorx::sql::CXQuery;
use dict_derive::FromPyObject;
use fehler::throw;
use log::debug;
use pyo3::prelude::*;
use pyo3::{exceptions::PyValueError, PyResult};
use std::convert::TryFrom;

#[derive(FromPyObject)]
pub struct PartitionQuery {
    query: String,
    num: usize,
}

impl PartitionQuery {
    pub fn new(
        query: &str,
        _column: &str,
        _min: Option<i64>,
        _max: Option<i64>,
        num: usize,
    ) -> Self {
        Self {
            query: query.into(),
            num,
        }
    }
}

pub fn read_sql<'a>(
    py: Python<'a>,
    conn: &str,
    return_type: &str,
    protocol: Option<&str>,
    queries: Option<Vec<String>>,
    partition_query: Option<PartitionQuery>,
    multi_access_plan: &str,
) -> PyResult<&'a PyAny> {
    debug!("conn: {}", conn);
    let source_conn = SourceConn::try_from(conn)?;
    let (queries, origin_query) = match (queries, partition_query) {
        (Some(queries), None) => (queries.into_iter().map(CXQuery::Naked).collect(), None),
        (None, Some(PartitionQuery { query, num })) => {
            let mut queries = vec![];
            let origin_query = Some(query.clone());
            let num = num as i64;

            for i in 0..num {
                queries.push(CXQuery::Naked(format!("FETCH ALL FROM multi_cursor{}", i)));
            }
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
            protocol.unwrap_or("binary"),
            multi_access_plan,
        )?),
        "arrow" => Ok(crate::arrow::write_arrow(
            py,
            &source_conn,
            origin_query,
            &queries,
            protocol.unwrap_or("binary"),
            multi_access_plan,
        )?),
        _ => Err(PyValueError::new_err(format!(
            "return type should be 'pandas' or 'arrow', got '{}'",
            return_type
        ))),
    }
}
