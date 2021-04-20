"""
Usage:
  tpch-ca.py [--protocol=<protocol>] <num>

Options:
  --protocol=<protocol>  The protocol to use [default: binary].
  -h --help              Show this screen.
  --version              Show version.
"""
from docopt import docopt
from contexttimer import Timer
import os
import sys
sys.path.append("../connectorx-python")
print(sys.path)

if __name__ == "__main__":
    import connectorx as cx

    args = docopt(__doc__, version="Naval Fate 2.0")
    conn = os.environ["POSTGRES_URL"]
    table = os.environ["POSTGRES_TABLE"]

    with Timer() as timer:
        df = cx.read_sql(
            conn,
            f"""SELECT * FROM {table}""",
            partition_on="l_orderkey",
            partition_num=int(args["<num>"]),
            protocol=args["--protocol"],
        )
    print("time in total:", timer.elapsed)

    print(df.head())
    print(len(df))
