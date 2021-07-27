mod errors;
mod typesystem;

pub use self::errors::SQLiteSourceError;
use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::{PartitionParser, Produce, Source, SourcePartition},
    sql::{count_query, get_limit, limit1_query, CXQuery},
    utils::DummyBox,
};
use anyhow::anyhow;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use fallible_streaming_iterator::FallibleStreamingIterator;
use fehler::{throw, throws};
use log::debug;
use owning_ref::OwningHandle;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Row, Rows, Statement};
use sqlparser::dialect::SQLiteDialect;
use std::convert::TryFrom;
pub use typesystem::SQLiteTypeSystem;

pub struct SQLiteSource {
    pool: Pool<SqliteConnectionManager>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<SQLiteTypeSystem>,
}

impl SQLiteSource {
    #[throws(SQLiteSourceError)]
    pub fn new(conn: &str, nconn: usize) -> Self {
        let manager = SqliteConnectionManager::file(conn);
        let pool = r2d2::Pool::builder()
            .max_size(nconn as u32)
            .build(manager)?;

        Self {
            pool,
            queries: vec![],
            names: vec![],
            schema: vec![],
        }
    }
}

impl Source for SQLiteSource
where
    SQLiteSourcePartition: SourcePartition<TypeSystem = SQLiteTypeSystem>,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = SQLiteSourcePartition;
    type TypeSystem = SQLiteTypeSystem;
    type Error = SQLiteSourceError;

    #[throws(SQLiteSourceError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorXError::UnsupportedDataOrder(data_order));
        }
    }

    fn set_queries<Q: ToString>(&mut self, queries: &[CXQuery<Q>]) {
        self.queries = queries.iter().map(|q| q.map(Q::to_string)).collect();
    }

    #[throws(SQLiteSourceError)]
    fn fetch_metadata(&mut self) {
        assert!(!self.queries.is_empty());
        let conn = self.pool.get()?;
        let mut names = vec![];
        let mut types = vec![];
        let mut num_empty = 0;

        // assuming all the partition queries yield same schema
        for (i, query) in self.queries.iter().enumerate() {
            match conn.query_row(
                &limit1_query(query, &SQLiteDialect {})?.as_str(),
                [],
                |row| {
                    for (j, col) in row.columns().iter().enumerate() {
                        if j >= names.len() {
                            names.push(col.name().to_string());
                        }
                        if j >= types.len() {
                            let vr = row.get_ref(j)?;
                            match SQLiteTypeSystem::try_from((col.decl_type(), vr.data_type())) {
                                Ok(t) => types.push(Some(t)),
                                Err(_) => {
                                    types.push(None);
                                }
                            }
                        } else if types[j] == None {
                            let vr = row.get_ref(j)?;
                            match SQLiteTypeSystem::try_from((col.decl_type(), vr.data_type())) {
                                Ok(t) => types[j] = Some(t),
                                Err(_) => {}
                            }
                        }
                    }
                    Ok(())
                },
            ) {
                Ok(()) => {
                    if !types.contains(&None) {
                        self.names = names;
                        self.schema = types.into_iter().map(|t| t.unwrap()).collect();
                        return;
                    }
                }
                Err(e) => {
                    match e {
                        rusqlite::Error::QueryReturnedNoRows => {
                            num_empty += 1; // make sure when all partition results are empty, do not throw error
                        }
                        _ => {}
                    }
                    if i == self.queries.len() - 1 && num_empty < self.queries.len() {
                        // tried the last query but still get an error
                        debug!("cannot get metadata for '{}', try next query: {}", query, e);
                        throw!(e)
                    }
                }
            }
        }

        // tried all queries but all get empty result set
        let mut stmt = conn.prepare(self.queries[0].as_str())?;
        let rows = stmt.query([])?;

        if let Some(cnames) = rows.column_names() {
            self.names = cnames.into_iter().map(|s| s.to_string()).collect();
            // set all columns as string (align with pandas)
            self.schema = vec![SQLiteTypeSystem::Text(false); self.names.len()];
        }
    }

    fn names(&self) -> Vec<String> {
        self.names.clone()
    }

    fn schema(&self) -> Vec<Self::TypeSystem> {
        self.schema.clone()
    }

    #[throws(SQLiteSourceError)]
    fn partition(self) -> Vec<Self::Partition> {
        let mut ret = vec![];
        for query in self.queries {
            let conn = self.pool.get()?;

            ret.push(SQLiteSourcePartition::new(conn, &query, &self.schema));
        }
        ret
    }
}

