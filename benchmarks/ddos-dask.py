"""
Usage:
  tpch-dask.py <num> [--conn=<conn>] [--table=<table>] [--index=<idx>] [--driver=<driver>]

Options:
  --conn=<conn>          The connection url to use [default: POSTGRES_URL].
  --table=<table>          The connection url to use [default: DDOS].
  --index=<idx>          The connection url to use [default: id].
  --driver=<driver>         The driver to use using sqlalchemy: https://docs.sqlalchemy.org/en/14/core/engines.html.
  -h --help     Show this screen.
  --version     Show version.

Drivers:
  PostgreSQL: postgresql, postgresql+psycopg2
  MySQL: mysql, mysql+mysqldb, mysql+pymysql
  Redshift: postgresql, redshift, redshift+psycopg2
"""

import os

import dask.dataframe as dd
from contexttimer import Timer
from docopt import docopt
from dask.distributed import Client, LocalCluster
from sqlalchemy.engine.url import make_url

if __name__ == "__main__":
    args = docopt(__doc__, version="Naval Fate 2.0")
    index_col = args["--index"]
    conn = os.environ[args["--conn"]]
    conn = make_url(conn)
    table = args["--table"]
    driver = args.get("--driver", None)
    npartition = int(args["<num>"])

    cluster = LocalCluster(n_workers=npartition, scheduler_port=0, memory_limit="230G")
    client = Client(cluster)

    # https://docs.sqlalchemy.org/en/13/core/engines.html#sqlite
    # 4 initial slashes is needed for Unix/Mac
    if conn.drivername == "sqlite":
        conn = f"sqlite:///{str(conn)[9:]}"
    elif driver is not None:
        conn = str(conn.set(drivername=driver))
    print(f"conn url: {conn}")

    with Timer() as timer:
        df = dd.read_sql_table(
            table,
            str(conn),
            index_col,
            npartitions=npartition,
            limits=(0, 7902474),
        ).compute()

    print(f"[Total] {timer.elapsed:.2f}s")

    print(df)
    print([(c, df[c].dtype) for c in df.columns])
