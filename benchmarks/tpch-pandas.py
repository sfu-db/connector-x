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
from clickhouse_driver import connect

if __name__ == "__main__":
    args = docopt(__doc__, version="1.0")
    conn = os.environ[args["--conn"]]
    table = os.environ["TPCH_TABLE"]

    if conn.startswith("sqlite"):
        conn = sqlite3.connect(conn[9:])
    elif conn.startswith("mysql"):
        if args["--conn"] == "CLICKHOUSE_URL":
            # clickhouse-driver uses native protocol: 9000
            conn = connect(f"clickhouse://{os.environ['CLICKHOUSE_USER']}:{os.environ['CLICKHOUSE_PASSWORD']}@{os.environ['CLICKHOUSE_HOST']}:9000/tpch") 
        else:
            engine = create_engine(f"mysql+mysqldb:{conn[6:]}")
            conn = engine.connect()
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

    print(df.head())
    print(df.tail())
    print(len(df))