pub struct SQLiteSourcePartition {
    conn: PooledConnection<SqliteConnectionManager>,
    query: CXQuery<String>,
    schema: Vec<SQLiteTypeSystem>,
    nrows: usize,
    ncols: usize,
}

impl SQLiteSourcePartition {
    pub fn new(
        conn: PooledConnection<SqliteConnectionManager>,
        query: &CXQuery<String>,
        schema: &[SQLiteTypeSystem],
    ) -> Self {
        Self {
            conn,
            query: query.clone(),
            schema: schema.to_vec(),
            nrows: 0,
            ncols: schema.len(),
        }
    }
}

impl SourcePartition for SQLiteSourcePartition {
    type TypeSystem = SQLiteTypeSystem;
    type Parser<'a> = SQLiteSourcePartitionParser<'a>;
    type Error = SQLiteSourceError;

    #[throws(SQLiteSourceError)]
    fn prepare(&mut self) {
        let dialect = SQLiteDialect {};
        self.nrows = match get_limit(&self.query, &dialect)? {
            None => {
                self.conn
                    .query_row(count_query(&self.query, &dialect)?.as_str(), [], |row| {
                        Ok(row.get::<_, i64>(0)? as usize)
                    })?
            }
            Some(n) => n,
        };
    }

    #[throws(SQLiteSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        SQLiteSourcePartitionParser::new(&self.conn, self.query.as_str(), &self.schema)?
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

pub struct SQLiteSourcePartitionParser<'a> {
    rows: OwningHandle<Box<Statement<'a>>, DummyBox<Rows<'a>>>,
    ncols: usize,
    current_col: usize,
}

impl<'a> SQLiteSourcePartitionParser<'a> {
    #[throws(SQLiteSourceError)]
    pub fn new(
        conn: &'a PooledConnection<SqliteConnectionManager>,
        query: &str,
        schema: &[SQLiteTypeSystem],
    ) -> Self {
        let stmt: Statement<'a> = conn.prepare(query)?;

        // Safety: DummyBox borrows the on-heap stmt, which is owned by the OwningHandle.
        // No matter how we move the owning handle (thus the Box<Statment>), the Statement
        // keeps its address static on the heap, thus the borrow of MyRows keeps valid.
        let rows: OwningHandle<Box<Statement<'a>>, DummyBox<Rows<'a>>> =
            OwningHandle::new_with_fn(Box::new(stmt), |stmt: *const Statement<'a>| unsafe {
                DummyBox((&mut *(stmt as *mut Statement<'_>)).query([]).unwrap())
            });
        Self {
            rows,
            ncols: schema.len(),
            current_col: 0,
        }
    }

    #[throws(SQLiteSourceError)]
    fn next_loc(&mut self) -> (&Row, usize) {
        let row: &Row = match self.current_col {
            0 => (*self.rows).next()?.ok_or_else(|| anyhow!("Sqlite EOF"))?,
            _ => (*self.rows)
                .get()
                .ok_or_else(|| anyhow!("Sqlite empty current row"))?,
        };
        let col = self.current_col;
        self.current_col = (self.current_col + 1) % self.ncols;
        (row, col)
    }
}

impl<'a> PartitionParser<'a> for SQLiteSourcePartitionParser<'a> {
    type TypeSystem = SQLiteTypeSystem;
    type Error = SQLiteSourceError;
}

macro_rules! impl_produce {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for SQLiteSourcePartitionParser<'a> {
                type Error = SQLiteSourceError;

                #[throws(SQLiteSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (row, col) = self.next_loc()?;
                    let val = row.get(col)?;
                    val
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for SQLiteSourcePartitionParser<'a> {
                type Error = SQLiteSourceError;

                #[throws(SQLiteSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (row, col) = self.next_loc()?;
                    let val = row.get(col)?;
                    val
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
