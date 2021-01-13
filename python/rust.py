import connector_agent
import pyarrow as pa
import time
import json
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
        # pa.field(
        #     "asks",
        #     pa.list_(
        #         pa.field(
        #             "entry",
        #             pa.struct(
        #                 [
        #                     pa.field("type", pa.utf8(), False),
        #                     pa.field("price", pa.float64(), False),
        #                     pa.field("qty", pa.float64(), False),
        #                 ]
        #             ),
        #             False,
        #         )
        #     ),
        #     False,
        # ),
        # pa.field(
        #     "bids",
        #     pa.list_(
        #         pa.field(
        #             "entry",
        #             pa.struct(
        #                 [
        #                     pa.field("type", pa.utf8(), False),
        #                     pa.field("price", pa.float64(), False),
        #                     pa.field("qty", pa.float64(), False),
        #                 ]
        #             ),
        #             False,
        #         )
        #     ),
        #     False,
        # ),
        pa.field("change_id", pa.uint64(), False),
        pa.field("instrument_name", pa.string(), False),
        pa.field("timestamp", pa.uint64(), False),
        pa.field("type", pa.string(), False),
        pa.field("prev_change_id", pa.uint64(), True),
    ]
)

if __name__ == "__main__":
    then = time.time()
    table = connector_agent.read_s3(
        BUCKET,
        KEYS,
        json.dumps(schema_to_json(SCHEMA)),
        "JsonL",
    )

    tb = pa.Table.from_arrays(
        [
            pa.chunked_array([pa.Array._import_from_c(*ptr) for ptr in ptrs])
            for ptrs in table.values()
        ],
        names=list(table.keys()),
    )

    print(tb.to_pandas())
    print(time.time() - then)
