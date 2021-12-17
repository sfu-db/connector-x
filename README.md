# ConnectorX [![status][ci_badge]][ci_page] [![discussions][discussion_badge]][discussion_page]

[ci_badge]: https://github.com/sfu-db/connector-x/workflows/ci/badge.svg
[ci_page]: https://github.com/sfu-db/connector-x/actions
[discussion_badge]: https://img.shields.io/badge/Forum-Github%20Discussions-blue
[discussion_page]: https://github.com/sfu-db/connector-x/discussions

Load data from <img src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/sources.gif" width="6.5%" style="margin-bottom: -2px"/> to <img src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/destinations.gif" width="7%" style="margin-bottom: -2px"/>, the fastest way.

ConnectorX enables you to load data from databases into Python in the fastest and most memory efficient way.

What you need is one line of code:

```python
import connectorx as cx

cx.read_sql("postgresql://username:password@server:port/database", "SELECT * FROM lineitem")
```

Optionally, you can accelerate the data loading using parallelism by specifying a partition column.

```python
import connectorx as cx

cx.read_sql("postgresql://username:password@server:port/database", "SELECT * FROM lineitem", partition_on="l_orderkey", partition_num=10)
```

The function will partition the query by **evenly** splitting the specified column to the amount of partitions.
ConnectorX will assign one thread for each partition to load and write data in parallel.
Currently, we support partitioning on **numerical** columns (**cannot contain NULL**) for **SPJA** queries. 

