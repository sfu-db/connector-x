import time
from io import BytesIO

import boto3
import pandas as pd

from keys import *

if __name__ == "__main__":
    then = time.time()

    s3 = boto3.client("s3")

    dfs = []
    for obj in KEYS:
        payload = s3.get_object(Bucket=BUCKET, Key=obj)
        body = payload["Body"].read()
        dfs.append(pd.read_json(BytesIO(body), lines=True, compression="gzip"))
    df = pd.concat(dfs, axis=0)
    print(df)
    print(time.time() - then)
