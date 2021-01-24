import io
import sys
import time
import numpy as np
import pandas as pd
import pyarrow as pa
from pyarrow import csv
from typing import Any, List
from sqlalchemy import create_engine
from multiprocessing import Pool
import multiprocessing
from keys import *

def func(sql: str) -> Any:
    engine = create_engine(PG_CONN)
    conn = engine.connect()
    cur = conn.connection.cursor()
    store = io.BytesIO()
    cur.copy_expert(f"COPY ({sql}) TO STDOUT WITH CSV HEADER;", store)
    store.seek(0)
    return csv.read_csv(store).to_pandas()

if __name__ == '__main__':
    multiprocessing.set_start_method('forkserver')
    t_num = int(sys.argv[1])
    sqls = get_sqls(t_num)
    print(f"numer of threads: {t_num}\nsqls: {sqls}")
    then = time.time()
    with Pool(t_num) as pool:
        dfs = pool.map(
            func,
            sqls,
        )
    print("concatenating", time.time() - then)
    print("blocks before:", dfs[0]._data)
    df = pd.concat(dfs)
    print("blocks after:", df._data)
    print("to pandas", time.time() - then)
    print(df)
    print(time.time() - then)
