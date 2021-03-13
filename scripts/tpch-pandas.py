import os

from contexttimer import Timer
from sqlalchemy import create_engine

import pandas as pd

if __name__ == "__main__":
    conn = os.environ["POSTGRES_URL"]
    table = os.environ["POSTGRES_TABLE"]

    engine = create_engine(conn)
    conn = engine.connect()

    with Timer() as timer:
        df = pd.read_sql(
            f"SELECT * FROM {table}",
            conn,
            parse_dates=[
                "l_shipdate",
                "l_commitdate",
                "l_receiptdate",
            ],
        )
    print(f"[Total] {timer.elapsed:.2f}s")

    conn.close()
    print(df.head())
