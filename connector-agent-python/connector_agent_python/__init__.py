from typing import Optional, Tuple, Union, List

import pandas as pd

from .connector_agent_python import read_pg
from .connector_agent_python import read_sql as _read_sql


def read_sql(
    conn: str,
    query: Union[List[str], str],
    *,
    return_type: str = "pandas",
    partition_on: Optional[str] = None,
    partition_range: Optional[Tuple[int, int]] = None,
    partition_num: Optional[int] = None,
) -> pd.DataFrame:
    if isinstance(query, str):
        if partition_on is None:
            queries = [query]
            partition_query = None
        else:
            partition_query = {
                "query": query,
                "column": partition_on,
                "min": partition_range[0],
                "max": partition_range[1],
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
        partition_query=partition_query,
    )
