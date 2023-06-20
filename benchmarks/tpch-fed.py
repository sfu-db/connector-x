"""
Usage:
  tpch-fed.py [--file=<file>] [--dir=<dir>] [--runs=<runs>]

Options:
  --file=<file>          Query file.
  --dir=<dir>            Query path.
  --runs=<runs>          # runs [default: 1].
  -h --help              Show this screen.
  --version              Show version.
"""

import os
import sys
import time
import connectorx as cx
from contexttimer import Timer
from docopt import docopt
from pathlib import Path

def run_query_from_file(query_file):
    with open(query_file, "r") as f:
        sql = f.read()
    print(f"file: {query_file}")

    with Timer() as timer:
        df = cx.read_sql(db_map, sql, return_type="arrow")
    print(f"time in total: {timer.elapsed}, {len(df)} rows, {len(df.columns)} cols")
    print(df.schema)
    # print(df)
    sys.stdout.flush()
    del df

if __name__ == "__main__":
    args = docopt(__doc__, version="Naval Fate 2.0")
    query_file = args["--file"]

    db_map = {}
    db_conns = os.environ["FED_CONN"]
    for conn in db_conns.split(','):
        db_map[conn.split('=', 1)[0]] = conn.split('=', 1)[1] 

    print(f"dbs: {db_map}")

    for i in range(int(args["--runs"])):
        print(f"=============== run {i} ================")
        print()
        sys.stdout.flush()
        if args["--file"]:
            filename = args["--file"]
            run_query_from_file(filename)
        elif args["--dir"]:
            for filename in sorted(Path(args["--dir"]).glob("q*.sql")):
                run_query_from_file(filename)
                time.sleep(2)

