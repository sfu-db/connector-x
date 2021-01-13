import time
import pandas as pd
from sqlalchemy import create_engine
from keys import *
# import tracemalloc

if __name__ == '__main__':
    # tracemalloc.start()
    engine = create_engine(PG_CONN)
    conn = engine.connect()
    then = time.time()
    df = pd.read_sql("select * from lineitem_s10;", conn)
    print("time", time.time() - then)
    conn.close()
    print(df)
    # _, peak = tracemalloc.get_traced_memory()
    # print(f"memory peak: {peak/10**9:.2f}G")
