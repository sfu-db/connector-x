"""
Usage:
  tpch-cx.py [--protocol=<protocol>] <num>

Options:
  --protocol=<protocol>  The protocol to use [default: binary].
  -h --help              Show this screen.
  --version              Show version.
"""
import os

import connectorx as cx
from contexttimer import Timer
from docopt import docopt

if __name__ == "__main__":

    args = docopt(__doc__, version="Naval Fate 2.0")
    conn = os.environ["POSTGRES_URL"]
    table = os.environ["POSTGRES_TABLE"]

    with Timer() as timer:
        df = cx.read_sql(
            conn,
            f"""SELECT * FROM {table}""",
            partition_on="l_orderkey",
            partition_num=int(args["<num>"]),
            protocol=args["--protocol"],
        )
    print("time in total:", timer.elapsed)

    print(df.head())
    print(len(df))
