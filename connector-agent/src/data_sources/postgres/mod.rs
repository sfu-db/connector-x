mod types;

use crate::data_order::DataOrder;
use crate::data_sources::{Parser, PartitionedSource, Produce, Source};
use crate::errors::{ConnectorAgentError, Result};
use crate::types::DataType;
use anyhow::anyhow;
use fehler::throw;
use postgres::{
    binary_copy::{BinaryCopyOutIter, BinaryCopyOutRow},
    fallible_iterator::FallibleIterator,
    types::FromSql,
};
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use types::PostgresDType;

type PgManager = PostgresConnectionManager<NoTls>;
type PgConn = PooledConnection<PgManager>;

pub struct PostgresSource {
    pool: Pool<PgManager>,
    queries: Vec<String>,
    names: Vec<String>,
    schema: Vec<DataType>,
}

impl PostgresSource {
    pub fn new(conn: &str) -> Self {
        let manager = PostgresConnectionManager::new(conn.parse().unwrap(), NoTls);
        let pool = Pool::new(manager).unwrap();

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
    type TypeSystem = DataType;

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
        for query in &self.queries {
            if let Ok(row) = conn.query_one(&format!("{} LIMIT 1", query)[..], &[]) {
                let (names, types) = row
                    .columns()
                    .into_iter()
                    .map(|col| (col.name().to_string(), DataType::from(col.type_())))
                    .unzip();
                self.names = names;
                self.schema = types;
                break;
            }
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
    schema: Vec<DataType>,
    nrows: usize,
    ncols: usize,
}

impl PostgresSourcePartition {
    pub fn new(conn: PgConn, query: &str, schema: &[DataType]) -> Self {
        Self {
            conn,
            query: query.to_string(),
            schema: schema.to_vec(),
            nrows: 0,
            ncols: 0,
        }
    }
}

impl PartitionedSource for PostgresSourcePartition {
    type TypeSystem = DataType;
    type Parser<'a> = PostgresSourceParser<'a>;

    fn prepare(&mut self) -> Result<()> {
        Ok(())
    }

    fn parser(&mut self) -> Result<Self::Parser<'_>> {
        let query = format!("COPY ({}) TO STDOUT WITH CSV", self.query);
        let reader = self.conn.copy_out(&*query)?; // unless reading the data, it seems like issue the query is fast
        let pgschema: Vec<_> = self.schema.iter().map(|dt| dt.dtype()).collect();
        let iter = BinaryCopyOutIter::new(reader, &pgschema);

        Ok(PostgresSourceParser::new(iter, self.ncols))
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
    pub fn new(iter: BinaryCopyOutIter<'a>, ncols: usize) -> Self {
        Self {
            iter,
            current_row: None,
            current_col: 0,
            ncols,
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
    type TypeSystem = DataType;
}

impl<'a, T> Produce<T> for PostgresSourceParser<'a>
where
    T: for<'r> FromSql<'r>,
{
    fn produce(&mut self) -> Result<T> {
        let cidx = self.next_col_idx()?;
        Ok(self.current_row.as_ref().unwrap().try_get(cidx)?)
    }
}

// impl<'a> Produce<Option<u64>> for PostgresParser<'a> {
//     fn produce(&mut self) -> Result<Option<u64>> {
//         match self.next_value() {
//             "" => Ok(None),
//             v => Ok(Some(v.parse().map_err(|_| {
//                 ConnectorAgentError::CannotParse(type_name::<u64>(), v.into())
//             })?)),
//         }
//     }
// }

// impl<'a> Produce<i64> for PostgresParser<'a> {
//     fn produce(&mut self) -> Result<i64> {
//         let v = self.next_value();
//         v.parse()
//             .map_err(|_| ConnectorAgentError::CannotParse(type_name::<i64>(), v.into()))
//     }
// }

// impl<'a> Produce<Option<i64>> for PostgresParser<'a> {
//     fn produce(&mut self) -> Result<Option<i64>> {
//         match self.next_value() {
//             "" => Ok(None),
//             v => Ok(Some(v.parse().map_err(|_| {
//                 ConnectorAgentError::CannotParse(type_name::<i64>(), v.into())
//             })?)),
//         }
//     }
// }

// impl<'a> Produce<f64> for PostgresParser<'a> {
//     fn produce(&mut self) -> Result<f64> {
//         let v = self.next_value();
//         v.parse()
//             .map_err(|_| ConnectorAgentError::CannotParse(type_name::<f64>(), v.into()))
//     }
// }

// impl<'a> Produce<Option<f64>> for PostgresParser<'a> {
//     fn produce(&mut self) -> Result<Option<f64>> {
//         match self.next_value() {
//             "" => Ok(None),
//             v => Ok(Some(v.parse().map_err(|_| {
//                 ConnectorAgentError::CannotParse(type_name::<f64>(), v.into())
//             })?)),
//         }
//     }
// }

// impl<'a> Produce<bool> for PostgresParser<'a> {
//     fn produce(&mut self) -> Result<bool> {
//         let v = self.next_value();
//         let v = match v {
//             "t" => true,
//             "f" => false,
//             _ => throw!(ConnectorAgentError::CannotParse(
//                 type_name::<bool>(),
//                 v.into()
//             )),
//         };
//         Ok(v)
//     }
// }

// impl<'a> Produce<Option<bool>> for PostgresParser<'a> {
//     fn produce(&mut self) -> Result<Option<bool>> {
//         let v = self.next_value();
//         let v = match v {
//             "t" => Some(true),
//             "f" => Some(false),
//             "" => None,
//             _ => throw!(ConnectorAgentError::CannotParse(
//                 type_name::<bool>(),
//                 v.into()
//             )),
//         };
//         Ok(v)
//     }
// }

// impl<'a> Produce<String> for PostgresParser<'a> {
//     fn produce(&mut self) -> Result<String> {
//         Ok(String::from(self.next_value()))
//     }
// }

// impl<'a> Produce<Option<String>> for PostgresParser<'a> {
//     fn produce(&mut self) -> Result<Option<String>> {
//         Ok(Some(String::from(self.next_value())))
//     }
// }

// impl<'a> Produce<DateTime<Utc>> for PostgresParser<'a> {
//     fn produce(&mut self) -> Result<DateTime<Utc>> {
//         let v = self.next_value();
//         v.parse()
//             .map_err(|_| ConnectorAgentError::CannotParse(type_name::<DateTime<Utc>>(), v.into()))
//     }
// }

// impl<'a> Produce<Option<DateTime<Utc>>> for PostgresParser<'a> {
//     fn produce(&mut self) -> Result<Option<DateTime<Utc>>> {
//         match self.next_value() {
//             "" => Ok(None),
//             v => Ok(Some(v.parse().map_err(|_| {
//                 ConnectorAgentError::CannotParse(type_name::<DateTime<Utc>>(), v.into())
//             })?)),
//         }
//     }
// }

// impl<'a> Produce<Date<Utc>> for PostgresParser<'a> {
//     fn produce(&mut self) -> Result<Date<Utc>> {
//         let v = self.next_value();
//         NaiveDate::parse_from_str(v, "%Y-%m-%d")
//             .map(|nd| Date::<Utc>::from_utc(nd, Utc))
//             .map_err(|_| ConnectorAgentError::CannotParse(type_name::<DateTime<Utc>>(), v.into()))
//     }
// }

// impl<'a> Produce<Option<Date<Utc>>> for PostgresParser<'a> {
//     fn produce(&mut self) -> Result<Option<Date<Utc>>> {
//         match self.next_value() {
//             "" => Ok(None),
//             v => Ok(Some(
//                 NaiveDate::parse_from_str(v, "%Y-%m-%d")
//                     .map(|nd| Date::<Utc>::from_utc(nd, Utc))
//                     .map_err(|_| {
//                         ConnectorAgentError::CannotParse(type_name::<DateTime<Utc>>(), v.into())
//                     })?,
//             )),
//         }
//     }
// }
