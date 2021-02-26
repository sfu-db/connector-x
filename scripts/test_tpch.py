from connector_agent_python import write_pandas
import os
from typing import List
import numpy as np


def get_sqls(count: int) -> List[str]:
    sqls = []
    split = np.linspace(0, 6000000, num=count + 1, endpoint=True, dtype=int)
    for i in range(len(split) - 1):
        sqls.append(
            f"""select * from lineitem where l_orderkey > {split[i]} and l_orderkey <= {split[i+1]}"""
        )
    return sqls


if __name__ == "__main__":
    # conn = os.environ["POSTGRES_URL"]
    conn = "postgres://postgres:postgres@localhost:6666/tpch"

    queries = get_sqls(1)

    schema = [
        "int64",
        "int64",
        "int64",
        "int64",
        "float64",
        "float64",
        "float64",
        "float64",
        "string",
        "string",
        "date",
        "date",
        "date",
        "string",
        "string",
        "string",
    ]
    df = write_pandas(conn, queries, schema, False)

    print(df.head())
    print(len(df))