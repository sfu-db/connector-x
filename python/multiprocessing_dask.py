import dask.dataframe as dd
import pandas as pd
import dask
from keys import *

import time


def json_engine(*args, **kwargs):
    df = pd.read_json(*args, **kwargs)
    df = df[
        [
            "change_id",
            "instrument_name",
            "timestamp",
            "type",
            "prev_change_id",
        ]
    ]
    return df


if __name__ == "__main__":
    then = time.time()
    dask.config.set(scheduler="multiprocessing")

    ddf = dd.read_json([f"s3://{BUCKET}/{k}" for k in KEYS], engine=json_engine)
    print(ddf.compute())
    print(time.time() - then)
