import io
import sys
import time
import numpy as np
import pandas as pd
import pyarrow as pa
from pyarrow import csv
from typing import Any, List
from sqlalchemy import create_engine
import multiprocessing
from multiprocessing import Pool, Manager
import itertools
from keys import *

def func(i: int, sql: str, results: List[Any], then: float) -> None:
    engine = create_engine(PG_CONN)
    conn = engine.connect()
    cur = conn.connection.cursor()
    store = io.BytesIO()
    cur.copy_expert(f"COPY ({sql}) TO STDOUT WITH CSV HEADER;", store)
    print("finish copy:", time.time()-then)
    store.seek(0)
    results[i] = csv.read_csv(store)
    print("finish read_csv:", time.time()-then)

if __name__ == '__main__':
    print(multiprocessing.get_start_method())
    t_num = int(sys.argv[1])
    sqls = get_sqls(t_num)
    print(f"numer of threads: {t_num}\nsqls: {sqls}")

    then = time.time()
    manager = Manager()
    results = manager.dict()
    with Pool(t_num) as pool:
        pool.starmap(
            func,
            zip(range(t_num), sqls, itertools.repeat(results), itertools.repeat(then))
        )
    print("on main process", time.time() - then)
    df = pa.concat_tables(results.values())
    print("finish concat", time.time() - then)
    df = df.to_pandas()
    print("finish to_pandas", time.time() - then)
    print(df)