Check out more detailed usage and examples [here](#detailed-usage-and-examples). A general introduction of the project can be found in this [blog post](https://towardsdatascience.com/connectorx-the-fastest-way-to-load-data-from-databases-a65d4d4062d5).

# Installation

```bash
pip install connectorx
```

# Performance

We compared different solutions in Python that provides the `read_sql` function, by loading a 10x TPC-H lineitem table (8.6GB) from Postgres into a DataFrame, with 4 cores parallelism.

## Time chart, lower is better.

<p align="center"><img alt="time chart" src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/pg-time.png"/></p>

## Memory consumption chart, lower is better.

<p align="center"><img alt="memory chart" src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/pg-mem.png"/></p>

In conclusion, ConnectorX uses up to **3x** less memory and **21x** less time (**3x** less memory and **13x** less time compared with Pandas.). More on [here](https://github.com/sfu-db/connector-x/blob/main/Benchmark.md#benchmark-result-on-aws-r54xlarge).

## How does ConnectorX achieve a lightening speed while keeping the memory footprint low?

We observe that existing solutions more or less do data copy multiple times when downloading the data.
Additionally, implementing a data intensive application in Python brings additional cost.

ConnectorX is written in Rust and follows "zero-copy" principle.
This allows it to make full use of the CPU by becoming cache and branch predictor friendly. Moreover, the architecture of ConnectorX ensures the data will be copied exactly once, directly from the source to the destination.

## How does ConnectorX download the data?

Upon receiving the query, e.g. `SELECT * FROM lineitem`, ConnectorX will first issue a `LIMIT 1` query `SELECT * FROM lineitem LIMIT 1` to get the schema of the result set.

Then, if `partition_on` is specified, ConnectorX will issue `SELECT MIN($partition_on), MAX($partition_on) FROM (SELECT * FROM lineitem)` to know the range of the partition column.
After that, the original query is split into partitions based on the min/max information, e.g. `SELECT * FROM (SELECT * FROM lineitem) WHERE $partition_on > 0 AND $partition_on < 10000`.
ConnectorX will then run a count query to get the partition size (e.g. `SELECT COUNT(*) FROM (SELECT * FROM lineitem) WHERE $partition_on > 0 AND $partition_on < 10000`). If the partition
is not specified, the count query will be `SELECT COUNT(*) FROM (SELECT * FROM lineitem)`.

Finally, ConnectorX will use the schema info as well as the count info to allocate memory and download data by executing the queries normally.

Once the downloading begins, there will be one thread for each partition so that the data are downloaded in parallel at the partition level. The thread will issue the query of the corresponding
partition to the database and then write the returned data to the destination row-wise or column-wise (depends on the database) in a streaming fashion. 

#### How to specify the partition number?

`partition_num` will determine how many queries we are going to split from the original one and issue to the database. Underlying, we use [rayon](https://github.com/rayon-rs/rayon) as our parallel executor, which adopts a pool of threads to handle each partitioned query. The number of threads in the pool equals to the number of logical cores on the machine. It is recommended to set the `partition_num` to the number of available logical cores.

#### How to choose the partition column?

`partition_on` specifies on which column we will do the partition as above procedure. In order to achieve the best performance, it is ideal that each partitioned query will return the same number of rows. And since we partition the column evenly, it is recommended that the numerical `partition_on` column is evenly distributed. Whether a column has index or not might also affect the performance depends on the source database. You can give it a try if you have multiple candidates. Also, you can manually partition the query if our partition method cannot match your need. ConnectorX will still return a whole dataframe with all the results of the list of queries you input.


# Supported Sources & Destinations

Supported protocols, data types and type mappings can be found [here](Types.md).
For more planned data sources, please check out our [discussion](https://github.com/sfu-db/connector-x/discussions/61).

## Sources
- [x] Postgres
- [x] Mysql
- [x] Mariadb (through mysql protocol)
- [x] Sqlite
- [x] Redshift (through postgres protocol)
- [x] Clickhouse (through mysql protocol)
- [x] SQL Server
- [x] Azure SQL Database (through mssql protocol)
- [x] Oracle
- [ ] Big Query - In Progress
- [ ] ...

## Destinations
- [x] Pandas
- [x] PyArrow
- [x] Modin (through Pandas)
- [x] Dask (through Pandas)
- [x] Polars (through PyArrow)
  
# Detailed Usage and Examples

Rust docs: [stable](https://docs.rs/connectorx) [nightly](https://sfu-db.github.io/connector-x/connectorx/)

## API

```python
connectorx.read_sql(conn: str, query: Union[List[str], str], *, return_type: str = "pandas", protocol: str = "binary", partition_on: Optional[str] = None, partition_range: Optional[Tuple[int, int]] = None, partition_num: Optional[int] = None)
```

Run the SQL query, download the data from database into a Pandas dataframe.

## Parameters
- `conn: str`: Connection string URI.
  - General supported URI scheme: `(postgres|postgressql|mysql|mssql)://username:password@addr:port/dbname`.
  - For now sqlite only support absolute path, example: `sqlite:///home/user/path/test.db`.
  - Please check out [here](Types.md) for more connection uri parameters supported for each database (e.g. trusted_connection for Mssql, sslmode for Postgres)
- `query: Union[str, List[str]]`: SQL query or list of SQL queries for fetching data.
- `return_type: str = "pandas"`: The return type of this function. It can be `arrow`, `pandas`, `modin`, `dask` or `polars`.
- `protocol: str = "binary"`: The protocol used to fetch data from source, default is `binary`. Check out [here](Types.md) to see more details.
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

# Next Plan

Checkout our [discussion][discussion_page] to participate in deciding our next plan!

# Historical Benchmark Results

https://sfu-db.github.io/connector-x/dev/bench/

# Developer's Guide
Please see [Developer's Guide](https://github.com/sfu-db/connector-x/blob/main/CONTRIBUTING.md) for information about developing ConnectorX.

# Supports

You are always welcomed to:
1. Ask questions in stackoverflow. Make sure to have #connectorx attached.
2. Ask questions & propose new ideas in our [forum][discussion_page].
3. Ask questions & join the discussion & send direct messages to us in our [discord](https://discord.gg/xwbkFNk) (under `CONNECTOR` category)

# Organizations and Projects using ConnectorX

[<img src="https://raw.githubusercontent.com/pola-rs/polars-static/master/logos/polars-logo-dark.svg" height="100" style="margin-bottom: -2px"/>](https://github.com/pola-rs/polars)
[<img src="https://raw.githubusercontent.com/sfu-db/dataprep/develop/assets/logo.png" height="100" style="margin-bottom: -2px"/>](https://dataprep.ai/)

To add your project/organization here, reply our post [here](https://github.com/sfu-db/connector-x/discussions/146)
