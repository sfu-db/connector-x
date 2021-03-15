"""
Usage:
  tpch-pyarrow.py

Options:
  -h --help     Show this screen.
  --version     Show version.
"""
import io
import os

from contexttimer import Timer
from pyarrow import csv
from sqlalchemy import create_engine
from docopt import docopt

if __name__ == "__main__":
    args = docopt(__doc__, version="1.0")
    conn = os.environ["POSTGRES_URL"]
    table = os.environ["POSTGRES_TABLE"]

    engine = create_engine(conn)
    conn = engine.connect()

    cur = conn.connection.cursor()
    store = io.BytesIO()
    with Timer() as timer:
        cur.copy_expert(
            f"COPY (SELECT * FROM {table}) TO STDOUT WITH CSV HEADER;", store
        )
    print(f"[Copy] {timer.elapsed:.2f}s")

    store.seek(0)

    with Timer() as timer:
        df = csv.read_csv(store, read_options=csv.ReadOptions(use_threads=False))
    print(f"[Read CSV] {timer.elapsed:.2f}s")

    with Timer() as timer:
        df = df.to_pandas()
        print(f"[To Pandas] {timer.elapsed:.2f}s")

    conn.close()
    print(df.head())
    # _, peak = tracemalloc.get_traced_memory()
    # print(f"memory peak: {peak/10**9:.2f}G")
