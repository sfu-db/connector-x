"""
Usage:
    tpch-queries-pd.py <id> [--conn=<conn>]

Options:
    --conn=<conn>             The connection url to use [default: POSTGRES_URL].
    -h --help                 Show this screen.
    --version                 Show version.
    """

import os

from pathlib import Path
from contexttimer import Timer
from sqlalchemy import create_engine
from docopt import docopt
import pandas as pd

if __name__ == "__main__":
    args = docopt(__doc__, version="1.0")
    conn = os.environ[args["--conn"]]
    print(f"conn url: {conn}")
    engine = create_engine(conn)
    conn = engine.connect()

    qid = args["<id>"]
    print(f"execute query id: {qid}")

    qdir = Path(os.environ["TPCH_QUERIES"], f"q{qid}.sql")
    print(f"load query from: {qdir}")

    with open(qdir, "r") as f:
        query = f.read()
    print(f"query: {query}")
    query = query.replace("%", "%%")

    with Timer() as timer:
        df = pd.read_sql(query, conn)
    print(f"[pd][QID: {qid} Total] {timer.elapsed:.2f}s")

    conn.close()
    print(df)
    print(f"result size: {len(df)}x{len(df.columns)}")
