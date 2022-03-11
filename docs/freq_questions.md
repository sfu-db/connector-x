# Frequently asked questions

## How to specify the partition number?

`partition_num` will determine how many queries we are going to split from the original one and issue to the database. Underlying, we use [rayon](https://github.com/rayon-rs/rayon) as our parallel executor, which adopts a pool of threads to handle each partitioned query. The number of threads in the pool equals to the number of logical cores on the machine. It is recommended to set the `partition_num` to the number of available logical cores.

## How to choose the partition column?

`partition_on` specifies on which column we will partition the query. In order to achieve the best performance, it is ideal that each partitioned query will return the same number of rows. And since we partition the column evenly, it is recommended that the numerical `partition_on` column is evenly distributed. Whether a column has index or not might also affect the performance depends on the source database. You can give it a try if you have multiple candidates. Also, you can manually partition the query if our partition method cannot match your need. ConnectorX will still return a whole dataframe with all the results of the list of queries you input.

## How to print log in Python?

Set the environment variable `RUST_LOG` to have a detailed look at Rust log.
```python
import os
os.environ["RUST_LOG"]="connectorx=debug,connectorx_python=debug"
import connectorx as cx

df = cx.read_sql(conn, query) // It will be more clear to test when no partitioning first
```

## Why is my query slow on ConnectorX?

ConnectorX is mainly targeting on the large query result fetching scenario. It speeds up the process by optimizing the client-side execution and saturating both network and machine resource through parallelism. When query execution on the database server is the bottleneck (for example when the result size is small, and/or the query is very complex), there will be overhead coming from metadata fetching. In ConnectorX, there are up to three info that will be fetched before issue the query to database:

* MIN, MAX query for partition range (if partition is enabled and `partition_range` is not given)
* COUNT query (if `return_type="pandas"`)
* schema fetching query, which gets type and name for each column in the result

For users who want to have pandas.DataFrame as final result. In order to avoid the costly COUNT query, one workaround is to use Arrow as an intermediate destination from ConnectorX and convert it into Pandas using Arrow’s [to_pandas API](https://arrow.apache.org/docs/python/generated/pyarrow.Table.html?pyarrow.Table.to_pandas). For example:

```Python
import connectorx as cx

table = cx.read_sql(db_uri, query, return_type="arrow") # or arrow2 https://github.com/jorgecarleitao/arrow2
df = table.to_pandas(split_blocks=False, date_as_object=False)
```