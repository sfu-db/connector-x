from typing import Optional, Tuple, Union, List

import pandas as pd

from .connectorx_python import read_pg
from .connectorx_python import read_sql as _read_sql

try:
    from importlib.metadata import version
    __version__ = version(__name__)
except:
  try:
    from importlib_metadata import version
    __version__ = version(__name__)
  except:
    pass

def read_sql(
    conn: str,
    query: Union[List[str], str],
    *,
    return_type: str = "pandas",
    protocol: str = "binary",
    partition_on: Optional[str] = None,
    partition_range: Optional[Tuple[int, int]] = None,
    partition_num: Optional[int] = None,
) -> pd.DataFrame:
    """
    Run the SQL query, download the data from database into a Pandas dataframe.

    Parameters
    ==========
    conn
      the connection string.
    query
      a SQL query or a list of SQL query.
    return_type
      the return type of this function. Currently only "pandas" is supported.
    partition_on
      the column to partition the result.
    partition_range
      the value range of the partition column.
    partition_num
      how many partition to generate.

    Examples
    ========
    Read a DataFrame from a SQL using a single thread:

    >>> postgres_url = "postgresql://username:password@server:port/database"
    >>> query = "SELECT * FROM lineitem"
    >>> read_sql(postgres_url, query)

    Read a DataFrame parallelly using 10 threads by automatically partitioning the provided SQL on the partition column:

    >>> postgres_url = "postgresql://username:password@server:port/database"
    >>> query = "SELECT * FROM lineitem"
    >>> read_sql(postgres_url, query, partition_on="partition_col", partition_num=10)

    Read a DataFrame parallelly using 2 threads by manually providing two partition SQLs:

    >>> postgres_url = "postgresql://username:password@server:port/database"
    >>> queries = ["SELECT * FROM lineitem WHERE partition_col <= 10", "SELECT * FROM lineitem WHERE partition_col > 10"]
    >>> read_sql(postgres_url, queries)

    """

    if isinstance(query, list) and len(query) == 1:
        query = query[0]

    if isinstance(query, str):
        if partition_on is None:
            queries = [query]
            partition_query = None
        else:
            partition_query = {
                "query": query,
                "column": partition_on,
                "min": partition_range[0] if partition_range else None,
                "max": partition_range[1] if partition_range else None,
                "num": partition_num,
            }
            queries = None
    elif isinstance(query, list):
        queries = query
        partition_query = None

        if partition_on is not None:
            raise ValueError("Partition on multiple queries is not supported.")
    else:
        raise ValueError("query must be either str or a list of str")

    return _read_sql(
        conn,
        return_type,
        queries=queries,
        protocol=protocol,
        partition_query=partition_query,
    )
