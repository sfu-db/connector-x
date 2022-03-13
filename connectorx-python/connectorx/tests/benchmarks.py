"""
This file is skipped during normal test because the file name is not started with benchmarks
"""
import os

from .. import read_sql


def read_sql_impl(conn: str, table: str):
    read_sql(
        conn,
        f"""SELECT * FROM {table}""",
        partition_on="L_ORDERKEY",
        partition_num=10,
    )


def bench_mysql(benchmark):
    benchmark(read_sql_impl, os.environ["MYSQL_URL"], os.environ["TPCH_TABLE"])


def bench_postgres(benchmark):
    benchmark(read_sql_impl,
              os.environ["POSTGRES_URL"], os.environ["TPCH_TABLE"])
