"""
Usage:
  tpch-rust-arrow.py <num>

Options:
  -h --help     Show this screen.
  --version     Show version.
"""
import json
import os
import time
from typing import List

import numpy as np
import pyarrow as pa
from connectorx import read_pg
from docopt import docopt


def get_sqls(table: str, count: int) -> List[str]:
    sqls = []
    split = np.linspace(0, 60000000, num=count + 1, endpoint=True, dtype=int)
    for i in range(len(split) - 1):

        sqls.append(
            f"""select  l_orderkey,
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
                l_comment from {table} where l_orderkey > {split[i]} and l_orderkey <= {split[i+1]}"""
        )
    return sqls


def field_to_json(field):
    json = {
        "name": field.name,
        "nullable": field.nullable,
    }
    if isinstance(field.type, pa.ListType):
        json = {
            **json,
            "type": {"name": "list"},
            "children": [field_to_json(field.type.value_field)],
        }
    elif field.type == pa.float64():
        json = {
            **json,
            "type": {"name": "floatingpoint", "precision": "DOUBLE"},
            "children": [],
        }
    elif field.type == pa.uint64():
        json = {
            **json,
            "type": {"name": "int", "bitWidth": 64, "isSigned": False},
            "children": [],
        }
    elif field.type == pa.string():
        json = {
            **json,
            "type": {"name": "utf8"},
            "children": [],
        }
    elif field.type == pa.date32():
        json = {
            **json,
            "type": {"name": "date", "unit": "DAY"},
            "children": [],
        }
    elif isinstance(field.type, pa.StructType):
        json = {
            **json,
            "type": {"name": "struct"},
            "children": [
                field_to_json(field.type[i]) for i in range(field.type.num_fields)
            ],
        }
    else:
        raise NotImplementedError(field.type)

    return json


def schema_to_json(schema):
    return {
        "fields": [field_to_json(schema.field(name)) for name in schema.names],
        "metadata": {},
    }


SCHEMA = pa.schema(
    [
        pa.field("l_orderkey", pa.uint64(), False),
        pa.field("l_partkey", pa.uint64(), False),
        pa.field("l_suppkey", pa.uint64(), False),
        pa.field("l_linenumber", pa.uint64(), False),
        pa.field("l_quantity", pa.float64(), False),
        pa.field("l_extendedprice", pa.float64(), False),
        pa.field("l_discount", pa.float64(), False),
        pa.field("l_tax", pa.float64(), False),
        pa.field("l_returnflag", pa.string(), False),
        pa.field("l_linestatus", pa.string(), False),
        # pa.field("l_shipdate", pa.date32(), False),
        # pa.field("l_commitdate", pa.date32(), False),
        # pa.field("l_receiptdate", pa.date32(), False),
        pa.field("l_shipdate", pa.string(), False),
        pa.field("l_commitdate", pa.string(), False),
        pa.field("l_receiptdate", pa.string(), False),
        pa.field("l_shipinstruct", pa.string(), False),
        pa.field("l_shipmode", pa.string(), False),
        pa.field("l_comment", pa.string(), False),
    ]
)


if __name__ == "__main__":
    args = docopt(__doc__, version="1.0")
    conn = os.environ["POSTGRES_URL"]
    table = os.environ["POSTGRES_TABLE"]

    queries = get_sqls(table, int(args["<num>"]))

    print(f"numer of threads: {int(args['<num>'])}\nsqls: {queries}")

    then = time.time()
    table = read_pg(
        conn,
        queries,
        json.dumps(schema_to_json(SCHEMA)),
    )
    print(f"finish read_pg:", time.time() - then)

    tb = pa.Table.from_arrays(
        [
            pa.chunked_array([pa.Array._import_from_c(*ptr) for ptr in ptrs])
            for ptrs in table.values()
        ],
        names=list(table.keys()),
    )
    print("finish concat:", time.time() - then)

    df = tb.to_pandas()
    print("finish to_pandas:", time.time() - then)
    print(df)
