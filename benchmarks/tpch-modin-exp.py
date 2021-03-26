"""
Usage:
  tpch-modin-exp.py <num>

Options:
  -h --help     Show this screen.
  --version     Show version.
"""

import os
os.environ["MODIN_ENGINE"] = "ray"
import ray
from contexttimer import Timer
from docopt import docopt


if __name__ == "__main__":
    args = docopt(__doc__, version="1.0")
    conn = os.environ["POSTGRES_URL"]
    table = os.environ["POSTGRES_TABLE"]

    partitions = int(args["<num>"])
    # ray.init(num_cpus=partitions, object_store_memory=10**10, _plasma_directory="/tmp")
    ray.init(num_cpus=partitions, object_store_memory=10**10)

    import modin.config as config
    import modin.experimental.pandas as pd

    config.NPartitions.put(partitions)
    with Timer() as timer:
        df = pd.read_sql(
            f"{table}", # use table here, a bug exists in modin experimental read_sql for query
            conn,
            parse_dates=[
                "l_shipdate",
                "l_commitdate",
                "l_receiptdate",
            ],
            partition_column="l_orderkey",
            lower_bound=0,
            upper_bound=60000000,
            max_sessions=partitions,
        )
    print(f"[Total] {timer.elapsed:.2f}s")

    print(df.head())
