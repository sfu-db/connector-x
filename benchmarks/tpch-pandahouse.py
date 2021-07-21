"""
Usage:
  tpch-pandahouse.py [--index=<idx>]

Options:
  --index=<idx>          The connection url to use [default: L_ORDERKEY].
  -h --help     Show this screen.
  --version     Show version.
"""

import os

from contexttimer import Timer
from docopt import docopt
import pandas as pd
from pandahouse import read_clickhouse

if __name__ == "__main__":
    args = docopt(__doc__, version="1.0")
    index_col = args["--index"]
    table = os.environ["TPCH_TABLE"]

    conn = {
        "host": f"http://{os.environ['CLICKHOUSE_HOST']}:8123", # 8123 is default clickhouse http port
        "database": os.environ["CLICKHOUSE_DB"],
        "user": os.environ["CLICKHOUSE_USER"],
        "password": os.environ["CLICKHOUSE_PASSWORD"],
    }
    print(conn)

    with Timer() as timer:
        df = read_clickhouse(f'SELECT * FROM {conn["database"]}.{table}', index_col=index_col, connection=conn)
    print(f"[Total] {timer.elapsed:.2f}s")

    print(df.head())
    print(df.tail())
    print(len(df))