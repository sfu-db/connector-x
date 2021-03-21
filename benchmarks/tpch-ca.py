"""
Usage:
  tpch-ca.py [--protocol=<protocol>] <num>

Options:
  --protocol=<protocol>  The protocol to use [default: binary].
  -h --help              Show this screen.
  --version              Show version.
"""
import os

from connector_agent_python import read_sql
from contexttimer import Timer
from docopt import docopt

if __name__ == "__main__":
    args = docopt(__doc__, version="Naval Fate 2.0")
    conn = os.environ["POSTGRES_URL"]
    table = os.environ["POSTGRES_TABLE"]

    with Timer() as timer:
        df = read_sql(
            conn,
            f"""SELECT 
              l_orderkey,
              l_partkey,
              l_suppkey,
              l_linenumber,
              l_quantity::float8,
              l_extendedprice::float8,
              l_discount::float8,
              l_tax::float8,
              l_returnflag,
              l_linestatus,
              l_shipdate,
              l_commitdate,
              l_receiptdate,                
              l_shipinstruct,
              l_shipmode,
              l_comment
            FROM {table}""",
            partition_on="l_orderkey",
            partition_num=int(args["<num>"]),
            protocol=args["--protocol"],
        )
    print("time in total:", timer.elapsed)

    print(df.head())
    print(len(df))
