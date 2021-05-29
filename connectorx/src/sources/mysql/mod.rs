mod typesystem;

use crate::data_order::DataOrder;
use crate::errors::{ConnectorAgentError, Result};
use crate::sources::{PartitionParser, Produce, Source, SourcePartition};

use r2d2::{Pool, PooledConnection};
use r2d2_mysql::{mysql::{consts::ColumnType, Opts, OptsBuilder, prelude::Queryable}, MysqlConnectionManager};

use sql::{count_query, get_limit, limit1_query};
use std::marker::PhantomData;

pub use typesystem::MysqlTypeSystem;

type MysqlManager = MysqlConnectionManager;
type MysqlConn = PooledConnection<MysqlManager>;

pub struct MysqlSource<P> {
    pool: Pool<MysqlManager>,
    queries: Vec<String>,
    names: Vec<String>,
    schema: Vec<MysqlTypeSystem>, // 4
    buf_size: usize,
    _protocol: PhantomData<P>,
}

impl<P> MysqlSource<P> {
    pub fn new(conn: &str, nconn: usize) -> Result<Self> {
        let manager = MysqlConnectionManager::new(OptsBuilder::from_opts(Opts::from_url(&url).unwrap()));
        let pool = r2d2::Pool::builder().max_size(4).build(manager).unwrap();

        Ok(Self {
            pool,
            queries: vec![],
            names: vec![],
            schema: vec![],
            buf_size: 32,
            _protocol: PhantomData,
        })
    }

    pub fn buf_size(&mut self, buf_size: usize) {
        self.buf_size = buf_size;
    }
}

impl<P> Source for MysqlSource<P>
where
    MysqlSourcePartition<P>: SourcePartition<TypeSystem = MysqlTypeSystem>, // 1
    P: Send,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = MysqlSourcePartition<P>;               // 2
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
                    self.names = vec!["test_int", "test_float", "test_str"];  //names;
                    self.schema = vec![ColumnType::MYSQL_TYPE_LONG,
                                       ColumnType::MYSQL_TYPE_DOUBLE,
                                       ColumnType::MYSQL_TYPE_BLOB];          //types;

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

pub struct MysqlPartition<P> {
    conn: MysqlConn,
    query: String,
    schema: Vec<MysqlTypeSystem>,  // 5
    nrows: usize,
    ncols: usize,
    buf_size: usize,
    _protocol: PhantomData<P>,
}


