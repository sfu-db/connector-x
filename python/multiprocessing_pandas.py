import time
from io import BytesIO
from multiprocessing import Pool
from typing import Any
from keys import *

import boto3
import pandas as pd


def func(key: str) -> Any:
    s3 = boto3.client("s3")
    payload = s3.get_object(Bucket=BUCKET, Key=key)
    body = payload["Body"].read()
    df = pd.read_json(BytesIO(body), lines=True, compression="gzip")
    return df


if __name__ == "__main__":
    then = time.time()
    with Pool(30) as pool:
        dfs = pool.map(
            func,
            KEYS,
        )
    print("concatenating", time.time() - then)
    df = pd.concat(dfs)
    print(df)
    print(time.time() - then)
