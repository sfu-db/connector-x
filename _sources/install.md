# Getting Started

Whether you write your book's content in Jupyter Notebooks (`.ipynb`) or
in regular markdown files (`.md`), you'll write in the same flavor of markdown
called **MyST Markdown**.

# Installation

## Pip

### Linux and OSX

To install ConnectorX on Linux and OSX, use the following command:

```bash
pip install connectorx
```
### Windows

To install ConnectorX on Windows, use the following command:

```bash
pip install connectorx
```

# Basic usage
ConnectorX enables you to run the SQL query, load data from databases into a Pandas Dataframe in the fastest and most memory efficient way.

## API
```python
connectorx.read_sql(conn: str, query: Union[List[str], str], *, return_type: str = "pandas", protocol: str = "binary", partition_on: Optional[str] = None, partition_range: Optional[Tuple[int, int]] = None, partition_num: Optional[int] = None)
```

## Parameters
- `conn: str`: Connection string URI.
  - General supported URI scheme: `(postgres|postgressql|mysql|mssql)://username:password@addr:port/dbname`.
  - For now sqlite only support absolute path, example: `sqlite:///home/user/path/test.db`.
  - Google BigQuery requires absolute path of the authentication JSON file, example: `bigquery:///home/user/path/auth.json`
  - Please check out [here](Types.md) for more connection uri parameters supported for each database (e.g. trusted_connection for Mssql, sslmode for Postgres)
- `query: Union[str, List[str]]`: SQL query or list of SQL queries for fetching data.
- `return_type: str = "pandas"`: The return type of this function. It can be `arrow`, `pandas`, `modin`, `dask` or `polars`.
- `protocol: str = "binary"`: The protocol used to fetch data from source, default is `binary`. Check out [here](./databases.md) to see more details.
- `partition_on: Optional[str]`: The column to partition the result.
- `partition_range: Optional[Tuple[int, int]]`: The value range of the partition column.
- `partition_num: Optioinal[int]`: The number of partitions to generate.
- `index_col: Optioinal[str]`: The index column to set for the result dataframe. Only applicable when `return_type` is `pandas`, `modin` or `dask`. 


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


