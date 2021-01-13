import sys
import numpy as np
import connector_agent
import pyarrow as pa
import time
import json
from typing import List
from keys import *

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


if __name__ == '__main__':
    t_num = int(sys.argv[1])
    sqls = get_sqls(t_num)
    print(f"numer of threads: {t_num}\nsqls: {sqls}")

    then = time.time()
    table = connector_agent.read_pg(
        sqls,
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
