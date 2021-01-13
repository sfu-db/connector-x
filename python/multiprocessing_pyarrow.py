import time
from gzip import decompress
from io import BytesIO
from multiprocessing import Pool
from typing import Any

import boto3
import pyarrow as pa
import pyarrow.json as paj

from keys import *

SCHEMA = pa.schema(
    [
        # pa.field(
        #     "asks",
        #     pa.list_(
        #         pa.struct(
        #             [
        #                 pa.field("type", pa.utf8()),
        #                 pa.field("price", pa.float64()),
        #                 pa.field("qty", pa.float64()),
        #             ]
        #         )
        #     ),
        #     False,
        # ),
        # pa.field(
        #     "bids",
        #     pa.list_(
        #         pa.struct(
        #             [
        #                 pa.field("type", pa.utf8()),
        #                 pa.field("price", pa.float64()),
        #                 pa.field("qty", pa.float64()),
        #             ]
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


def func(key: str) -> Any:
    s3 = boto3.client("s3")
    payload = s3.get_object(Bucket=BUCKET, Key=key)
    body = payload["Body"].read()
    body = decompress(body)
    return paj.read_json(
        BytesIO(body),
        parse_options=paj.ParseOptions(
            explicit_schema=SCHEMA,
        ),
        read_options=paj.ReadOptions(use_threads=False),
    )


if __name__ == "__main__":
    then = time.time()
    with Pool(30) as pool:
        dfs = pool.map(
            func,
            KEYS,
        )
    print("concatenating", time.time() - then)
    df = pa.concat_tables(dfs).to_pandas()
    print(df)
    print(time.time() - then)
