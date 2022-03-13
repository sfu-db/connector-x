use crate::source_router::SourceConn;
use connectorx::sql::CXQuery;
use dict_derive::FromPyObject;
use fehler::throw;
use pyo3::prelude::*;
use pyo3::{exceptions::PyValueError, PyResult};
use std::convert::TryFrom;

#[derive(FromPyObject)]
pub struct PartitionQuery {
    query: String,
    column: String,
    min: Option<i64>,
    max: Option<i64>,
    num: usize,
}

impl PartitionQuery {
    pub fn new(query: &str, column: &str, min: Option<i64>, max: Option<i64>, num: usize) -> Self {
        Self {
            query: query.into(),
            column: column.into(),
            min,
            max,
            num,
        }
    }
}

pub fn partition(part: &PartitionQuery, source_conn: &SourceConn) -> PyResult<Vec<CXQuery>> {
    let mut queries = vec![];
    let num = part.num as i64;
    let (min, max) = match (part.min, part.max) {
        (None, None) => source_conn.get_col_range(&part.query, &part.column)?,
        (Some(min), Some(max)) => (min, max),
        _ => throw!(PyValueError::new_err(
            "partition_query range can not be partially specified",
        )),
    };

    let partition_size = (max - min + 1) / num;

    for i in 0..num {
        let lower = min + i * partition_size;
        let upper = match i == num - 1 {
            true => max + 1,
            false => min + (i + 1) * partition_size,
        };
        let partition_query =
            source_conn.get_part_query(&part.query, &part.column, lower, upper)?;
        queries.push(partition_query);
    }
    Ok(queries)
}

pub fn read_sql<'a>(
    py: Python<'a>,
    conn: &str,
    return_type: &str,
    protocol: Option<&str>,
    queries: Option<Vec<String>>,
    partition_query: Option<PartitionQuery>,
) -> PyResult<&'a PyAny> {
    let source_conn = SourceConn::try_from(conn)?;
    let (queries, origin_query) = match (queries, partition_query) {
        (Some(queries), None) => (queries.into_iter().map(CXQuery::Naked).collect(), None),
        (None, Some(part)) => {
            let origin_query = Some(part.query.clone());
            let queries = partition(&part, &source_conn)?;
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
        )?),
        "arrow" => Ok(crate::arrow::write_arrow(
            py,
            &source_conn,
            origin_query,
            &queries,
            protocol.unwrap_or("binary"),
        )?),
        "arrow2" => Ok(crate::arrow2::write_arrow(
            py,
            &source_conn,
            origin_query,
            &queries,
            protocol.unwrap_or("binary"),
        )?),
        _ => Err(PyValueError::new_err(format!(
            "return type should be 'pandas' or 'arrow', got '{}'",
            return_type
        ))),
    }
}
