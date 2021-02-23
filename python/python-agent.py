import os

from connector_agent_python import write_pandas

if __name__ == "__main__":
    conn = os.environ["POSTGRES_URL"]

    queries = [
        "select * from example where id < 1",
        "select * from example where id >= 1",
    ]

    schema = ["int64", "uint64"]
    df = write_pandas(conn, queries, schema)

    print(df)
