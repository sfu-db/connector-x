use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::sources::sql::{count_query, get_limit, limit1_query};
use crate::sources::{PartitionParser, Produce, Source, SourcePartition};

use anyhow::anyhow;
use fehler::throw;
use log::debug;

use r2d2::{Pool, PooledConnection};
use r2d2_mysql::{
    mysql::{prelude::Queryable, Opts, OptsBuilder, QueryResult, Text},
    MysqlConnectionManager,
};

use crate::sources::mysql::typesystem::MysqlTypeSystem::{Double, Long};
use r2d2_mysql::mysql::Row;
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
        let manager =
            MysqlConnectionManager::new(OptsBuilder::from_opts(Opts::from_url(&conn).unwrap()));
        let pool = r2d2::Pool::builder()
            .max_size(nconn as u32)
            .build(manager)
            .unwrap();
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
    MysqlSourcePartition: SourcePartition<TypeSystem = MysqlTypeSystem>,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = MysqlSourcePartition;
    type TypeSystem = MysqlTypeSystem;

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

        // let mut conn = self.pool.get()?;
        // let mut success = false;
        // let mut error = None;
        for query in &self.queries {
            // assuming all the partition queries yield same schema
            self.names = vec!["test_int".to_string(), "test_float".to_string()]; // hardcode
            self.schema = vec![Long(true), Double(true)]; // hardcode
            // success = true;
            // match conn.query_first("select * from test_table limit 1") {
            //     Ok(row) => {
            //         // let (names, types) = row // 获得这一行的names和types
            //         //     .columns_ref()
            //         //     .into_iter()
            //         //     .map(|col| {
            //         //         (
            //         //             col.name().to_string(),
            //         //             PostgresTypeSystem::from(col.type_()),
            //         //         )
            //         //     })
            //         //     .unzip();
            //         self.names = vec!["test_int".to_string(), "test_float".to_string()]; // hardcode
            //         self.schema = vec![Long(true), Double(true)]; // hardcode
            //
            //         success = true;
            //         break;
            //     }
            //     Err(e) => {
            //         debug!("cannot get metadata for '{}', try next query: {}", query, e);
            //         error = Some(e);
            //     }
            // }
        }

        // if !success {
        //     throw!(anyhow!(
        //         "Cannot get metadata for the queries, last error: {:?}",
        //         error
        //     ))
        // }

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
            ret.push(MysqlSourcePartition::new(
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
    schema: Vec<MysqlTypeSystem>,
    nrows: usize,
    ncols: usize,
    buf_size: usize,
    // _protocol: PhantomData<P>,
}

impl MysqlSourcePartition {
    pub fn new(conn: MysqlConn, query: &str, schema: &[MysqlTypeSystem], buf_size: usize) -> Self {
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

impl SourcePartition for MysqlSourcePartition {
    type TypeSystem = MysqlTypeSystem;
    type Parser<'a> = MysqlSourcePartitionParser<'a>;

    fn prepare(&mut self) -> Result<()> {
        self.nrows = match get_limit(&self.query)? {
            // now get_limit using PostgreDialect
            None => {
                let row: Option<usize> = self.conn.query_first(&count_query(&self.query)?).unwrap();
                row.unwrap()
            }
            Some(n) => n,
        };
        Ok(())
    }

    fn parser(&mut self) -> Result<Self::Parser<'_>> {
        let query = self.query.clone();
        let mut iter = self.conn.query_iter(query).unwrap();
        Ok(MysqlSourcePartitionParser::new(
            iter,
            &self.schema,
            self.buf_size,
        ))
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

pub struct MysqlSourcePartitionParser<'a> {
    iter: QueryResult<'a, 'a, 'a, Text>,
    buf_size: usize,
    rowbuf: Vec<Row>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
}

impl<'a> MysqlSourcePartitionParser<'a> {
    pub fn new(
        iter: QueryResult<'a, 'a, 'a, Text>, // hardcode
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
                if let Some(item) = self.iter.next() {
                    self.rowbuf.push(item.unwrap());
                } else {
                    break;
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

impl<'a> PartitionParser<'a> for MysqlSourcePartitionParser<'a> {
    type TypeSystem = MysqlTypeSystem;
}

impl<'r, 'a> Produce<'r, i32> for MysqlSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<i32> {
        let (ridx, cidx) = self.next_loc()?;
        let res:i32 = self.rowbuf[ridx].get(cidx).unwrap();
        Ok(res)
    }
}

impl<'r, 'a> Produce<'r, f32> for MysqlSourcePartitionParser<'a> {
    fn produce(&mut self) -> Result<f32> {
        let (ridx, cidx) = self.next_loc()?;
        let res:f32 = self.rowbuf[ridx].get(cidx).unwrap();
        Ok(res)
    }
}
