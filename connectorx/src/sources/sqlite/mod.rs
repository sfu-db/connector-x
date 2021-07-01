mod typesystem;

use fallible_streaming_iterator::FallibleStreamingIterator;

use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::sources::{PartitionParser, Produce, Source, SourcePartition};
use crate::sql::{count_query, get_limit, limit1_query};
use anyhow::anyhow;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use derive_more::{Deref, DerefMut};
use fehler::throw;
use log::debug;
use owning_ref::OwningHandle;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Row, Rows, Statement};
use sqlparser::dialect::SQLiteDialect;
pub use typesystem::SqliteTypeSystem;

#[derive(Deref, DerefMut)]
struct DummyBox<T>(T);

pub struct SqliteSource {
    pool: Pool<SqliteConnectionManager>,
    queries: Vec<String>,
    names: Vec<String>,
    schema: Vec<SqliteTypeSystem>,
}

impl SqliteSource {
    pub fn new(conn: &str, nconn: usize) -> Result<Self> {
        let manager = SqliteConnectionManager::file(conn);
        let pool = r2d2::Pool::builder()
            .max_size(nconn as u32)
            .build(manager)?;

        Ok(Self {
            pool,
            queries: vec![],
            names: vec![],
            schema: vec![],
        })
    }
}

impl Source for SqliteSource
where
    SqliteSourcePartition: SourcePartition<TypeSystem = SqliteTypeSystem>,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = SqliteSourcePartition;
    type TypeSystem = SqliteTypeSystem;

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
        let conn = self.pool.get()?;
        let mut success = false;
        let mut zero_tuple = true;
        let mut error = None;
        for query in &self.queries {
            // assuming all the partition queries yield same schema
            let mut names = vec![];
            let mut types = vec![];

            match conn.query_row(&limit1_query(query, &SQLiteDialect {})?[..], [], |row| {
                row.columns().iter().enumerate().for_each(|(i, col)| {
                    zero_tuple = false;
                    names.push(col.name().to_string());
                    match row.get_ref(i) {
                        Ok(vr) => {
                            types.push(SqliteTypeSystem::from((col.decl_type(), vr.data_type())))
                        }
                        Err(e) => {
                            debug!("cannot get ref at {} on query: {}", i, query);
                            error = Some(e);
                            types.clear(); // clear types and return directly when error occurs
                            return;
                        }
                    }
                });
                Ok(())
            }) {
                Ok(_) => {}
                Err(e) => {
                    match e {
                        rusqlite::Error::QueryReturnedNoRows => {}
                        _ => zero_tuple = false,
                    }
                    debug!("cannot get metadata for '{}', try next query: {}", query, e);
                    error = Some(e);
                }
            }

            if !names.is_empty() && !types.is_empty() {
                success = true;
                self.names = names;
                self.schema = types;
                break;
            }
        }

        if !success {
            if zero_tuple {
                let mut stmt = conn.prepare(self.queries[0].as_str())?;
                let rows = stmt.query([])?;
                // if query contains limit, column_names will be None
                match rows.column_names() {
                    Some(cnames) => {
                        self.names = cnames.into_iter().map(|s| s.to_string()).collect();
                        // set all columns as string (align with pandas)
                        self.schema = vec![SqliteTypeSystem::Text(false); self.names.len()];
                        return Ok(());
                    }
                    None => {}
                }
            }
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

            ret.push(SqliteSourcePartition::new(conn, &query, &self.schema));
        }
        Ok(ret)
    }
}

pub struct SqliteSourcePartition {
    conn: PooledConnection<SqliteConnectionManager>,
    query: String,
    schema: Vec<SqliteTypeSystem>,
    nrows: usize,
    ncols: usize,
}

impl SqliteSourcePartition {
    pub fn new(
        conn: PooledConnection<SqliteConnectionManager>,
        query: &str,
        schema: &[SqliteTypeSystem],
    ) -> Self {
        Self {
            conn,
            query: query.to_string(),
            schema: schema.to_vec(),
            nrows: 0,
            ncols: schema.len(),
        }
    }
}

impl SourcePartition for SqliteSourcePartition {
    type TypeSystem = SqliteTypeSystem;
    type Parser<'a> = SqliteSourcePartitionParser<'a>;

    fn prepare(&mut self) -> Result<()> {
        let dialect = SQLiteDialect {};
        self.nrows = match get_limit(&self.query, &dialect)? {
            None => self
                .conn
                .query_row(&count_query(&self.query, &dialect)?[..], [], |row| {
                    Ok(row.get::<_, i64>(0)? as usize)
                })?,
            Some(n) => n,
        };
        Ok(())
    }

    fn parser(&mut self) -> Result<Self::Parser<'_>> {
        Ok(SqliteSourcePartitionParser::new(
            &self.conn,
            self.query.as_str(),
            &self.schema,
        )?)
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

pub struct SqliteSourcePartitionParser<'a> {
    rows: OwningHandle<Box<Statement<'a>>, DummyBox<Rows<'a>>>,
    ncols: usize,
    current_col: usize,
}

impl<'a> SqliteSourcePartitionParser<'a> {
    pub fn new(
        conn: &'a PooledConnection<SqliteConnectionManager>,
        query: &str,
        schema: &[SqliteTypeSystem],
    ) -> Result<Self> {
        let stmt: Statement<'a> = conn.prepare(query)?;

        // Safety: DummyBox borrows the on-heap stmt, which is owned by the OwningHandle.
        // No matter how we move the owning handle (thus the Box<Statment>), the Statement
        // keeps its address static on the heap, thus the borrow of MyRows keeps valid.
        let rows: OwningHandle<Box<Statement<'a>>, DummyBox<Rows<'a>>> =
            OwningHandle::new_with_fn(Box::new(stmt), |stmt: *const Statement<'a>| unsafe {
                DummyBox((&mut *(stmt as *mut Statement<'_>)).query([]).unwrap())
            });
        Ok(Self {
            rows,
            ncols: schema.len(),
            current_col: 0,
        })
    }

    fn next_loc(&mut self) -> Result<(&Row, usize)> {
        let row: &Row = match self.current_col {
            0 => (*self.rows).next()?.ok_or_else(|| anyhow!("Sqlite EOF"))?,
            _ => (*self.rows)
                .get()
                .ok_or_else(|| anyhow!("Sqlite empty current row"))?,
        };
        let col = self.current_col;
        self.current_col = (self.current_col + 1) % self.ncols;
        Ok((row, col))
    }
}

impl<'a> PartitionParser<'a> for SqliteSourcePartitionParser<'a> {
    type TypeSystem = SqliteTypeSystem;
}

macro_rules! impl_produce {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for SqliteSourcePartitionParser<'a> {
                fn produce(&'r mut self) -> Result<$t> {
                    let (row, col) = self.next_loc()?;
                    let val = row.get(col)?;
                    Ok(val)
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for SqliteSourcePartitionParser<'a> {
                fn produce(&'r mut self) -> Result<Option<$t>> {
                    let (row, col) = self.next_loc()?;
                    let val = row.get(col)?;
                    Ok(val)
                }
            }
        )+
    };
}

impl_produce!(
    bool,
    i64,
    i32,
    i16,
    f64,
    Box<str>,
    NaiveDate,
    NaiveTime,
    NaiveDateTime,
    Vec<u8>,
);
