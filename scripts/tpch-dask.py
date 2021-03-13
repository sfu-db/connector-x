"""
Usage:
  tpch-dask.py <num>

Options:
  -h --help     Show this screen.
  --version     Show version.
"""

import os

import dask.dataframe as dd
from contexttimer import Timer
from docopt import docopt

if __name__ == "__main__":
    args = docopt(__doc__, version="Naval Fate 2.0")
    conn = os.environ["POSTGRES_URL"]
    table = os.environ["POSTGRES_TABLE"]

    with Timer() as timer:
        df = dd.read_sql_table(
            table,
            conn,
            "l_orderkey",
            npartitions=int(args["<num>"]),
            limits=(0, 60000000),
        ).compute()

    print(f"[Total] {timer.elapsed:.2f}s")

    print(df.head())
