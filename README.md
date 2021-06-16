# ConnectorX [![status][ci_badge]][ci_page] [![docs][docs_badge]][docs_page]

[ci_badge]: https://github.com/sfu-db/connector-agent/workflows/ci/badge.svg
[ci_page]: https://github.com/sfu-db/connector-agent/actions
[docs_badge]: https://github.com/sfu-db/connector-agent/workflows/docs/badge.svg
[docs_page]: https://sfu-db.github.io/connector-agent/connector_agent/

Load data from <img src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/sources.gif" width="6.5%" style="margin-bottom: -2px"/> to <img src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/destinations.gif" width="7%" style="margin-bottom: -2px"/>, the fastest way.

**Currently only support Postgres to Pandas. MySQL is in development.**
For more data sources, please check out our [discussion](https://github.com/sfu-db/connector-x/discussions/61).

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
Currently, we support partitioning on **integer** columns for **SPJA** queries.

Check out more detailed usage and examples [here](#detailed-usage-and-examples).

# Installation

```bash
pip install connectorx
```

# Performance

We compared different solutions in Python that provides the `read_sql` function, by loading a 10x TPC-H lineitem table (8.6GB) from Postgres into a DataFrame, with 4 cores parallelism.

## Time chart, lower is better.

<p align="center"><img alt="time chart" src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/time.jpg"/></p>

## Memory consumption chart, lower is better.

<p align="center"><img alt="memory chart" src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/memory.jpg"/></p>

In conclusion, ConnectorX uses up to **3x** less memory and **11x** less time.

## How does ConnectorX achieve a lightening speed while keeping the memory footprint low?

We observe that existing solutions more or less do data copy multiple times when downloading the data.
Additionally, implementing a data intensive application in Python brings additional cost.

ConnectorX is written in Rust and follows "zero-copy" principle.
This allows it to make full use of the CPU by becoming cache and branch predictor friendly. Moreover, the architecture of ConnectorX ensures the data will be copied exactly once, directly from the source to the destination.

# Detailed Usage and Examples

## API

```python
connectorx.read_sql(conn: str, query: Union[List[str], str], *, return_type: str = "pandas", protocol: str = "binary", partition_on: Optional[str] = None, partition_range: Optional[Tuple[int, int]] = None, partition_num: Optional[int] = None)
```

Run the SQL query, download the data from database into a Pandas dataframe.

## Parameters
- **conn**(str): Connection string uri. Currently only PostgreSQL is supported.
- **query**(string or list of string): SQL query or list of SQL queries for fetching data.
- **return_type**(string, optional(default `"pandas"`)): The return type of this function. Currently only "pandas" is supported.
- **partition_on**(string, optional(default `None`)): The column to partition the result.
- **partition_range**(tuple of int, optional(default `None`)): The value range of the partition column.
- **partition_num**(int, optional(default `None`)): The number of partitions to generate.

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

Checkout our [discussions](https://github.com/sfu-db/connector-x/discussions) to participate in deciding our next plan!
