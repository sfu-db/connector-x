"""
Usage:
  tpch-dask.py <num> [--conn=<conn>] [--index=<idx>]

Options:
  --conn=<conn>          The connection url to use [default: POSTGRES_URL].
  --index=<idx>          The connection url to use [default: l_orderkey].
  -h --help     Show this screen.
  --version     Show version.
"""

import os

import dask.dataframe as dd
from contexttimer import Timer
from docopt import docopt
from dask.distributed import Client, LocalCluster

if __name__ == "__main__":
    args = docopt(__doc__, version="Naval Fate 2.0")
    index_col = args["--index"]
    conn = os.environ[args["--conn"]]
    table = os.environ["TPCH_TABLE"]
    npartition = int(args["<num>"])
    cluster = LocalCluster(n_workers=npartition, scheduler_port=0, memory_limit="230G")
    client = Client(cluster)

    # https://docs.sqlalchemy.org/en/13/core/engines.html#sqlite
    # 4 initial slashes is needed for Unix/Mac
    if conn.startswith("sqlite"):
      conn = f"sqlite:///{conn[9:]}"

    with Timer() as timer:
        df = dd.read_sql_table(
            table,
            conn,
            index_col,
            npartitions=npartition,
            limits=(0, 60000000),
        ).compute()

    print(f"[Total] {timer.elapsed:.2f}s")

    print(df.head())
