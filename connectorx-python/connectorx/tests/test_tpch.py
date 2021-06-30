import os

import pandas as pd
import time
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def tpch_mysql_url() -> str:
    conn = os.environ["TPCH_MYSQL_URL"]
    return conn


def tpch_cx(tpch_mysql_url: str) -> None:
    query = "select * from REGION limit 1"
    start = time.time()
    df = read_sql(tpch_mysql_url, query)
    end = time.time()
    expected = pd.DataFrame(
        index=range(1),
        data={
            "R_REGIONKEY": pd.Series([0], dtype="Int64"),
            "R_NAME": pd.Series(["AFRICA"], dtype="object"),
            "R_COMMENT": pd.Series(["lar deposits. blithely final packages cajole. regular waters are final requests. regular accounts are according to "], dtype="object"),
        }
    )
    print('------------------------')
    print(end-start)
    print('------------------------')
    # print("time in total:", timer.elapsed)
    assert_frame_equal(df, expected, check_names=True)
