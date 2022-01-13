"""
Usage:
    tpch-queries-cx.py <id> [--conn=<conn>] [--ret=<ret>] [--part=<part>] [--protocol=<protocol>] [--force-parallel]

Options:
    --ret=<ret>               The return type [default: pandas].
    --conn=<conn>             The connection url to use [default: POSTGRES_URL].
    --part=<part>             The number of partitions to use [default: 1].
    --protocol=<protocol>     The protocol to use [default: binary].
    --force-parallel          Force parallelism by setting variables
    -h --help                 Show this screen.
    --version                 Show version.
    """

import os

from pathlib import Path
from contexttimer import Timer
from docopt import docopt
import connectorx as cx

if __name__ == "__main__":
    args = docopt(__doc__, version="1.0")
    conn = os.environ[args["--conn"]]
    print(f"conn url: {conn}")

    ret = args["--ret"]
    print(f"return type: {ret}")

    qid = args["<id>"]
    print(f"execute query id: {qid}")

    part = int(args["--part"])
    print(f"# partitions: {part}")

    # multi_access_plan = "force_parallel" if args["--force-parallel"] else "default"
    # print(f"plan: {multi_access_plan}")

    if part > 1:
        qdir = Path(f"{os.environ['TPCH_QUERIES']}_part", f"q{qid}.sql")
        with open(qdir, "r") as f:
            part_col = f.readline()[:-1] # first line is partition key, remove last '\n'
            query = f.read()
    else:
        qdir = Path(os.environ["TPCH_QUERIES"], f"q{qid}.sql")
        with open(qdir, "r") as f:
            part_col = ""
            query = f.read()
    print(f"load query from: {qdir}")
    print(f"query: {query}")
    print(f"partition on : {part_col}")
    query = query.replace("%", "%%")

    with Timer() as timer:
        if ret == "pandas":
            if part > 1:
                # df = cx.read_sql(conn, query, partition_on=part_col, partition_num=part, protocol=args["--protocol"], multi_access_plan=multi_access_plan)
                df = cx.read_sql(conn, query, partition_on=part_col, partition_num=part, protocol=args["--protocol"])
            else:
                df = cx.read_sql(conn, query, protocol=args["--protocol"])
        elif ret == "arrow":
            if part > 1:
                # table = cx.read_sql(conn, query, return_type="arrow", partition_on=part_col, partition_num=part, protocol=args["--protocol"], multi_access_plan=multi_access_plan)
                table = cx.read_sql(conn, query, return_type="arrow", partition_on=part_col, partition_num=part, protocol=args["--protocol"])
            else:
                table = cx.read_sql(conn, query, return_type="arrow", protocol=args["--protocol"])
            print(f"get arrow table time: {timer.elapsed:.2f}s")
            df = table.to_pandas(split_blocks=False, date_as_object=False)
    print(f"[cx][QID: {qid} Total] {timer.elapsed:.2f}s")

    print(df)
