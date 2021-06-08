use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::sources::{PartitionParser, Produce, Source, SourcePartition};
use crate::sources::sql::{limit1_query, count_query, get_limit};

use r2d2::{Pool, PooledConnection};
use r2d2_mysql::{mysql::{consts::ColumnType, Opts, OptsBuilder, prelude::Queryable}, MysqlConnectionManager};

use sql::{count_query, get_limit, limit1_query};
use std::marker::PhantomData;

pub use typesystem::MysqlTypeSystem;

mod typesystem;

type MysqlManager = MysqlConnectionManager;
type MysqlConn = PooledConnection<MysqlManager>;

pub struct MysqlSource {
    pool: Pool<MysqlManager>,
    queries: Vec<String>,
    names: Vec<String>,
    schema: Vec<MysqlTypeSystem>,
    buf_size: usize,
}

impl MysqlSource {
    pub fn new(conn: &str, nconn: usize) -> Result<Self> {
        let manager = MysqlConnectionManager::new(OptsBuilder::from_opts(Opts::from_url(&conn).unwrap()));
        let pool = r2d2::Pool::builder().max_size(nconn as u32).build(manager).unwrap();

        Ok(Self {
            pool,
            queries: vec![],
            names: vec![],
            schema: vec![],
            buf_size: 32,
        })
    }

    pub fn buf_size(&mut self, buf_size: usize) {
        self.buf_size = buf_size;
    }
}

impl Source for MysqlSource
where
    MysqlSourcePartition: SourcePartition<TypeSystem = MysqlTypeSystem>, // 1
    P: Send,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = MysqlSourcePartition;               // 2
    type TypeSystem = MysqlTypeSystem;                      // 3

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
            match conn.query_first(&limit1_query(query)?[..]) { // &limit1_query(query)?[..]这个还没理解
                Ok(row) => {
                    // let (names, types) = row // 获得这一行的names和types
                    //     .columns_ref()
                    //     .into_iter()
                    //     .map(|col| {
                    //         (
                    //             col.name().to_string(),
                    //             PostgresTypeSystem::from(col.type_()),
                    //         )
                    //     })
                    //     .unzip();
                    self.names = vec!["test_int", "test_float"];  //names;
                    self.schema = vec![ColumnType::MYSQL_TYPE_LONG,
                                       ColumnType::MYSQL_TYPE_DOUBLE];          //types;

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
            ret.push(MysqlSourcePartition::<P>::new(
                conn,
                &query,
                &self.schema,
                self.buf_size,
            ));
        }
        Ok(ret)
    }
}

pub struct MysqlSourcePartition {
    conn: MysqlConn,
    query: String,
    schema: Vec<MysqlTypeSystem>,  // 5
    nrows: usize,
    ncols: usize,
    buf_size: usize,
    // _protocol: PhantomData<P>,
}

impl MysqlSourcePartition {
    pub fn new(conn: PgConn, query: &str, schema: &[MysqlTypeSystem], buf_size: usize) -> Self {
        Self {
            conn,
            query: query.to_string(),
            schema: schema.to_vec(),
            nrows: 0,
            ncols: schema.len(),
            buf_size,
            // _protocol: PhantomData,
        }
    }
}

impl SourcePartition for MysqlSourcePartition<Binary> {
    type TypeSystem = MysqlTypeSystem;
    type Parser<'a> = PostgresBinarySourcePartitionParser<'a>;

    fn prepare(&mut self) -> Result<()> {
        self.nrows = match get_limit(&self.query)? {
            None => {
                let row = self.conn.query_first(&count_query(&self.query)?)?; // 这里我应该写错的
                row.get::<_, i64>(0) as usize
            }
            Some(n) => n,
        };
        Ok(())
    }

    // fn parser(&mut self) -> Result<Self::Parser<'_>> {
    //     let reader = self.conn.query(&*query)?; // unless reading the data, it seems like issue the query is fast
    //     let pg_schema: Vec<_> = self.schema.iter().map(|&dt| dt.into()).collect();
    //     let iter = BinaryCopyOutIter::new(reader, &pg_schema);
    //
    //     Ok(PostgresBinarySourcePartitionParser::new(
    //         iter,
    //         &self.schema,
    //         self.buf_size,
    //     ))
    // }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

pub struct PostgresBinarySourcePartitionParser<'a> {
    iter: BinaryCopyOutIter<'a>,
    buf_size: usize,
    rowbuf: Vec<BinaryCopyOutRow>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
}

impl<'a> PostgresBinarySourcePartitionParser<'a> {
    pub fn new(
        iter: BinaryCopyOutIter<'a>,
        schema: &[MysqlTypeSystem],
        buf_size: usize,
    ) -> Self {
        Self {
            iter,
            buf_size,
            rowbuf: Vec::with_capacity(buf_size),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
        }
    }

    fn next_loc(&mut self) -> Result<(usize, usize)> {
        if self.current_row >= self.rowbuf.len() {
            if !self.rowbuf.is_empty() {
                self.rowbuf.drain(..);
            }

            for _ in 0..self.buf_size {
                match self.iter.next()? {
                    Some(row) => {
                        self.rowbuf.push(row);
                    }
                    None => break,
                }
            }

            if self.rowbuf.is_empty() {
                throw!(anyhow!("Postgres EOF"));
            }
            self.current_row = 0;
            self.current_col = 0;
        }

        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        Ok(ret)
    }
}

