import io
import sys
import time
import gzip
import numpy as np
import pandas as pd
import pyarrow as pa
from pyarrow import csv
from typing import Any, List
from sqlalchemy import create_engine
from multiprocessing import Pool
import multiprocessing
import itertools
import tempfile
from keys import *

def func(i: int, sql: str, then: float) -> Any:
    engine = create_engine(PG_CONN)
    conn = engine.connect()
    cur = conn.connection.cursor()
    store = io.BytesIO()
    cur.copy_expert(f"COPY ({sql}) TO PROGRAM 'gzip -c' WITH CSV HEADER;", store)
    print("before csv parsing:", time.time()-then)
    store.seek(0)
    print(store.getvalue()[:100])
    with gzip.GzipFile(fileobj=store, mode='r') as dec_store:
        return csv.read_csv(dec_store)

# def func(i: int, sql: str, then: float) -> Any:
#     engine = create_engine('postgresql://postgres:postgres@localhost:6666/tpch')
#     conn = engine.connect()
#     cur = conn.connection.cursor()
#     with gzip.open(f'/tmp/lineitem{i}.csv.gz', 'wb') as store:
#         cur.copy_expert(f"COPY ({sql}) TO STDOUT WITH CSV HEADER;", store)
#         print("before csv parsing:", time.time()-then)
#         store.seek(0)
#         return csv.read_csv(store)

if __name__ == '__main__':
    # multiprocessing.set_start_method('forkserver')
    t_num = int(sys.argv[1])
    sqls = get_sqls(t_num)
    print(f"numer of threads: {t_num}\nsqls: {sqls}")
    then = time.time()
    with Pool(t_num) as pool:
        dfs = pool.starmap(
            func,
            zip(range(t_num), sqls, itertools.repeat(then))
        )
    print("concatenating", time.time() - then)
    df = pa.concat_tables(dfs)
    print("to pandas", time.time() - then)
    df = df.to_pandas()
    print(df)
    print(time.time() - then)
