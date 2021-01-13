import io
import sys
import time
import numpy as np
import pandas as pd
from typing import Any, List
from sqlalchemy import create_engine
from multiprocessing import Pool
import itertools
from keys import *

def func(sql: str, then: float) -> Any:
    engine = create_engine(PG_CONN)
    conn = engine.connect()
    cur = conn.connection.cursor()
    store = io.BytesIO()
    # cur.copy_expert(f"COPY ({sql}) TO STDOUT WITH CSV HEADER;", store)
    cur.copy_expert(f"COPY ({sql}) TO STDOUT WITH BINARY;", store)
    print("finish copy:", time.time()-then)
    store.seek(0)
    df = pd.read_csv(store)
    print("finish read_csv:", time.time()-then)
    return df

if __name__ == '__main__':
    t_num = int(sys.argv[1])
    sqls = get_sqls(t_num)
    print(f"numer of threads: {t_num}\nsqls: {sqls}")
    then = time.time()
    with Pool(t_num) as pool:
        dfs = pool.starmap(
            func,
            zip(sqls, itertools.repeat(then))
        )
    print("on main process", time.time() - then)
    df = pd.concat(dfs)
    print("finish concat", time.time() - then)
    print(df)
