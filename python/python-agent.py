import os

from connector_agent_python import write_pandas

if __name__ == "__main__":
    conn = os.environ["POSTGRES_URL"]

    queries = [
        "select * from example where id < 1",
        "select * from example where id >= 1",
    ]

    schema = ["uint64", "UInt64", "float64"]
    df = write_pandas(conn, queries, schema)

    print(df)
