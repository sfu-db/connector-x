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
from dask.distributed import Client, LocalCluster

if __name__ == "__main__":
    args = docopt(__doc__, version="Naval Fate 2.0")
    conn = os.environ["POSTGRES_URL"]
    table = os.environ["POSTGRES_TABLE"]
    npartition = int(args["<num>"])
    cluster = LocalCluster(n_workers=npartition, scheduler_port=0, memory_limit="130G")
    client = Client(cluster)

    with Timer() as timer:
        df = dd.read_sql_table(
            table,
            conn,
            "l_orderkey",
            npartitions=npartition,
            limits=(0, 60000000),
        ).compute()

    print(f"[Total] {timer.elapsed:.2f}s")

    print(df.head())
