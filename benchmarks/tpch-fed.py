"""
Usage:
  tpch-fed.py [--file=<file>] [--dir=<dir>] [--runs=<runs>] [--print]

Options:
  --file=<file>          Query file.
  --dir=<dir>            Query path.
  --runs=<runs>          # runs [default: 1].
  --print                Print query result.
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

def run_query_from_file(query_file, doprint=False, ntries=0):
    with open(query_file, "r") as f:
        sql = f.read()
    print(f"file: {query_file}")

    try:
        with Timer() as timer:
            df = cx.read_sql(db_map, sql, return_type="arrow")
        print(f"time in total: {timer.elapsed:.2f}, {len(df)} rows, {len(df.columns)} cols")
        if doprint:
            print(df)
        del df
        # print(df.schema)
        # print(df)
    except RuntimeError as e:
        print(e)
        if ntries >= 5:
            raise
        print("retry in 10 seconds...")
        sys.stdout.flush()
        time.sleep(10)
        run_query_from_file(query_file, ntries+1)

    sys.stdout.flush()

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
            run_query_from_file(filename, args["--print"])
        elif args["--dir"]:
            for filename in sorted(Path(args["--dir"]).glob("q*.sql")):
                run_query_from_file(filename, args["--print"])
                time.sleep(2)

