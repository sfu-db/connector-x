mod typesystem;

use fallible_streaming_iterator::FallibleStreamingIterator;

use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::sources::{PartitionParser, Produce, Source, SourcePartition};
use anyhow::anyhow;
use fehler::throw;
use owning_ref::OwningRefMut;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Row, Rows, Statement};
pub use typesystem::SqliteTypeSystem;

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
    rows: OwningRefMut<Box<Statement<'a>>, Rows<'a>>,
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
        let rows: OwningRefMut<Box<Statement<'a>>, Rows<'a>> = OwningRefMut::new(Box::new(stmt))
            .map_mut(|stmt: &mut Statement<'a>| &mut stmt.query([]).unwrap());
        Ok(Self {
            rows,
            ncols: schema.len(),
            current_col: 0,
        })
    }
}

impl<'a> PartitionParser<'a> for SqliteSourcePartitionParser<'a> {
    type TypeSystem = SqliteTypeSystem;
}

impl<'r, 'a> Produce<'r, i32> for SqliteSourcePartitionParser<'a> {
    fn produce(&'r mut self) -> Result<i32> {
        let row: &Row = match self.current_col {
            0 => self.rows.next()?.ok_or_else(|| anyhow!("Sqlite EOF"))?,
            _ => self
                .rows
                .get()
                .ok_or_else(|| anyhow!("Sqlite empty current row"))?,
        };

        let val = row.get(self.current_col)?;
        self.current_col = (self.current_col + 1) % self.ncols;
        Ok(val)
    }
}
