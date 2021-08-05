"""
Usage:
  tpch-cx.py [--protocol=<protocol>]

Options:
  --protocol=<protocol>  The protocol to use [default: prestodb].
  -h --help              Show this screen.
  --version              Show version.
"""
import os

from docopt import docopt
import prestodb
from pyhive import presto
from sqlalchemy.engine import create_engine
import pandas as pd
from contexttimer import Timer

if __name__ == "__main__":
    args = docopt(__doc__, version="Naval Fate 2.0")
    proto = args["--protocol"]
    table = os.environ["TPCH_TABLE"]

    if proto == "prestodb":
        conn = prestodb.dbapi.connect(
            host=os.environ["PRESTO_HOST"],
            port=int(os.environ["PRESTO_PORT"]),
            user=os.environ["PRESTO_USER"],
            catalog=os.environ["PRESTO_CATALOG"],
            schema=os.environ["PRESTO_SCHEMA"],
        )
        cur = conn.cursor()
        with Timer() as timer:
            cur.execute(f'SELECT * FROM {table}')
            rows = cur.fetchall()
        print(f"fetch all: {timer.elapsed:.2f}")

        with Timer() as timer:
            df = pd.DataFrame(rows)
        print(f"to df: {timer.elapsed:.2f}")

    elif proto == "pyhive-pd":
        connection = presto.connect(
            host=os.environ["PRESTO_HOST"],
            port=int(os.environ["PRESTO_PORT"]),
            username=os.environ["PRESTO_USER"],
            catalog=os.environ["PRESTO_CATALOG"],
            schema=os.environ["PRESTO_SCHEMA"],
        )

        with Timer() as timer:
            df = pd.read_sql("select * from lineitem", connection)
        print(f"Time in total: {timer.elapsed:.2f}")
    elif proto == "pyhive":
        connection = presto.connect(
            host=os.environ["PRESTO_HOST"],
            port=int(os.environ["PRESTO_PORT"]),
            username=os.environ["PRESTO_USER"],
            catalog=os.environ["PRESTO_CATALOG"],
            schema=os.environ["PRESTO_SCHEMA"],
        )
        cur = connection.cursor()
        with Timer() as timer:
            cur.execute(f'SELECT * FROM {table}')
            rows = cur.fetchall()
        print(f"fetch all: {timer.elapsed:.2f}")

        with Timer() as timer:
            df = pd.DataFrame(rows)
        print(f"to df: {timer.elapsed:.2f}")
    elif proto == "sqlalchemy":
        engine = create_engine(f'presto://{os.environ["PRESTO_USER"]}@{os.environ["PRESTO_HOST"]}:{os.environ["PRESTO_PORT"]}/{os.environ["PRESTO_CATALOG"]}/{os.environ["PRESTO_SCHEMA"]}')
        conn = engine.connect()
        with Timer() as timer:
            df = pd.read_sql(f"SELECT * FROM {table}", conn)
        print(f"Time in total: {timer.elapsed:.2f}")

    print(df.head())
    print(len(df))
