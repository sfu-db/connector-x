"""
Usage:
  tpch-modin.py <num>

Options:
  -h --help     Show this screen.
  --version     Show version.
"""

import os

import modin.config as config
import modin.pandas as pd
from contexttimer import Timer
from docopt import docopt
from dask.distributed import Client, LocalCluster

if __name__ == "__main__":
    args = docopt(__doc__, version="Naval Fate 2.0")
    conn = os.environ["POSTGRES_URL"]
    table = os.environ["POSTGRES_TABLE"]

    partitions = int(args["<num>"])
    config.NPartitions.put(partitions)

    cluster = LocalCluster(n_workers=partitions, scheduler_port=0, memory_limit="130G")
    client = Client(cluster)

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
