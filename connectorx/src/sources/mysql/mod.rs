use mysql::{Opts,OptsBuilder};
use mysql::prelude::Queryable;
use r2d2::{Pool, PooledConnection};
use r2d2_mysql::MysqlConnectionManager;

// pub struct MysqlSource<P> {
//     pool: Pool<PgManager>,
//     queries: Vec<String>,
//     names: Vec<String>,
//     schema: Vec<PostgresTypeSystem>,
//     buf_size: usize,
//     _protocol: PhantomData<P>,
// }

impl<P> MysqlSource<P> {
    pub fn new(conn: &str, nconn: usize) -> Result<Self> {
        let o = Opts::
        let opts = Opts::from_url(&url).unwrap();
        let builder = OptsBuilder::from_opts(opts);
        let manager = MysqlConnectionManager::new(builder);
        let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());

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
