"""
Usage:
  tpch-fed.py --file=<file>

Options:
  --file=<file>  Query file.
  -h --help              Show this screen.
  --version              Show version.
"""
import os

import connectorx as cx
from contexttimer import Timer
from docopt import docopt
import pandas as pd


if __name__ == "__main__":
    args = docopt(__doc__, version="Naval Fate 2.0")
    query_file = args["--file"]

    db_map = {
        "db1": os.environ["DB1"],
        "db2": os.environ["DB2"],
    }
    print(f"dbs: {db_map}")

    with open(query_file, "r") as f:
        sql = f.read()
    print(f"file: {query_file}")

    with Timer() as timer:
        df = cx.read_sql2(sql, db_map, return_type="pandas")
    print("time in total:", timer.elapsed)

    print(df)
