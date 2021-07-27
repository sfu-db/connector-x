"""
Usage:
  tpch-modin.py <num> [--conn=<conn>]

Options:
  --conn=<conn>          The connection url to use [default: POSTGRES_URL].
  -h --help     Show this screen.
  --version     Show version.
"""

import os

import modin.config as config
import modin.pandas as pd
from contexttimer import Timer
from docopt import docopt
from dask.distributed import Client, LocalCluster

# modin adopts the fastest mysqlclient connector for mysql

if __name__ == "__main__":
    args = docopt(__doc__, version="1.0")
    conn = os.environ[args["--conn"]]
    table = os.environ["TPCH_TABLE"]

    partitions = int(args["<num>"])
    config.NPartitions.put(partitions)

    cluster = LocalCluster(n_workers=partitions, scheduler_port=0, memory_limit="230G")
    client = Client(cluster)

    # https://docs.sqlalchemy.org/en/13/core/engines.html#sqlite
    # 4 initial slashes is needed for Unix/Mac
    if conn.startswith("sqlite"):
      conn = f"sqlite:///{conn[9:]}"

    with Timer() as timer:
        df = pd.read_sql(
            f"SELECT * FROM {table}",
            conn,
            parse_dates=[
                "l_shipdate",
                "l_commitdate",
                "l_receiptdate",
            ],
        )
    print(f"[Total] {timer.elapsed:.2f}s")

    print(df.head())
