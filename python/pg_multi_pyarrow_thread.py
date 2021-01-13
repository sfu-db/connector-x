import io
import sys
import time
import numpy as np
import pandas as pd
import pyarrow as pa
from pyarrow import csv
from typing import Any, List
from sqlalchemy import create_engine
from multiprocessing.dummy import Pool as ThreadPool
import itertools
from keys import *

def func(sql: str, then: float) -> Any:
    engine = create_engine(PG_CONN)
    conn = engine.connect()
    cur = conn.connection.cursor()
    store = io.BytesIO()
    cur.copy_expert(f"COPY ({sql}) TO STDOUT WITH CSV HEADER;", store)
    print("finish copy:", time.time()-then)
    store.seek(0)
    # df = csv.read_csv(store, read_options=csv.ReadOptions(use_threads=False))
    df = csv.read_csv(store)
    print("finish read_csv:", time.time()-then)
    return df

if __name__ == '__main__':
    t_num = int(sys.argv[1])
    sqls = get_sqls(t_num)
    print(f"numer of threads: {t_num}\nsqls: {sqls}")
    then = time.time()
    with ThreadPool(t_num) as pool:
        dfs = pool.starmap(
            func,
            zip(sqls, itertools.repeat(then))
        )
    print("on main process", time.time() - then)
    df = pa.concat_tables(dfs)
    print("finish concat", time.time() - then)
    df = df.to_pandas()
    print("finish to_pandas", time.time() - then)
    print(df)
