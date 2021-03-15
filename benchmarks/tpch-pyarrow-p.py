"""
Usage:
  tpch-pyarrow-p.py <num>

Options:
  -h --help     Show this screen.
  --version     Show version.
"""
import io
import itertools
import os
from multiprocessing import Pool
from typing import Any, List

import numpy as np
import pyarrow as pa
from contexttimer import Timer
from docopt import docopt
from pyarrow import csv
from sqlalchemy import create_engine


def get_sqls(table: str, count: int) -> List[str]:
    sqls = []
    split = np.linspace(0, 60000000, num=count + 1, endpoint=True, dtype=int)
    for i in range(len(split) - 1):

        sqls.append(
            f"""SELECT
                    l_orderkey,
                    l_partkey,
                    l_suppkey,
                    l_linenumber,
                    l_quantity::float8,
                    l_extendedprice::float8,
                    l_discount::float8,
                    l_tax::float8,
                    l_returnflag,
                    l_linestatus,
                    l_shipdate,
                    l_commitdate,
                    l_receiptdate,                
                    l_shipinstruct,
                    l_shipmode,
                    l_comment
                FROM {table} 
                WHERE l_orderkey > {split[i]} and l_orderkey <= {split[i+1]}"""
        )
    return sqls


def func(id: int, conn: str, query: str) -> Any:
    engine = create_engine(conn)
    conn = engine.connect()
    cur = conn.connection.cursor()
    store = io.BytesIO()

    with Timer() as timer:
        cur.copy_expert(f"COPY ({query}) TO STDOUT WITH CSV HEADER;", store)
    print(f"[Copy {id}] {timer.elapsed:.2f}s")

    store.seek(0)
    with Timer() as timer:
        df = csv.read_csv(store, read_options=csv.ReadOptions(use_threads=False))
    print(f"[Read CSV {id}] {timer.elapsed:.2f}s")

    return df


if __name__ == "__main__":
    args = docopt(__doc__, version="1.0")
    conn = os.environ["POSTGRES_URL"]
    table = os.environ["POSTGRES_TABLE"]

    queries = get_sqls(table, int(args["<num>"]))

    print(f"number of threads: {len(queries)}\nsqls: {queries}")

    with Timer() as timer, Pool(len(queries)) as pool:
        dfs = pool.starmap(
            func, zip(range(len(queries)), itertools.repeat(conn), queries)
        )

    print(f"[All Jobs] {timer.elapsed:.2f}s")

    with Timer() as timer:
        df = pa.concat_tables(dfs)
    print(f"[Concat] {timer.elapsed:.2f}s")

    with Timer() as timer:
        df = df.to_pandas()
    print(f"[To Pandas] {timer.elapsed:.2f}s")

    print(df.head())
