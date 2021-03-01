mod sql;
mod types;

use crate::data_order::DataOrder;
use crate::data_sources::{Parser, PartitionedSource, Produce, Source};
use crate::errors::{ConnectorAgentError, Result};
use anyhow::anyhow;
use fehler::throw;
use log::debug;
use postgres::{
    binary_copy::{BinaryCopyOutIter, BinaryCopyOutRow},
    fallible_iterator::FallibleIterator,
    types::FromSql,
};
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use sql::{count_query, limit1_query};
pub use types::PostgresDTypes;

type PgManager = PostgresConnectionManager<NoTls>;
type PgConn = PooledConnection<PgManager>;

pub struct PostgresSource {
    pool: Pool<PgManager>,
    queries: Vec<String>,
    names: Vec<String>,
    schema: Vec<PostgresDTypes>,
}

impl PostgresSource {
    pub fn new(conn: &str, nconn: usize) -> Self {
        let manager = PostgresConnectionManager::new(conn.parse().unwrap(), NoTls);
        let pool = Pool::builder().max_size(nconn as u32).build(manager).unwrap();

        Self {
            pool,
            queries: vec![],
            names: vec![],
            schema: vec![],
        }
    }
}

impl Source for PostgresSource {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = PostgresSourcePartition;
    type TypeSystem = PostgresDTypes;

    fn set_data_order(&mut self, data_order: DataOrder) -> Result<()> {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order));
        }
        Ok(())
    }

    fn set_queries<Q: AsRef<str>>(&mut self, queries: &[Q]) {
        self.queries = queries.iter().map(|q| q.as_ref().to_string()).collect();
    }

    fn fetch_metadata(&mut self) -> Result<()> {
        assert!(self.queries.len() != 0);

        let mut conn = self.pool.get()?;
        let mut success = false;
        let mut error = None;
        for query in &self.queries {
            // assuming all the partition queries yield same schema
            match conn.query_one(&limit1_query(query)[..], &[]) {
                Ok(row) => {
                    let (names, types) = row
                        .columns()
                        .into_iter()
                        .map(|col| (col.name().to_string(), PostgresDTypes::from(col.type_())))
                        .unzip();

                    self.names = names;
                    self.schema = types;

                    success = true;
                    break;
                }
                Err(e) => {
                    debug!("cannot get metadata for '{}', try next query: {}", query, e);
                    error = Some(e);
                }
            }
        }

        if !success {
            throw!(anyhow!(
                "Cannot get metadata for the queries, last error: {:?}",
                error
            ))
        }

        Ok(())
    }

    fn names(&self) -> Vec<String> {
        self.names.clone()
    }

    fn schema(&self) -> Vec<Self::TypeSystem> {
        self.schema.clone()
    }

    fn partition(self) -> Result<Vec<Self::Partition>> {
        let mut ret = vec![];
        for query in self.queries {
            let conn = self.pool.get()?;

            ret.push(PostgresSourcePartition::new(conn, &query, &self.schema));
        }
        Ok(ret)
    }
}

pub struct PostgresSourcePartition {
    conn: PgConn,
    query: String,
    schema: Vec<PostgresDTypes>,
    nrows: usize,
    ncols: usize,
}

impl PostgresSourcePartition {
    pub fn new(conn: PgConn, query: &str, schema: &[PostgresDTypes]) -> Self {
        Self {
            conn,
            query: query.to_string(),
            schema: schema.to_vec(),
            nrows: 0,
            ncols: schema.len(),
        }
    }
}

impl PartitionedSource for PostgresSourcePartition {
    type TypeSystem = PostgresDTypes;
    type Parser<'a> = PostgresSourceParser<'a>;

    fn prepare(&mut self) -> Result<()> {
        let row = self.conn.query_one(&count_query(&self.query)[..], &[])?;
        self.nrows = row.get::<_, i64>(0) as usize;
        Ok(())
    }

    fn parser(&mut self) -> Result<Self::Parser<'_>> {
        let query = format!("COPY ({}) TO STDOUT WITH BINARY", self.query);
        let reader = self.conn.copy_out(&*query)?; // unless reading the data, it seems like issue the query is fast
        let pg_schema: Vec<_> = self.schema.iter().map(|&dt| dt.into()).collect();
        let iter = BinaryCopyOutIter::new(reader, &pg_schema);

        Ok(PostgresSourceParser::new(iter, &self.schema))
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

pub struct PostgresSourceParser<'a> {
    iter: BinaryCopyOutIter<'a>,
    current_row: Option<BinaryCopyOutRow>,
    ncols: usize,
    current_col: usize,
}

impl<'a> PostgresSourceParser<'a> {
    pub fn new(iter: BinaryCopyOutIter<'a>, schema: &[PostgresDTypes]) -> Self {
        Self {
            iter,
            current_row: None,
            current_col: 0,
            ncols: schema.len(),
        }
    }

    fn next_col_idx(&mut self) -> Result<usize> {
        if self.current_row.is_none() || self.current_col >= self.ncols {
            // first time or new row
            match self.iter.next()? {
                Some(row) => {
                    self.current_row = Some(row);
                    self.current_col = 1;
                    Ok(0)
                }
                None => throw!(anyhow!("Postgres EOF")),
            }
        } else {
            let ret = Ok(self.current_col);
            self.current_col += 1;
            ret
        }
    }
}

