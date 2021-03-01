"""
Usage:
  test_tpch.py <num>

Options:
  -h --help     Show this screen.
  --version     Show version.
"""
import os
from typing import List

import numpy as np
from connector_agent_python import write_pandas
from docopt import docopt


def get_sqls(count: int) -> List[str]:
    sqls = []
    split = np.linspace(0, 6000000, num=count + 1, endpoint=True, dtype=int)
    for i in range(len(split) - 1):

        sqls.append(
            f"""select  l_orderkey,
                l_partkey,
                l_suppkey,
                l_linenumber,
                l_quantity::float8,
                l_extendedprice::float8,
                l_discount::float8,
                l_tax::float8,
                l_returnflag,
                l_linestatus,
                l_shipdate,
                l_commitdate,
                l_receiptdate,                
                l_shipinstruct,
                l_shipmode,
                l_comment
                from lineitem where l_orderkey > {split[i]} and l_orderkey <= {split[i+1]}"""
        )
    return sqls


if __name__ == "__main__":
    args = docopt(__doc__, version="Naval Fate 2.0")
    conn = os.environ["POSTGRES_URL"]

    queries = get_sqls(int(args["<num>"]))

    df = write_pandas(conn, queries, False)

    print(df.head())
    print(len(df))
