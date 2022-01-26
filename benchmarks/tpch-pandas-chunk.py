"""
Usage:
    tpch-pandas-chunk.py [--conn=<conn>] [--csize=<csize>] [--driver=<driver>]

Options:
    --conn=<conn>             The connection url to use [default: POSTGRES].
    --csize=<csize>           Chunk size [default: 1000].
    --driver=<driver>         The driver to use using sqlalchemy: https://docs.sqlalchemy.org/en/14/core/engines.html.
    -h --help                 Show this screen.
    --version                 Show version.
"""

import os
from contexttimer import Timer
from docopt import docopt
import pandas as pd
from sqlalchemy import create_engine
from sqlalchemy.engine.url import make_url
import time

if __name__ == "__main__":
    args = docopt(__doc__, version="1.0")
    conn = os.environ[args["--conn"]]
    chunksize = int(args["--csize"])
    driver = args.get("--driver", None)
    conn = make_url(conn)
    if driver is not None:
        conn = conn.set(drivername=driver)
    if conn.drivername == "sqlite":
        conn = conn.set(database="/" + conn.database)
    print(f"chunksize: {chunksize}, conn url: {str(conn)}")

    with Timer() as timer:
        engine = create_engine(conn)
        conn = engine.connect().execution_options(
            stream_results=True, max_row_buffer=chunksize)
        dfs = []
        with Timer() as stream_timer:
            for df in pd.read_sql("SELECT * FROM lineitem",
                                  conn, parse_dates=[
                                      "l_shipdate",
                                      "l_commitdate",
                                      "l_receiptdate",
                                      "L_SHIPDATE",
                                      "L_COMMITDATE",
                                      "L_RECEIPTDATE",], chunksize=chunksize):
                dfs.append(df)
        print(f"time iterate batches: {stream_timer.elapsed}")
        df = pd.concat(dfs)
    print(f"time in total: {timer.elapsed}s")
    time.sleep(3) # capture peak memory

    conn.close()
    print(df)
    print(df.info(memory_usage="deep"))
    #  print(df._data.blocks)

    #  print("======")
    #  print(len(dfs))
    #  for d in dfs:
    #      print(d.info(memory_usage="deep"))
    #      print(d._data.blocks)
    #      break
