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
            f"""select * from lineitem where l_orderkey > {split[i]} and l_orderkey <= {split[i+1]}"""
        )
    return sqls


if __name__ == "__main__":
    args = docopt(__doc__, version="Naval Fate 2.0")
    conn = os.environ["POSTGRES_URL"]

    queries = get_sqls(int(args["<num>"]))

    schema = [
        "int64",
        "int64",
        "int64",
        "int64",
        "float64",
        "float64",
        "float64",
        "float64",
        "string",
        "string",
        "date",
        "date",
        "date",
        "string",
        "string",
        "string",
    ]
    df = write_pandas(conn, queries, schema, False)

    print(df.head())
    print(len(df))
