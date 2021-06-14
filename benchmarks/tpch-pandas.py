"""
Usage:
  tpch-pandas.py [--conn=<conn>]

Options:
  --conn=<conn>          The connection url to use [default: POSTGRES_URL].
  -h --help     Show this screen.
  --version     Show version.
"""

import os

from contexttimer import Timer
from sqlalchemy import create_engine
from docopt import docopt
import pandas as pd
import sqlite3

if __name__ == "__main__":
    args = docopt(__doc__, version="1.0")
    conn = os.environ[args["--conn"]]
    table = os.environ["TPCH_TABLE"]

    if conn.startswith("sqlite://"):
        conn = sqlite3.connect(conn[9:])
        with Timer() as timer:
            df = pd.read_sql(
                f"SELECT * FROM {table}",
                conn,
            )
        print(f"[Total] {timer.elapsed:.2f}s")
        conn.close()

    else:
        engine = create_engine(conn)
        conn = engine.connect()
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
        conn.close()
        engine.close()

    print(df.head())
    print(len(df))
