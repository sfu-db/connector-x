from typing import Optional, Tuple, Union, List, Dict, Any

from .connectorx import (
    read_sql as _read_sql,
    partition_sql as _partition_sql,
    read_sql2 as _read_sql2,
    get_meta as _get_meta,
)

try:
    from importlib.metadata import version

    __version__ = version(__name__)
except:
    try:
        from importlib_metadata import version

        __version__ = version(__name__)
    except:
        pass

import os

dir_path = os.path.dirname(os.path.realpath(__file__))
# check whether it is in development env or installed
if (
    not os.path.basename(os.path.abspath(os.path.join(dir_path, "..")))
    == "connectorx-python"
):
    if "J4RS_BASE_PATH" not in os.environ:
        os.environ["J4RS_BASE_PATH"] = os.path.join(dir_path, "dependencies")
if "CX_REWRITER_PATH" not in os.environ:
    os.environ["CX_REWRITER_PATH"] = os.path.join(
        dir_path, "dependencies/federated-rewriter.jar"
    )


def rewrite_conn(conn: str, protocol: Optional[str] = None):
    if not protocol:
        # note: redshift/clickhouse are not compatible with the 'binary' protocol, and use other database
        # drivers to connect. set a compatible protocol and masquerade as the appropriate backend.
        backend, connection_details = conn.split(":", 1) if conn else ("", "")
        if "redshift" in backend:
            conn = f"postgresql:{connection_details}"
            protocol = "cursor"
        elif "clickhouse" in backend:
            conn = f"mysql:{connection_details}"
            protocol = "text"
        else:
            protocol = "binary"
    return conn, protocol


def get_meta(
    conn: str,
    query: str,
    protocol: Optional[str] = None,
):
    """
    Get metadata (header) of the given query (only for pandas)

    Parameters
    ==========
    conn
      the connection string.
    query
      a SQL query or a list of SQL queries.
    protocol
      backend-specific transfer protocol directive; defaults to 'binary' (except for redshift
      connection strings, where 'cursor' will be used instead).

    """
    conn, protocol = rewrite_conn(conn, protocol)
    result = _get_meta(conn, protocol, query)
    df = reconstruct_pandas(result)
    return df


def partition_sql(
    conn: str,
    query: str,
    partition_on: str,
    partition_num: int,
    partition_range: Optional[Tuple[int, int]] = None,
):
    """
    Partition the sql query

    Parameters
    ==========
    conn
      the connection string.
    query
      a SQL query or a list of SQL queries.
    partition_on
      the column on which to partition the result.
    partition_num
      how many partitions to generate.
    partition_range
      the value range of the partition column.
    """
    partition_query = {
        "query": query,
        "column": partition_on,
        "min": partition_range[0] if partition_range else None,
        "max": partition_range[1] if partition_range else None,
        "num": partition_num,
    }
    return _partition_sql(conn, partition_query)


def read_sql_pandas(
    sql: Union[List[str], str],
    con: Union[str, Dict[str, str]],
    index_col: Optional[str] = None,
    protocol: Optional[str] = None,
    partition_on: Optional[str] = None,
    partition_range: Optional[Tuple[int, int]] = None,
    partition_num: Optional[int] = None,
):
    """
    Run the SQL query, download the data from database into a dataframe.
    First several parameters are in the same name and order with `pandas.read_sql`.

    Parameters
    ==========
    Please refer to `read_sql`

    Examples
    ========
    Read a DataFrame from a SQL query using a single thread:

    >>> # from pandas import read_sql
    >>> from connectorx import read_sql_pandas as read_sql
    >>> postgres_url = "postgresql://username:password@server:port/database"
    >>> query = "SELECT * FROM lineitem"
    >>> read_sql(query, postgres_url)

    """
    return read_sql(
        con,
        sql,
        return_type="pandas",
        protocol=protocol,
        partition_on=partition_on,
        partition_range=partition_range,
        partition_num=partition_num,
        index_col=index_col,
    )