impl<'a> Parser<'a> for PostgresSourceParser<'a> {
    type TypeSystem = PostgresDTypes;
}

impl<'a, T> Produce<T> for PostgresSourceParser<'a>
where
    T: for<'r> FromSql<'r>,
{
    fn produce(&mut self) -> Result<T> {
        let cidx = self.next_col_idx()?;
        let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
        Ok(val)
    }
}

// impl<'a> Produce<Option<f64>> for PostgresSourceParser<'a> {
//     fn produce(&mut self) -> Result<Option<f64>> {
//         let cidx = self.next_col_idx()?;
//         let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
//         Ok(val)
//     }
// }

// impl<'a> Produce<i64> for PostgresSourceParser<'a> {
//     fn produce(&mut self) -> Result<i64> {
//         let cidx = self.next_col_idx()?;
//         let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
//         Ok(val)
//     }
// }

// impl<'a> Produce<Option<i64>> for PostgresSourceParser<'a> {
//     fn produce(&mut self) -> Result<Option<i64>> {
//         let cidx = self.next_col_idx()?;
//         let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
//         Ok(val)
//     }
// }

// impl<'a> Produce<bool> for PostgresSourceParser<'a> {
//     fn produce(&mut self) -> Result<bool> {
//         let cidx = self.next_col_idx()?;
//         let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
//         Ok(val)
//     }
// }

// impl<'a> Produce<Option<bool>> for PostgresSourceParser<'a> {
//     fn produce(&mut self) -> Result<Option<bool>> {
//         let cidx = self.next_col_idx()?;
//         match &self.pgschema[cidx] {
//             &Type::BOOL => {
//                 let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
//                 Ok(val)
//             }
//             t => {
//                 throw!(ConnectorAgentError::CannotParse(
//                     type_name::<Option<bool>>(),
//                     t.name().into()
//                 ))
//             }
//         }
//     }
// }

// impl<'a> Produce<String> for PostgresSourceParser<'a> {
//     fn produce(&mut self) -> Result<String> {
//         let cidx = self.next_col_idx()?;
//         match &self.pgschema[cidx] {
//             &Type::TEXT | &Type::VARCHAR | &Type::BPCHAR => {
//                 let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
//                 Ok(val)
//             }
//             t => {
//                 throw!(ConnectorAgentError::CannotParse(
//                     type_name::<String>(),
//                     t.name().into()
//                 ))
//             }
//         }
//     }
// }

// impl<'a> Produce<Option<String>> for PostgresSourceParser<'a> {
//     fn produce(&mut self) -> Result<Option<String>> {
//         let cidx = self.next_col_idx()?;
//         match &self.pgschema[cidx] {
//             &Type::TEXT | &Type::VARCHAR | &Type::BPCHAR => {
//                 let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
//                 Ok(val)
//             }
//             t => {
//                 throw!(ConnectorAgentError::CannotParse(
//                     type_name::<String>(),
//                     t.name().into()
//                 ))
//             }
//         }
//     }
// }

// impl<'a> Produce<DateTime<Utc>> for PostgresSourceParser<'a> {
//     fn produce(&mut self) -> Result<DateTime<Utc>> {
//         let cidx = self.next_col_idx()?;
//         match &self.pgschema[cidx] {
//             &Type::TIMESTAMPTZ => {
//                 let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
//                 Ok(val)
//             }
//             &Type::TIMESTAMP => {
//                 let val: NaiveDateTime = self.current_row.as_ref().unwrap().try_get(cidx)?;
//                 let val = DateTime::from_utc(val, Utc);
//                 Ok(val)
//             }
//             &Type::DATE => {
//                 let val: NaiveDate = self.current_row.as_ref().unwrap().try_get(cidx)?;
//                 let val = DateTime::from_utc(val.and_hms(0, 0, 0), Utc);
//                 Ok(val)
//             }
//             t => {
//                 throw!(ConnectorAgentError::CannotParse(
//                     type_name::<DateTime<Utc>>(),
//                     t.name().into()
//                 ))
//             }
//         }
//     }
// }

// impl<'a> Produce<Option<DateTime<Utc>>> for PostgresSourceParser<'a> {
//     fn produce(&mut self) -> Result<Option<DateTime<Utc>>> {
//         let cidx = self.next_col_idx()?;
//         match &self.pgschema[cidx] {
//             &Type::TIMESTAMPTZ => {
//                 let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
//                 Ok(val)
//             }
//             &Type::TIMESTAMP => {
//                 let val: Option<NaiveDateTime> =
//                     self.current_row.as_ref().unwrap().try_get(cidx)?;
//                 let val = val.map(|d| DateTime::from_utc(d, Utc));
//                 Ok(val)
//             }
//             &Type::DATE => {
//                 let val: Option<NaiveDate> = self.current_row.as_ref().unwrap().try_get(cidx)?;
//                 let val = val.map(|d| DateTime::from_utc(d.and_hms(0, 0, 0), Utc));
//                 Ok(val)
//             }
//             t => {
//                 throw!(ConnectorAgentError::CannotParse(
//                     type_name::<Option<DateTime<Utc>>>(),
//                     t.name().into()
//                 ))
//             }
//         }
//     }
// }
