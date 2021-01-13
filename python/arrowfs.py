import time
from gzip import decompress
from io import BytesIO
from multiprocessing import Pool
from typing import Any

from pyarrow import fs
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
    s3, path = fs.FileSystem.from_uri(f"s3://{BUCKET}")
    return paj.read_json(
        s3.open_input_stream(f"{path}/{key}"),
        parse_options=paj.ParseOptions(
            explicit_schema=SCHEMA,
        ),
        read_options=paj.ReadOptions(use_threads=False),
    )
    # with s3.open_input_stream(f"{path}/{key}") as file:
        # body = decompress(file.read())
        # body = file.read()
    # return paj.read_json(
        # BytesIO(body),
        # parse_options=paj.ParseOptions(
            # explicit_schema=SCHEMA,
        # ),
        # read_options=paj.ReadOptions(use_threads=False),
    # )


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