def read_sql(
    conn: Union[str, Dict[str, str]],
    query: Union[List[str], str],
    *,
    return_type: str = "pandas",
    protocol: Optional[str] = None,
    partition_on: Optional[str] = None,
    partition_range: Optional[Tuple[int, int]] = None,
    partition_num: Optional[int] = None,
    index_col: Optional[str] = None,
):
    """
    Run the SQL query, download the data from database into a dataframe.

    Parameters
    ==========
    conn
      the connection string, or dict of connection string mapping for federated query.
    query
      a SQL query or a list of SQL queries.
    return_type
      the return type of this function; one of "arrow(2)", "pandas", "modin", "dask" or "polars(2)".
    protocol
      backend-specific transfer protocol directive; defaults to 'binary' (except for redshift
      connection strings, where 'cursor' will be used instead).
    partition_on
      the column on which to partition the result.
    partition_range
      the value range of the partition column.
    partition_num
      how many partitions to generate.
    index_col
      the index column to set; only applicable for return type "pandas", "modin", "dask".

    Examples
    ========
    Read a DataFrame from a SQL query using a single thread:

    >>> postgres_url = "postgresql://username:password@server:port/database"
    >>> query = "SELECT * FROM lineitem"
    >>> read_sql(postgres_url, query)

    Read a DataFrame in parallel using 10 threads by automatically partitioning the provided SQL on the partition column:

    >>> postgres_url = "postgresql://username:password@server:port/database"
    >>> query = "SELECT * FROM lineitem"
    >>> read_sql(postgres_url, query, partition_on="partition_col", partition_num=10)

    Read a DataFrame in parallel using 2 threads by explicitly providing two SQL queries:

    >>> postgres_url = "postgresql://username:password@server:port/database"
    >>> queries = ["SELECT * FROM lineitem WHERE partition_col <= 10", "SELECT * FROM lineitem WHERE partition_col > 10"]
    >>> read_sql(postgres_url, queries)

    """
    if isinstance(query, list) and len(query) == 1:
        query = query[0]

    if isinstance(conn, dict):
        assert partition_on is None and isinstance(
            query, str
        ), "Federated query does not support query partitioning for now"
        assert (
            protocol is None
        ), "Federated query does not support specifying protocol for now"
        result = _read_sql2(query, conn)
        df = reconstruct_arrow(result)
        if return_type == "pandas":
            df = df.to_pandas(date_as_object=False, split_blocks=False)
        if return_type == "polars":
            try:
                import polars as pl
            except ModuleNotFoundError:
                raise ValueError("You need to install polars first")

            try:
                # api change for polars >= 0.8.*
                df = pl.from_arrow(df)
            except AttributeError:
                df = pl.DataFrame.from_arrow(df)
        return df

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

    conn, protocol = rewrite_conn(conn, protocol)

    if return_type in {"modin", "dask", "pandas"}:
        try:
            import pandas
        except ModuleNotFoundError:
            raise ValueError("You need to install pandas first")

        result = _read_sql(
            conn,
            "pandas",
            queries=queries,
            protocol=protocol,
            partition_query=partition_query,
        )
        df = reconstruct_pandas(result)

        if index_col is not None:
            df.set_index(index_col, inplace=True)

        if return_type == "modin":
            try:
                import modin.pandas as mpd
            except ModuleNotFoundError:
                raise ValueError("You need to install modin first")

            df = mpd.DataFrame(df)
        elif return_type == "dask":
            try:
                import dask.dataframe as dd
            except ModuleNotFoundError:
                raise ValueError("You need to install dask first")

            df = dd.from_pandas(df, npartitions=1)

    elif return_type in {"arrow", "arrow2", "polars", "polars2"}:
        try:
            import pyarrow
        except ModuleNotFoundError:
            raise ValueError("You need to install pyarrow first")

        result = _read_sql(
            conn,
            "arrow2" if return_type in {"arrow2", "polars", "polars2"} else "arrow",
            queries=queries,
            protocol=protocol,
            partition_query=partition_query,
        )
        df = reconstruct_arrow(result)
        if return_type in {"polars", "polars2"}:
            try:
                import polars as pl
            except ModuleNotFoundError:
                raise ValueError("You need to install polars first")

            try:
                df = pl.DataFrame.from_arrow(df)
            except AttributeError:
                # api change for polars >= 0.8.*
                df = pl.from_arrow(df)
    else:
        raise ValueError(return_type)

    return df


def reconstruct_arrow(result: Tuple[List[str], List[List[Tuple[int, int]]]]):
    import pyarrow as pa

    names, ptrs = result
    rbs = []
    if len(names) == 0:
        raise ValueError("Empty result")

    for chunk in ptrs:
        rb = pa.RecordBatch.from_arrays(
            [pa.Array._import_from_c(*col_ptr) for col_ptr in chunk], names
        )
        rbs.append(rb)
    return pa.Table.from_batches(rbs)


def reconstruct_pandas(df_infos: Dict[str, Any]):
    import pandas as pd

    data = df_infos["data"]
    headers = df_infos["headers"]
    block_infos = df_infos["block_infos"]

    nrows = data[0][0].shape[-1] if isinstance(data[0], tuple) else data[0].shape[-1]
    blocks = []
    for binfo, block_data in zip(block_infos, data):
        if binfo.dt == 0:  # NumpyArray
            blocks.append(
                pd.core.internals.make_block(block_data, placement=binfo.cids)
            )
        elif binfo.dt == 1:  # IntegerArray
            blocks.append(
                pd.core.internals.make_block(
                    pd.core.arrays.IntegerArray(block_data[0], block_data[1]),
                    placement=binfo.cids[0],
                )
            )
        elif binfo.dt == 2:  # BooleanArray
            blocks.append(
                pd.core.internals.make_block(
                    pd.core.arrays.BooleanArray(block_data[0], block_data[1]),
                    placement=binfo.cids[0],
                )
            )
        elif binfo.dt == 3:  # DatetimeArray
            blocks.append(
                pd.core.internals.make_block(
                    pd.core.arrays.DatetimeArray(block_data), placement=binfo.cids
                )
            )
        else:
            raise ValueError(f"unknown dt: {binfo.dt}")

    block_manager = pd.core.internals.BlockManager(
        blocks, [pd.Index(headers), pd.RangeIndex(start=0, stop=nrows, step=1)]
    )
    df = pd.DataFrame(block_manager)
    return df
