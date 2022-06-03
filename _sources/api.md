# Basic usage
ConnectorX enables you to run the SQL query, load data from databases into a Pandas Dataframe in the fastest and most memory efficient way.

## API
```python
connectorx.read_sql(conn: Union[str, Dict[str, str]], query: Union[List[str], str], *, return_type: str = "pandas", protocol: str = "binary", partition_on: Optional[str] = None, partition_range: Optional[Tuple[int, int]] = None, partition_num: Optional[int] = None)
```

## Parameters
- `conn: Union[str, Dict[str, str]]`: Connection string URI for querying single database or dict of database names (key) and connection string URIs (value) for querying multiple databases.
  - Please check out [here](https://sfu-db.github.io/connector-x/databases.html) for connection string examples of each database
- `query: Union[str, List[str]]`: SQL query or list of partitioned SQL queries for fetching data.
- `return_type: str = "pandas"`: The return type of this function. It can be `arrow` (`arrow2`), `pandas`, `modin`, `dask` or `polars`.
- `protocol: str = "binary"`: The protocol used to fetch data from source, default is `binary`. Check out [here](./databases.md) to see more details.
- `partition_on: Optional[str]`: The column to partition the result.
- `partition_range: Optional[Tuple[int, int]]`: The value range of the partition column.
- `partition_num: Optional[int]`: The number of partitions to generate.
- `index_col: Optional[str]`: The index column to set for the result dataframe. Only applicable when `return_type` is `pandas`, `modin` or `dask`. 


## Examples
- Read a DataFrame from a SQL using a single thread

  ```python
  import connectorx as cx

  postgres_url = "postgresql://username:password@server:port/database"
  query = "SELECT * FROM lineitem"

  cx.read_sql(postgres_url, query)
  ```

- Read a DataFrame parallelly using 10 threads by automatically partitioning the provided SQL on the partition column (`partition_range` will be automatically  queried if not given)

  ```python
  import connectorx as cx

  postgres_url = "postgresql://username:password@server:port/database"
  query = "SELECT * FROM lineitem"

  cx.read_sql(postgres_url, query, partition_on="l_orderkey", partition_num=10)
  ```

- Read a DataFrame parallelly using 2 threads by manually providing two partition SQLs (the schemas of all the query results should be same)

  ```python
  import connectorx as cx

  postgres_url = "postgresql://username:password@server:port/database"
  queries = ["SELECT * FROM lineitem WHERE l_orderkey <= 30000000", "SELECT * FROM lineitem WHERE l_orderkey > 30000000"]

  cx.read_sql(postgres_url, queries)

  ```
  
- Read a DataFrame parallelly using 4 threads from a more complex query

  ```python
  import connectorx as cx

  postgres_url = "postgresql://username:password@server:port/database"
  query = f"""
  SELECT l_orderkey,
         SUM(l_extendedprice * ( 1 - l_discount )) AS revenue,
         o_orderdate,
         o_shippriority
  FROM   customer,
         orders,
         lineitem
  WHERE  c_mktsegment = 'BUILDING'
         AND c_custkey = o_custkey
         AND l_orderkey = o_orderkey
         AND o_orderdate < DATE '1995-03-15'
         AND l_shipdate > DATE '1995-03-15'
  GROUP  BY l_orderkey,
            o_orderdate,
            o_shippriority 
  """

  cx.read_sql(postgres_url, query, partition_on="l_orderkey", partition_num=4)

  ```

- Read a DataFrame from a SQL joined from multiple databases (experimental, only support PostgreSQL for now)

  ```python
  import connectorx as cx

  db1 = "postgresql://username1:password1@server1:port1/database1"
  db2 = "postgresql://username2:password2@server2:port2/database2"
  query = "SELECT * FROM db1.nation n, db2.region r where n.n_regionkey = r.r_regionkey"

  cx.read_sql({"db1": db1, "db2": db2}, query)

  ```

